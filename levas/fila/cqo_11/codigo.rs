// ---------- %%LEVA%%a leva: os inscritos das copas italianas: a Copa da Itália, a Copa da Série C e os dois clubes da Supercopa (ita_cup.cpp, ita_c_cup.cpp, ita_super.cpp) ----------

unsafe fn cqo11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

/// Os clubes da tabela de clubes ([0xae23b4], 0x245 bytes) que a passada olha: [0xae235c] − 2 × [0xae2350] (sem as
/// seleções [inferência]).
unsafe fn ita_cup_club_count_without_nations() -> i32 {
    (cqo11_d(0xae235c) as i32).wrapping_sub((cqo11_d(0xae2350) as i32).wrapping_mul(2))
}

unsafe fn cqo11_clube(i: i32) -> usize {
    cqo11_d(0xae23b4).wrapping_add((i as isize * 0x245) as usize)
}

/// O id da competição apontada por `c + o` (+0x57 a divisão, +0x5b a da temporada anterior [inferência]), ou None.
unsafe fn cqo11_comp(c: usize, o: usize) -> Option<usize> {
    let d = cqo11_d(c + o);
    if d == 0 { None } else { Some(cqo11_d(d)) }
}

/// O clube é da Itália (+0x53 a nação; [0x9cf360] o id da Itália).
unsafe fn is_italian_club(c: usize) -> bool {
    cqo11_comp(c, 0x53) == Some(cqo11_d(0x9cf360))
}

/// A posição na temporada anterior (+0x5f, byte com sinal; 0 o campeão [inferência]).
unsafe fn last_season_position(c: usize) -> i8 {
    *((c + 0x5f) as *const i8)
}

/// O sorteio de 0 a n − 1 (0x90cfc0: push n; add esp, 4 -> cdecl).
unsafe fn cqo11_sortear(n: i32) -> i32 {
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    sortear(n)
}

/// Embaralha `n` ponteiros a partir de `v` (0x4afb40: push n; push v; add esp junto -> cdecl; já em Rust).
unsafe fn shuffle_pointers(v: usize, n: u32) {
    let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4afb40usize);
    f(v, n);
}

/// O qsort de `n` ponteiros a partir de `v` com a comparação `cmp` (0x945333: 4 pushes; add esp, 0x10 -> cdecl).
unsafe fn sort_pointers(v: usize, n: u32, cmp: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(v, n, 4, cmp);
}

/// A lista de ponteiros (+0xee) com o máximo `max` em +0xf2 (dword): 0x944e46 de 4 × max; sem memória, a caixa `linha`.
unsafe fn alloc_entrant_list(this: usize, max: u32, arquivo: usize, linha: u32) -> Option<usize> {
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, max);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(max * 4);
    core::ptr::write_unaligned((this + 0xee) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(arquivo, linha);
        return None;
    }
    Some(l)
}

/// Põe `c` no fim da lista (+0xee) e devolve verdadeiro quando a conta chega ao máximo +0xf2 (dword).
unsafe fn push_entrant(this: usize, n: &mut i32, c: usize) -> bool {
    *n += 1;
    core::ptr::write_unaligned((cqo11_d(this + 0xee) + (*n as usize - 1) * 4) as *mut u32, c as u32);
    *n == cqo11_d(this + 0xf2) as i32
}

/// { 0x63faf0 } (thiscall, ret 0, ita_cup.cpp): `ItaCup::select_entrants()`: os 48 inscritos da Copa da Itália (+0xf2 =
/// 0x30; a lista de 0xc0 bytes em +0xee; sem memória, a caixa da linha 0x2e1 e 0). Os 32 dos grupos num vetor local:
/// os clubes italianos da temporada (passada única) que não vieram da Série B ([0x9cf574]) entre os 3 primeiros nem da
/// Série A ([0x9cf570]) entre os 15 primeiros: os da Série B atual, os que vieram da Série B em 4º, 5º ou abaixo do 17º,
/// e os que vieram da C1 ([0x9cf578]/[0x9cf57c]) em 4º, 5º ou 6º; faltando, os da C1 fora da Série B pela posição (6º,
/// 7º...). O vetor vai ao qsort (0x640590) e é repartido em 4 potes de 8: 8 voltas, cada uma tira um clube sorteado de
/// cada pote (0x90cfc0(8), seguindo para o próximo do pote se já saiu). Depois os 3 primeiros da Série B anterior e os
/// da Série A anterior do 10º ao 15º (até completar 48) e, sem conferir antes, os 9 primeiros da Série A anterior.
/// Devolve 1.
unsafe fn select_coppa_italia_entrants(this: usize) -> (i32, i32) {
    let l = match alloc_entrant_list(this, 0x30, 0x9ea23c, 0x2e1) {
        Some(l) => l,
        None => return (0, 0),
    };
    let (serie_a, serie_b, c1a, c1b) = (cqo11_d(0x9cf570), cqo11_d(0x9cf574), cqo11_d(0x9cf578), cqo11_d(0x9cf57c));
    let mut v = [0usize; 32];
    let mut k = 0usize;
    let n = ita_cup_club_count_without_nations();
    'grupos: {
        let mut i = 0;
        while i < n {
            let c = cqo11_clube(i);
            i += 1;
            if !is_italian_club(c) {
                continue;
            }
            let antes = cqo11_comp(c, 0x5b);
            if antes == Some(serie_b) && last_season_position(c) < 3 || antes == Some(serie_a) && last_season_position(c) < 0xf {
                continue;
            }
            if cqo11_comp(c, 0x57) == Some(serie_b) {
                v[k] = c;
                k += 1;
                if k == 32 {
                    break 'grupos;
                }
                continue;
            }
            let p = last_season_position(c);
            let entra = match antes {
                None => false,
                Some(x) if x == serie_b => p == 3 || p == 4 || p > 0x10,
                Some(x) => (x == c1a || x == c1b) && (p == 3 || p == 4 || p == 5),
            };
            if entra {
                v[k] = c;
                k += 1;
                if k == 32 {
                    break 'grupos;
                }
            }
        }
        let mut lugar = 5i8;
        while k < 32 {
            let mut i = 0;
            while i < n {
                let c = cqo11_clube(i);
                i += 1;
                if !is_italian_club(c) {
                    continue;
                }
                let agora = cqo11_comp(c, 0x57);
                if agora.is_none() || agora == Some(serie_b) {
                    continue;
                }
                let antes = cqo11_comp(c, 0x5b);
                if (antes == Some(c1a) || antes == Some(c1b)) && last_season_position(c) == lugar {
                    crate::comparacao::ramo("cqo_11 completar com a c1");
                    v[k] = c;
                    k += 1;
                    if k == 32 {
                        break;
                    }
                }
            }
            lugar = lugar.wrapping_add(1);
        }
    }
    sort_pointers(v.as_mut_ptr() as usize, k as u32, 0x640590);
    let mut m = 0i32;
    for _ in 0..8 {
        let mut pote = 0usize;
        while pote < 32 {
            crate::comparacao::ramo("cqo_11 potes da copa da italia");
            let mut r = cqo11_sortear(8) as i8 as i32; // cqo11: o clube sorteado de cada pote
            while v[pote + r as usize] == 0 {
                r += 1;
                if r == 8 {
                    r = 0;
                }
            }
            m += 1;
            core::ptr::write_unaligned((l + (m as usize - 1) * 4) as *mut u32, v[pote + r as usize] as u32);
            v[pote + r as usize] = 0;
            pote += 8;
        }
    }
    'b: {
        let mut i = 0;
        while i < ita_cup_club_count_without_nations() {
            let c = cqo11_clube(i);
            i += 1;
            if !is_italian_club(c) {
                continue;
            }
            let antes = cqo11_comp(c, 0x5b);
            let p = last_season_position(c);
            if antes == Some(serie_b) && p < 3 || antes == Some(serie_a) && p > 8 && p < 0xf { // cqo11: os que entram na 2ª rodada
                if push_entrant(this, &mut m, c) {
                    break 'b;
                }
            }
        }
    }
    let mut i = 0;
    while i < ita_cup_club_count_without_nations() {
        let c = cqo11_clube(i);
        i += 1;
        if is_italian_club(c) && cqo11_comp(c, 0x5b) == Some(serie_a) && last_season_position(c) < 9 {
            if push_entrant(this, &mut m, c) {
                break;
            }
        }
    }
    (1, m)
}

unsafe extern "thiscall" fn ita_cup_select_entrants(this: usize) -> i32 {
    select_coppa_italia_entrants(this).0
}

/// Lado a lado para as rotinas que alocam a lista de ponteiros +0xee e sorteiam: o +0xf2, o [0xb67a34] e o estado do
/// sorteador a partir do mesmo estado (a cópia sorteia sem contar); a original deixa a lista dela em +0xee, a cópia faz
/// a sua, comparada nos inscritos que ela gravou e liberada.
unsafe fn compare_select_entrants(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o();
        lo.set(cqo11_d(this + 0xee));
        r
    }, || {
        let (r, n) = crate::aleatorio::sem_contar(c);
        lr.set(cqo11_d(this + 0xee));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xee) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = nr.get().max(0) as usize * 4;
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    let ver = |l: usize| if l == 0 { Vec::new() } else { core::slice::from_raw_parts(l as *const u32, k / 4).to_vec() };
    let (vo, vr) = (ver(lo), ver(lr));
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} {:x?} x {:x?} lista {} ({} bytes) {:x?} x {:x?}", nome, this, a, b, igual, mo, mr, lista, k, vo, vr));
    a
}

unsafe extern "thiscall" fn comparar_ita_cup_select_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_select_entrants("ita_cup_select_entrants", this, || o(this), || select_coppa_italia_entrants(this))
}

/// { 0x63dfe0 } (thiscall, ret 0, ita_c_cup.cpp): `ItaCCup::select_entrants()`: os 90 inscritos da Copa da Série C (+0xf2
/// = 0x5a; a lista de 0x168 bytes em +0xee; sem memória, a caixa da linha 0x322 e 0): os clubes italianos da C1 e da C2
/// ([0x9cf578]..[0x9cf588]) pela ordem da tabela, até 90. Dos 80 primeiros, os que vieram da Série B ([0x9cf574]) e os
/// que vieram da C1 em 4º, 5º ou 6º vão, um a um, para as vagas 80, 81... (trocados com o que estava lá). Os 80 primeiros
/// são embaralhados (0x4afb40), ordenados pela longitude (qsort com 0x53fe50) e cada uma das 8 dezenas deles
/// embaralhada (os grupos por região [inferência]); as 10 últimas vagas ficam como estão. Devolve 1.
unsafe fn select_serie_c_cup_entrants(this: usize) -> (i32, i32) {
    let l = match alloc_entrant_list(this, 0x5a, 0x9ea204, 0x322) {
        Some(l) => l,
        None => return (0, 0),
    };
    let divisoes = [cqo11_d(0x9cf578), cqo11_d(0x9cf57c), cqo11_d(0x9cf580), cqo11_d(0x9cf584), cqo11_d(0x9cf588)];
    let mut m = 0i32;
    let mut i = 0;
    while i < ita_cup_club_count_without_nations() {
        let c = cqo11_clube(i);
        i += 1;
        if !is_italian_club(c) {
            continue;
        }
        match cqo11_comp(c, 0x57) {
            Some(d) if divisoes.contains(&d) => {
                if push_entrant(this, &mut m, c) {
                    break;
                }
            }
            _ => {}
        }
    }
    let (serie_b, c1a, c1b) = (cqo11_d(0x9cf574), cqo11_d(0x9cf578), cqo11_d(0x9cf57c));
    let mut j = 0x50i32;
    for idx in 0..0x50usize {
        let c = cqo11_d(l + idx * 4);
        let antes = match cqo11_comp(c, 0x5b) {
            Some(x) => x,
            None => continue,
        };
        let p = last_season_position(c);
        if antes == serie_b || (antes == c1a || antes == c1b) && (p == 3 || p == 4 || p == 5) {
            crate::comparacao::ramo("cqo_11 vagas do fim da copa da serie c");
            let e = l + (j as i16 as isize * 4) as usize;
            let t = cqo11_d(e);
            core::ptr::write_unaligned(e as *mut u32, c as u32);
            j += 1;
            core::ptr::write_unaligned((l + idx * 4) as *mut u32, t as u32);
            if j as i16 as i32 >= cqo11_d(this + 0xf2) as i32 {
                break;
            }
        }
    }
    shuffle_pointers(l, 0x50);
    sort_pointers(l, 0x50, 0x53fe50);
    crate::comparacao::ramo("cqo_11 dezenas da copa da serie c");
    for k in 0..8usize {
        shuffle_pointers(l + k * 0x28, 0xa); // cqo11: as dezenas da Copa da Série C
    }
    (1, m)
}

unsafe extern "thiscall" fn ita_cup_select_entrants_63dfe0(this: usize) -> i32 {
    select_serie_c_cup_entrants(this).0
}

unsafe extern "thiscall" fn comparar_ita_cup_select_entrants_63dfe0(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_select_entrants("ita_cup_select_entrants_63dfe0", this, || o(this), || select_serie_c_cup_entrants(this))
}

/// { 0x65fa60 } (thiscall, ret 0, ita_super.cpp): `ItaSuper::init_team_list()`: a lista de dois (+0x3e = 2, 0xc bytes
/// em +0xa7; sem memória, a caixa da linha 0xc2 e −1). O campeão da Série A ([0x9cf570]; sem ele, a caixa 0xcb e 0) e o
/// da Copa da Itália ([0x9cf71c]; sem ele, 0xd4 e 0); se é o mesmo clube, o segundo vira o vice da Copa (sem ele, 0xe0
/// e 0). Classe e situação 0 nos dois; devolve 1.
unsafe fn init_supercoppa_teams(this: usize) -> (i32, i32) {
    const F: usize = 0x9ea934;
    let l = match alloc_two_team_list(this, F, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm12_colocado(0x9cf570, 0); // cqo11: o campeão da Série A
    if !set_team_slot(l, 0, a, F, 0xcb) {
        return (0, 0);
    }
    let b = cqm12_colocado(0x9cf71c, 0);
    if !set_team_slot(l, 1, b, F, 0xd4) {
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqo_11 dobradinha italiana");
        if !set_team_slot(l, 1, cqm12_colocado(0x9cf71c, 1), F, 0xe0) {
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqo_11 clubes da supercopa da italia");
    finish_two_team_list(l)
}

unsafe extern "thiscall" fn ita_cup_init_team_list(this: usize) -> i32 {
    init_supercoppa_teams(this).0
}

unsafe extern "thiscall" fn comparar_ita_cup_init_team_list(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    compare_team_list("ita_cup_init_team_list", this, || o(this), || init_supercoppa_teams(this))
}
