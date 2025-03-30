default:
    @just --list

coverage_report:
    @echo -e "\e[3;33mWarning:\e[0m \e[3;2mNightly needs to be used for branch coverage!\n"
    cargo +nightly llvm-cov --all-features --workspace --branch --html
    @echo -e "Coverage report can be found at \e[1m./target/llvm-cov/html\e[0m"

