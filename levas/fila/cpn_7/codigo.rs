// ---------- %%LEVA%%a leva: a busca do olheiro e os amistosos: se a busca cobre a nação, tirar o amistoso marcado e os pares de nações que não jogam amistoso (player_search.cpp, friendly.cpp) ----------

unsafe fn cpn7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn7_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpn7_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x7b7490 } (thiscall, ret 4, player_search.cpp): `PLAYER_SEARCH::scout_covers_nation(nacao)`: a busca de olheiro
/// (tipo +0xa = 4) com o olheiro conferido (índice +0x180 > −1 e 0x53e990 com os nomes +0x174..+0x17c e a data +0x184)
/// e com clube (+0x39) cobre a `nacao`? Com missão marcada (0x7aefd0, que copia as missões +0x18c..+0x19d sobre elas
/// mesmas): a nação do clube da missão (+0x53 do clube +0x190), a nação da missão (+0x18c = id +0 da nação), a região
/// (+0x194, byte com sinal, = +0x76 da nação), ou com a base (+0x199) a mesma região. Sem missão: as duas nações com
/// +0x7f = 2, a mesma região (+0x76) da nação do clube, a nação não estrangeira para o olheiro (0x53d980 com 1) ou os
/// olheiros do clube cobrindo a nação (0x7f9190 do [0xb74448]). Devolve 1 ou 0.
unsafe extern "thiscall" fn scout_covers_nation(this: usize, nacao: usize) -> u32 {
    let mut pessoa = 0usize;
    let i = cpn7_d(this + 0x180) as i32;
    if i > -1 {
        let confere: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(0x53e990usize);
        let p = cpn7_d(0xae23bc).wrapping_add((i as usize).wrapping_mul(0x6e));
        if confere(p, cpn7_d(this + 0x174), cpn7_d(this + 0x178), cpn7_d(this + 0x17c), this + 0x184) != 0 {
            pessoa = cpn7_d(0xae23bc).wrapping_add(cpn7_d(this + 0x180).wrapping_mul(0x6e));
        }
    }
    if cpn7_b(this + 0xa) != 4 || pessoa == 0 || nacao == 0 {
        return 0;
    }
    let clube = cpn7_d(pessoa + 0x39);
    if clube == 0 {
        return 0;
    }
    let missao: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(0x7aefd0usize);
    if missao(this, this + 0x190, this + 0x18c, this + 0x194, this + 0x195, this + 0x199, this + 0x19d) != 0 {
        let c = cpn7_d(this + 0x190) as i32;
        if c > -1 && cpn7_d(cpn7_d(0xae23b4).wrapping_add((c as usize).wrapping_mul(0x245)) + 0x53) == nacao {
            return 1;
        }
        let n = cpn7_d(this + 0x18c) as i32;
        if n > -1 && cpn7_d(nacao) as i32 == n {
            return 1;
        }
        let r = cpn7_b(this + 0x194);
        if (r as i8) > -1 && cpn7_b(nacao + 0x76) == r {
            return 1;
        }
        // a original compara de novo o mesmo byte quando há base
        if cpn7_d(this + 0x199) != 0 && cpn7_b(nacao + 0x76) == r {
            return 1;
        }
        return 0;
    }
    let nc = cpn7_d(clube + 0x53);
    if cpn7_b(nc + 0x7f) == 2 && cpn7_b(nacao + 0x7f) == 2 {
        return 1;
    }
    if cpn7_b(nc + 0x76) == cpn7_b(nacao + 0x76) {
        crate::comparison::branch("cpn_7 mesma regiao");
        return 1; // a mesma região da nação do clube
    }
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    if estrangeira(pessoa, nacao, 1) == 0 {
        crate::comparison::branch("cpn_7 nacao do olheiro");
        return 1; // a nação não é estrangeira para o olheiro
    }
    let cobrem: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7f9190usize);
    (cobrem(cpn7_d(0xb74448), nacao, clube) != 0) as u32
}

/// { 0x5c87b0 } (thiscall, ret 8, friendly.cpp): `Friendly::remove_friendly_match(jogo, limpar_vaga)`: tira da lista
/// [+0x34d4] cada item que aponta para o `jogo` (0x68d1f0; o cursor +0xa volta ao começo antes, depois de cada um e no
/// fim), com `limpar_vaga` zera a vaga do amistoso marcado (0x5c6740 pelo dia +0, ano +2, clube +0x14 e adversário
/// +0x10 do jogo: +0x10 = 0 e +0x18 = 0) e tira o jogo da lista [+0x34c8] (0x68d1f0, que o libera). Jogo nulo ou sem
/// a lista +0x34c8: a original (caixa "Error").
unsafe extern "thiscall" fn remove_friendly_match(this: usize, jogo: usize, limpar_vaga: u32) -> u32 {
    if jogo == 0 || cpn7_d(this + 0x34c8) == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%)(this, jogo, limpar_vaga);
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let recomecar = || {
        let l = cpn7_d(this + 0x34d4);
        cpn7_por(l + 0xa, cpn7_d(l + 2));
    };
    if cpn7_d(this + 0x34d4) != 0 {
        recomecar();
        let mut e = proximo(cpn7_d(this + 0x34d4));
        while e != 0 {
            if cpn7_d(e) == jogo {
                tirar(cpn7_d(this + 0x34d4), e);
                recomecar();
            }
            e = proximo(cpn7_d(this + 0x34d4));
        }
        recomecar();
    }
    if limpar_vaga != 0 {
        // ponytail: a original empurra dia e ano em registradores com os 16 bits altos de outros valores; a 0x5c6740 só
        // lê os 16 bits baixos
        let vaga: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(0x5c6740usize);
        let v = vaga(this, *(jogo as *const u16) as u32, *((jogo + 2) as *const u16) as u32, cpn7_d(jogo + 0x14), cpn7_d(jogo + 0x10));
        if v != 0 {
            cpn7_por(v + 0x10, 0);
            *((v + 0x18) as *mut u8) = 0;
        }
    }
    crate::comparison::branch("cpn_7 tirar o amistoso");
    tirar(cpn7_d(this + 0x34c8), jogo) // tira o jogo da lista dos amistosos
}

/// { 0x5cc2d0 } (thiscall, friendly.cpp): `Friendly::init_banned_nation_pairs()`: +0x3506 = 4 pares e +0x3502 um vetor
/// novo de 0x20 bytes (malloc 0x944e46) com os pares de nações que não jogam amistoso entre si: Irã e EUA, Iraque e
/// qualquer uma (−1), Vietnã e EUA, as duas Coreias (as chaves [0x9cf354], [0x9cf4f8], [0x9cf358], [0x9cf508],
/// [0x9cf404], [0x9cf484]). Sem memória: a caixa "Error" (linha 0x1800) e +0x3506 = 0.
unsafe extern "thiscall" fn init_banned_nation_pairs(this: usize) {
    *((this + 0x3506) as *mut u8) = 4;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(0x20);
    cpn7_por(this + 0x3502, v);
    if v == 0 {
        show_error_at(0x9cd0a0, 0x1800);
        *((this + 0x3506) as *mut u8) = 0;
        return;
    }
    for (k, chave) in [0x9cf354usize, 0x9cf4f8, 0x9cf358, 0, 0x9cf508, 0x9cf4f8, 0x9cf404, 0x9cf484].into_iter().enumerate() {
        cpn7_por(cpn7_d(this + 0x3502) + 4 * k, if chave == 0 { usize::MAX } else { cpn7_d(chave) });
    }
}

/// Comparador: lado a lado pelo valor e pelas missões +0x18c..+0x1a0 (a 0x7aefd0 as copia sobre elas mesmas).
unsafe extern "thiscall" fn comparar_scout_covers_nation(this: usize, nacao: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    let reg = [(this + 0x18c, 0x15)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, nacao), || scout_covers_nation(this, nacao));
    PLACAR.record(igual && r == n, || format!("scout_covers_nation(nacao {:#x}) original={} rust={} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}

/// Comparador: a 0x68d1f0 libera o que tira: o plano antes (os itens que ficam nas duas listas e a vaga do amistoso);
/// a original roda; confere as listas, o cursor da +0x34d4 no começo e a vaga zerada. Jogo nulo ou sem a lista
/// +0x34c8: só a original (a caixa).
unsafe extern "thiscall" fn comparar_remove_friendly_match(this: usize, jogo: usize, limpar_vaga: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    if jogo == 0 || cpn7_d(this + 0x34c8) == 0 {
        return o(this, jogo, limpar_vaga);
    }
    // ponytail: a 0x68d1f0 acha o nó pelo conteúdo do item; o plano supõe itens de conteúdo distinto (o normal)
    let l4 = cpn7_d(this + 0x34d4);
    let esperado4: Vec<usize> = if l4 != 0 { list_items_capped(l4).into_iter().filter(|&e| cpn7_d(e) != jogo).collect() } else { Vec::new() };
    let mut esperado8 = list_items_capped(cpn7_d(this + 0x34c8));
    if let Some(k) = esperado8.iter().position(|&e| e == jogo) {
        esperado8.remove(k);
    }
    let v = if limpar_vaga != 0 {
        let vaga: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(0x5c6740usize);
        vaga(this, *(jogo as *const u16) as u32, *((jogo + 2) as *const u16) as u32, cpn7_d(jogo + 0x14), cpn7_d(jogo + 0x10))
    } else {
        0
    };
    let r = o(this, jogo, limpar_vaga);
    let l4 = cpn7_d(this + 0x34d4);
    let itens4 = if l4 != 0 { list_items_capped(l4) } else { Vec::new() };
    let itens8 = list_items_capped(cpn7_d(this + 0x34c8));
    let ok = itens4 == esperado4
        && (l4 == 0 || cpn7_d(l4 + 0xa) == cpn7_d(l4 + 2))
        && itens8 == esperado8
        && (v == 0 || (cpn7_d(v + 0x10) == 0 && cpn7_b(v + 0x18) == 0));
    PLACAR.record(ok, || format!("remove_friendly_match({:#x}, {}) itens {} e {} (esperava {} e {}), vaga {:#x}", jogo, limpar_vaga, itens4.len(), itens8.len(), esperado4.len(), esperado8.len(), v));
    r
}

/// Comparador: o malloc não pode rodar duas vezes: a original roda e confere o vetor e a conta com o plano.
unsafe extern "thiscall" fn comparar_init_banned_nation_pairs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let v = cpn7_d(this + 0x3502);
    let ok = if v == 0 {
        cpn7_b(this + 0x3506) == 0
    } else {
        let esperado = [cpn7_d(0x9cf354), cpn7_d(0x9cf4f8), cpn7_d(0x9cf358), usize::MAX, cpn7_d(0x9cf508), cpn7_d(0x9cf4f8), cpn7_d(0x9cf404), cpn7_d(0x9cf484)];
        cpn7_b(this + 0x3506) == 4 && (0..8).all(|k| cpn7_d(v + 4 * k) == esperado[k])
    };
    PLACAR.record(ok, || format!("init_banned_nation_pairs: conta {} vetor {:#x}", cpn7_b(this + 0x3506), v));
}
