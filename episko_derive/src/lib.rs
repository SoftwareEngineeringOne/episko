#![deny(clippy::pedantic)]
use deluxe::ExtractAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Type};

// Attribute structs using deluxe's derive capabilities
#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(db))]
struct DbStructAttr {
    table: String,
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(db))]
struct DbFieldAttr {
    col: String,
}

struct FieldInfo {
    ident: Ident,
    attr: DbFieldAttr,
    ty: Type,
}

struct DbObjectMeta {
    ident: syn::Ident,
    generics: syn::Generics,
    table: String,
    fields: Vec<FieldInfo>,
    id_type: Type,
    id_field_ident: Ident,
}

impl DbObjectMeta {
    fn from_derive_input(mut input: DeriveInput) -> deluxe::Result<Self> {
        // Extract struct-level attributes
        let DbStructAttr { table } = DbStructAttr::extract_attributes(&mut input)?;

        // Extract field information
        let fields: Vec<_> = if let syn::Data::Struct(s) = &mut input.data {
            s.fields
                .iter_mut()
                .filter_map(|field| {
                    let ident = field.ident.clone()?;
                    DbFieldAttr::extract_attributes(field)
                        .ok()
                        .map(|attr| FieldInfo {
                            ident,
                            attr,
                            ty: field.ty.clone(),
                        })
                })
                .collect()
        } else {
            return Err(deluxe::Error::new_spanned(
                input,
                "DatabaseObject can only be derived for structs",
            ));
        };

        let id_field_info = fields.iter().find(|f| f.attr.col == "id").ok_or_else(|| {
            deluxe::Error::new_spanned(&input, "No field with #[db(col = \"id\")] found")
        })?;
        let id_field_ident = id_field_info.ident.clone();

        // Find ID field
        let id_type = fields
            .iter()
            .find(|f| f.attr.col == "id")
            .ok_or_else(|| {
                deluxe::Error::new_spanned(&input, "No field with #[db(col = \"id\")] found")
            })?
            .ty
            .clone();

        let ident = input.ident.clone();
        let generics = input.generics.clone();

        Ok(Self {
            ident,
            generics,
            table,
            fields,
            id_type,
            id_field_ident,
        })
    }

    fn generate_impl(&self) -> TokenStream {
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();
        let ident = &self.ident;

        let columns = self.fields.iter().map(|f| &f.attr.col);
        let binds = self.fields.iter().map(|f| &f.ident);
        let id_type = &self.id_type;

        let insert_sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table,
            columns
                .clone()
                .map(::std::string::String::as_str)
                .collect::<Vec<_>>()
                .join(", "),
            std::iter::repeat("?")
                .take(columns.len())
                .collect::<Vec<_>>()
                .join(", ")
        );

        let select_sql = format!(
            "SELECT {} FROM {} WHERE id = ?",
            columns
                .map(::std::string::String::as_str)
                .collect::<Vec<_>>()
                .join(", "),
            self.table
        );

        let exists_sql = format!("SELECT EXISTS(SELECT 1 FROM {} WHERE id = ?)", self.table);
        let exists_sql_literal = syn::LitStr::new(&exists_sql, proc_macro2::Span::call_site());
        let id_field_ident = self.id_field_ident.clone();

        let remove_sql = format!("DELETE FROM {} WHERE id = ?", self.table);
        let remove_sql_literal = syn::LitStr::new(&remove_sql, proc_macro2::Span::call_site());

        quote! {
            impl #impl_generics crate::database::DatabaseObject for #ident #type_generics #where_clause {
                type Id = #id_type;

                fn write_to_db<'e>(
                    &'e self,
                    executor: impl ::sqlx::SqliteExecutor<'e> + 'e,
                ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = crate::database::Result<()>> + Send + 'e>> {
                    Box::pin(async move {
                        ::sqlx::query(#insert_sql)
                            #(
                                .bind(&self.#binds)
                            )*
                            .execute(executor)
                            .await?;
                        Ok(())
                    })
                }

                fn from_db<'e>(
                    id: Self::Id,
                    executor: impl ::sqlx::SqliteExecutor<'e> + 'e,
                ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = crate::database::Result<Self>> + Send + 'e>> {
                    Box::pin(async move {
                        Ok(::sqlx::query_as::<_, Self>(#select_sql)
                            .bind(id)
                            .fetch_one(executor)
                            .await?)
                    })
                }

                fn exists<'e>(
                    &'e self,
                    executor: impl ::sqlx::SqliteExecutor<'e> + 'e,
                ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = crate::database::Result<bool>>
                    + Send + 'e>> {
                    Box::pin(async move {
                        let exists: bool = ::sqlx::query_scalar(#exists_sql_literal)
                            .bind(&self.#id_field_ident)
                            .fetch_one(executor)
                            .await?;
                        Ok(exists)
                    })
                }

                fn remove_from_db<'e>(
                    &'e self,
                    executor: impl ::sqlx::SqliteExecutor<'e> + 'e,
                ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = crate::database::Result<()>>
                    + Send + 'e>> {
                    Box::pin(async move {
                        ::sqlx::query(#remove_sql_literal)
                            .bind(&self.#id_field_ident)
                            .execute(executor)
                            .await?;
                        Ok(())
                    })
                }
            }
        }
    }
}

/// Macro for easily creating [`DatabaseObjects`] which
/// can be written to/read from a sqlite database.
///
/// # Example
///
///
/// # Panics
///
/// The macro will panic during compilation when:
/// - no `#[db(table = "table"]` is assigned
/// - no `#[db(col = "id")]` is assigned,
/// - any other invalid statements are set.
#[proc_macro_derive(DatabaseObject, attributes(db))]
pub fn db_object_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);

    DbObjectMeta::from_derive_input(syn::parse2(input).unwrap())
        .map_or_else(syn::Error::into_compile_error, |meta| meta.generate_impl())
        .into()
}
