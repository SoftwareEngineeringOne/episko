use episko_lib::metadata::{BuildSystem, Ide, Language, Metadata, property::Property as _};
use rand::Rng;

pub fn generate_metadata(num: usize) -> Metadata {
    pub const IDES: [&str; 4] = ["VSCode", "IntelliJ", "Sublime", "Vim"];
    pub const CATEGORIES: [&str; 5] = ["Web", "CLI", "GUI", "Embedded", "AI"];
    pub const LANGUAGES: [&str; 5] = ["Rust", "Python", "JavaScript", "Go", "C++"];
    pub const BUILD_SYSTEMS: [&str; 5] = ["Cargo", "Make", "CMake", "NPM", "Bazel"];

    let mut rng = rand::rng();
    let with_ide = rng.random_bool(1.0 / 2.0);

    let categories_amount = rng.random_range(0..=5);
    let languages_amount = rng.random_range(0..=5);
    let build_systems_amount = rng.random_range(0..=5);

    let mut builder = Metadata::builder()
        .title(&format!("Test Project {}", num + 1))
        .directory("/tmp");

    if with_ide {
        let ide = rng.random_range(0..4);
        builder = builder.preferred_ide(Ide::new(IDES[ide]));
    }

    for i in 0..categories_amount {
        builder = builder.add_category(CATEGORIES[i]);
    }

    for i in 0..languages_amount {
        builder = builder.add_language(Language::new(LANGUAGES[i]));
    }

    for i in 0..build_systems_amount {
        builder = builder.add_build_system(BuildSystem::new(BUILD_SYSTEMS[i]));
    }

    if num % 2 == 0 {
        builder = builder.description(&format!("This is test project {}", num + 1));
    }

    builder.build().expect("create test metadata instance")
}
