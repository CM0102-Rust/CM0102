// ---------- %%LEVA%%a leva: o jogo com jogador observado, a lista do olheiro do humano e o registro da primeira convocação (scout_manager.cpp, staff_records.cpp) ----------

unsafe fn cps8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// { 0x7f9720 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::fixture_has_scouted_player(partida)` (pela
/// partida_sem_humano 0x6b66a0): competição (+0) negativa, 0. Um clube da lista vigiada (LLIST estática 0xb74438,
/// cursor reposto; +0 do item) igual a +0xc ou +0x10 da partida, 1. Depois os pedidos de observação ([this+0x5c],
/// 0x11 bytes, a pessoa em +4, conta [this+0x64], relidos a cada passo): a pessoa cujo clube (+0x39) ou seleção (+0x24)
/// é um dos dois da partida (+0x1c primeiro, depois +0x20), que não está cedida (0x8c7560, fora de seleção 0x53b900),
/// nem lesionada (0x62c400 = 2), nem suspensa (0x55d270), nem impedida (0x7757d0), e que ainda não jogou (0x8cf5e0
/// = 0), dá 1. Senão 0.
unsafe extern "thiscall" fn jogo_com_observado(this: usize, partida: usize) -> u32 {
    let (c1, c2) = (cps8_d(partida + 0x1c), cps8_d(partida + 0x20));
    if cps8_i(partida) < 0 {
        crate::comparacao::ramo("cps_8 competicao negativa");
        return 0; // cps8: competição negativa
    }
    core::ptr::write_unaligned(0xb74442 as *mut u32, cps8_d(0xb7443a) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut e = proximo(0xb74438);
    while e != 0 {
        let v = cps8_d(e);
        if cps8_d(partida + 0xc) == v || cps8_d(partida + 0x10) == v {
            return 1;
        }
        e = proximo(0xb74438);
    }
    if cps8_d(this + 0x5c) == 0 || cps8_i(this + 0x64) <= 0 {
        crate::comparacao::ramo("cps_8 sem pedidos");
        return 0; // cps8: sem pedidos de observação
    }
    let selecao: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x53b900usize);
    let cedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let impedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x7757d0usize);
    let ja_jogou: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x8cf5e0usize);
    let mut k = 0i32;
    while k < cps8_i(this + 0x64) {
        let id = cps8_i(cps8_d(this + 0x5c).wrapping_add(k as usize * 0x11) + 4);
        k += 1;
        if id <= -1 {
            continue;
        }
        let p = cps8_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
        let (clube, sel) = (cps8_d(p + 0x39), cps8_d(p + 0x24));
        let x = if c1 == clube || c1 == sel {
            c1
        } else if c2 == clube || c2 == sel {
            c2
        } else {
            continue;
        };
        if selecao(x) == 0 && cedido(cps8_d(0xde1f54), p, x, 0) != 0 {
            continue;
        }
        if lesao(cps8_d(0xb63d28), p) == 2 {
            continue;
        }
        if suspenso(cps8_d(0xae28f0), p, cps8_d(partida + 0x14), 1, 0) != 0 {
            continue;
        }
        if impedido(cps8_d(0xb74318), p, 0xae2c90, x, 0, 0) != 0 {
            continue;
        }
        if ja_jogou(cps8_d(0xde1f54), p, partida) == 0 {
            return 1;
        }
    }
    0
}

unsafe extern "thiscall" fn comparar_jogo_com_observado(this: usize, partida: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, partida);
    let n = jogo_com_observado(this, partida);
    PLACAR.registrar(r == n, || format!("jogo_com_observado(partida {:x}) original={} rust={}", partida, r, n));
    r
}

/// O gerente do humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]).
unsafe fn cps8_gerente_da_vez() -> usize {
    let k = core::ptr::read_unaligned(0xb77c7a as *const i16) as isize;
    cps8_d((0xb74c26isize + 0x300 * k) as usize)
}

/// { 0x7f7490 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::is_player_shortlisted(pessoa, gerente)` (pelos
/// clubes interessados 0x842180, pelo estatuto 0x849d60, pela 0x8cde70 e por telas): pessoa nula, a caixa "Error"
/// (linha 0x231) e 0; sem gerente, o do humano da vez (sem ele, a caixa da linha 0x23b e 0). A vaga do humano (byte
/// +0 do gerente − byte de [0xae2364] + 16, com sinal, 0..15): com a edição aberta ([this + vaga × 4 + 0xc]), a
/// resposta dela (0x804540); senão a primeira das 10 buscas (0x7f6cd0) de tipo 1 (+0xa) responde (0x7af600). Sem ela, 0.
unsafe extern "thiscall" fn na_lista_do_olheiro(this: usize, pessoa: usize, gerente: usize) -> u32 {
    if pessoa == 0 {
        aviso_em(0xa7b094, 0x231);
        return 0;
    }
    let m = if gerente != 0 { gerente } else { cps8_gerente_da_vez() };
    if m == 0 {
        aviso_em(0xa7b094, 0x23b);
        return 0;
    }
    let vaga = (*(m as *const u8)).wrapping_sub(*(0xae2364 as *const u8)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&vaga) {
        return 0;
    }
    let s = cps8_d(this + vaga as usize * 4 + 0xc);
    if s != 0 {
        let da_edicao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x804540usize);
        return da_edicao(s, pessoa);
    }
    // ponytail: a original empurra um dword local com o índice só no byte baixo; a 0x7f6cd0 lê só esse byte
    let busca: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    let tem: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7af600usize);
    for i in 0..10u32 {
        let l = busca(this, i, m);
        if l != 0 && *((l + 0xa) as *const u8) == 1 {
            let r = tem(l, pessoa);
            crate::comparacao::ramo("cps_8 lista do olheiro");
            return r; // cps8: a resposta da lista do olheiro
        }
    }
    0
}

unsafe extern "thiscall" fn comparar_na_lista_do_olheiro(this: usize, pessoa: usize, gerente: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    if pessoa == 0 || (gerente == 0 && cps8_gerente_da_vez() == 0) {
        return o(this, pessoa, gerente);
    }
    let r = o(this, pessoa, gerente);
    let n = na_lista_do_olheiro(this, pessoa, gerente);
    PLACAR.registrar(r == n, || format!("na_lista_do_olheiro(pessoa {:x}, gerente {:x}) original={} rust={}", pessoa, gerente, r, n));
    r
}

/// { 0x86cc50 } (thiscall, ret 8, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_first_cap_record(pessoa, adversário)` (pelo resultado internacional 0x76b600): o histórico da
/// pessoa (tipo 6: o clube contratante +0x39 ou −1, o id do adversário na partida e a idade +0x18 com sinal), no arquivo (0x58e340).
/// O clube é lido antes do teste da pessoa, como na original; pessoa nula: a caixa "Error" (linha 0x1010).
unsafe extern "thiscall" fn registrar_a_primeira_convocacao(this: usize, pessoa: usize, adversario: usize) {
    crate::comparacao::ramo("cps_8 primeira convocacao");
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 6;
    cpq4_por(&mut e, 8, cpq4_clube_de(pessoa));
    cpq4_por(&mut e, 0xd, cps8_i(adversario));
    cpq4_por(&mut e, 0x11, *((pessoa + 0x18) as *const i8) as i32);
    if pessoa != 0 {
        cpq4_historico(true, this, cps8_d(pessoa), &e);
    } else {
        aviso_em(0xa83b78, 0x1010);
    }
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia não grava mais nada: só a original; a
// cópia leva o ramo.
unsafe extern "thiscall" fn comparar_registrar_a_primeira_convocacao(this: usize, pessoa: usize, adversario: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%)(this, pessoa, adversario)
}
