// ---------- 219a leva: o save: os filtros da caixa de arquivos (o jogo salvo, o início rápido e a versão) (Setup.cpp) ----------

/// { 0x838e10 } (cdecl, Setup.cpp): `check_saved_game_file(caminho)`: filtro da caixa de arquivos
/// dos jogos salvos: 1 se o contêiner tem o id 0x16 (`.sav`, 0x9331c0), senão 2. A original só
/// acerta `ax`: os 16 bits altos de `eax` ficam os da resposta do id, como aqui. Caminho nulo é o
/// erro de Setup.cpp 0x3a1b (caixa "Error"): a cópia chama a original.
unsafe extern "C" fn filtro_do_jogo_salvo(caminho: usize) -> u32 {
    if caminho == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u32>(565)(caminho);
    }
    let id_do_conteiner: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x9331c0usize);
    let id = id_do_conteiner(caminho);
    (id & 0xffff0000) | if id != 0x16 { 2 } else { 1 }
}

// O id só lê o arquivo e libera o que aloca: as duas rodam lado a lado. Caminho nulo (caixa de
// erro) só a original.
unsafe extern "C" fn comparar_filtro_do_jogo_salvo(caminho: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(565)(caminho);
    if caminho == 0 {
        return r;
    }
    let n = filtro_do_jogo_salvo(caminho);
    PLACAR.registrar(r == n, || format!("filtro_do_jogo_salvo {:#x} x {:#x}", r, n));
    r
}

/// { 0x839290 } (cdecl, Setup.cpp): `check_quick_start_file(caminho)`: filtro da caixa de arquivos
/// do início rápido: 1 se o contêiner tem o id 0x17 (`.qst`) e a versão gravada é a do exe
/// (0x838eb0), senão 2. Os 16 bits altos de `eax` são os da última chamada (o id, ou a versão).
/// Caminho nulo: erro de Setup.cpp 0x3a6d, a cópia chama a original.
unsafe extern "C" fn filtro_do_inicio_rapido(caminho: usize) -> u32 {
    if caminho == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u32>(566)(caminho);
    }
    let id_do_conteiner: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x9331c0usize);
    let versao_valida: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x838eb0usize);
    crate::comparacao::ramo("save filtro do inicio rapido");
    let id = id_do_conteiner(caminho);
    if id != 0x17 {
        return (id & 0xffff0000) | 2;
    }
    let v = versao_valida(caminho);
    (v & 0xffff0000) | if v != 0 { 1 } else { 2 }
}

// ponytail: a versão (0x838eb0) abre o general.dat e não o fecha quando dá certo (o ZFILE e o
// arquivo ficam abertos, como na original): rodar duas vezes deixaria dois. Só a original; a cópia
// leva o ramo e roda no modo Rust (só a caixa de arquivos chama).
unsafe extern "C" fn comparar_filtro_do_inicio_rapido(caminho: usize) -> u32 {
    orig::<unsafe extern "C" fn(usize) -> u32>(566)(caminho)
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
unsafe extern "C" fn versao_do_jogo_salvo_valida(caminho: usize) -> u32 {
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
unsafe extern "C" fn comparar_versao_do_jogo_salvo_valida(caminho: usize) -> u32 {
    orig::<unsafe extern "C" fn(usize) -> u32>(567)(caminho)
}

// ---------- 219a leva: o save: as regras FIFA de transferência (gravar e carregar) e o resumo do save (Setup.cpp, file_screens.cpp) ----------

/// A caixa "Error" das regras FIFA: o texto "v%s %s.%s %d" (versão 0x90d130, arquivo e extensão de
/// Setup.cpp, linha) em 0xae24d0, a caixa 0x5e8290 e [0xb67a34] = 0. Como o `aviso_em`, mas com os
/// argumentos que o sprintf da original lê da pilha (nome, extensão e linha) passados explícitos.
unsafe fn aviso_do_setup(linha: u32) {
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
unsafe extern "C" fn gravar_regras_fifa(save: usize) {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let gravar: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(0x932e10usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    crate::comparacao::ramo("save gravar regras fifa");
    if save == 0 {
        aviso_do_setup(0x3b24);
        return;
    }
    let mut z = abrir_item(save, 0xa82518, 1, 0x16, 4);
    if z == 0 {
        aviso_do_setup(0x3b2c);
        return;
    }
    if gravar(0xae233c, 1, 1, z) != 1 {
        aviso_do_setup(0x3b32);
        fechar_item(&mut z);
        return;
    }
    if gravar(0xa83608, 1, 1, z) != 1 {
        aviso_do_setup(0x3b38);
        fechar_item(&mut z);
        return;
    }
    fechar_item(&mut z);
}

// ponytail: acrescenta um item ao contêiner; não roda duas vezes. Só a original; a cópia leva o
// ramo e é provada pela prova do save (o .qst gravado com e sem a leva).
unsafe extern "C" fn comparar_gravar_regras_fifa(save: usize) {
    orig::<unsafe extern "C" fn(usize)>(568)(save)
}

/// { 0x839760 } (cdecl, Setup.cpp): `load_fifa_transfer_rules(save)`: lê os 2 bytes do item
/// `fifa_transfer_rules.zip` (0x9326e0 lendo) em [0xae233c] e [0xa83608]; se [0xae233c] = 1, a
/// regra em vigor [0xa83608] vira 5. Fecha o item. Erros: a caixa "Error" (Setup.cpp 0x3b4f, 0x3b57,
/// 0x3b5d, 0x3b63), como na gravação.
unsafe extern "C" fn carregar_regras_fifa(save: usize) {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let ler_item: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(0x932c30usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    crate::comparacao::ramo("save carregar regras fifa");
    if save == 0 {
        aviso_do_setup(0x3b4f);
        return;
    }
    let mut z = abrir_item(save, 0xa82518, 0, 0x16, 4);
    if z == 0 {
        aviso_do_setup(0x3b57);
        return;
    }
    if ler_item(0xae233c, 1, 1, z) != 1 {
        aviso_do_setup(0x3b5d);
        fechar_item(&mut z);
        return;
    }
    if ler_item(0xa83608, 1, 1, z) != 1 {
        aviso_do_setup(0x3b63);
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
unsafe extern "C" fn comparar_carregar_regras_fifa(save: usize) {
    orig::<unsafe extern "C" fn(usize)>(569)(save)
}

/// O `strcat` da original (repne scasb + rep movs): `fonte` com o zero no fim de `destino`.
unsafe fn emendar_texto(destino: *mut u8, fonte: *const u8) {
    let d = destino.add(core::ffi::CStr::from_ptr(destino as *const core::ffi::c_char).to_bytes().len());
    copiar_texto(d, fonte);
}

/// O `strcpy` da original, com o zero.
unsafe fn copiar_texto(destino: *mut u8, fonte: *const u8) {
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
unsafe extern "C" fn montar_resumo_do_save(resumo: *mut u8) {
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
            emendar_texto(destino, nacao.as_ptr());
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
            copiar_texto(clube_txt, (clube + 0x38) as *const u8);
        } else if nacao_do != 0 {
            copiar_texto(clube_txt, (nacao_do + 0x38) as *const u8);
        } else {
            *clube_txt = 0;
        }
    }
    let mut copia = [0u8; 8];
    copia_data(resumo as usize + 0xf64, copia.as_mut_ptr(), 0xae2c90);
    copiar_texto(resumo.add(0x7d0), versao());
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
unsafe extern "C" fn comparar_montar_resumo_do_save(resumo: *mut u8) {
    let regioes = [(resumo as usize, 0xf6c), (resumo as usize + 0xf6c, 8), (0xdd7140, 8)];
    let (_, _, _, mo, mr) = crate::comparacao::lado_a_lado(&regioes,
        || orig::<unsafe extern "C" fn(*mut u8)>(570)(resumo),
        || montar_resumo_do_save(resumo));
    PLACAR.registrar(mo.0[0] == mr.0[0], || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let k = (0..a.len()).find(|&k| a[k] != b[k]).unwrap_or(0);
        format!("montar_resumo_do_save: primeira diferença em +{:#x}: {:#04x} x {:#04x}", k, a[k], b[k])
    });
}

// ---------- 219a leva: o save: as ligações com o banco (gravar e carregar) e liberar os dados do jogo (Database.cpp, Setup.cpp) ----------

/// { 0x524d30 } (cdecl, Database.cpp): `save_database_for_game(save, a4, tipo)`: o passo 1 do
/// `save_game_data`: `save_database(save, 1, a4, tipo)` (0x52dad0, para o contêiner), devolvendo a
/// resposta dela. DESLIGADA (DESLIGADAS no motor.rs): o save_database grava no index.dat 408 bytes de
/// locais sem valor (lixo de ponteiro, a partir de +0x5cb); chamado desta cópia ele roda na pilha
/// particular e o lixo gravado muda (prova do save da 219a). Volta quando a cópia reproduzir o resíduo.
unsafe extern "C" fn gravar_banco_no_save(save: usize, a4: u32, tipo: u32) -> u32 {
    let gravar_banco: unsafe extern "C" fn(usize, u32, u32, u32) -> u32 = core::mem::transmute(0x52dad0usize);
    crate::comparacao::ramo("save gravar banco");
    gravar_banco(save, 1, a4, tipo)
}

// ponytail: grava as tabelas do banco no contêiner; não roda duas vezes. Só a original; a cópia
// leva o ramo e é provada pela prova do save.
unsafe extern "C" fn comparar_gravar_banco_no_save(save: usize, a4: u32, tipo: u32) -> u32 {
    orig::<unsafe extern "C" fn(usize, u32, u32) -> u32>(571)(save, a4, tipo)
}

/// { 0x524d10 } (cdecl, Database.cpp): `load_database_from_stream(save, compartilhado)`: o passo 2
/// do `load_game_data`: `load_database(save, compartilhado, 1)` (0x528540, do contêiner), devolvendo
/// a resposta dela (1 = certo).
unsafe extern "C" fn carregar_banco_do_save(save: usize, compartilhado: u32) -> u32 {
    let carregar_banco: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x528540usize);
    crate::comparacao::ramo("save carregar banco");
    carregar_banco(save, compartilhado, 1)
}

// ponytail: carrega o banco inteiro do contêiner (aloca as tabelas); não roda duas vezes. Só a
// original; a cópia leva o ramo (a etapa de carregar da prova do save a exercita).
unsafe extern "C" fn comparar_carregar_banco_do_save(save: usize, compartilhado: u32) -> u32 {
    orig::<unsafe extern "C" fn(usize, u32) -> u32>(572)(save, compartilhado)
}

/// { 0x82ff40 } (cdecl, Setup.cpp): `free_game_data(tela)`: o gerente de telas (0xb74450) solta as
/// telas (0x7ff5f0) e, se `tela` ≠ 0, mais 0x7fc5e0; 0x6d5b50; depois cada subsistema guardado num
/// global, na ordem da original: destrutor (thiscall) + `delete` (0x944c94) e o global zerado; os
/// vetores soltos pelo `free` (0x9452ca); dois objetos pelo destrutor virtual (vtable[0](1)); os
/// `*.tmp` da pasta temporária (0x90cd50), [0xae2338], as tabelas do banco (0x528080), a data do
/// jogo (0x54a130: 1, 0, 2001, −1) e três marcas zeradas. Devolve 1.
unsafe extern "C" fn liberar_dados_do_jogo(tela: u32) -> u32 {
    let soltar_telas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7ff5f0usize);
    let soltar_mais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7fc5e0usize);
    let antes: unsafe extern "C" fn() = core::mem::transmute(0x6d5b50usize);
    let competicoes: unsafe extern "C" fn() = core::mem::transmute(0x669290usize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let livre: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let limpar_temporarios: unsafe extern "C" fn(usize) = core::mem::transmute(0x90cd50usize);
    let liberar_tabelas: unsafe extern "C" fn() = core::mem::transmute(0x528080usize);
    let acertar_data: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32) -> usize = core::mem::transmute(0x54a130usize);
    crate::comparacao::ramo("save liberar dados do jogo");
    // destrutor + delete + global zerado
    let objetos = |lista: &[(usize, usize)]| {
        for &(g, destrutor) in lista {
            let p = *(g as *const usize);
            if p != 0 {
                let d: unsafe extern "thiscall" fn(usize) = core::mem::transmute(destrutor);
                d(p);
                apagar(p);
                *(g as *mut usize) = 0;
            }
        }
    };
    // free + global zerado
    let soltos = |lista: &[usize]| {
        for &g in lista {
            let p = *(g as *const usize);
            if p != 0 {
                livre(p);
                *(g as *mut usize) = 0;
            }
        }
    };
    // destrutor virtual (vtable[0], com 1 = apagar) + global zerado
    let virtual_ = |g: usize| {
        let p = *(g as *const usize);
        if p != 0 {
            let d: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(**(p as *const *const usize));
            d(p, 1);
            *(g as *mut usize) = 0;
        }
    };
    soltar_telas(0xb74450);
    if tela != 0 {
        soltar_mais(0xb74450);
    }
    antes();
    objetos(&[(0xb67a50, 0x6b4190)]);
    let p = *(0xadadfc as *const usize);
    if p != 0 {
        livre(p);
        *(0xadadfc as *mut usize) = 0;
        *(0xadadf8 as *mut usize) = 0;
    }
    competicoes();
    objetos(&[(0xb6b7c8, 0x72b200), (0xae2a60, 0x5af6a0), (0xb67a44, 0x68e0e0), (0xad9da8, 0x442e20),
              (0xad9dac, 0x4448a0), (0xde1d08, 0x8961e0), (0xae28f0, 0x555790), (0xde1f44, 0x8b1070),
              (0xad9c60, 0x413410), (0xde1f54, 0x8bbe20), (0xb63d28, 0x62a8a0), (0xae2a3c, 0x5a5120),
              (0xad9c6c, 0x4185f0), (0xae2a30, 0x5698d0), (0xb7433c, 0x794ba0), (0xdd7ef4, 0x85b7f0)]);
    virtual_(0xae2c74);
    objetos(&[(0xb74318, 0x76a710), (0xde2f70, 0x90d2e0), (0xde1f58, 0x903570), (0xae1000, 0x5a8f60),
              (0xdd7f00, 0x8687c0), (0xb6b7c0, 0x7293c0), (0xb74350, 0x7c0d00), (0xb5e134, 0x58e330),
              (0xae2a58, 0x5aa0d0), (0xb63c98, 0x5fb730), (0xb74448, 0x7f6aa0), (0xb7431c, 0x785d00)]);
    // [0xdd7ef0]: só o delete, sem destrutor
    let p = *(0xdd7ef0 as *const usize);
    if p != 0 {
        apagar(p);
        *(0xdd7ef0 as *mut usize) = 0;
    }
    virtual_(0xb74434);
    objetos(&[(0xae1050, 0x4c74f0), (0xae2a38, 0x59a5a0)]);
    soltos(&[0xae2d0c, 0xae2d14, 0xae2d1c, 0xae2d20, 0xae2d24]);
    objetos(&[(0xb7434c, 0x7a56f0), (0xb74340, 0x79b800), (0xdd7ee4, 0x84be10), (0xde2f74, 0x9227d0),
              (0xb74430, 0x7da690)]);
    limpar_temporarios(0xa813ac);
    soltos(&[0xae2338]);
    liberar_tabelas();
    acertar_data(0xae2c90, 1, 0, 0x7d1, -1);
    *(0xae2c88 as *mut u8) = 0;
    *(0xdd711c as *mut u32) = 0;
    *(0xdd7130 as *mut u32) = 0;
    1
}

// ponytail: destrói todos os subsistemas e apaga os *.tmp; não roda duas vezes. Só a original; a
// cópia leva o ramo e roda no modo Rust (abertura da carreira, fim do início rápido).
unsafe extern "C" fn comparar_liberar_dados_do_jogo(tela: u32) -> u32 {
    orig::<unsafe extern "C" fn(u32) -> u32>(573)(tela)
}
