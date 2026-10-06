//! Pontes para rotinas do proprio jogo que as versoes Rust ainda chamam (CRT e utilitarios da SI).

/// Mensagem de erro padrao do jogo (o "assert" da SI), na mesma sequencia de chamadas da original:
///   _splitpath(fonte, 0, 0, nome, ext); versao(); sprintf(0xae24d0, "v%s %s.%s %d", versao, nome, ext, linha);
///   caixa("Error", 0xae24d0, 0); [0xb67a34] = 0.
/// `fonte` e o endereco da string do caminho do .cpp no exe.
pub unsafe fn erro(fonte: usize, linha: u32) {
    let splitpath: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn() -> usize = core::mem::transmute(0x90d130usize);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 0x100], [0u8; 0x100]);
    splitpath(fonte, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let v = versao();
    sprintf(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), linha);
    caixa(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
}

/// atexit da CRT do jogo (usado pela inicializacao de variaveis estaticas locais do C++).
pub unsafe fn atexit(f: usize) {
    let g: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x94627ausize);
    g(f);
}
