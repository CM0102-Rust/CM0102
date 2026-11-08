use super::*;

// ---------- 284a leva: a Copa da UEFA: os inscritos (os clubes das vagas da classificação da UEFA, completados até 118 com clubes de nações fracas, e a lista dos que vêm da Intertoto) (uefa_cup.cpp) ----------




/// Os inscritos menos a metade (truncada, com sinal) dos clubes da preliminar (+0xb2, byte com sinal).
pub(crate) unsafe fn cqs9_conta(this: usize, k: i16) -> i32 {
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
pub(crate) unsafe fn cqs9_inscritos(this: usize) -> i32 {
    const F: usize = 0xaa19e0;
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x91);
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let l = zerada(0xa1, 6);
    write_u32_from_usize(this + 0xa7, l);
    if l == 0 {
        aviso_em(F, 0x1bf);
        return -1;
    }
    *((this + 0xb2) as *mut u8) = 0;
    let base = read_u32_as_usize(G_CLUB_TABLE);
    let mut k: i16 = 0;
    let mut i: i16 = 0;
    if (read_u32_as_usize(G_CLUB_COUNT) as i32) > 0 {
        loop {
            let c = base + i as u16 as usize * 0x245;
            if read_u32_as_usize(c + 0x1db) == read_u32_as_usize(KEY_CLUB_COMP_UEFA_CUP) {
                let e = k as i32 as usize * 6;
                k = k.wrapping_add(1);
                write_u32_from_usize(read_u32_as_usize(this + 0xa7) + e, c);
                *((read_u32_as_usize(this + 0xa7) + e + 4) as *mut u8) = *((c + 0x1df) as *const u8);
                *((read_u32_as_usize(this + 0xa7) + e + 5) as *mut u8) = 0;
                if *((c + 0x1df) as *const i8) >= 0xa { // cqs9: os da preliminar
                    *((this + 0xb2) as *mut u8) = (*((this + 0xb2) as *const u8)).wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
            if i as i32 >= read_u32_as_usize(G_CLUB_COUNT) as i32 {
                break;
            }
        }
    }
    if cqs9_conta(this, k) < 0x76 {
        let lim = |_: ()| (read_u32_as_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_u32_as_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
        let mut j: i16 = 0;
        if lim(()) > 0 {
            loop {
                if cqs9_conta(this, k) >= 0x76 { // cqs9: completa até 118
                    break;
                }
                let c = base + j as u16 as usize * 0x245;
                let nac = read_u32_as_usize(c + 0x53);
                if nac != 0 && *((nac + 0x11c) as *const u8) == 0 && read_i16(nac + 0x8e) < 0x7d0 && read_u32_as_usize(c + 0x1db) as i32 == -1 {
                    crate::comparacao::ramo("cqs_9 clube completando a uefa");
                    let e = k as i32 as usize * 6;
                    k = k.wrapping_add(1);
                    write_u32_from_usize(read_u32_as_usize(this + 0xa7) + e, c);
                    *((read_u32_as_usize(this + 0xa7) + e + 4) as *mut u8) = 3; // cqs9: a classe de quem completa
                    *((read_u32_as_usize(this + 0xa7) + e + 5) as *mut u8) = 0;
                    write_u32_from_usize(c + 0x1db, read_u32_as_usize(read_u32_as_usize(this + 4)));
                }
                j = j.wrapping_add(1);
                if j as i32 >= lim(()) {
                    break;
                }
            }
        }
    }
    while cqs9_conta(this, k) > 0x76 {
        write_u32_from_usize(read_u32_as_usize(this + 0xa7) + k as i32 as usize * 6 - 6, 0);
        k = k.wrapping_sub(1);
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    ordenar(read_u32_as_usize(this + 0xa7), k as i32 as u32, 6, 0x522190);
    let b2 = *((this + 0xb2) as *const i8);
    if b2 < 0 || b2 % 2 != 0 {
        return 0;
    }
    crate::comparacao::ramo("cqs_9 inscritos da uefa");
    core::ptr::write_unaligned((this + 0x36) as *mut i16, 0);
    let n = k.wrapping_add(0x1b);
    core::ptr::write_unaligned((this + 0x38) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar((n as i32 * 6) as u32);
    write_u32_from_usize(this + 0x14, t);
    if t == 0 {
        aviso_em(F, 0x21a);
        return 0;
    }
    if k > 0 {
        for q in 0..k as usize {
            let e = read_u32_as_usize(this + 0xa7) + q * 6;
            let c = read_u32_as_usize(e);
            if *((e + 4) as *const u8) == 1 && c != 0 {
                let m = read_i16(this + 0x36) as i32 as usize * 6;
                write_u32_from_usize(read_u32_as_usize(this + 0x14) + m, c);
                *((read_u32_as_usize(this + 0x14) + read_i16(this + 0x36) as i32 as usize * 6 + 4) as *mut u8) = 3;
                core::ptr::write_unaligned((this + 0x36) as *mut i16, read_i16(this + 0x36).wrapping_add(1));
            }
        }
    }
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    1
}

pub(crate) unsafe extern "thiscall" fn cqs9_uefa_inscritos(this: usize) -> i32 {
    cqs9_inscritos(this)
}

/// Lado a lado a partir do mesmo estado: +0x14, +0x36..+0x39, +0x3e, +0xb2, [0xb67a34] e o +0x1db de cada clube; as duas
/// listas (+0xa7 com 161 vagas e +0x14 com +0x38 vagas) de cada lado comparadas e as da cópia liberadas.
pub(crate) unsafe extern "thiscall" fn comparar_cqs9_uefa_inscritos(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(2086);
    let (a0, t0) = (read_u32_as_usize(this + 0xa7), read_u32_as_usize(this + 0x14));
    let base = read_u32_as_usize(G_CLUB_TABLE);
    let mut reg: Vec<(usize, usize)> = vec![(this + 0x36, 4), (this + 0x3e, 2), (this + 0xb2, 1), (0xb67a34, 4)];
    reg.extend((0..read_u32_as_usize(G_CLUB_COUNT)).map(|i| (base + i * 0x245 + 0x1db, 4)));
    let lo = core::cell::Cell::new((0usize, 0usize));
    let lr = core::cell::Cell::new((0usize, 0usize));
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set((read_u32_as_usize(this + 0xa7), read_u32_as_usize(this + 0x14)));
        write_u32_from_usize(this + 0xa7, a0);
        write_u32_from_usize(this + 0x14, t0);
        r
    }, || {
        let r = cqs9_inscritos(this);
        lr.set((read_u32_as_usize(this + 0xa7), read_u32_as_usize(this + 0x14)));
        write_u32_from_usize(this + 0xa7, lo.get().0);
        write_u32_from_usize(this + 0x14, lo.get().1);
        r
    });
    let ((ao, to), (ar, tr)) = (lo.get(), lr.get());
    let igual_em = |x: usize, y: usize, n: usize| (x == 0 && y == 0) || (x != 0 && y != 0 && core::slice::from_raw_parts(x as *const u8, n) == core::slice::from_raw_parts(y as *const u8, n));
    let listas = igual_em(ao, ar, 0xa1 * 6) && (a != 1 || igual_em(to, tr, read_i16(this + 0x36).max(0) as usize * 6));
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    if ar != 0 && ar != ao {
        soltar(ar);
    }
    if tr != 0 && tr != to && tr != t0 {
        soltar(tr);
    }
    PLACAR.registrar(igual && a == b && listas, || format!("cqs9_uefa_inscritos({:#x}) devolveu {} x {} memória {} listas {}", this, a, b, igual, listas));
    a
}

/// rule: the UEFA Cup has, only when there are preliminary clubs (+0xb2, those with coefficient 10 or more
/// [inference]), a preliminary round drawn on 28/6 and played on 6/7 (b teams); then: 1st qualifying round drawn on
/// 28/7 and played on 10/8 (82 teams, 0x52 − b/2 new), 2nd on 14/9 (96 teams, 55 new), 1st round on 19/10 (48 teams),
/// 2nd on 23/11 (32 teams, 8 new), 3rd on 1/3 of the following year, round of 16 on 15/3, quarter-finals on 5/4 and the
/// single-match final on 16/5 (venue 4, prize of 1.5 million to the winner); two legs 14 days apart (7 in the 3rd round
/// and the round of 16); category 7 in qualifying and 8 after
/// Original: `UefaCup::create_round_table` (0x9025c0, uefa_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x9025c0 } (thiscall, ret 0x10, uefa_cup.cpp; a virtual +0x3c da Copa da UEFA): `UefaCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, saida (se pedida) = 0, rodadas = 9, extra = 0; a tabela (0x944e46 de 0x3a8;
/// sem memória, a caixa da linha 0xe0 e 0) com as rodadas de `cqq10_uefa_rodadas` e rodadas = as gravadas. Fora da −1,
/// 0.
pub(crate) unsafe extern "thiscall" fn cqq10_uefa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = 9;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar(0x3a8);
    if t == 0 {
        aviso_em(0xaa19e0, 0xe0);
        return 0;
    }
    crate::comparacao::ramo("cqq_10 tabela da uefa");
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    let (tab, n) = cqq10_uefa_rodadas(*((this + 0xb2) as *const i8));
    for (i, r) in tab[..n].iter().enumerate() {
        cql10_rodada(t, i as u32, r, ano);
    }
    *rodadas = n as i16;
    t
}

pub(crate) unsafe extern "thiscall" fn comparar_cqq10_uefa_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1977);
    cql10_comparar("cqq10_uefa_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cqq10_uefa_tabela(this, fase, r, e, s))
}

/// rule: in the UEFA Cup, the winner, the runner-up and every knocked-out club leave European competition (the club
/// mark goes back to −1); the career records are the base cup ones (winner, runner-up, qualification and knock-out by
/// round)
/// Original: `UefaCup::update_staff_records_for_status` (0x9030e0, uefa_cup.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x9030e0 } (thiscall, ret 0x18, uefa_cup.cpp; a virtual +0x44 da Copa da UEFA): `UefaCup::
/// update_staff_records_for_status(clube, desfecho, a3, a4, jogo, a6)`: com o desfecho (byte) 1, 2 ou 0xff (campeão,
/// vice ou eliminado) o clube sai da competição europeia (+0x1db = −1); depois os registros da base (0x519890: os seis
/// argumentos como chegaram, o desfecho como dword; mov ecx, this -> thiscall, ret 0x18), cuja devolução sai.
pub(crate) unsafe extern "thiscall" fn cqi10_uefa_registros(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let d = desfecho as u8;
    if d == 1 || d == 2 || d == 0xff {
        if d == 0xff {
            crate::comparacao::ramo("cqi_10 caiu na uefa");
        }
        core::ptr::write_unaligned((clube + 0x1db) as *mut i32, -1); // cqi10: quem termina na UEFA sai da competição europeia
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    base(this, clube, desfecho, a3, a4, jogo, a6)
}

pub(crate) unsafe extern "thiscall" fn comparar_cqi10_uefa_registros(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1687);
    if clube == 0 {
        return o(this, clube, desfecho, a3, a4, jogo, a6);
    }
    let d = desfecho as u8;
    let antes = read_u32_as_usize(clube + 0x1db);
    let previsto = if d == 1 || d == 2 || d == 0xff { 0xffff_ffff } else { antes };
    let r = o(this, clube, desfecho, a3, a4, jogo, a6);
    let depois = read_u32_as_usize(clube + 0x1db);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("cqi10_uefa_registros({:#x}, desfecho {}) +0x1db {:x} -> {:x}, previsto {:x}, devolveu {}", clube, d, antes, depois, previsto, r));
    r
}
