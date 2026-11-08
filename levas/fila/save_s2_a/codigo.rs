// ---------- %%LEVA%%a leva: o save: os filtros da caixa de arquivos (o jogo salvo, o início rápido e a versão) (Setup.cpp) ----------

/// { 0x838e10 } (cdecl, Setup.cpp): `check_saved_game_file(caminho)`: filtro da caixa de arquivos
/// dos jogos salvos: 1 se o contêiner tem o id 0x16 (`.sav`, 0x9331c0), senão 2. A original só
/// acerta `ax`: os 16 bits altos de `eax` ficam os da resposta do id, como aqui. Caminho nulo é o
/// erro de Setup.cpp 0x3a1b (caixa "Error"): a cópia chama a original.
unsafe extern "C" fn check_saved_game_file(caminho: usize) -> u32 {
    if caminho == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(caminho);
    }
    let zget_archive_id: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x9331c0usize);
    let id = zget_archive_id(caminho);
    (id & 0xffff0000) | if id != 0x16 { 2 } else { 1 }
}

// O id só lê o arquivo e libera o que aloca: as duas rodam lado a lado. Caminho nulo (caixa de
// erro) só a original.
unsafe extern "C" fn comparar_check_saved_game_file(caminho: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(caminho);
    if caminho == 0 {
        return r;
    }
    let n = check_saved_game_file(caminho);
    PLACAR.registrar(r == n, || format!("check_saved_game_file {:#x} x {:#x}", r, n));
    r
}

/// { 0x839290 } (cdecl, Setup.cpp): `check_quick_start_file(caminho)`: filtro da caixa de arquivos
/// do início rápido: 1 se o contêiner tem o id 0x17 (`.qst`) e a versão gravada é a do exe
/// (0x838eb0), senão 2. Os 16 bits altos de `eax` são os da última chamada (o id, ou a versão).
/// Caminho nulo: erro de Setup.cpp 0x3a6d, a cópia chama a original.
unsafe extern "C" fn check_quick_start_file(caminho: usize) -> u32 {
    if caminho == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(caminho);
    }
    let zget_archive_id: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x9331c0usize);
    let versao_valida: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x838eb0usize);
    crate::comparacao::ramo("save filtro do inicio rapido");
    let id = zget_archive_id(caminho);
    if id != 0x17 {
        return (id & 0xffff0000) | 2;
    }
    let v = versao_valida(caminho);
    (v & 0xffff0000) | if v != 0 { 1 } else { 2 }
}

// ponytail: a versão (0x838eb0) abre o general.dat e não o fecha quando dá certo (o ZFILE e o
// arquivo ficam abertos, como na original): rodar duas vezes deixaria dois. Só a original; a cópia
// leva o ramo e roda no modo Rust (só a caixa de arquivos chama).
unsafe extern "C" fn comparar_check_quick_start_file(caminho: usize) -> u32 {
    orig::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(caminho)
}

/// A caixa "Load Game Error" de `valid_saved_game_version`: o texto "v%s %s.%s %d (%s)" (versão
/// 0x90d130, arquivo e extensão de Setup.cpp, linha, motivo) em 0xae24d0, o título traduzido em
/// 0xae28bc, a mensagem traduzida (0x9af334) em 0xae26c4 e a caixa 0x5e8290. Ao contrário do
/// `aviso_em`, não zera [0xb67a34].
unsafe fn erro_de_carga_do_setup(linha: u32, motivo: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa801dc, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    // a 0x90d130 ignora os argumentos (devolve a versão); o sprintf da original lê nome, extensão,
    // linha e motivo da pilha que ficou da chamada: aqui vão explícitos
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, motivo);
    traduzir(0xae28bc, 0x9af3e0);
    traduzir(0xae26c4, 0x9af334, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// { 0x838eb0 } (cdecl, Setup.cpp): `valid_saved_game_version(caminho)`: abre o item `general.dat`
/// do contêiner (0x9326e0 lendo, a4 0x15, tipo 4), lê a versão gravada (4 bytes) e o resumo do save
/// (0xf70 bytes se |versão| < 0x3e027, senão 0xf74) e devolve 1 se o texto da versão do resumo
/// (+0x7d0) é igual ao do exe (0x90d130), senão 0. Cada leitura que falha dá a caixa "Load Game
/// Error" (Setup.cpp 0x3a36, 0x3a3d, 0x3a48, 0x3a55) e 0. Como a original, quando dá certo o item
/// NÃO é fechado (o ZFILE e o arquivo ficam abertos). O quadro SEH da original só protege a data
/// local (0x54a110, sem destrutor), que cai dentro do resumo (+0xf64) e é sobrescrita pela leitura.
unsafe extern "C" fn valid_saved_game_version(caminho: usize) -> u32 {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let ler_item: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(0x932c30usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    let versao: unsafe extern "C" fn() -> *const u8 = core::mem::transmute(0x90d130usize);
    crate::comparacao::ramo("save versao do jogo salvo");
    let mut resumo = [0u8; 0xf74];
    let mut v = 0i32;
    data_padrao(resumo.as_mut_ptr() as usize + 0xf64);
    let mut z = abrir_item(caminho, 0x9c7b84, 0, 0x15, 4);
    if z == 0 {
        erro_de_carga_do_setup(0x3a36, 0xa82494);
        return 0;
    }
    if ler_item(&mut v as *mut i32 as usize, 4, 1, z) != 1 {
        erro_de_carga_do_setup(0x3a3d, 0xa8244c);
        fechar_item(&mut z);
        return 0;
    }
    if v.wrapping_abs() < 0x3e027 {
        if ler_item(resumo.as_mut_ptr() as usize, 0xf70, 1, z) != 1 {
            erro_de_carga_do_setup(0x3a48, 0xa82408);
            fechar_item(&mut z);
            return 0;
        }
        // o resumo antigo não tem o último campo (+0xf70): a original o zera na pilha
        resumo[0xf70..0xf74].fill(0);
    } else if ler_item(resumo.as_mut_ptr() as usize, 0xf74, 1, z) != 1 {
        erro_de_carga_do_setup(0x3a55, 0xa82408);
        fechar_item(&mut z);
        return 0;
    }
    let exe = core::ffi::CStr::from_ptr(versao() as *const core::ffi::c_char).to_bytes();
    let gravada = core::ffi::CStr::from_ptr(resumo.as_ptr().add(0x7d0) as *const core::ffi::c_char).to_bytes();
    (exe == gravada) as u32
}

// ponytail: abre o item e, quando dá certo, não o fecha (o ZFILE e o arquivo ficam abertos, como na
// original); nos erros, a caixa. Não roda duas vezes: só a original; a cópia leva o ramo e roda no
// modo Rust (só a caixa de arquivos, pelo filtro do início rápido, chama).
unsafe extern "C" fn comparar_valid_saved_game_version(caminho: usize) -> u32 {
    orig::<unsafe extern "C" fn(usize) -> u32>(%%I2%%)(caminho)
}
