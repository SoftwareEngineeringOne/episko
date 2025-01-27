use std::error::Error;

use episkos_lib::metadata::{property::Property, Language};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Creating Language...");
    let language1 = Language::new("Rust", Some("1.84"));
    let language2 = Language::new("rust", Some("1.84"));
    let language3 = Language::new("Rust", None);

    println!("Language 1: {:#?}", language1);
    println!("Language 2: {:#?}", language2);
    println!("Language 3: {:#?}", language3);

    println!("Canonical 1: {}", language1.canonical());
    println!("Canonical 2: {}", language2.canonical());
    println!("Canonical 3: {}", language3.canonical());

    println!("Id 1: {:x?}", language1.generate_id());
    println!("Id 2: {:x?}", language2.generate_id());
    println!("Id 3: {:x?}", language3.generate_id());
    // let categories = vec![Category::new("Cool"), Category::new("University")];
    // let languages = vec![
    //     Language::new("Rust", "1.84"),
    //     Language::new("Typescript", ""),
    // ];
    // let build_systems = vec![
    //     BuildSystem::new("Cargo", "1.83"),
    //     BuildSystem::new("Bun", "1.1.43"),
    // ];
    // let ide = Ide::new("neovim");
    // let metadata = Metadata::builder()
    //     .title("Hello, World!")
    //     .directory(Path::new("./"))?
    //     .languages(languages)
    //     .categories(categories)
    //     .build_systems(build_systems)
    //     .preffered_ide(ide)
    //     .description("SoftwareEngineering Project")
    //     .build()?;
    //
    // metadata.write_file(&metadata.directory())?;
    //
    // let read_metadata = Metadata::from_file(Path::new("."))?;

    Ok(())
}
