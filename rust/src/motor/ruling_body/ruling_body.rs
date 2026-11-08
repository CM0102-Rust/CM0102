use super::*;

/// Matches of ban by foul code (9..12) and ruling body scale (+6); None when no ban.
///
/// [pt] Os jogos de suspensão pelo código da falta (9 a 12) e pela escala do órgão (+6), na tabela de saltos 0x7e9028;
/// None quando não há suspensão (escala abaixo de 3 nos códigos 9 e 10, código fora de 9..=12).
pub(crate) fn ban_matches(code: i32, e: i8) -> Option<u8> {
    let x = e as i32;
    Some(match code {
        9 if e < 3 => return None,
        9 if e < 8 => 1,
        9 => 2 + (e >= 0xf) as u8,
        10 if e < 3 => return None,
        10 if e < 5 => 1,
        10 if e < 9 => 2,
        10 if e < 0xd => 3,
        10 => 4 + (e >= 0x11) as u8,
        11 => ((x * 8) / 20 + 4) as u8,
        12 => ((x * 28) / 20 + 12) as u8,
        _ => return None,
    })
}

/// Days of ban (same steps in weeks) by foul code and scale; None when no ban.
///
/// [pt] Os dias de suspensão (tabela 0x7e9038): os mesmos degraus em semanas.
pub(crate) fn ban_days(code: i32, e: i8) -> Option<u16> {
    let x = e as i32;
    Some(match code {
        9 if e < 3 => return None,
        9 if e < 8 => 7,
        9 => if e >= 0xf { 21 } else { 14 },
        10 if e < 3 => return None,
        10 if e < 5 => 7,
        10 if e < 9 => 14,
        10 if e < 0xd => 21,
        10 => if e >= 0x11 { 35 } else { 28 },
        11 => (((x * 8) / 20 + 4) * 7) as u16,
        12 => (((x * 28) / 20 + 12) * 7) as u16,
        _ => return None,
    })
}

/// Builds the red card ban: the matches or days from the offence code (9 to 12) and the body's scale, today's date and
/// the player; 0 when there is no ban.
///
/// Ret 0xc: `RulingBody::make_points_ban(record, player, by_matches)`, from each country's bodies (rb_*.cpp, when
/// processing a match's red card): the offence code (+0x2e with +0x2c = 0, +0x2f with +0x2c = 1; another type: 0)
/// gives, through the body's scale (+6), the matches (by_matches ≠ 0; the days stay 0xffff) or the days (the matches
/// stay 0xff) of the ban. In the record: +0 matches, +1 = 0, +2 days, +4 today's date (`[0xae2c90]`, 0x417c10), +0x10 =
/// 3, +0x1c the player's id (+0), +0x6f = (+0x48 of the player == 0). Returns 1; with no ban, 0 without writing
/// anything.
/// Original: `RulingBody::make_points_ban` (0x7e8e10, ruling_body.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7e8e10 } (thiscall, ret 0xc, ruling_body.cpp): `RulingBody::make_points_ban(registro, jogador, por_jogos)`, pelos
/// órgãos de cada país (rb_*.cpp, ao processar o cartão vermelho de uma partida): o código da falta (+0x2e com +0x2c =
/// 0, +0x2f com +0x2c = 1; outro tipo: 0) dá, pela escala do órgão (+6), os jogos (por_jogos ≠ 0; os dias ficam
/// 0xffff) ou os dias (os jogos ficam 0xff) da suspensão. No registro: +0 jogos, +1 = 0, +2 dias, +4 a data de hoje
/// ([0xae2c90], 0x417c10), +0x10 = 3, +0x1c o id do jogador (+0), +0x6f = (+0x48 do jogador == 0). Devolve 1; sem
/// suspensão, 0 sem gravar nada.
pub(crate) unsafe extern "thiscall" fn make_points_ban(this: usize, r: usize, j: usize, by_games: u32) -> u32 {
    let code = match *((j + 0x2c) as *const u8) {
        0 => *((j + 0x2e) as *const i8) as i32, // cpu10: the foul of the first type
        1 => *((j + 0x2f) as *const i8) as i32,
        _ => return 0,
    };
    let escala = *((this + 6) as *const i8);
    let (jogos, dias) = if by_games != 0 {
        match ban_matches(code, escala) {
            Some(n) => (n, 0xffffu16),
            None => return 0,
        }
    } else {
        match ban_days(code, escala) {
            Some(n) => (0xffu8, n),
            None => return 0,
        }
    };
    crate::comparison::branch("cpu_10 punicao");
    *(r as *mut u8) = jogos; // cpu10: the suspension matches
    *((r + 1) as *mut u8) = 0;
    *((r + 2) as *mut u16) = dias;
    let mut today = [0u32; 2];
    exe::date_operator_3d(r + 4, today.as_mut_ptr() as usize, G_CURRENT_DATE);
    *((r + 0x10) as *mut u8) = 3;
    *((r + 0x1c) as *mut u32) = read_usize(j) as u32;
    *((r + 0x6f) as *mut u8) = (*((j + 0x48) as *const u8) == 0) as u8;
    1 // cpu10: the punishment carried out
}

/// At the new season, releases bans 2 (or 4) years old, frees the entries without a ban, clears the bookings of the
/// others and compacts the table.
///
/// Ret 0: `RulingBody::start_new_season()`: for each occupied disciplinary entry (14 bytes in `[this+0xb]`, count
/// +0x13; id −1 is a free slot): with a ban (+6, signed index into the 20-byte discipline table `[[0xae28f0]+4]`) 2 or
/// more years old (4 if the body has neither +0x17 nor +0x1b; the ban year in +6 against today's `[0xae2c92]`),
/// releases the ban (0x556d00) and frees the entry; without a ban, frees the entry; otherwise zeroes the bookings (+4,
/// +5, +8 and the low 6 bits of +0xd). Freeing: the player's stored slot in the body (0x55cb60 through slot +4) with
/// the same +5 gets −1 in +6; without it, writes the slot (0x55cb20, type +5, −1); the entry's id becomes −1. At the
/// end compacts the entries (0x7e9050, in this batch).
/// Original: `RulingBody::start_new_season` (0x7e7ed0, ruling_body.cpp, thiscall).
///
/// [pt] { 0x7e7ed0 } (thiscall, ret 0, ruling_body.cpp): `RulingBody::start_new_season()`: para cada entrada disciplinar
/// ocupada (14 bytes em [this+0xb], conta +0x13; id −1 é vaga livre): com suspensão (+6, índice com sinal na tabela de
/// 20 bytes da disciplina [[0xae28f0]+4]) de 2 anos ou mais (4 se o órgão não tem +0x17 nem +0x1b; o ano da suspensão
/// em +6 contra o de hoje [0xae2c92]), solta a suspensão (0x556d00) e libera a entrada; sem suspensão, libera a
/// entrada; senão zera os cartões (+4, +5, +8 e os 6 bits baixos de +0xd). Liberar: a vaga guardada do jogador no
/// órgão (0x55cb60 pela vaga +4) com o mesmo +5 fica com −1 em +6; sem ela, grava a vaga (0x55cb20, tipo +5, −1); o id
/// da entrada vira −1. No fim compacta as entradas (0x7e9050, nesta leva).
pub(crate) unsafe extern "thiscall" fn ruling_body_start_new_season(this: usize) {
    crate::comparison::branch("cpu_10 nova temporada");
    let mut e = read_usize(this + 0xb);
    let mut i = 0i32;
    while i < read_i32(this + 0x13) {
        let id = read_i32(e);
        if id != -1 {
            let s = *((e + 6) as *const i16);
            let free = if s == -1 {
                true
            } else {
                let ban = read_usize(read_usize(G_DISCIPLINE) + 4).wrapping_add((s as i32 as usize).wrapping_mul(20));
                let years = if read_i32(this + 0x17) == -1 && read_i32(this + 0x1b) == -1 { 4 } else { 2 };
                let year = *((ban + 6) as *const u16) as i32;
                if *(G_CURRENT_DATE_YEAR as *const u16) as i32 - year >= years {
                    crate::comparison::branch("cpu_10 suspensao velha");
                    // simplification: the original pushes the index with the high 16 bits of the previous edx; 0x556d00 reads only the word.
                    exe::discipline_release_ban(read_usize(G_DISCIPLINE), id as u32, s as u16 as u32);
                    true
                } else {
                    false
                }
            };
            if free {
                let id = read_i32(e);
                let g = exe::discipline_find_held_ban(read_usize(G_DISCIPLINE), id, *((this + 4) as *const i8));
                if g != 0 && *((g + 5) as *const u8) == *((this + 5) as *const u8) {
                    *((g + 6) as *mut u32) = 0xffff_ffff;
                } else {
                    exe::discipline_set_slot(read_usize(G_DISCIPLINE), read_i32(e), *((this + 4) as *const i8), *((this + 5) as *const u8), 0xffff_ffff);
                }
                *(e as *mut u32) = 0xffff_ffff;
            } else {
                let f = *((e + 0xd) as *const u8);
                *((e + 4) as *mut u8) = 0;
                *((e + 5) as *mut u8) = 0;
                *((e + 8) as *mut u16) = 0;
                *((e + 0xd) as *mut u8) = f & 0xc0;
            }
        }
        i += 1;
        e += 0xe;
    }
    exe::rulingbody_compact_entries(this);
}

/// Comparador: a original solta suspensões e realoca (não pode rodar duas vezes): só ela roda.
// simplification: the copy carries the ”cpu_10 nova temporada” counter to prove in Rust mode; the compaction (compared) checks the end.

/// Removes the free entries from the body's discipline table, fixes the stored index of each moved player and
/// reallocates with 100 spare rows.
///
/// Ret 0: `RulingBody::compact_entries()`: removes the free entries (id −1) from the 14-byte table `[this+0xb]`,
/// bringing the occupied ones to the front; each one that moves fixes the player's stored index (0x55cb60 through slot
/// +4: with the same +5, the new index in +6; without it, 0x55cb20 with type +5 and the index). Count +0x13 = n,
/// capacity +0xf = n + 100 and reallocates (0x945667) with (n + 100) × 14 bytes; out of memory, the box (line 0x594)
/// and continues.
/// Original: `RulingBody::compact_entries` (0x7e9050, ruling_body.cpp, thiscall).
///
/// [pt] { 0x7e9050 } (thiscall, ret 0, ruling_body.cpp): `RulingBody::compact_entries()`: tira as entradas livres (id −1)
/// da tabela de 14 bytes [this+0xb], trazendo as ocupadas para a frente; cada uma que muda de lugar acerta o índice
/// guardado do jogador (0x55cb60 pela vaga +4: com o mesmo +5, o novo índice em +6; sem ela, 0x55cb20 com o tipo +5 e
/// o índice). Conta +0x13 = n, capacidade +0xf = n + 100 e realoca (0x945667) com (n + 100) × 14 bytes; sem memória, a
/// caixa (linha 0x594) e segue.
pub(crate) unsafe extern "thiscall" fn compact_entries(this: usize) {
    let mut target = read_usize(this + 0xb);
    let mut source = target;
    let mut n = 0u32;
    let mut i = 0i32;
    while i < read_i32(this + 0x13) {
        if read_i32(source) != -1 {
            if source != target {
                core::ptr::copy(source as *const u8, target as *mut u8, 0xe);
                let g = exe::discipline_find_held_ban(read_usize(G_DISCIPLINE), read_i32(source), *((this + 4) as *const i8));
                if g != 0 && *((g + 5) as *const u8) == *((this + 5) as *const u8) {
                    *((g + 6) as *mut u32) = n;
                } else {
                    exe::discipline_set_slot(read_usize(G_DISCIPLINE), read_i32(source), *((this + 4) as *const i8), *((this + 5) as *const u8), n);
                }
            }
            target += 0xe;
            n += 1;
        }
        i += 1;
        source += 0xe;
    }
    *((this + 0x13) as *mut u32) = n;
    *((this + 0xf) as *mut u32) = n + 100;
    let p = exe::crt_realloc(read_usize(this + 0xb), (n + 100) * 14);
    *((this + 0xb) as *mut u32) = p as u32;
    if p == 0 {
        show_error_at(0xa7a858, 0x594);
    }
}

