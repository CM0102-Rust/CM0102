use super::*;

/// Club record with the given id (0x245 bytes each).
///
/// [pt] O clube `id` ([0xae23b4], 0x245 bytes cada).
pub(crate) unsafe fn month_award_club_by_id(id: i32) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE))
}

/// The comparers' rehearsal run. The copy runs without the news item (`exe::month_award_post_winner_news`), without the award record (`add_month_award_record`)
/// and without the club lists (`add_player_to_club_shortlists`), because these allocate and cannot run twice.
///
/// [pt] O ensaio dos comparadores: a cópia roda sem a notícia (0x767330), sem o registro do prêmio (0x86c8c0) e sem as
/// listas dos clubes (0x842c00), que alocam e não podem rodar duas vezes.
pub(crate) static mut MONTH_AWARD_DRY_RUN: bool = false;

/// Picks the best player of the month ratings with the requested age and records him as the month winner, with his
/// club.
///
/// `MONTH_AWARD::award_best_rated_player(ratings, age)`, called at the turn of the month by the nations' awards
/// (England's at 0x579a37..0x579adc). It takes the best of the ratings (`find_best_player`) in the award's league (`[+9]`, or −1)
/// and with the requested age; when one is found, it records the winner (`set_winner`) with the person's club (+0x39, or
/// −1), and the month (+0xc) and year (+0xd) of the ratings. The high bytes of the month and the year are passed
/// exactly as the original pushes them (taken from the person table in the register).
/// Original: `MONTH_AWARD::award_best_rated_player` (0x767150, month_award.cpp, thiscall, ret 8).
///
/// [pt] { 0x767150 } (thiscall, ret 8, month_award.cpp): `MONTH_AWARD::award_best_rated_player(notas, idade)`, na virada do
/// mês pelos prêmios das nações (os da Inglaterra em 0x579a37..0x579adc). O melhor das notas (0x768130) na liga do
/// prêmio ([+9], ou −1) e com a idade pedida; achou: grava o vencedor (0x7671c0) com o clube da pessoa (+0x39, ou −1),
/// o mês (+0xc) e o ano (+0xd) das notas. Os bytes altos do mês e do ano vão como a original os empurra (da tabela de
/// pessoas no registrador).
pub(crate) unsafe extern "thiscall" fn award_best_rated_player(this: usize, notes: usize, age: u32) {
    let league = {
        let l = read_usize(this + 9);
        if l != 0 {
            read_i32(l)
        } else {
            -1
        }
    };
    let r = exe::month_ratings_find_best_player(notes, league, age);
    if r == 0 {
        return;
    }
    crate::comparison::branch("cpw_6 awarded the best of the month");
    let id = read_i32(r);
    let table = read_usize(G_STAFF_TABLE) as u32;
    let club = read_usize(month_ratings_staff_by_id(id) + 0x39);
    let year = (table & 0xffff0000) | read_i16(notes + 0xd) as u16 as u32;
    let month = read_u8(notes + 0xc) as u32;
    if club != 0 {
        exe::month_award_set_winner(this, id, read_i32(club), (year & 0xffffff00) | month, year);
    } else {
        exe::month_award_set_winner(this, id, -1, month, year);
    }
}

// simplification: its job is to call the player of the month and the winner’s slot, which have their own comparer (side by side and in
// rehearsal); the slot calls the news and the award record, which allocate: original only; the copy takes the branch.

/// Raises the award winner's reputation: the port directly in dry run, else the original `award_raise_winner_reputation`.
///
/// [pt] A reputação do premiado: no ensaio, a cópia direto (sem as listas dos clubes); senão pelo endereço (0x412f00).
pub(crate) unsafe fn raise_winner_reputation(person: usize, club: usize, nation: u32, world: u32, player: u32) {
    if MONTH_AWARD_DRY_RUN {
        return award_raise_winner_reputation(person, club, nation, world, player);
    }
    exe::award_raise_winner_reputation(person, club, nation, world, player);
}

/// Finds the month slot, stores the person and the club, raises the winner's reputation, sends the news and records the
/// award in the history.
///
/// `MONTH_AWARD::set_winner(person, club, month, year)`. It looks for the month's slot (12 slots of 11 bytes: month at
/// +0x25, year at +0x26); without one, it does nothing. It stores the person (+0x1d) and the club (+0x21); when there
/// is a person, it raises the award winner's reputation (`award_raise_winner_reputation`, with the award's reputations +0x15 and +0x17 and the
/// player flag +0x19); then the news (`exe::month_award_post_winner_news`) and the award record (`add_month_award_record` on `[0xdd7f00]`, with the award's +5).
/// Original: `MONTH_AWARD::set_winner` (0x7671c0, month_award.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7671c0 } (thiscall, ret 0x10, month_award.cpp): `MONTH_AWARD::set_winner(pessoa, clube, mês, ano)`. Acha a vaga
/// do mês (12 vagas de 11 bytes: mês em +0x25, ano em +0x26); sem ela, nada. Grava a pessoa (+0x1d) e o clube (+0x21);
/// com pessoa, a reputação do premiado (0x412f00, com as reputações +0x15 e +0x17 do prêmio e a marca de jogador
/// +0x19); a notícia (0x767330) e o registro do prêmio (0x86c8c0 em [0xdd7f00], com o +5 do prêmio).
pub(crate) unsafe extern "thiscall" fn set_winner(this: usize, id: i32, club_id: i32, month: u32, year: u32) {
    let mut slot = None;
    for k in 0..12usize {
        let v = this + k * 11;
        if read_u8(v + 0x25) == month as u8 && read_i16(v + 0x26) == year as u16 as i16 {
            slot = Some(k);
            break;
        }
    }
    let k = match slot {
        Some(k) => k,
        None => return,
    };
    let v = this + k * 11;
    write_u32(v + 0x1d, id as u32);
    write_u32(v + 0x21, club_id as u32);
    if id > -1 {
        let club = if club_id > -1 { month_award_club_by_id(club_id) } else { 0 };
        // the high bytes of the reputations come from dword +0x19 in the register, as the original pushes
        let mark = read_usize(this + 0x19) as u32;
        let high = mark & 0xffff0000;
        crate::comparison::branch("cpw_6 winner of the month");
        raise_winner_reputation(month_ratings_staff_by_id(id), club, high | read_i16(this + 0x15) as u16 as u32, high | read_i16(this + 0x17) as u16 as u32, mark);
    }
    if MONTH_AWARD_DRY_RUN {
        return;
    }
    exe::month_award_post_winner_news(this, (this as u32 & 0xffffff00) | k as u32);
    let cid = read_i32(v + 0x21);
    let club = if cid != -1 { month_award_club_by_id(cid) } else { 0 };
    exe::staff_records_add_month_award_record(read_usize(G_STAFF_RECORDS), month_ratings_staff_by_id(read_i32(v + 0x1d)), club, read_usize(this + 5) as u32, month, year);
}

/// Adds the regions the winner's reputation writes: player record +9..+0xe, non-player record +8..+0xd and the club's
/// board row word (+6 of 73 bytes).
///
/// [pt] As regiões que a reputação do premiado grava: o registro do jogador (+0x61: +9..+0xe), o do não jogador (+0x69:
/// +8..+0xd) e a linha da diretoria do clube (73 bytes em [[0xb67a44]], palavra +6).
pub(crate) unsafe fn winner_reputation_regions(regions: &mut Vec<(usize, usize)>, person: usize, club: usize) {
    let person = Staff(person);
    let p = person.player();
    if !p.is_null() {
        regions.push((p.addr() + 9, 6));
    }
    let np = person.non_player();
    if !np.is_null() {
        regions.push((np.addr() + 8, 6));
    }
    if club != 0 {
        let rows = read_usize(read_usize(G_MANAGER_MANAGER));
        regions.push((rows.wrapping_add((read_i32(club) as usize).wrapping_mul(73)).wrapping_add(6), 2));
    }
}

/// Dry-run comparer: runs the port without allocating calls, then restores and runs the original from the same region
/// state, comparing the regions.
///
/// [pt] O comparador em ensaio: a cópia sem as chamadas que alocam e a original a partir do mesmo estado das `regioes`.
pub(crate) unsafe fn compare_month_award(name: &str, regions: &[(usize, usize)], original: impl FnOnce(), copy: impl FnOnce()) {
    let before = crate::comparison::MemorySnapshot::read(regions);
    MONTH_AWARD_DRY_RUN = true;
    crate::rng::pause_trace(copy);
    MONTH_AWARD_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(regions);
    before.write_back();
    original();
    let mo = crate::comparison::MemorySnapshot::read(regions);
    SCOREBOARD.record(mo == mr, || format!("{} original={:x?} rust={:x?}", name, mo, mr));
}

/// Adds the low 16 bits of v to the 16-bit word at a.
///
/// [pt] A palavra `a` mais o valor do _ftol (16 bits baixos, como o `add word` da original).
pub(crate) unsafe fn add_to_word(a: usize, v: i32) {
    write_i16(a, read_i16(a).wrapping_add(v as i16));
}

/// Clamps the signed 16-bit word at a to at most 10000 and at least `piso`.
///
/// [pt] Teto de 10000 e piso `piso` na palavra `a` (as comparações com sinal da original).
pub(crate) unsafe fn clamp_word(a: usize, floor: i16) {
    if read_i16(a) > 10000 {
        write_i16(a, 10000);
    }
    if read_i16(a) <= floor {
        write_i16(a, floor);
    }
}

