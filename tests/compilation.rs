extern crate compiletest_rs as ct;

use std::path::PathBuf;

fn test_compile(mode: &str) {
    let mut config = ct::default_config();

    config.mode = mode.parse().unwrap();
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    config.target_rustcflags = Some("\
        -L target/debug \
        -L target/debug/deps \
    ".into());

    ct::run_tests(&config);
}

#[test]
fn compilation_errors() {
    test_compile("compile-fail");
}

#[test]
fn compilation_ok() {
    test_compile("run-pass");
}
