use super::*;

/// Entrants k minus half (truncated) the preliminary-round count (+0xb2).
///
/// [pt] Os inscritos menos a metade (truncada, com sinal) dos clubes da preliminar (+0xb2, byte com sinal).
pub(crate) unsafe fn entrants_minus_half_prelim(this: Cup, k: i16) -> i32 {
    k as i32 - (read_i8(this.addr() + uefa_cup::PRELIMINARY_CLUB_COUNT) as i32) / 2
}

/// `UefaCup::build_entrants()` (0x902b90, thiscall, ret 0, uefa_cup.cpp; called from the constructor, 2001-000): builds
/// and sorts the 118-entrant list and the Intertoto list; returns 1, 0 or −1.
///
/// +0x3e = 145; the list (calloc `exe::crt_calloc`(161, 6) at +0xa7; out of memory, the error box for line 0x1bf and −1); +0xb2
/// = 0.
///
/// 1) In club table order, the clubs the European rules put in the UEFA Cup (+0x1db = [0x9cf6f0]), each with class =
/// its coefficient (+0x1df) and status 0; those with coefficient 10 or more (signed) are counted in +0xb2 (the
/// preliminary-round ones [inference]).
///
/// 2) If the entrants minus half of +0xb2 do not reach 118, it fills up (among the first [0xae235c] − 2 × [0xae2350]
/// clubs, until reaching 118) with clubs with no European cup (+0x1db = −1) from a nation (+0x53) without the +0x11c
/// mark and with a nation coefficient (+0x8e, signed word) below 2000: each with class 3, status 0 and +0x1db = the cup
/// id ([+4] +0).
///
/// 3) If over 118, removes the last ones (the slot is zeroed) down to 118.
///
/// The list goes to qsort (`exe::crt_qsort`, 6 bytes, `cup_compare_entries_by_level_desc`: by class). With +0xb2 even (and not negative): +0x36 = 0, +0x38
/// = the entrants + 27, the list of those coming from the Intertoto [inference] (`exe::crt_malloc` of +0x38 × 6 at +0x14; out of
/// memory, the error box for line 0x21a and 0) with the class-1 entrants, each set to class 3 and counted in +0x36;
/// +0x3e = +0x38; returns 1. With +0xb2 odd or negative, 0.
///
/// [pt] { 0x902b90 } (thiscall, ret 0, uefa_cup.cpp; do construtor, 2001-000): `UefaCup::build_entrants()`: +0x3e = 145; a
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
pub(crate) unsafe fn uefa_cup_build_entrants(this: Cup) -> i32 {
    const F: usize = 0xaa19e0;
    this.set_team_count(0x91);
    let l = exe::crt_calloc(0xa1, 6);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(F, 0x1bf);
        return -1;
    }
    write_u8(this.addr() + uefa_cup::PRELIMINARY_CLUB_COUNT, 0);
    let base = read_usize(G_CLUB_TABLE);
    let mut k: i16 = 0;
    let mut i: i16 = 0;
    if (read_usize(G_CLUB_COUNT) as i32) > 0 {
        loop {
            let c = base + i as u16 as usize * club::SIZE;
            if read_usize(c + club::CONTINENTAL_COMP_ID) == read_usize(KEY_CLUB_COMP_UEFA_CUP) {
                let e = k as i32 as usize * 6;
                k = k.wrapping_add(1);
                write_usize(this.team_table() + e, c);
                write_u8(this.team_table() + e + 4, read_u8(c + club::EURO_SEEDING));
                write_u8(this.team_table() + e + 5, 0);
                if read_i8(c + club::EURO_SEEDING) >= 0xa { // cqs9: those in the preliminary
                    write_u8(this.addr() + uefa_cup::PRELIMINARY_CLUB_COUNT, read_u8(this.addr() + uefa_cup::PRELIMINARY_CLUB_COUNT).wrapping_add(1));
                }
            }
            i = i.wrapping_add(1);
            if i as i32 >= read_usize(G_CLUB_COUNT) as i32 {
                break;
            }
        }
    }
    if entrants_minus_half_prelim(this, k) < 0x76 {
        let lim = |_: ()| (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
        let mut j: i16 = 0;
        if lim(()) > 0 {
            loop {
                if entrants_minus_half_prelim(this, k) >= 0x76 { // cqs9: fills up to 118
                    break;
                }
                let c = base + j as u16 as usize * club::SIZE;
                let nation = read_usize(c + club::NATION);
                if nation != 0 && *((nation + nation::LEAGUE_SELECTED) as *const u8) == 0 && read_i16(nation + nation::REPUTATION) < 0x7d0 && read_usize(c + club::CONTINENTAL_COMP_ID) as i32 == -1 {
                    crate::comparison::branch("cqs_9 club filling uefa");
                    let e = k as i32 as usize * 6;
                    k = k.wrapping_add(1);
                    write_usize(this.team_table() + e, c);
                    write_u8(this.team_table() + e + 4, 3); // cqs9: the class of those who complete it
                    write_u8(this.team_table() + e + 5, 0);
                    write_usize(c + club::CONTINENTAL_COMP_ID, read_usize(read_usize(this.addr() + 4)));
                }
                j = j.wrapping_add(1);
                if j as i32 >= lim(()) {
                    break;
                }
            }
        }
    }
    while entrants_minus_half_prelim(this, k) > 0x76 {
        write_usize(this.team_table() + k as i32 as usize * 6 - 6, 0);
        k = k.wrapping_sub(1);
    }
    exe::crt_qsort(this.team_table(), k as i32 as u32, 6, 0x522190);
    let b2 = read_i8(this.addr() + uefa_cup::PRELIMINARY_CLUB_COUNT);
    if b2 < 0 || b2 % 2 != 0 {
        return 0;
    }
    crate::comparison::branch("cqs_9 uefa entrants");
    this.set_entrant_count(0);
    let n = k.wrapping_add(0x1b);
    this.set_entrant_capacity(n);
    let t = exe::crt_malloc((n as i32 * 6) as u32);
    this.set_entrant_book(t);
    if t == 0 {
        show_error_at(F, 0x21a);
        return 0;
    }
    if k > 0 {
        for q in 0..k as usize {
            let e = this.team_table() + q * 6;
            let c = read_usize(e);
            if read_u8(e + 4) == 1 && c != 0 {
                let m = this.entrant_count() as i32 as usize * 6;
                write_usize(this.entrant_book() + m, c);
                write_u8(this.entrant_book() + this.entrant_count() as i32 as usize * 6 + 4, 3);
                this.set_entrant_count(this.entrant_count().wrapping_add(1));
            }
        }
    }
    this.set_team_count(n);
    1
}

/// UefaCup::build_entrants hook entry: builds and sorts the 118-club UEFA Cup entrants list and the Intertoto list (see
/// uefa_cup_build_entrants); returns 1, 0 or -1.
pub(crate) unsafe extern "thiscall" fn build_entrants(this: usize) -> i32 {
    uefa_cup_build_entrants(Cup(this))
}

/// rule: the UEFA Cup has, only when there are preliminary clubs (+0xb2, those with coefficient 10 or more
/// [inference]), a preliminary round drawn on 28/6 and played on 6/7 (b teams); then: 1st qualifying round drawn on
/// 28/7 and played on 10/8 (82 teams, 0x52 − b/2 new), 2nd on 14/9 (96 teams, 55 new), 1st round on 19/10 (48 teams),
/// 2nd on 23/11 (32 teams, 8 new), 3rd on 1/3 of the following year, round of 16 on 15/3, quarter-finals on 5/4 and the
/// single-match final on 16/5 (venue 4, prize of 1.5 million to the winner); two legs 14 days apart (7 in the 3rd round
/// and the round of 16); category 7 in qualifying and 8 after
/// Original: `UefaCup::create_round_table` (0x9025c0, uefa_cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x9025c0 } (thiscall, ret 0x10, uefa_cup.cpp; a virtual +0x3c da Copa da UEFA): `fase.create_round_table(/// &rodadas, &extra, &saida)`: na fase −1, saida (se pedida) = 0, rodadas = 9, extra = 0; a tabela (0x944e46 de 0x3a8;
/// sem memória, a caixa da linha 0xe0 e 0) com as rodadas de `uefa_cup_rounds` e rodadas = as gravadas. Fora da −1,
/// 0.
pub(crate) unsafe extern "thiscall" fn uefa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = Cup(this);
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = 9;
    *extra = 0;
    let t = exe::crt_malloc(0x3a8);
    if t == 0 {
        show_error_at(0xaa19e0, 0xe0);
        return 0;
    }
    crate::comparison::branch("cqq_10 uefa fixtures");
    let year = this.year() as i32 as u32;
    let (tab, n) = uefa_cup_rounds(read_i8(this.addr() + uefa_cup::PRELIMINARY_CLUB_COUNT));
    for (i, r) in tab[..n].iter().enumerate() {
        eng_cc_cup_write_cup_round(t, i as u32, r, year);
    }
    *rounds = n as i16;
    t
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
pub(crate) unsafe extern "thiscall" fn uefa_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, a3: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let club = Club(club);
    let d = outcome as u8;
    if d == 1 || d == 2 || d == 0xff {
        if d == 0xff {
            crate::comparison::branch("cqi_10 dropped into uefa");
        }
        write_i32(club.addr() + club::CONTINENTAL_COMP_ID, -1); // cqi10: whoever finishes in the UEFA leaves the European competition
    }
    exe::cup_update_staff_records_for_status(this, club, outcome, a3, a4, game, a6)
}

