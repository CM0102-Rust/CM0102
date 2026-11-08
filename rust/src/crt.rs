//! Bridges to the game's own routines that the Rust versions still call (CRT and SI utilities).
//!
//! [pt] Pontes para rotinas do proprio jogo que as versoes Rust ainda chamam (CRT e utilitarios da SI).

/// The game's standard error message (SI's "assert"), in the same call sequence as the original:
///   _splitpath(source, 0, 0, name, ext); version(); sprintf(0xae24d0, "v%s %s.%s %d", version, name, ext, line);
///   box("Error", 0xae24d0, 0); [0xb67a34] = 0.
/// `source` is the address of the .cpp path string in the exe.
///
/// [pt] Mensagem de erro padrao do jogo (o "assert" da SI), na mesma sequencia de chamadas da original:
///   _splitpath(fonte, 0, 0, nome, ext); versao(); sprintf(0xae24d0, "v%s %s.%s %d", versao, nome, ext, linha);
///   caixa("Error", 0xae24d0, 0); [0xb67a34] = 0.
/// `fonte` e o endereco da string do caminho do .cpp no exe.
pub unsafe fn error_box(source: usize, line: u32) {
    let splitpath: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let version: unsafe extern "C" fn() -> usize = core::mem::transmute(0x90d130usize);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let message_box: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5e8290usize);
    let (mut name, mut ext) = ([0u8; 0x100], [0u8; 0x100]);
    splitpath(source, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version();
    sprintf(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), line);
    message_box(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
}

/// The game CRT's atexit (used by the initialization of C++ local static variables).
///
/// [pt] atexit da CRT do jogo (usado pela inicializacao de variaveis estaticas locais do C++).
pub unsafe fn atexit(f: usize) {
    let g: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x94627ausize);
    g(f);
}
