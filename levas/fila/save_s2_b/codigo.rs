// ---------- %%LEVA%%a leva: o save: as regras FIFA de transferência (gravar e carregar) e o resumo do save (Setup.cpp, file_screens.cpp) ----------

/// A caixa "Error" das regras FIFA: o texto "v%s %s.%s %d" (versão 0x90d130, arquivo e extensão de
/// Setup.cpp, linha) em 0xae24d0, a caixa 0x5e8290 e [0xb67a34] = 0. Como o `show_error_at`, mas com os
/// argumentos que o sprintf da original lê da pilha (nome, extensão e linha) passados explícitos.
unsafe fn setup_warning(linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa801dc, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s, nome.as_ptr(), ext.as_ptr(), linha);
    caixa(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
}

/// { 0x8395b0 } (cdecl, Setup.cpp): `save_fifa_transfer_rules(save)`: o último item do save,
/// `fifa_transfer_rules.zip` (0x9326e0 gravando, a4 0x16, tipo 4; no contêiner já criado vale o
/// cabeçalho dele): 2 bytes, [0xae233c] e [0xa83608], e `zclose`. Caminho nulo, item que não abre
/// ou escrita que falha: a caixa "Error" (Setup.cpp 0x3b24, 0x3b2c, 0x3b32, 0x3b38); nas escritas,
/// depois da caixa o item é fechado. A original não devolve nada (`eax` sobra).
unsafe extern "C" fn save_fifa_transfer_rules(save: usize) {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let gravar: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(0x932e10usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    crate::comparison::branch("save gravar regras fifa");
    if save == 0 {
        setup_warning(0x3b24);
        return;
    }
    let mut z = abrir_item(save, 0xa82518, 1, 0x16, 4);
    if z == 0 {
        setup_warning(0x3b2c);
        return;
    }
    if gravar(0xae233c, 1, 1, z) != 1 {
        setup_warning(0x3b32);
        fechar_item(&mut z);
        return;
    }
    if gravar(0xa83608, 1, 1, z) != 1 {
        setup_warning(0x3b38);
        fechar_item(&mut z);
        return;
    }
    fechar_item(&mut z);
}

// ponytail: acrescenta um item ao contêiner; não roda duas vezes. Só a original; a cópia leva o
// ramo e é provada pela prova do save (o .qst gravado com e sem a leva).
unsafe extern "C" fn compare_save_fifa_transfer_rules(save: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(save)
}

/// { 0x839760 } (cdecl, Setup.cpp): `load_fifa_transfer_rules(save)`: lê os 2 bytes do item
/// `fifa_transfer_rules.zip` (0x9326e0 lendo) em [0xae233c] e [0xa83608]; se [0xae233c] = 1, a
/// regra em vigor [0xa83608] vira 5. Fecha o item. Erros: a caixa "Error" (Setup.cpp 0x3b4f, 0x3b57,
/// 0x3b5d, 0x3b63), como na gravação.
unsafe extern "C" fn load_fifa_transfer_rules(save: usize) {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let ler_item: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(0x932c30usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    crate::comparison::branch("save carregar regras fifa");
    if save == 0 {
        setup_warning(0x3b4f);
        return;
    }
    let mut z = abrir_item(save, 0xa82518, 0, 0x16, 4);
    if z == 0 {
        setup_warning(0x3b57);
        return;
    }
    if ler_item(0xae233c, 1, 1, z) != 1 {
        setup_warning(0x3b5d);
        fechar_item(&mut z);
        return;
    }
    if ler_item(0xa83608, 1, 1, z) != 1 {
        setup_warning(0x3b63);
        fechar_item(&mut z);
        return;
    }
    if *(0xae233c as *const u8) == 1 {
        *(0xa83608 as *mut u8) = 5;
    }
    fechar_item(&mut z);
}

// ponytail: abre e lê o contêiner (aloca o ZFILE) e só roda na carga; segue a regra do bloco do save
// (arquivo = só a original). A cópia leva o ramo; a etapa de carregar da prova do save a exercita.
unsafe extern "C" fn compare_load_fifa_transfer_rules(save: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I1%%)(save)
}

/// O `strcat` da original (repne scasb + rep movs): `fonte` com o zero no fim de `destino`.
unsafe fn append_text(destino: *mut u8, fonte: *const u8) {
    let d = destino.add(core::ffi::CStr::from_ptr(destino as *const core::ffi::c_char).to_bytes().len());
    copy_text(d, fonte);
}

/// O `strcpy` da original, com o zero.
unsafe fn copy_text(destino: *mut u8, fonte: *const u8) {
    let n = core::ffi::CStr::from_ptr(fonte as *const core::ffi::c_char).to_bytes_with_nul().len();
    core::ptr::copy(fonte, destino, n);
}

/// { 0x591f80 } (cdecl, file_screens.cpp): `build_save_game_summary(resumo)`: o resumo que o
/// `save_game_data` grava no `general.dat` (logo depois da versão):
/// - +0: as nações da tabela de ligas (0xb63d60, 0x23 de 0x48 bytes) com o bit 2 em +0x11c, a
///   primeira pelo sprintf com o próprio nome como formato (0x944c9f), as outras por ", <%s>"
///   traduzido (0x66f4e0); +1000 (0x3e8): o mesmo com o bit 1;
/// - 16 técnicos, do último da tabela de pessoas (0xae23bc, 0x6e bytes) para trás: humano (+0x5f
///   = 1) dá o nome em +0x7e4 + 0x78 × i (0x53dba0, 200) e em +0x820 + 0x78 × i "clube / nação"
///   (+0x39 e +0x24, nome em +0x38 de cada), só um deles, ou nada; não humano só zera o nome (o
///   clube fica como estava no buffer do chamador);
/// - +0xf64 a data do jogo (0x417c10 de 0xae2c90), +0x7d0 a versão (0x90d130), +0xf6c o tempo de
///   jogo ([0xdd7140] += time() − [0xdd7144]; [0xdd7144] = time()) e +0xf70 o time().
unsafe extern "C" fn build_save_game_summary(resumo: *mut u8) {
    let formatar: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let nome_do: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(0x53dba0usize);
    let copia_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let versao: unsafe extern "C" fn() -> *const u8 = core::mem::transmute(0x90d130usize);
    let relogio: unsafe extern "C" fn(*mut i32) -> i32 = core::mem::transmute(0x946ebbusize);
    let mut nacao = [0u8; 100];
    *resumo = 0;
    *resumo.add(0x3e8) = 0;
    for (bit, destino) in [(2u8, resumo), (1u8, resumo.add(0x3e8))] {
        let mut primeira = true;
        for i in 0..0x23usize {
            let n = *((0xb63d60 + i * 0x48) as *const usize);
            if n == 0 || *((n + 0x11c) as *const u8) & bit == 0 {
                continue;
            }
            if primeira {
                formatar(nacao.as_mut_ptr(), n + 0x38);
                primeira = false;
            } else {
                traduzir(nacao.as_mut_ptr(), 0x9c8134, n + 0x38);
            }
            append_text(destino, nacao.as_ptr());
        }
        if bit == 1 && primeira {
            *resumo.add(0x3e8) = 0;
        }
    }
    let quantos = *(0xae2364 as *const i32);
    let tabela = *(0xae23bc as *const usize);
    for i in 0..0x10i32 {
        let clube_txt = resumo.add(0x820 + 0x78 * i as usize);
        let p = tabela.wrapping_add((quantos - i - 1).wrapping_mul(0x6e) as usize);
        if *((p + 0x5f) as *const u8) != 1 {
            *clube_txt.sub(0x3c) = 0;
            continue;
        }
        nome_do(p, clube_txt.sub(0x3c), 200, 1);
        let clube = core::ptr::read_unaligned((p + 0x39) as *const usize);
        let nacao_do = core::ptr::read_unaligned((p + 0x24) as *const usize);
        if clube != 0 && nacao_do != 0 {
            traduzir(clube_txt, 0x9c80f4, clube + 0x38, nacao_do + 0x38);
        } else if clube != 0 {
            copy_text(clube_txt, (clube + 0x38) as *const u8);
        } else if nacao_do != 0 {
            copy_text(clube_txt, (nacao_do + 0x38) as *const u8);
        } else {
            *clube_txt = 0;
        }
    }
    let mut copia = [0u8; 8];
    copia_data(resumo as usize + 0xf64, copia.as_mut_ptr(), 0xae2c90);
    copy_text(resumo.add(0x7d0), versao());
    let mut agora = 0i32;
    let t = relogio(&mut agora);
    let total = (*(0xdd7140 as *const i32)).wrapping_add(t.wrapping_sub(*(0xdd7144 as *const i32)));
    *(0xdd7140 as *mut i32) = total;
    core::ptr::write_unaligned(resumo.add(0xf6c) as *mut i32, total);
    *(0xdd7144 as *mut i32) = t;
    let t2 = relogio(core::ptr::null_mut());
    core::ptr::write_unaligned(resumo.add(0xf70) as *mut i32, t2);
}

// Lado a lado sobre o resumo inteiro (0xf74) e o relógio do tempo de jogo ([0xdd7140], [0xdd7144]),
// restaurados entre as duas; compara só +0..+0xf6c: o tempo de jogo e o time() (+0xf6c, +0xf70) são
// relógio (fixos só no modo simulação), como na prova do save.
unsafe extern "C" fn compare_build_save_game_summary(resumo: *mut u8) {
    let regioes = [(resumo as usize, 0xf6c), (resumo as usize + 0xf6c, 8), (0xdd7140, 8)];
    let (_, _, _, mo, mr) = crate::comparison::side_by_side(&regioes,
        || original_fn::<unsafe extern "C" fn(*mut u8)>(%%I2%%)(resumo),
        || build_save_game_summary(resumo));
    SCOREBOARD.record(mo.0[0] == mr.0[0], || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let k = (0..a.len()).find(|&k| a[k] != b[k]).unwrap_or(0);
        format!("build_save_game_summary: primeira diferença em +{:#x}: {:#04x} x {:#04x}", k, a[k], b[k])
    });
}
