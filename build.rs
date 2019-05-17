extern crate cc;

fn main() {
    println!("cargo:rerun-if-env-changed=UNICODE");
    let mut cfg = cc::Build::new();
    if std::env::var_os("UNICODE").is_some() {
        cfg.define("UNICODE", None);
    }
    cfg.file("src/val.cpp")
        .cpp(true)
        .compile("libval.a");
}
