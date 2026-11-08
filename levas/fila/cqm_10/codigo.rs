// ---------- %%LEVA%%a leva: os inscritos das copas inglesas: a Copa da Liga, a segunda chave do Troféu da Liga e os dois clubes da Supercopa (eng_cc_cup.cpp, eng_auto_cup.cpp, eng_charity.cpp) ----------

unsafe fn cqm10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqm10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube `i` da tabela dos clubes ([0xae23b4], registros de 0x245 bytes).
unsafe fn eng_cc_cup_club_by_index(i: i16) -> usize {
    cqm10_d(0xae23b4) + i as isize as usize * 0x245
}

/// O id da competição da divisão do clube (+0x57, ponteiro para o registro da competição; +0 o id), ou None sem divisão.
unsafe fn division_comp_id(c: usize) -> Option<usize> {
    let d = cqm10_d(c + 0x57);
    if d == 0 { None } else { Some(cqm10_d(d)) }
}

/// Grava a inscrição `k` da lista (+0xa7, 6 bytes: o clube, +4 a classe, +5 a situação).
unsafe fn eng_cc_cup_write_entrant(this: usize, k: i32, clube: usize, classe: u8) {
    let e = cqm10_d(this + 0xa7) + k as usize * 6;
    core::ptr::write_unaligned(e as *mut u32, clube as u32);
    *((e + 4) as *mut u8) = classe;
    *((e + 5) as *mut u8) = 0;
}

/// Uma passada pelos clubes da divisão `id` ([id], o id da competição) fora das copas europeias (+0x1db = −1), até a
/// lista encher (+0x3e); a classe de cada um vem de `classe(clube)`.
unsafe fn eng_cc_cup_enter_division_clubs(this: usize, n: &mut i32, id: usize, classe: impl Fn(usize) -> u8) {
    let mut i: i16 = 0;
    while (i as i32) < cqm10_d(0xae235c) as i32 {
        let c = eng_cc_cup_club_by_index(i);
        if let Some(d) = division_comp_id(c) {
            if *n != cqm10_w(this + 0x3e) as i32 && d == cqm10_d(id) && cqm10_d(c + 0x1db) as i32 == -1 {
                eng_cc_cup_write_entrant(this, *n, c, classe(c));
                *n += 1;
                if *n == cqm10_w(this + 0x3e) as i32 {
                    return;
                }
            }
        }
        i = i.wrapping_add(1);
    }
}

/// A classe de quem veio da divisão `de` (+0x5b, a competição da temporada anterior; +0x5f a posição nela, byte): `base`
/// menos a posição.
unsafe fn came_from_division(c: usize, de: usize) -> bool {
    let p = cqm10_d(c + 0x5b);
    p != 0 && cqm10_d(p) == cqm10_d(de)
}

/// { 0x56ccc0 } (thiscall, ret 0, eng_cc_cup.cpp): `EngCcCup::collect_entrants()`: +0x3e = 92 e +0xb2 = 0; a lista
/// (0x944e46 de 0x228 em +0xa7; sem memória, a caixa da linha 0x18f e −1). Fora das copas europeias, na ordem: a Third
/// ([0x9cf5c8]) e a Second ([0x9cf5c4]) com classe 0; a First ([0x9cf5c0]) com 0x46 − posição se veio da Premier
/// ([0x9cf5bc]), 0x28 − posição se já era da First, 0 senão; a Premier com 0x78 − posição se já era da Premier, 0x64 −
/// posição se veio da First, 0 senão. Depois os clubes das quatro divisões nas copas europeias (+0x1db ≠ −1), classe
/// 0x78, cada um somando 1 ao +0xb2. A lista vai ao qsort (0x945333 com 0x5221c0: a classe menor primeiro, no empate a
/// reputação) e a classe vira 1 (Premier), 2 (First) ou 3 (os outros). Lista cheia: +0xb2 no máximo 12 (byte com sinal) e
/// 1; senão 0. Devolve também quantos entraram (para o comparador).
unsafe fn collect_league_cup_entrants(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x5c);
    *((this + 0xb2) as *mut u8) = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x228);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x9c4224, 0x18f);
        return (-1, 0);
    }
    let mut n = 0i32;
    eng_cc_cup_enter_division_clubs(this, &mut n, 0x9cf5c8, |_| 0);
    eng_cc_cup_enter_division_clubs(this, &mut n, 0x9cf5c4, |_| 0);
    eng_cc_cup_enter_division_clubs(this, &mut n, 0x9cf5c0, |c| {
        let pos = *((c + 0x5f) as *const u8);
        if came_from_division(c, 0x9cf5bc) {
            crate::comparacao::ramo("cqm_10 caiu da premier");
            0x46u8.wrapping_sub(pos) // cqm10: quem caiu da Premier
        } else if came_from_division(c, 0x9cf5c0) {
            0x28u8.wrapping_sub(pos)
        } else {
            0
        }
    });
    eng_cc_cup_enter_division_clubs(this, &mut n, 0x9cf5bc, |c| {
        let pos = *((c + 0x5f) as *const u8);
        if came_from_division(c, 0x9cf5bc) {
            0x78u8.wrapping_sub(pos)
        } else if came_from_division(c, 0x9cf5c0) {
            0x64u8.wrapping_sub(pos)
        } else {
            0
        }
    });
    let mut i: i16 = 0;
    while (i as i32) < cqm10_d(0xae235c) as i32 {
        let c = eng_cc_cup_club_by_index(i);
        if let Some(d) = division_comp_id(c) {
            if n != cqm10_w(this + 0x3e) as i32
                && (d == cqm10_d(0x9cf5bc) || d == cqm10_d(0x9cf5c0) || d == cqm10_d(0x9cf5c4) || d == cqm10_d(0x9cf5c8))
                && cqm10_d(c + 0x1db) as i32 != -1
            {
                crate::comparacao::ramo("cqm_10 clube nas copas europeias");
                *((this + 0xb2) as *mut u8) = (*((this + 0xb2) as *const u8)).wrapping_add(1);
                eng_cc_cup_write_entrant(this, n, c, 0x78);
                n += 1;
                if n == cqm10_w(this + 0x3e) as i32 {
                    break;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(cqm10_d(this + 0xa7), n as u32, 6, 0x5221c0);
    crate::comparacao::ramo("cqm_10 inscritos da copa da liga");
    let mut k: i16 = 0;
    while (k as i32) < n {
        let e = cqm10_d(this + 0xa7) + k as usize * 6;
        let c = cqm10_d(e);
        let d = if c != 0 { division_comp_id(c) } else { None };
        *((e + 4) as *mut u8) = match d {
            Some(x) if x == cqm10_d(0x9cf5bc) => 1, // cqm10: a classe da Premier
            Some(x) if x == cqm10_d(0x9cf5c0) => 2,
            _ => 3,
        };
        k = k.wrapping_add(1);
    }
    if n != cqm10_w(this + 0x3e) as i32 {
        return (0, n);
    }
    if *((this + 0xb2) as *const i8) > 0xc {
        *((this + 0xb2) as *mut u8) = 0xc;
    }
    (1, n)
}

unsafe extern "thiscall" fn eng_cc_cup_collect_entrants(this: usize) -> i32 {
    collect_league_cup_entrants(this).0
}

/// Lado a lado para as rotinas que alocam a lista +0xa7: as regiões `reg` (com o [0xb67a34]) a partir do mesmo estado; a
/// original deixa a lista dela em +0xa7, a cópia faz a sua, que é comparada nos `bytes(n)` primeiros bytes e liberada (o
/// +0xa7 volta à da original).
unsafe fn eng_cc_cup_compare_entrant_list(nome: &str, this: usize, reg: &[(usize, usize)], o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32),
    bytes: impl Fn(i32, i32) -> usize) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(reg, || {
        let r = o();
        lo.set(cqm10_d(this + 0xa7));
        r
    }, || {
        let (r, n) = c();
        lr.set(cqm10_d(this + 0xa7));
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

unsafe extern "thiscall" fn comparar_eng_cc_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    eng_cc_cup_compare_entrant_list("eng_cc_cup_collect_entrants", this, &[(this + 0x3e, 2), (this + 0xb2, 1), (0xb67a34, 4)], || o(this),
        || collect_league_cup_entrants(this), |_, n| n.max(0) as usize * 6)
}

/// { 0x56b3c0 } (thiscall, ret 0, eng_auto_cup.cpp): `EngAutoCup::collect_entrants()`: +0x3e = 28; a lista (0x944e46 de
/// 0xa8 em +0xa7; sem memória, a caixa da linha 0x19f e −1) com a segunda metade dos 56 clubes escolhidos (o vetor de
/// ponteiros em +0xb2, a partir do 28º: +0x70), classe e situação 0. Devolve 1.
unsafe fn collect_auto_cup_entrants(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x1c);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xa8);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x9c4180, 0x19f);
        return (-1, 0);
    }
    crate::comparacao::ramo("cqm_10 segunda chave do trofeu da liga");
    let mut k: i16 = 0;
    if cqm10_w(this + 0x3e) > 0 {
        loop {
            let c = cqm10_d(cqm10_d(this + 0xb2) + 0x70 + k as usize * 4); // cqm10: o k-ésimo da segunda metade
            eng_cc_cup_write_entrant(this, k as i32, c, 0);
            k = k.wrapping_add(1);
            if k >= cqm10_w(this + 0x3e) {
                break;
            }
        }
    }
    (1, k as i32)
}

unsafe extern "thiscall" fn eng_auto_cup_collect_entrants(this: usize) -> i32 {
    collect_auto_cup_entrants(this).0
}

unsafe extern "thiscall" fn comparar_eng_auto_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    eng_cc_cup_compare_entrant_list("eng_auto_cup_collect_entrants", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || collect_auto_cup_entrants(this), |_, n| n.max(0) as usize * 6)
}

/// O clube que terminou em `lugar` (0 o campeão, 1 o vice) na última temporada da competição de id [id] (a competição do
/// banco: [0xae23d0] + 0x6b × id; 0x4afd80: push lugar; push competição; add esp, 8 -> cdecl).
unsafe fn latest_placed_club(id: usize, lugar: u32) -> usize {
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    colocado(cqm10_d(0xae23d0).wrapping_add(cqm10_d(id).wrapping_mul(0x6b)), lugar)
}

/// { 0x56d780 } (thiscall, ret 0, eng_charity.cpp): `EngCharity::select_teams()`: +0x3e = 2; a lista (0x944e46 de 0xc em
/// +0xa7; sem memória, a caixa da linha 0xc5 e −1). O campeão da Premier ([0x9cf5bc]; sem ele, a caixa 0xce e 0) e o da
/// Copa da Inglaterra ([0x9cf74c]; sem ele, 0xd7 e 0); se é o mesmo clube, o segundo vira o vice da Premier (sem ele,
/// 0xe3 e 0). Classe e situação 0 nos dois; devolve 1.
unsafe fn select_charity_shield_teams(this: usize) -> (i32, i32) {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xc);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x9c425c, 0xc5);
        return (-1, 0);
    }
    let a = latest_placed_club(0x9cf5bc, 0); // cqm10: o campeão inglês
    core::ptr::write_unaligned(l as *mut u32, a as u32);
    if a == 0 {
        show_error_at(0x9c425c, 0xce);
        return (0, 0);
    }
    let b = latest_placed_club(0x9cf74c, 0);
    core::ptr::write_unaligned((l + 6) as *mut u32, b as u32);
    if b == 0 {
        show_error_at(0x9c425c, 0xd7);
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqm_10 dobradinha inglesa");
        let b = latest_placed_club(0x9cf5bc, 1);
        core::ptr::write_unaligned((l + 6) as *mut u32, b as u32);
        if b == 0 {
            show_error_at(0x9c425c, 0xe3);
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqm_10 clubes da supercopa inglesa");
    *((l + 4) as *mut u8) = 0;
    *((l + 5) as *mut u8) = 0;
    *((l + 0xa) as *mut u8) = 0;
    *((l + 0xb) as *mut u8) = 0;
    (1, 2)
}

unsafe extern "thiscall" fn eng_cc_cup_select_teams(this: usize) -> i32 {
    select_charity_shield_teams(this).0
}

unsafe extern "thiscall" fn comparar_eng_cc_cup_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    eng_cc_cup_compare_entrant_list("eng_cc_cup_select_teams", this, &[(this + 0x3e, 2), (0xb67a34, 4)], || o(this), || select_charity_shield_teams(this),
        |r, _| if r == 1 { 12 } else { 0 })
}
