use camino::Utf8PathBuf;

pub mod cli;



pub fn create( data: Option<String> ) {
    // TODO: Get all information from the user in a interactive mode or given data and create a new manifest
    match data {
        None => todo!(),
        Some(data) => todo!()
    }
    if (data.is_none()) {
        // Interactive mode
    } else {
        // Non-interactive mode
    }
}

pub fn remove( file: Utf8PathBuf ) {
    // TODO: Call the method to remove the file
}

pub fn add( file: Utf8PathBuf ) {
    // TODO: Call to add the file to the system
}

pub fn validate( file: Utf8PathBuf ) {
    // TODO: Call to validate a given file
}
