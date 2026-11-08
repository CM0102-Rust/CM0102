// Exporta DirectDrawCreate sem a decoracao stdcall (_DirectDrawCreate@12), como o cm0102.exe importa.
//
// Also generates OUT_DIR/indices.rs: one constant per hooked routine, `IDX_<NAME>` = its position in the tables of
// `motor::instalar` (the `rust` list), so `orig::<F>(IDX_ADD_TRANSFER_OFFER)` names the original it calls instead of a
// bare number. Read from src/motor.rs on every build, so it can't go out of sync.
// [pt] Gera também OUT_DIR/indices.rs: uma constante por rotina desviada, `IDX_<NOME>` = a posição dela nas tabelas.
fn main() {
    let raiz = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let def = raiz.join("ddraw.def");
    println!("cargo:rustc-cdylib-link-arg=/DEF:{}", def.display());
    println!("cargo:rerun-if-changed=ddraw.def");

    let motor = raiz.join("src").join("motor.rs");
    println!("cargo:rerun-if-changed={}", motor.display());
    let s = std::fs::read_to_string(&motor).unwrap();
    let i = s.find("let rust: [usize; N] = [").expect("rust list in motor.rs");
    let fim = i + s[i..].find("];").unwrap();
    let mut saida = String::new();
    let mut k = 0usize;
    for linha in s[i..fim].lines().skip(1) {
        let l = linha.trim();
        if let Some(nome) = l.strip_suffix(" as *const () as usize,") {
            saida.push_str(&format!("pub(crate) const IDX_{}: usize = {};\n", nome.to_uppercase(), k));
            k += 1;
        }
    }
    std::fs::write(std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("indices.rs"), saida).unwrap();
}
