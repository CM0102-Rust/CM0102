// Exporta DirectDrawCreate sem a decoracao stdcall, com o mesmo .def da ddraw.dll do projeto.
fn main() {
    let def = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("..").join("rust").join("ddraw.def");
    println!("cargo:rustc-cdylib-link-arg=/DEF:{}", def.display());
    println!("cargo:rerun-if-changed=../rust/ddraw.def");
}
