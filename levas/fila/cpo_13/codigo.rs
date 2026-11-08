// ---------- %%LEVA%%a leva: as seleções: a competição atual da nação e os clubes com jogo na data; a disciplina: tirar o jogador regenerado (national_teams.cpp, discipline.cpp) ----------

unsafe fn cpo13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpo13_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// A competição (id) é uma das fases finais grandes ([0x9cf79c], [0x9cf7a0], [0x9cf7a4], [0x9cf7a8], [0x9cf7ac],
/// [0x9cf790], [0x9cf8a0], [0x9cf78c], [0x9cf964])?
unsafe fn is_major_finals(id: u32) -> bool {
    [0x9cf79c, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964usize]
        .iter()
        .any(|&g| cpo13_d(g) as u32 == id)
}

/// O que a 0x775220 grava: a competição atual da nação ([this+4] + id × 110 + 4), a da fila ([this+8] + id × 4) e se
/// avisa o gerenciador dos técnicos.
struct NationCurrentCompUpdate {
    vaga: usize,
    nova: Option<u32>,
    fila: usize,
    na_fila: Option<u32>,
    avisa: bool,
}

/// O plano da 0x775220 (só lê): com competição atual (≠ −1) que é fase final grande, a nova vai para a fila e a atual
/// fica (sem aviso); senão, se a nova é fase final grande e há atual, a atual vai para a fila; a nova vira a atual e o
/// gerenciador dos técnicos ([0xb67a44], se existe) é avisado.
unsafe fn current_comp_plan(this: usize, nacao: usize, comp: usize) -> NationCurrentCompUpdate {
    let id = cpo13_d(nacao);
    let vaga = cpo13_d(this + 4).wrapping_add(id.wrapping_mul(110)) + 4;
    let fila = cpo13_d(this + 8).wrapping_add(id.wrapping_mul(4));
    let atual = cpo13_d(vaga) as u32;
    let nova = cpo13_d(comp) as u32;
    let mut p = NationCurrentCompUpdate { vaga, nova: Some(nova), fila, na_fila: None, avisa: cpo13_d(0xb67a44) != 0 };
    if atual != 0xffff_ffff {
        let r = cpo13_d(0xae23d0).wrapping_add((atual as usize).wrapping_mul(107)); // registros de 107 bytes
        if r != 0 && is_major_finals(cpo13_d(r) as u32) {
            return NationCurrentCompUpdate { vaga, nova: None, fila, na_fila: Some(nova), avisa: false };
        }
        if is_major_finals(nova) {
            p.na_fila = Some(atual);
        }
    }
    p
}

/// { 0x775220 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::set_current_competition(nacao, comp)`: a
/// competição atual da nação vira `comp` (plano em `current_comp_plan`: a fase final grande em curso fica na frente e a
/// outra vai para a fila) e o gerenciador dos técnicos é avisado (0x6a3770 com o clube da seleção principal, 0x53b3d0,
/// e 1). Nação ou competição nula: a caixa "Error" (e volta).
unsafe extern "thiscall" fn set_current_competition(this: usize, nacao: usize, comp: usize) {
    if nacao == 0 || comp == 0 {
        show_error_at(0xa5b2a4, 0x1bb2);
        return;
    }
    let p = current_comp_plan(this, nacao, comp);
    if let Some(f) = p.na_fila {
        cpo13_por(p.fila, f);
    }
    let Some(nova) = p.nova else {
        crate::comparison::branch("cpo_13 fase final fica");
        return;
    };
    crate::comparison::branch("cpo_13 competicao atual");
    cpo13_por(p.vaga, nova);
    let g = cpo13_d(0xb67a44);
    if g != 0 {
        let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
        let avisar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
        avisar(g, principal(nacao), 1);
    }
}

/// Comparador: a 0x6a3770 (o aviso) não pode rodar duas vezes: o plano antes, a original roda; confere a competição
/// atual e a da fila. Nação ou competição nula: só a original (caixa).
unsafe extern "thiscall" fn comparar_set_current_competition(this: usize, nacao: usize, comp: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    if nacao == 0 || comp == 0 {
        return o(this, nacao, comp);
    }
    let p = current_comp_plan(this, nacao, comp);
    let (atual, fila) = (cpo13_d(p.vaga) as u32, cpo13_d(p.fila) as u32);
    o(this, nacao, comp);
    let (atual2, fila2) = (cpo13_d(p.vaga) as u32, cpo13_d(p.fila) as u32);
    let ok = atual2 == p.nova.unwrap_or(atual) && fila2 == p.na_fila.unwrap_or(fila);
    SCOREBOARD.record(ok, || format!("set_current_competition(nação {}, comp {}) atual {:#x} -> {:#x} (esperava {:x?}), fila {:#x} -> {:#x} (esperava {:x?})",
        cpo13_d(nacao), cpo13_d(comp), atual, atual2, p.nova, fila, fila2, p.na_fila));
}

/// Marca o clube `c` (se existe e o id < n) com o código na tabela [this+0x24].
unsafe fn mark_club_code(this: usize, c: usize, n: i32, codigo: u8) {
    if c != 0 && (cpo13_d(c) as i32) < n {
        *(cpo13_d(this + 0x24).wrapping_add(cpo13_d(c)) as *mut u8) = codigo;
    }
}

/// O clube tem nação marcada (+0x53, +0x11c ≠ 0)?
unsafe fn has_flagged_nation(c: usize) -> bool {
    let n = cpo13_d(c + 0x53);
    n != 0 && *((n + 0x11c) as *const u8) != 0
}

/// { 0x7775c0 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::mark_clubs_with_fixtures(data)`: a tabela de um
/// byte por clube ([this+0x24], n = clubes [0xae235c] − 2 × nações [0xae2350]; calloc na primeira, sem memória a
/// caixa "Error") recebe, para cada jogo não amistoso (0x5c6710) da data nas 3 fatias (0x5ae6a0) de cada uma das 0x21
/// entradas de 0x48 bytes de 0xb63df0 (a nação +0 ativa em +0x11c, ou sem nação; o grupo +0x14): sem nação, 3 para os
/// clubes (+0x1c, +0x20) de nação ativa; com nação, 1 quando a competição do jogo (+0x14) é a da liga do mandante (+0x57)
/// ou a +0x60 do visitante, senão 2, para os dois. Lida pela remarcação dos jogos dos clubes com convocados (0x7777e0).
unsafe extern "thiscall" fn mark_clubs_with_fixtures(this: usize, data: usize) {
    let n = (cpo13_d(0xae235c) as i32).wrapping_sub((cpo13_d(0xae2350) as i32).wrapping_mul(2));
    if cpo13_d(this + 0x24) == 0 {
        let calloc: unsafe extern "C" fn(i32, u32) -> usize = core::mem::transmute(0x945501usize);
        let p = calloc(n, 1);
        cpo13_por(this + 0x24, p as u32);
        if p == 0 {
            show_error_at(0xa5b2a4, 0x2229);
            return;
        }
    }
    let get_fixture_list_for_date: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x5ae6a0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    for i in 0..0x21usize {
        let e = 0xb63df0 + i * 0x48;
        let nacao = cpo13_d(e);
        if nacao != 0 && *((nacao + 0x11c) as *const u8) == 0 {
            continue;
        }
        for fatia in 0..3u32 {
            // a original empurra a fatia num dword de pilha com o alto de lixo e o grupo no al: a 0x5abf10 só lê os bytes
            let lista = get_fixture_list_for_date(data, fatia, *((e + 0x14) as *const u8) as u32);
            if lista == 0 {
                continue;
            }
            cpo13_por(lista + 0xa, cpo13_d(lista + 2) as u32);
            let mut jogo = proximo(lista);
            while jogo != 0 {
                if amistoso(cpo13_d(0xae2c74), cpo13_d(jogo + 0x14)) == 0 {
                    let (casa, fora) = (cpo13_d(jogo + 0x1c), cpo13_d(jogo + 0x20));
                    if cpo13_d(e) == 0 {
                        if casa != 0 && has_flagged_nation(casa) {
                            mark_club_code(this, casa, n, 3);
                        }
                        if fora != 0 && has_flagged_nation(fora) {
                            mark_club_code(this, fora, n, 3);
                        }
                    } else {
                        let comp = cpo13_d(jogo + 0x14);
                        // o visitante +0x60 é lido sem conferir o ponteiro (como a original)
                        let codigo = if casa != 0 && (cpo13_d(casa + 0x57) == comp || cpo13_d(fora + 0x60) == comp) { 1 } else { 2 };
                        crate::comparison::branch("cpo_13 clubes com jogo");
                        mark_club_code(this, casa, n, codigo); // o mandante
                        mark_club_code(this, fora, n, codigo); // o visitante
                    }
                }
                jogo = proximo(lista);
            }
        }
    }
}

/// Comparador: na primeira (sem tabela: calloc) só a original; depois, lado a lado na tabela (n bytes). A lista do
/// dia e o amistoso só leem (o cursor da lista volta ao começo a cada uso).
unsafe extern "thiscall" fn comparar_mark_clubs_with_fixtures(this: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let n = (cpo13_d(0xae235c) as i32).wrapping_sub((cpo13_d(0xae2350) as i32).wrapping_mul(2));
    if cpo13_d(this + 0x24) == 0 || n <= 0 {
        return o(this, data);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(cpo13_d(this + 0x24), n as usize)], || o(this, data), || mark_clubs_with_fixtures(this, data));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<(usize, u8, u8)> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).map(|i| (i, a[i], b[i])).collect();
        format!("mark_clubs_with_fixtures({:x}) clube, original, rust: {:?}", core::ptr::read_unaligned(data as *const u32), dif)
    });
}

/// O órgão disciplinar da regra `regra` (byte com sinal): [[this] + 4 × regra].
unsafe fn cpo13_orgao(this: usize, regra: u8) -> usize {
    cpo13_d(cpo13_d(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)))
}

/// A entrada da pessoa no órgão pela vaga dela (0x7e82e0 sem criar: só lê).
unsafe fn cpo13_entrada(orgao: usize, id: u32) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x7e82e0usize);
    achar(orgao, id, 0)
}

/// { 0x556830 } (thiscall, ret 4, discipline.cpp): `Discipline::remove_player(id)`: do regen (0x7abac0): apaga o
/// histórico de suspensões (0x55e1d0), libera as 4 vagas do registro de 29 bytes (com entrada no órgão: a suspensão
/// solta, 0x556d00, a vaga livre e a entrada +0 = −1), tira da lista [this+0x1c] as suspensões guardadas dele
/// (0x68d1f0, enquanto a conta +0x1c > 0) e o registro fica 0xff com a conta 0.
unsafe extern "thiscall" fn discipline_remove_player(this: usize, id: u32) {
    let r = cpo13_d(this + 0x14).wrapping_add((id as usize).wrapping_mul(29));
    let apagar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x55e1d0usize);
    apagar(this, cpo13_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e)));
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if regra == 0xff {
            continue;
        }
        let e = cpo13_entrada(cpo13_orgao(this, regra), id);
        if e == 0 {
            continue;
        }
        crate::comparison::branch("cpo_13 regen solta a vaga");
        let i = core::ptr::read_unaligned((e + 6) as *const i16);
        if i > -1 {
            // o índice no ax; o alto do eax é o do ponteiro da entrada (a 0x556d00 só lê a palavra)
            let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x556d00usize);
            soltar(this, id, (e as u32 & 0xffff_0000) | i as u16 as u32);
        }
        let v = cpo13_d(this + 0x14).wrapping_add((id as usize).wrapping_mul(29) + k * 5);
        *(v as *mut u8) = 0xff;
        cpo13_por(v + 1, 0xffff_ffff);
        cpo13_por(e, 0xffff_ffff);
    }
    let lista = this + 0x1c;
    cpo13_por(lista + 0xa, cpo13_d(this + 0x1e) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let remover: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let mut item = proximo(lista);
    while item != 0 && *((r + 0x1c) as *const i8) > 0 {
        if cpo13_d(item) as u32 == id {
            remover(lista, item); // libera o item
            *((r + 0x1c) as *mut u8) = (*((r + 0x1c) as *const u8)).wrapping_sub(1);
        }
        item = proximo(lista);
    }
    core::ptr::write_bytes(r as *mut u8, 0xff, 29);
    *((r + 0x1c) as *mut u8) = 0;
}

/// Comparador: a 0x556830 solta suspensões e tira (e libera) itens da lista: o plano antes (as entradas das vagas), a
/// original roda; confere o registro (0xff, conta 0) e as entradas livres (+0 = −1).
unsafe extern "thiscall" fn comparar_discipline_remove_player(this: usize, id: u32) {
    let r = cpo13_d(this + 0x14).wrapping_add((id as usize).wrapping_mul(29));
    let mut entradas = Vec::new();
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if regra != 0xff {
            let e = cpo13_entrada(cpo13_orgao(this, regra), id);
            if e != 0 {
                entradas.push(e);
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I2%%)(this, id);
    let mut depois = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, depois.as_mut_ptr(), 29);
    let ok = depois[..0x1c].iter().all(|&b| b == 0xff) && depois[0x1c] == 0
        && entradas.iter().all(|&e| cpo13_d(e) as u32 == 0xffff_ffff);
    SCOREBOARD.record(ok, || format!("discipline_remove_player({}) registro {:02x?} entradas {:x?}", id, depois, entradas));
}
