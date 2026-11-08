// ---------- %%LEVA%%a leva: a Copa da UEFA: os inscritos (os clubes das vagas da classificação da UEFA, completados até 118 com clubes de nações fracas, e a lista dos que vêm da Intertoto) (uefa_cup.cpp) ----------

unsafe fn cqs9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs9_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// Os inscritos menos a metade (truncada, com sinal) dos clubes da preliminar (+0xb2, byte com sinal).
unsafe fn entrants_minus_half_prelim(this: usize, k: i16) -> i32 {
    k as i32 - (*((this + 0xb2) as *const i8) as i32) / 2
}

/// { 0x902b90 } (thiscall, ret 0, uefa_cup.cpp; do construtor, 2001-000): `UefaCup::build_entrants()`: +0x3e = 145; a
/// lista (calloc 0x945501(161, 6) em +0xa7; sem memória, a caixa da linha 0x1bf e −1); +0xb2 = 0.
/// 1) Pela ordem da tabela dos clubes, os que as regras europeias puseram na Copa da UEFA (+0x1db = [0x9cf6f0]), cada um
///    com a classe = o seu coeficiente (+0x1df) e a situação 0; os de coeficiente 10 ou mais (com sinal) contam em +0xb2
///    (os da preliminar [inferência]).
/// 2) Se os inscritos menos a metade de +0xb2 não chegam a 118, completa (entre os [0xae235c] − 2 × [0xae2350] primeiros
///    clubes, até chegar a 118) com os clubes sem copa europeia (+0x1db = −1) de nação (+0x53) sem a marca +0x11c e de
///    coeficiente da nação (+0x8e, word com sinal) abaixo de 2000: cada um com a classe 3, a situação 0 e +0x1db = o id da
///    copa ([+4] +0).
/// 3) Passando de 118, tira os últimos (a vaga fica zerada) até 118.
/// A lista vai ao qsort (0x945333, 6 bytes, 0x522190: pela classe). Com +0xb2 par (e não negativo): +0x36 = 0, +0x38 =
/// os inscritos + 27, a lista dos que vêm da Intertoto [inferência] (0x944e46 de +0x38 × 6 em +0x14; sem memória, a
/// caixa da linha 0x21a e 0) com os inscritos de classe 1, cada um com a classe 3, contados em +0x36; +0x3e = +0x38;
/// devolve 1. Com +0xb2 ímpar ou negativo, 0.
unsafe fn uefa_cup_build_entrants(this: usize) -> i32 {
    const F: usize = 0xaa19e0;
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x91);
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let l = zerada(0xa1, 6);
    cqs9_pd(this + 0xa7, l);
    if l == 0 {
        show_error_at(F, 0x1bf);
        return -1;
    }
    *((this + 0xb2) as *mut u8) = 0;
    let base = cqs9_d(0xae23b4);
    let mut k: i16 = 0;
    let mut i: i16 = 0;
    if (cqs9_d(0xae235c) as i32) > 0 {
        loop {
            let c = base + i as u16 as usize * 0x245;
            if cqs9_d(c + 0x1db) == cqs9_d(0x9cf6f0) {
                let e = k as i32 as usize * 6;
                k = k.wrapping_add(1);
                cqs9_pd(cqs9_d(this + 0xa7) + e, c);
                *((cqs9_d(this + 0xa7) + e + 4) as *mut u8) = *((c + 0x1df) as *const u8);
                *((cqs9_d(this + 0xa7) + e + 5) as *mut u8) = 0;
                if *((c + 0x1df) as *const i8) >= 0xa { // cqs9: os da preliminar
                    *((this + 0xb2) as *mut u8) = (*((this + 0xb2) as *const u8)).wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
            if i as i32 >= cqs9_d(0xae235c) as i32 {
                break;
            }
        }
    }
    if entrants_minus_half_prelim(this, k) < 0x76 {
        let lim = |_: ()| (cqs9_d(0xae235c) as i32).wrapping_sub((cqs9_d(0xae2350) as i32).wrapping_mul(2));
        let mut j: i16 = 0;
        if lim(()) > 0 {
            loop {
                if entrants_minus_half_prelim(this, k) >= 0x76 { // cqs9: completa até 118
                    break;
                }
                let c = base + j as u16 as usize * 0x245;
                let nac = cqs9_d(c + 0x53);
                if nac != 0 && *((nac + 0x11c) as *const u8) == 0 && cqs9_w(nac + 0x8e) < 0x7d0 && cqs9_d(c + 0x1db) as i32 == -1 {
                    crate::comparison::branch("cqs_9 clube completando a uefa");
                    let e = k as i32 as usize * 6;
                    k = k.wrapping_add(1);
                    cqs9_pd(cqs9_d(this + 0xa7) + e, c);
                    *((cqs9_d(this + 0xa7) + e + 4) as *mut u8) = 3; // cqs9: a classe de quem completa
                    *((cqs9_d(this + 0xa7) + e + 5) as *mut u8) = 0;
                    cqs9_pd(c + 0x1db, cqs9_d(cqs9_d(this + 4)));
                }
                j = j.wrapping_add(1);
                if j as i32 >= lim(()) {
                    break;
                }
            }
        }
    }
    while entrants_minus_half_prelim(this, k) > 0x76 {
        cqs9_pd(cqs9_d(this + 0xa7) + k as i32 as usize * 6 - 6, 0);
        k = k.wrapping_sub(1);
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(cqs9_d(this + 0xa7), k as i32 as u32, 6, 0x522190);
    let b2 = *((this + 0xb2) as *const i8);
    if b2 < 0 || b2 % 2 != 0 {
        return 0;
    }
    crate::comparison::branch("cqs_9 inscritos da uefa");
    core::ptr::write_unaligned((this + 0x36) as *mut i16, 0);
    let n = k.wrapping_add(0x1b);
    core::ptr::write_unaligned((this + 0x38) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 * 6) as u32);
    cqs9_pd(this + 0x14, t);
    if t == 0 {
        show_error_at(F, 0x21a);
        return 0;
    }
    if k > 0 {
        for q in 0..k as usize {
            let e = cqs9_d(this + 0xa7) + q * 6;
            let c = cqs9_d(e);
            if *((e + 4) as *const u8) == 1 && c != 0 {
                let m = cqs9_w(this + 0x36) as i32 as usize * 6;
                cqs9_pd(cqs9_d(this + 0x14) + m, c);
                *((cqs9_d(this + 0x14) + cqs9_w(this + 0x36) as i32 as usize * 6 + 4) as *mut u8) = 3;
                core::ptr::write_unaligned((this + 0x36) as *mut i16, cqs9_w(this + 0x36).wrapping_add(1));
            }
        }
    }
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    1
}

unsafe extern "thiscall" fn build_entrants(this: usize) -> i32 {
    uefa_cup_build_entrants(this)
}

/// Lado a lado a partir do mesmo estado: +0x14, +0x36..+0x39, +0x3e, +0xb2, [0xb67a34] e o +0x1db de cada clube; as duas
/// listas (+0xa7 com 161 vagas e +0x14 com +0x38 vagas) de cada lado comparadas e as da cópia liberadas.
unsafe extern "thiscall" fn comparar_build_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    let (a0, t0) = (cqs9_d(this + 0xa7), cqs9_d(this + 0x14));
    let base = cqs9_d(0xae23b4);
    let mut reg: Vec<(usize, usize)> = vec![(this + 0x36, 4), (this + 0x3e, 2), (this + 0xb2, 1), (0xb67a34, 4)];
    reg.extend((0..cqs9_d(0xae235c)).map(|i| (base + i * 0x245 + 0x1db, 4)));
    let lo = core::cell::Cell::new((0usize, 0usize));
    let lr = core::cell::Cell::new((0usize, 0usize));
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set((cqs9_d(this + 0xa7), cqs9_d(this + 0x14)));
        cqs9_pd(this + 0xa7, a0);
        cqs9_pd(this + 0x14, t0);
        r
    }, || {
        let r = uefa_cup_build_entrants(this);
        lr.set((cqs9_d(this + 0xa7), cqs9_d(this + 0x14)));
        cqs9_pd(this + 0xa7, lo.get().0);
        cqs9_pd(this + 0x14, lo.get().1);
        r
    });
    let ((ao, to), (ar, tr)) = (lo.get(), lr.get());
    let igual_em = |x: usize, y: usize, n: usize| (x == 0 && y == 0) || (x != 0 && y != 0 && core::slice::from_raw_parts(x as *const u8, n) == core::slice::from_raw_parts(y as *const u8, n));
    let listas = igual_em(ao, ar, 0xa1 * 6) && (a != 1 || igual_em(to, tr, cqs9_w(this + 0x36).max(0) as usize * 6));
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if ar != 0 && ar != ao {
        soltar(ar);
    }
    if tr != 0 && tr != to && tr != t0 {
        soltar(tr);
    }
    SCOREBOARD.record(igual && a == b && listas, || format!("build_entrants({:#x}) devolveu {} x {} memória {} listas {}", this, a, b, igual, listas));
    a
}
