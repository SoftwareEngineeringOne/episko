// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![deny(clippy::pedantic)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use env_logger::Env;

#[tokio::main]
async fn main() {
    let env = Env::default()
        .filter_or("EPISKO_LOG_LEVEL", "trace")
        .write_style_or("EPISKO_LOG_STYLE", "auto");

    env_logger::init_from_env(env);
    gui_lib::run().await.expect("starting application");
}

#[cfg(test)]
mod tests {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[test]
    fn sanity_test() {
        let a = 1;
        let b = 2;
        let sum = 1 + 2;

        assert_eq!(add(a, b), sum);
    }
}
