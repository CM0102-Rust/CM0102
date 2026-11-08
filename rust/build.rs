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
    // A fixed image base below every fixed region of the allocator (0x0b000000 DLL region, 0x0ff00000 private stack,
    // 0x10000000 visible region, 0x18000000..0x58000000 game heap) and no ASLR: each new build is a "new" image to
    // Windows, and after hundreds of builds since boot ASLR placed it inside the game heap region, whose reservation
    // then failed (non-reproducible simulation, 08/11/2026). The machine code does not change, only where it is loaded.
    // [pt] Base fixa abaixo das regiões fixas do alocador e sem ASLR (o ASLR passou a pôr a DLL dentro da região da heap).
    println!("cargo:rustc-cdylib-link-arg=/BASE:0x05000000");
    println!("cargo:rustc-cdylib-link-arg=/DYNAMICBASE:NO");

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
