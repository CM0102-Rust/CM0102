// ---------- %%LEVA%%a leva: os inscritos das copas francesas: a Copa da França, a Copa da Liga francesa e os dois clubes da Supercopa (fra_cup.cpp, fra_lge_cup.cpp, fra_super.cpp) ----------

unsafe fn cqm11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqm11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
unsafe fn cqm11_clube(i: i16) -> usize {
    cqm11_d(0xae23b4) + i as isize as usize * 0x245
}

/// O id da competição apontada por `c + o` (+0 do registro), ou None sem ela.
unsafe fn comp_id_at(c: usize, o: usize) -> Option<usize> {
    let d = cqm11_d(c + o);
    if d == 0 { None } else { Some(cqm11_d(d)) }
}

/// Grava a inscrição `n` da lista (+0xa7, 6 bytes: o clube, +4 classe 0, +5 situação 0) e conta.
unsafe fn add_entrant(this: usize, n: &mut i32, clube: usize) {
    let e = cqm11_d(this + 0xa7) + *n as usize * 6;
    core::ptr::write_unaligned(e as *mut u32, clube as u32);
    *((e + 4) as *mut u8) = 0;
    *((e + 5) as *mut u8) = 0;
    *n += 1;
}

/// Uma passada pelos clubes (os `ate` primeiros, índice em 16 bits) que passam em `entra(clube, n)`; com `para`, a
/// passada acaba quando a lista enche (+0x3e).
unsafe fn enter_clubs_pass(this: usize, n: &mut i32, ate: impl Fn() -> i32, para: bool, entra: impl Fn(usize, i32) -> bool) {
    let mut i: i16 = 0;
    while (i as i32) < ate() {
        let c = cqm11_clube(i);
        if entra(c, *n) {
            add_entrant(this, n, c);
            if para && *n == cqm11_w(this + 0x3e) as i32 {
                return;
            }
        }
        i = i.wrapping_add(1);
    }
}

/// Os clubes da divisão [id] enquanto cabem (n < +0x3e), até encher.
unsafe fn fra_cup_enter_division_clubs(this: usize, n: &mut i32, id: usize) {
    enter_clubs_pass(this, n, || cqm11_d(0xae235c) as i32, true, |c, n| comp_id_at(c, 0x57) == Some(cqm11_d(id)) && n < cqm11_w(this + 0x3e) as i32);
}

/// O limite dos amadores da Copa da França: 0xb8 em 2001 (+0x40 = 0x7d1), 0xb6 nos outros anos.
unsafe fn french_cup_amateur_limit(this: usize) -> i32 {
    if cqm11_w(this + 0x40) == 0x7d1 { 0xb8 } else { 0xb6 }
}

/// { 0x5baa50 } (thiscall, ret 0, fra_cup.cpp): `FraCup::collect_entrants()`: +0x3e = 222; a lista (0x944e46 de 0x534 em
/// +0xa7; sem memória, a caixa da linha 0x16f e −1). Na ordem: todos os da divisão [0x9cf5d4] (sem conferir o espaço); os
/// da divisão [0x9cf764] ligados (+0x60) à [0x9cf5d8]; os franceses (+0x53 = [0x9cf300]) da [0x9cf764] sem ligação; os
/// franceses sem divisão que não são seleções (0x53b900), entre os clubes antes das 2 × [0xae2350] seleções do fim — estas
/// três até `french_cup_amateur_limit`; a Segunda ([0x9cf5d0]) e a Primeira ([0x9cf5cc]) enquanto cabem. Devolve 1 com a lista
/// cheia, senão 0 (e quantos entraram, para o comparador).
unsafe fn french_cup_collect_entrants(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0xde);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x534);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x9ccb28, 0x16f);
        return (-1, 0);
    }
    let mut n = 0i32;
    let todos = || cqm11_d(0xae235c) as i32;
    enter_clubs_pass(this, &mut n, todos, false, |c, _| comp_id_at(c, 0x57) == Some(cqm11_d(0x9cf5d4)));
    enter_clubs_pass(this, &mut n, todos, true, |c, n| {
        comp_id_at(c, 0x57) == Some(cqm11_d(0x9cf764)) && comp_id_at(c, 0x60) == Some(cqm11_d(0x9cf5d8)) && n < french_cup_amateur_limit(this)
    });
    enter_clubs_pass(this, &mut n, todos, true, |c, n| {
        comp_id_at(c, 0x53) == Some(cqm11_d(0x9cf300)) && comp_id_at(c, 0x57) == Some(cqm11_d(0x9cf764)) && cqm11_d(c + 0x60) == 0
            && n < french_cup_amateur_limit(this)
    });
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    enter_clubs_pass(this, &mut n, || cqm11_d(0xae235c) as i32 - 2 * cqm11_d(0xae2350) as i32, true, |c, n| {
        cqm11_d(c + 0x53) != 0 && selecao(c) == 0 && cqm11_d(c + 0x57) == 0 && comp_id_at(c, 0x53) == Some(cqm11_d(0x9cf300))
            && n < french_cup_amateur_limit(this)
    });
    crate::comparacao::ramo("cqm_11 inscritos da copa da franca");
    fra_cup_enter_division_clubs(this, &mut n, 0x9cf5d0); // cqm11: a Segunda francesa na Copa da França
    fra_cup_enter_division_clubs(this, &mut n, 0x9cf5cc); // cqm11: a Primeira francesa na Copa da França
    ((n == cqm11_w(this + 0x3e) as i32) as i32, n)
}

unsafe extern "thiscall" fn fra_cup_collect_entrants(this: usize) -> i32 {
    french_cup_collect_entrants(this).0
}

/// Lado a lado para as rotinas que alocam a lista +0xa7 (como `eng_cc_cup_compare_entrant_list`): as regiões `reg` (com o [0xb67a34]) a
/// partir do mesmo estado; a lista da cópia é comparada nos `bytes(devolução, n)` primeiros bytes e liberada.
unsafe fn cqm11_comparar(nome: &str, this: usize, reg: &[(usize, usize)], o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32),
    bytes: impl Fn(i32, i32) -> usize) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(reg, || {
        let r = o();
        lo.set(cqm11_d(this + 0xa7));
        r
    }, || {
        let (r, n) = c();
        lr.set(cqm11_d(this + 0xa7));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = bytes(a, nr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} lista {} ({} bytes)", nome, this, a, b, igual, lista, k));
    a
}

unsafe extern "thiscall" fn comparar_fra_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    cqm11_comparar("fra_cup_collect_entrants", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || french_cup_collect_entrants(this),
        |_, n| n.clamp(0, 0xde) as usize * 6)
}

/// { 0x5bd160 } (thiscall, ret 0, fra_lge_cup.cpp): `FraLgeCup::collect_entrants()`: +0x3e = 54; a lista (0x944e46 de
/// 0x144 em +0xa7; sem memória, a caixa da linha 0x146 e −1). Na ordem: os da divisão [0x9cf5d4] que não vieram (+0x5b,
/// com competição) da [0x9cf5d8], até 16 em 2001 (+0x40 = 0x7d1) e 14 nos outros anos; os da [0x9cf5d4] que vieram da
/// [0x9cf5d8] enquanto a lista tem menos de 16; a Segunda ([0x9cf5d0]) e a Primeira ([0x9cf5cc]) enquanto cabem.
/// Devolve 1 com a lista cheia, senão 0.
unsafe fn french_league_cup_collect_entrants(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x36);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x144);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x9ccb98, 0x146);
        return (-1, 0);
    }
    let mut n = 0i32;
    let todos = || cqm11_d(0xae235c) as i32;
    let limite = if cqm11_w(this + 0x40) == 0x7d1 { 0x10 } else { 0xe };
    enter_clubs_pass(this, &mut n, todos, true, |c, n| {
        comp_id_at(c, 0x57) == Some(cqm11_d(0x9cf5d4)) && cqm11_d(c + 0x5b) != 0 && comp_id_at(c, 0x5b) != Some(cqm11_d(0x9cf5d8)) && n < limite
    });
    // a original testa o tamanho antes de cada clube: a passada para quando a lista chega a 16
    let mut i: i16 = 0;
    while (i as i32) < cqm11_d(0xae235c) as i32 && n * 6 < 0x60 {
        let c = cqm11_clube(i);
        if comp_id_at(c, 0x57) == Some(cqm11_d(0x9cf5d4)) && comp_id_at(c, 0x5b) == Some(cqm11_d(0x9cf5d8)) {
            add_entrant(this, &mut n, c);
            if n == cqm11_w(this + 0x3e) as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    crate::comparacao::ramo("cqm_11 inscritos da copa da liga francesa");
    fra_cup_enter_division_clubs(this, &mut n, 0x9cf5d0); // cqm11: a Segunda francesa na Copa da Liga
    fra_cup_enter_division_clubs(this, &mut n, 0x9cf5cc); // cqm11: a Primeira francesa na Copa da Liga
    ((n == cqm11_w(this + 0x3e) as i32) as i32, n)
}

unsafe extern "thiscall" fn fra_cup_collect_entrants_5bd160(this: usize) -> i32 {
    french_league_cup_collect_entrants(this).0
}

unsafe extern "thiscall" fn comparar_fra_cup_collect_entrants_5bd160(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    cqm11_comparar("fra_cup_collect_entrants_5bd160", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || french_league_cup_collect_entrants(this),
        |_, n| n.clamp(0, 0x36) as usize * 6)
}

/// O clube que terminou em `lugar` (0 o campeão, 1 o vice) na última temporada da competição de id [id] (0x4afd80: push
/// lugar; push competição do banco [0xae23d0] + 0x6b × id; add esp, 8 -> cdecl).
unsafe fn cqm11_colocado(id: usize, lugar: u32) -> usize {
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    colocado(cqm11_d(0xae23d0).wrapping_add(cqm11_d(id).wrapping_mul(0x6b)), lugar)
}

/// { 0x5bf770 } (thiscall, ret 0, fra_super.cpp): `FraSuper::select_teams()`: +0x3e = 2; a lista (0x944e46 de 0xc em
/// +0xa7; sem memória, a caixa da linha 0xc4 e −1). O campeão da Primeira ([0x9cf5cc]; sem ele, a caixa 0xcd e 0) e o
/// da Copa da França ([0x9cf70c]; sem ele, 0xd6 e 0); se é o mesmo clube, o segundo vira o vice da Primeira (sem ele, 0xe2
/// e 0). Classe e situação 0 nos dois; devolve 1.
unsafe fn french_super_cup_select_teams(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xc);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x9ccc48, 0xc4);
        return (-1, 0);
    }
    let a = cqm11_colocado(0x9cf5cc, 0); // cqm11: o campeão francês
    core::ptr::write_unaligned(l as *mut u32, a as u32);
    if a == 0 {
        show_error_at(0x9ccc48, 0xcd);
        return (0, 0);
    }
    let b = cqm11_colocado(0x9cf70c, 0);
    core::ptr::write_unaligned((l + 6) as *mut u32, b as u32);
    if b == 0 {
        show_error_at(0x9ccc48, 0xd6);
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqm_11 dobradinha francesa");
        let b = cqm11_colocado(0x9cf5cc, 1);
        core::ptr::write_unaligned((l + 6) as *mut u32, b as u32);
        if b == 0 {
            show_error_at(0x9ccc48, 0xe2);
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqm_11 clubes da supercopa francesa");
    *((l + 4) as *mut u8) = 0;
    *((l + 5) as *mut u8) = 0;
    *((l + 0xa) as *mut u8) = 0;
    *((l + 0xb) as *mut u8) = 0;
    (1, 2)
}

unsafe extern "thiscall" fn fra_super_select_teams(this: usize) -> i32 {
    french_super_cup_select_teams(this).0
}

unsafe extern "thiscall" fn comparar_fra_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    cqm11_comparar("fra_super_select_teams", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || french_super_cup_select_teams(this),
        |r, _| if r == 1 { 12 } else { 0 })
}
