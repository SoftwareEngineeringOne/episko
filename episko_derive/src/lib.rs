//! This crate contains the derive Macros for the [`DatabaseObject`] trait
//! from [`episko_lib`].
//!
//! # Example
//! ```ignore
//! #[derive(DatabaseObject)]
//! #[db(table = "ExampleProperty")] // Required!
//! struct ExampleProperty {
//!     #[db(col = "id")] // Column with name "id" is required
//!     id: i32,
//!     #[db(col = "name")] // Any other columns are optional
//!     name: String,
//!     #[db(col = "version")]
//!     version: Option<String>
//! }
//!
//! // Implementations for DatabaseObject will now be generated according
//! // to the specified attributes.
//! ```
#![deny(clippy::pedantic)]
use deluxe::ExtractAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Type};

/// Attributes for the struct level `#[db(..)]` macro.
#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(db))]
struct DbStructAttr {
    /// Specifies the name for the relevant table
    table: String,
}

/// Attributes for the field level `#[db(..)]` macro.
#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(db))]
struct DbFieldAttr {
    /// Specifies the column name of the field
    col: String,
}

/// Stores information about a field within the relevant struct.
struct FieldInfo {
    ident: Ident,
    attr: DbFieldAttr,
    ty: Type,
}

/// Main struct relating to the macro target struct.
struct DbObjectMeta {
    /// Ident of the struct itself
    ident: syn::Ident,
    /// Generics of the struct, used for each generated "impl"
    generics: syn::Generics,
    /// The table name provided via the `#[db(table = "...")]` macro
    table: String,
    /// The fields of the struct along with their information
    fields: Vec<FieldInfo>,
    /// The type of the field marked with `#[db(col = "id")]`
    id_type: Type,
    /// The ident of the field marked with `#[db(col = "id")]`
    id_field_ident: Ident,
}

impl DbObjectMeta {
    /// Create [`DbObjectMeta`] based on the given [`DeriveInput`].
    ///
    /// # Errors
    /// - [`deluxe::Result`] when any of the parsing steps fail
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

    /// Generates the necessary `impl` block for the target struct.
    #[allow(clippy::too_many_lines)]
    fn generate_impl(&self) -> TokenStream {
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();
        let ident = &self.ident;

        let columns = self.fields.iter().map(|f| &f.attr.col);
        let binds = self.fields.iter().map(|f| &f.ident);
        let id_type = &self.id_type;

        let insert_sql = format!(
            "INSERT OR IGNORE INTO {} ({}) VALUES ({})",
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

        let all_sql = format!(
            "SELECT id, name, null as version FROM {} GROUP BY name",
            self.table
        );
        let all_sql_literal = syn::LitStr::new(&all_sql, proc_macro2::Span::call_site());

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

                fn all_names<'e>(
                    executor: impl ::sqlx::SqliteExecutor<'e> + 'e,
                ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = crate::database::Result<Vec<Self>>>
                    + Send + 'e>> {
                    Box::pin(async move {
                        Ok(::sqlx::query_as::<_, Self>(#all_sql_literal)
                                .fetch_all(executor)
                                .await?
                        )
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
/// # Panics
///
/// The macro will panic during compilation of the parent crate when:
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
