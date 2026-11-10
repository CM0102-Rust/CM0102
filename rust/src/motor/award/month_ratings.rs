use super::*;

/// Staff record of id (G_STAFF_TABLE, 0x6e bytes).
///
/// [pt] A pessoa `id` da tabela de pessoas ([0xae23bc], 0x6e bytes cada).
pub(crate) unsafe fn month_ratings_staff_by_id(id: i32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE))
}

/// Records every player's rating from a match into the month ratings.
///
/// `MONTH_RATINGS::record_match_ratings(game, match)`, called by 0x7683a0 for every match of the leagues that have a
/// monthly award. Both sides of the match (+0x190 and +0x190 + 0x625), 20 slots of 0x49 bytes each: except the slot
/// marked 0xfb at +0x1d and, from the 12th slot on, the substitute who did not come on (+0x32 = 0xff), the person (id
/// at +0) below the person count [0xae2364] goes to the month ratings (`add_player_rating`).
/// Original: `MONTH_RATINGS::record_match_ratings` (0x767ee0, month_ratings.cpp, thiscall, ret 8).
///
/// [pt] { 0x767ee0 } (thiscall, ret 8, month_ratings.cpp): `MONTH_RATINGS::record_match_ratings(jogo, partida)`, pelo
/// 0x7683a0 a cada partida das ligas com prêmio do mês. Os dois lados da partida (+0x190 e +0x190 + 0x625), 20 vagas de
/// 0x49 bytes cada: fora a vaga marcada 0xfb em +0x1d e, da 12ª em diante, a reserva que não entrou (+0x32 = 0xff), a
/// pessoa (id em +0) abaixo da conta de pessoas [0xae2364] vai para as notas do mês (0x767fa0).
pub(crate) unsafe extern "thiscall" fn record_match_ratings(this: usize, game: usize, partida: usize) {
    let _ = game;
    let mut side = partida.wrapping_add(0x190);
    for l in 0..2 { // cpw5: os dois lados
        for k in 0..0x14usize {
            let e = side + k * 0x49;
            if read_u8(e + 0x1d) == 0xfb {
                continue;
            }
            if k >= 0xb && read_u8(e + 0x32) == 0xff {
                continue;
            }
            let p = month_ratings_staff_by_id(read_i32(e));
            if p != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            if l == 1 {
                crate::comparison::branch("cpw_5 away side ratings");
            }
            exe::month_ratings_add_player_rating(this, e);
        }
        side += 0x625;
    }
}

/// Adds a slot's match rating (+0x35) to word +8 of a 12-byte record and counts one more game at +0xa.
///
/// [pt] A nota da partida (+0x35 da vaga, sem sinal) soma na palavra +8 do registro de 12 bytes e o byte +0xa conta mais um
/// jogo.
pub(crate) unsafe fn add_match_rating(r: usize, e: usize) {
    let s = (r + 8) as *mut u16;
    core::ptr::write_unaligned(s, core::ptr::read_unaligned(s).wrapping_add(read_u8(e + 0x35) as u16));
    *((r + 0xa) as *mut u8) = read_u8(r + 0xa).wrapping_add(1);
}

/// Finds or appends the person's record in the month ratings table, adds the rating and counts one match.
///
/// `MONTH_RATINGS::add_player_rating(slot)`. The table at [this] (count at +4, capacity at +8) has 12-byte records: +0
/// person, +4 club's league (or −1), +8 sum of the ratings, +0xa matches, +0xb age. It finds the slot's person (+0) or
/// appends it (growing by 300 with malloc `exe::crt_malloc` or realloc `exe::crt_realloc` when full; the league from the person's club
/// +0x39 and that club's +0x57; the age from the person's +0x18) and adds the rating.
/// Original: `MONTH_RATINGS::add_player_rating` (0x767fa0, month_ratings.cpp, thiscall, ret 4).
///
/// [pt] { 0x767fa0 } (thiscall, ret 4, month_ratings.cpp): `MONTH_RATINGS::add_player_rating(vaga)`. A tabela em [this]
/// (conta em +4, capacidade em +8) tem registros de 12 bytes: +0 pessoa, +4 liga do clube (ou −1), +8 soma das notas,
/// +0xa jogos, +0xb idade. Acha a pessoa da vaga (+0) ou acrescenta (cresce 300 com malloc 0x944e46 ou realloc
/// 0x945667 quando cheia; a liga pelo clube +0x39 da pessoa e o +0x57 dele; a idade pelo +0x18 da pessoa) e soma a nota.
pub(crate) unsafe extern "thiscall" fn add_player_rating(this: usize, e: usize) {
    let id = read_i32(e);
    let n = read_i32(this + 4);
    let mut table = read_usize(this);
    for k in 0..n.max(0) as usize {
        let r = table + k * 12;
        if read_i32(r) == id {
            return add_match_rating(r, e);
        }
    }
    let cap = read_i32(this + 8);
    if n == cap {
        let new = cap.wrapping_add(300);
        write_u32(this + 8, new as u32);
        let bytes = (new as u32).wrapping_mul(12);
        table = if table == 0 {
            exe::crt_malloc(bytes)
        } else {
            exe::crt_realloc(table, bytes)
        };
        if table == 0 {
            // simplification: out of memory (the original’s ”Error” box): restores the capacity and lets the original redo it
            write_u32(this + 8, cap as u32);
            return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_PLAYER_RATING)(this, e);
        }
        write_u32(this, table as u32);
    }
    let p = Staff(month_ratings_staff_by_id(id));
    let club = Club((p.club_job()).addr());
    let mut league = -1i32;
    if !club.is_null() {
        let l = club.division().addr();
        if l != 0 {
            league = read_i32(l);
        }
    }
    let r = table + (n as usize) * 12;
    write_u32(r, id as u32);
    write_u32(r + 4, league as u32);
    core::ptr::write_unaligned((r + 8) as *mut u16, 0);
    *((r + 0xb) as *mut u8) = (p.age() as u8);
    crate::comparison::branch("cpw_5 new player in the month");
    *((r + 0xa) as *mut u8) = 0; // cpw5: starts with no matches
    write_u32(this + 4, n.wrapping_add(1) as u32);
    add_match_rating(r, e);
}

/// Sorts the month ratings by average and returns the best player of a league, with an optional age filter.
///
/// `MONTH_RATINGS::find_best_player(league, age)`, used by the monthly award (`award_best_rated_player`). It sorts the table by average
/// (qsort `exe::crt_qsort` with 0x768200: those with fewer than 3 matches go to the end) and returns the first record of the
/// league (−1: any) with at least 3 matches (then 2, 1 and 0); with an age (not 0xff), those above it are only counted
/// and the youngest only qualifies after 3 have been counted; with a league, the current league of the person's club
/// (+0x39, +0x57) must also be the requested one. None: 0.
/// Original: `MONTH_RATINGS::find_best_player` (0x768130, month_ratings.cpp, thiscall, ret 8).
///
/// [pt] { 0x768130 } (thiscall, ret 8, month_ratings.cpp): `MONTH_RATINGS::find_best_player(liga, idade)`, pelo prêmio do
/// mês (0x767150). Ordena a tabela pela média (qsort 0x945333 com 0x768200: quem tem menos de 3 jogos vai para o fim) e
/// devolve o primeiro registro da liga (−1: qualquer) com pelo menos 3 jogos (depois 2, 1 e 0); com idade (não 0xff),
/// os acima dela só contam e o mais novo só vale depois de 3 contados; com liga, a liga atual do clube da pessoa (+0x39,
/// +0x57) também tem de ser a pedida. Nenhum: 0.
pub(crate) unsafe extern "thiscall" fn find_best_player(this: usize, league: i32, age: u32) -> usize {
    crate::comparison::branch("cpw_5 sorted the ratings");
    exe::crt_qsort(read_usize(this), read_usize(this + 4) as u32, 0xc, 0x768200); // cpw5: sorts by the average
    let table = read_usize(this);
    let n = read_i32(this + 4);
    let age = age as u8;
    let mut contados = 0i32;
    let mut minimum = 3i8;
    loop {
        for k in 0..n.max(0) as usize {
            let r = table + k * 12;
            if league != -1 && read_i32(r + 4) != league {
                continue;
            }
            if (read_u8(r + 0xa) as i8) < minimum {
                continue;
            }
            if age != 0xff {
                if read_u8(r + 0xb) as i8 > age as i8 {
                    contados += 1;
                    continue;
                }
                if contados < 3 {
                    continue;
                }
            }
            if league == -1 {
                return r;
            }
            let c = Club(read_usize(month_ratings_staff_by_id(read_i32(r)) + 0x39));
            if !c.is_null() {
                let l = c.division().addr();
                if l != 0 && read_i32(l) == league {
                    return r;
                }
            }
        }
        minimum -= 1;
        if minimum < 0 {
            return 0;
        }
    }
}

