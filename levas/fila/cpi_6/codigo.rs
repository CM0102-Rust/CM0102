// ---------- %%LEVA%%a leva: a notícia para os técnicos humanos de uma nação, a missão do olheiro da busca e se a copa terminou ----------

unsafe fn cpi6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x786520 } (thiscall, ret 8, news.cpp): `News::send_to_nation_club_managers(noticia, nacao)`: a notícia fica com
/// o alcance da nação (+0xde = 2) e vai (0x785d40 com 0) para cada um dos 16 técnicos humanos (os últimos 16 de
/// [0xae23bc], 0x6e bytes) ativos (+0x5f = 1) com clube (+0x39) da `nacao` (+0x53); a primeira entrega recusada (0)
/// encerra. Devolve 0. Notícia ou nação nulas: a original (caixa "Error").
unsafe extern "thiscall" fn send_to_nation_club_managers(this: usize, noticia: usize, nacao: usize) -> u32 {
    if noticia == 0 || nacao == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%)(this, noticia, nacao); // caixa: só a original
    }
    *((noticia + 0xde) as *mut u8) = 2; // o alcance da nação
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x785d40usize);
    for i in 0..0x10usize {
        let p = cpi6_d(0xae23bc).wrapping_add(cpi6_d(0xae2364).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(0x6e));
        if *((p + 0x5f) as *const u8) != 1 {
            continue;
        }
        let clube = cpi6_d(p + 0x39);
        if clube == 0 || cpi6_d(clube + 0x53) != nacao {
            continue;
        }
        if entregar(this, noticia, p, 0) == 0 {
            break; // a entrega recusada encerra
        }
    }
    0
}

/// O olheiro da busca confere (o tipo +0xa é 4, o índice +0x180 > −1 e a pessoa ainda confere com os nomes e a data
/// guardados pela 0x53e990)? Devolve a pessoa, ou nenhuma (a original mostra a caixa).
unsafe fn check_search_scout(this: usize) -> Option<usize> {
    let i = cpi6_d(this + 0x180) as i32;
    if *((this + 0xa) as *const u8) != 4 || i <= -1 {
        return None;
    }
    let p = cpi6_d(0xae23bc).wrapping_add((i as usize).wrapping_mul(0x6e));
    let confere: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(0x53e990usize);
    if confere(p, cpi6_d(this + 0x174), cpi6_d(this + 0x178), cpi6_d(this + 0x17c), this + 0x184) == 0 || p == 0 {
        return None;
    }
    Some(p)
}

/// { 0x7aefd0 } (thiscall, ret 0x18, player_search.cpp): `PLAYER_SEARCH::get_scout_assignment(clube, nacao, regiao,
/// comp, base, adversario)`: com o olheiro conferido, se alguma das missões está marcada (clube +0x190, nação +0x18c,
/// região +0x194 byte, competição +0x195 acima de −1; base +0x199, adversário +0x19d não nulos), copia as seis para
/// as saídas e devolve 1; nenhuma: 0. Olheiro que não confere: a original (caixa "Error" e 0).
unsafe extern "thiscall" fn get_scout_assignment(this: usize, clube: *mut u32, nacao: *mut u32, regiao: *mut u8, comp: *mut u32, base: *mut u32, adversario: *mut u32) -> u32 {
    if check_search_scout(this).is_none() {
        return original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, *mut u32, *mut u32, *mut u32) -> u32>(%%I1%%)(this, clube, nacao, regiao, comp, base, adversario); // caixa: só a original
    }
    let marcada = cpi6_d(this + 0x190) as i32 > -1
        || cpi6_d(this + 0x18c) as i32 > -1
        || *((this + 0x194) as *const i8) > -1
        || cpi6_d(this + 0x195) as i32 > -1 // a competição
        || cpi6_d(this + 0x199) != 0
        || cpi6_d(this + 0x19d) != 0;
    if !marcada {
        return 0;
    }
    clube.write_unaligned(cpi6_d(this + 0x190) as u32);
    nacao.write_unaligned(cpi6_d(this + 0x18c) as u32);
    *regiao = *((this + 0x194) as *const u8);
    comp.write_unaligned(cpi6_d(this + 0x195) as u32);
    base.write_unaligned(cpi6_d(this + 0x199) as u32);
    adversario.write_unaligned(cpi6_d(this + 0x19d) as u32);
    1
}

/// { 0x521e00 } (thiscall, ret 8, Cup.cpp): `Cup::check_complete(fases, _)`: 0 se a copa não começou (+0xb1). Com
/// `fases` = 1 e sem fase marcada (+0x44 = 0xff): cada fase de 0 até a atual (+0x30, vetor +0xc) tem de estar completa
/// (método +0x10 com 0, 1) e a atual tem de ser a última (+0x2c − 1); senão 0. Fora isso, 1.
unsafe extern "thiscall" fn cup_check_complete(this: usize, fases: u32, _x: u32) -> u8 {
    if *((this + 0xb1) as *const u8) == 0 {
        return 0;
    }
    if fases != 1 || *((this + 0x44) as *const u8) != 0xff {
        return 1;
    }
    let mut i = 0i32;
    while i <= cpi6_d(this + 0x30) as i32 {
        let fase = cpi6_d(cpi6_d(this + 0xc).wrapping_add(i as usize * 4));
        let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(cpi6_d(cpi6_d(fase) + 0x10));
        if completa(fase, 0, 1) == 0 {
            return 0; // uma fase incompleta
        }
        i += 1;
    }
    (cpi6_d(this + 0x30) as i32 >= (cpi6_d(this + 0x2c) as i32).wrapping_sub(1)) as u8
}

/// Comparador: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_send_to_nation_club_managers(this: usize, noticia: usize, nacao: usize) -> u32 {
    crate::comparacao::ramo("send_to_nation_club_managers");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%)(this, noticia, nacao)
}

/// Comparador: lado a lado pelas seis saídas (a 0x53e990 só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_get_scout_assignment(this: usize, clube: *mut u32, nacao: *mut u32, regiao: *mut u8, comp: *mut u32, base: *mut u32, adversario: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, *mut u32, *mut u32, *mut u32) -> u32>(%%I1%%);
    if check_search_scout(this).is_none() {
        return o(this, clube, nacao, regiao, comp, base, adversario);
    }
    let reg = [(clube as usize, 4), (nacao as usize, 4), (regiao as usize, 1), (comp as usize, 4), (base as usize, 4), (adversario as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, clube, nacao, regiao, comp, base, adversario),
        || get_scout_assignment(this, clube, nacao, regiao, comp, base, adversario));
    PLACAR.registrar(igual && r == n, || format!("get_scout_assignment original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparador: pelo valor (o método das fases só lê).
unsafe extern "thiscall" fn comparar_cup_check_complete(this: usize, fases: u32, x: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(%%I2%%)(this, fases, x);
    let n = cup_check_complete(this, fases, x);
    PLACAR.registrar(r == n, || format!("cup_check_complete({:x}, {}) original={} rust={}", this, fases, r, n));
    r
}
