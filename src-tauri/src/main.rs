// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    app_lib::run();
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
