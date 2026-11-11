use super::*;

/// Returns the competition (107-byte records) indexed by +4 of the person's 110-byte record in the national-teams table
/// ([this+4]); 0 without a person or with a negative index.
pub(crate) unsafe extern "thiscall" fn person_competition(this: usize, person: usize) -> usize {
    if person == 0 {
        return 0;
    }
    let reg = read_usize(this + 4).wrapping_add((read_i32(person) as usize).wrapping_mul(110));
    let i = read_i32(reg + 4);
    if i < 0 {
        return 0;
    }
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((i as usize).wrapping_mul(107))
}

/// Byte o of the national-team block (class 0/1/2 by flag and second team via `exe::club_is_national_team`) of the club's nation in
/// [this+4] records (110 bytes each).
pub(crate) unsafe fn national_block_byte(this: usize, club: Club, flag: u32, o: usize) -> u8 {
    let class: usize = if exe::club_is_national_team(club) == 0 {
        if flag == 0 { 0 } else { 1 }
    } else if flag == 0 {
        2
    } else {
        1
    };
    exe::club_is_national_team(club);
    let nation = read_i32(club.nation().addr());
    read_u8(read_usize(this + 4).wrapping_add((nation as usize).wrapping_mul(110)).wrapping_add(class * 20 + o))
}

/// Returns byte +0x42 of the national-team block of the club's nation (class by `flag` and second team).
pub(crate) unsafe extern "thiscall" fn national_side_byte_42(this: usize, club: usize, flag: u32) -> u8 {
    national_block_byte(this, Club(club), flag, 0x42)
}

/// Returns byte +0x43 of the national-team block for the club's nation and class (see national_block_byte).
pub(crate) unsafe extern "thiscall" fn national_side_byte_43(this: usize, club: usize, flag: u32) -> u8 {
    national_block_byte(this, Club(club), flag, 0x43)
}

/// Returns 1 if the club is a national team (`exe::database_is_national_team`) or one of the two MLS All-Stars clubs (East/West), else 0.
pub(crate) unsafe extern "stdcall" fn national_or_special(club: usize) -> u32 {
    if exe::database_is_national_team(club) != 0 {
        return 1;
    }
    let id = read_u32(club);
    (id == read_u32(G_KEY_CLUB_MLS_ALL_STARS_EAST) || id == read_u32(G_KEY_CLUB_MLS_ALL_STARS_WEST)) as u32
}

/// Tells whether person `pessoa` dislikes the main national team of his nation (+0x1a). Returns 1/0.
pub(crate) unsafe extern "stdcall" fn dislikes_national_side(person: usize) -> u32 {
    let person = Staff(person);
    let s = exe::database_get_national_team(person.nation());
    (exe::staff_dislikes_club(person, s) != 0) as u32
}

/// Copies the date and the byte +8 from `origem` into this record. Returns `this`.
pub(crate) unsafe extern "thiscall" fn copy_date_and_mark(this: usize, source: usize) -> usize {
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(this, tmp.as_mut_ptr() as usize, source);
    write_u8(this + 8, read_u8(source + 8));
    this
}

/// Address of block k (20 bytes) of the person's 110-byte record in [this+4].
pub(crate) unsafe fn person_block(this: usize, person: usize, k: i8) -> usize {
    read_usize(this + national_teams_fields::NATION_RECORDS)
        .wrapping_add((read_i32(person) as usize).wrapping_mul(110))
        .wrapping_add((k as isize * 20) as usize)
}

/// Sets byte +0x44 of block `k` of the person's national-teams record to 1.
pub(crate) unsafe extern "thiscall" fn set_person_44(this: usize, person: usize, k: i8) {
    write_u8(person_block(this, person, k) + 0x44, 1);
}

/// Returns signed byte +0x44 of block `k` (20 bytes) of person `pessoa`'s 110-byte record in [this+4].
pub(crate) unsafe extern "thiscall" fn person_byte_44(this: usize, person: usize, k: i8) -> i32 {
    read_i8(person_block(this, person, k) + 0x44) as i32
}

/// Returns the address of field +0x3a of block `k` of the person's 110-byte record.
pub(crate) unsafe extern "thiscall" fn person_field_3a(this: usize, person: usize, k: i8) -> usize {
    person_block(this, person, k) + 0x3a
}

/// Classifies a person by national side: 0xff with no national job (+0x24), 2 if it is a second national team, else bit
/// 3 of the PlayingSquad byte (+0x5e). Returns only the low byte.
pub(crate) unsafe extern "stdcall" fn class_by_national_side(person: usize) -> u8 {
    let person = Staff(person);
    let national_team = person.national_job();
    if national_team.is_null() {
        return 0xff;
    }
    if exe::club_is_national_team(national_team) != 0 {
        return 2;
    }
    (person.playing_squad() >> 3) & 1
}

/// Values (dword pointed to by each node's data) of a linked list (head +2, node data +0, next +6) without touching the
/// cursor.
pub(crate) unsafe fn list_item_values(list: usize) -> Vec<u32> {
    let mut v = Vec::new();
    if list == 0 {
        return v;
    }
    let mut in_the = read_usize(list + 2);
    while in_the != 0 && v.len() < 0x10000 {
        v.push(read_usize(read_usize(in_the)) as u32);
        in_the = read_usize(in_the + 6);
    }
    v
}

/// The rehearsal of the report (`send_club_international_report_news`): the random draws come from the original routine's tape and the news item is
/// not delivered.
pub(crate) static mut INTERNATIONAL_REPORT_TAPE: Option<(Vec<(i32, i32)>, usize, bool)> = None;

/// Random draw: replays from the CQF11 comparison tape when set (marks failure on mismatch), else calls the game's
/// random.
pub(crate) unsafe fn random_or_tape(n: i32) -> i32 {
    let f = &mut *core::ptr::addr_of_mut!(INTERNATIONAL_REPORT_TAPE);
    match f {
        Some((v, k, falhou)) => match v.get(*k) {
            Some(&(m, r)) if m == n && !*falhou => {
                *k += 1;
                r
            }
            _ => {
                *falhou = true;
                0
            }
        },
        None => {
            exe::utils_random(n)
        }
    }
}

/// Code of how a player did in the match from his slot in the result (+0x190 + 0x49 x k): appearance type, rated, man
/// of the match, extra flag; 0 if absent.
pub(crate) unsafe fn match_performance_code(result: MatchRecord, side: usize, player: Staff) -> u32 {
    let id = read_usize(player.addr());
    for k in 0..0x14usize {
        let e = MatchRecordRow(result.addr() + match_record::ROW_STAFF_ID + side * match_record::SIDE_SIZE + k * match_record_row::SIZE);
        if read_usize(e.addr()) != id {
            continue;
        }
        let mut c: u32 = match e.goals() {
            1 => 2,
            2 => 8,
            3 => 0x20,
            4..=8 => 0x40,
            _ => 0,
        };
        if e.minute_left_pitch() != 0xff {
            c |= 4;
        }
        if (result.man_of_the_match_id() as usize) == read_usize(player.player().addr()) {
            c |= 0x10;
        }
        if (e.penalties_taken() as i32) - (e.penalties_scored() as i32) == 1 {
            c |= 1;
        }
        return c;
    }
    0
}

/// Sends the club international report news: the match squad is sorted by club; each group of the same club becomes
/// news 0x24 to its manager: the match data, the star player, how many, up to 4 players (the star first, the others by
/// how they did), random(2) and the codes; the group players leave the array.
///
/// Ret 0x18; SEH frame for the local news item: `NationalTeams::send_club_international_report_news(club, match,
/// result, star, squad, count)`, from the result between national sides (`record_international_result`), for each side. The squad (a vector
/// of `count` persons, the low byte) is sorted by club (`exe::crt_qsort` with `compare_by_club`); each group of players of the same club
/// becomes news 0x24 to the club's manager (`send_to_club_manager`): the match record for the club (`fill_match_record_from_fixture`, only once) and its data
/// (0x7e4de0), the star player (id and keys, if he is from the club; otherwise −1 in the four), how many there are, the
/// club, up to 4 players (the star first, the others by the code of how they did, from highest to lowest), 0x420e9,
/// random(2) and the codes. The group's players leave the vector (zeroed).
/// Original: `NationalTeams::send_club_international_report_news` (0x7706c0, national_teams.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn send_club_international_report_news(this: usize, club: usize, game: usize, result: usize, highlight: usize, squad: *mut usize, count: u32) {
    let game = Fixture(game);
    let highlight = Staff(highlight);
    let _ = this;
    let q = count as u8 as i8;
    // 0x945333 (qsort): push 0x778ef0; push 4; push count; push squad; add esp, 0x10 -> cdecl; 0x77bbe0: push 0; push
    // 0x24; lea ecx, news -> thiscall; 0x785790: push value; push index; lea ecx, news -> thiscall, ret 8;
    // 0x7e4750: push &record; push club; push match; add esp, 0xc -> cdecl; 0x7e4de0: push &index; push &news;
    // push &record; add esp, 0xc -> cdecl; 0x7861e0: push club; push &news; mov ecx, [0xb7431c] -> thiscall, ret 8
    exe::crt_qsort(squad as usize, (q as i32) as u32, 4, 0x778ef0);
    if q <= 0 {
        return;
    }
    let side = (club != game.home_club().addr()) as usize;
    let mut record = [0u8; 0x20]; // the Date at +0 (0x54a110 on entry), the rest is filled by 0x7e4750
    let reg = record.as_mut_ptr() as usize;
    exe::date_date_54a110(reg);
    let mut built = false;
    let mut news = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = news.as_mut_ptr() as usize;
    let mut i: i8 = 1;
    loop {
        let slot = squad.add(i as usize - 1);
        let first = Staff(*slot);
        if !first.is_null() && !first.club_job().is_null() {
            let mut codes = [0u32; 4];
            let mut players = [0usize; 4];
            codes[0] = match_performance_code(MatchRecord(result), side, first);
            players[0] = first.addr();
            let mut how_many: i8 = 1;
            let mut j = i;
            while j < q {
                let pj = squad.add(j as usize);
                let other = Staff(*pj);
                if !other.is_null() {
                    if other.club_job() != first.club_job() {
                        break;
                    }
                    if how_many < 4 {
                        let k = how_many as usize;
                        codes[k] = match_performance_code(MatchRecord(result), side, other);
                        players[k] = other.addr();
                        if other == highlight {
                            // the highlight goes to the front (swap with the first, code together)
                            players[k] = players[0];
                            players[0] = other.addr();
                            codes.swap(0, k);
                        }
                    }
                    how_many += 1;
                    *pj = 0;
                }
                j += 1;
            }
            exe::new_newsitem(nt, 0x24, 0);
            if !built {
                built = true;
                exe::fill_match_record_from_fixture(game, club, reg);
            }
            let mut idx: u8 = 0;
            exe::write_match_record_to_news(reg, nt, &mut idx as *mut u8);
            let mut pr = |v: u32| {
                exe::newsitem_set_slot(nt, (idx as u32) as u32 as i8, v);
                idx = idx.wrapping_add(1);
            };
            if highlight.club_job() == first.club_job() {
                pr(read_usize(highlight.addr()) as u32);
                pr(read_usize(highlight.first_name() + names::ID) as u32);
                pr(read_usize(highlight.second_name() + names::ID) as u32);
                pr(read_usize(highlight.common_name() + names::ID) as u32);
            } else {
                for _ in 0..4 {
                    pr(u32::MAX);
                }
            }
            pr(how_many as i32 as u32);
            pr(read_usize(first.club_job().addr()) as u32);
            // the ones after the first, by code, from largest to smallest (up to the 3rd position and 4 players)
            let end = how_many as i32 - 1;
            let mut a: i8 = 1;
            if end > 1 {
                while a < 3 {
                    let mut b = a + 1;
                    while b < how_many && b < 4 {
                        if (codes[a as usize] as i32) < (codes[b as usize] as i32) { // cqf11: the order of the players
                            codes.swap(a as usize, b as usize);
                            players.swap(a as usize, b as usize);
                        }
                        b += 1;
                    }
                    a += 1;
                    if a as i32 >= end {
                        break;
                    }
                }
            }
            let n = how_many.min(4);
            for k in 0..n.max(0) as usize {
                let p = Staff(players[k]);
                pr(read_usize(p.addr()) as u32);
                pr(read_usize(p.first_name() + names::ID) as u32);
                pr(read_usize(p.second_name() + names::ID) as u32);
                pr(read_usize(p.common_name() + names::ID) as u32);
            }
            pr(0x420e9);
            covered!("cqf_11 club news");
            let k_draw = idx;
            idx = idx.wrapping_add(1);
            let r = random_or_tape(2); // cqf11: the news random draw
            exe::newsitem_set_slot(nt, (k_draw as u32) as u32 as i8, r as u32);
            for k in 0..n.max(0) as usize {
                exe::newsitem_set_slot(nt, (idx as u32) as u32 as i8, codes[k]);
                idx = idx.wrapping_add(1);
            }
            if (*core::ptr::addr_of!(INTERNATIONAL_REPORT_TAPE)).is_none() {
                exe::news_send_to_club_manager(read_usize(G_NEWS), nt, first.club_job());
            }
        }
        let previous = i;
        i = i.wrapping_add(1);
        if previous >= q {
            break;
        }
    }
}

/// Posts the call-up news: the called-up players (flag 4, or 8 for the second team) of human-managed clubs, grouped by
/// club, become news 0x1d to the manager: national side, club, type, how many, the opponents (given list or built by
/// `national_teams_collect_fixture_opponents`) and up to 3 players.
///
/// Ret 0x1c; SEH frame for the local news item: `national_teams_post_call_up_news(national_side, second, match, type,
/// no_opponents, opponents, count)`, from the call-up (`select_squad`) and from the screen. Of the 50 slots of the national
/// side's squad (+0xd7, pointers), each player with the flag (+0x5e: 8 for the second team, otherwise 4) and a club
/// with a human manager (`news_club_has_human_manager`) joins his club's group; when the next one in the slot is not from the same club (or
/// lacks the flag, or the list ended), news 0x1d goes to the club's manager (`send_to_club_manager`): the national side, the club,
/// `no_opponents`, the type, how many, `second`, how many opponents and, with fewer than 4, their ids (from the given
/// list or built once by `national_teams_collect_fixture_opponents` with the match, when there is no list and no `no_opponents`), and up to 3 players (id
/// and keys). The list built here is destroyed and freed at the end (`exe::drop_llist`, `exe::crt_operator_delete`).
/// Original: `national_teams_post_call_up_news` (0x76e0b0, national_teams.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn national_teams_post_call_up_news(this: usize, national_team: usize, second: u32, game: usize, kind: u32, sem_adversarios: u32, adversarios: usize, count: u32) {
    let national_team = Club(national_team);
    // 0x7875e0: push club; mov ecx, [0xb7431c] -> thiscall, ret 4; 0x76d560: push match; push national team; mov ecx, this ->
    // thiscall, ret 8; 0x77bbe0 / 0x785790 / 0x68d1b0 / 0x68d040: lea ecx (ou mov ecx, lista) -> thiscall; 0x7861e0:
    // push club; push &news; mov ecx, [0xb7431c] -> thiscall, ret 8; 0x944c94 (delete): push; add esp, 4 -> cdecl
    let mut list = 0usize;
    let mut how_many_adv: i16 = 0;
    if adversarios != 0 {
        list = adversarios;
        how_many_adv = count as u8 as i8 as i16;
    }
    let mark: u8 = if second != 0 { 8 } else { 4 };
    let mut n: i8 = 0;
    let mut three = [0usize; 3];
    let mut news = [0u32; 0x38];
    let nt = news.as_mut_ptr() as usize;
    for idx in 1..=0x32usize {
        let p = Staff(national_team.squad(idx - 1));
        if p.is_null() || p.playing_squad() & mark == 0 || p.club_job().is_null() || exe::news_club_has_human_manager(read_usize(G_NEWS), p.club_job()) == 0 {
            continue;
        }
        if list == 0 && sem_adversarios == 0 && game != 0 {
            list = exe::national_teams_collect_fixture_opponents(this, national_team, game);
            how_many_adv = read_i16(list);
        }
        if n < 3 {
            three[n as usize] = p.addr();
        }
        let club = p.club_job();
        n = n.wrapping_add(1);
        if idx < 0x32 {
            let next = Staff(national_team.squad(idx));
            if !next.is_null() && next.club_job() == club && next.playing_squad() & mark != 0 {
                continue;
            }
        }
        covered!("cqf_11 call-up news");
        exe::new_newsitem(nt, 0x1d, 0);
        exe::newsitem_set_slot(nt, (0) as u32 as i8, read_usize(national_team.addr()) as u32);
        exe::newsitem_set_slot(nt, (1) as u32 as i8, read_usize(club.addr()) as u32);
        exe::newsitem_set_slot(nt, (2) as u32 as i8, sem_adversarios);
        exe::newsitem_set_slot(nt, (3) as u32 as i8, kind);
        exe::newsitem_set_slot(nt, (4) as u32 as i8, n as i32 as u32);
        exe::newsitem_set_slot(nt, (5) as u32 as i8, second);
        exe::newsitem_set_slot(nt, (6) as u32 as i8, how_many_adv as i32 as u32);
        if how_many_adv < 4 && list != 0 {
            write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
            let mut k: u8 = 7;
            let mut item = exe::llist_get_next(list);
            while item != 0 {
                exe::newsitem_set_slot(nt, (k as u32) as u32 as i8, read_usize(read_usize(item)) as u32);
                k = k.wrapping_add(1);
                item = exe::llist_get_next(list);
            }
        }
        if n <= 3 && n > 0 {
            let mut k: u8 = 0xa;
            for t in 0..n as usize {
                let j = Staff(three[t]);
                exe::newsitem_set_slot(nt, (k as u32) as u32 as i8, read_usize(j.addr()) as u32);
                exe::newsitem_set_slot(nt, (k.wrapping_add(1) as u32) as u32 as i8, read_usize(j.first_name() + names::ID) as u32);
                exe::newsitem_set_slot(nt, (k.wrapping_add(2) as u32) as u32 as i8, read_usize(j.second_name() + names::ID) as u32);
                exe::newsitem_set_slot(nt, (k.wrapping_add(3) as u32) as u32 as i8, read_usize(j.common_name() + names::ID) as u32);
                k = k.wrapping_add(4);
            }
        }
        exe::news_send_to_club_manager(read_usize(G_NEWS), nt, club);
        n = 0;
    }
    if list != 0 && adversarios == 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
}

/// O ensaio da `national_teams_collect_fixture_opponents`: os valores que iriam para a lista nova, sem alocar.
pub(crate) static mut FIXTURE_OPPONENTS_PLAN: Option<Vec<u32>> = None;

/// Returns a new list with the match opponent (its nation's first team, for the second team) and the opponents of the
/// fixtures from tomorrow to 13 days later that are not finals.
///
/// Ret 8; SEH frame for the local list: `national_teams_collect_fixture_opponents(national_side, match)`, from the
/// call-up news (`national_teams_post_call_up_news`) and from the request to the manager (`exe::national_teams_post_squad_request_news`). A new list (`exe::crt_operator_new` with 0xe, `exe::new_llist`)
/// with the match opponent (the away side if the home side is from the national side's nation, otherwise the home side;
/// for the second team, the main national side of its nation, `exe::database_get_national_u21_team`) and the opponents of the national side's other
/// matches from tomorrow to 13 days later (`find_fixtures`, with byte +0x40 of the match) that are not final-stage matches
/// (the nine national side competitions). Returns the list.
/// Original: `national_teams_collect_fixture_opponents` (0x76d560, national_teams.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn national_teams_collect_fixture_opponents(this: usize, national_team: usize, game: usize) -> usize {
    let game = Fixture(game);
    let national_team = Club(national_team);
    let _ = this;
    // 0x68d020 / 0x68d040 / 0x68d1b0: lea ecx (ou mov ecx, bloco) -> thiscall; 0x944cf1 (new): push 0xe; add esp, 4 ->
    // cdecl; 0x68d0d0: push 4; push &valor; mov ecx, lista -> thiscall, ret 8; 0x5409b0 / 0x53b4f0: push; add esp, 4 ->
    // cdecl; 0x549ef0: push +0x28; push +0x2a; lea ecx -> thiscall, ret 8; 0x54c5b0: push n; push &output; mov ecx, date
    // -> thiscall, ret 8; 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24
    let dry_run = (*core::ptr::addr_of!(FIXTURE_OPPONENTS_PLAN)).is_some();
    let mut local = [0u8; 16];
    let l = Llist(local.as_mut_ptr() as usize);
    exe::new_llist(l);
    let mut nl = 0usize;
    if !dry_run {
        let m = exe::crt_operator_new(0xe);
        if m != 0 {
            nl = exe::new_llist(m);
        }
    }
    let por = |v: usize| {
        if dry_run {
            if let Some(p) = &mut *core::ptr::addr_of_mut!(FIXTURE_OPPONENTS_PLAN) {
                p.push(read_usize(v) as u32);
            }
        } else {
            exe::llist_add_tail(nl, v, 4);
        }
    };
    // the 9th argument of 0x5acbe0: the dword of the `jogo` argument on the stack (or of 0x53b4f0 written over it) with the low byte
    // replaced by the match’s +0x40
    let mut cell: usize = game.addr();
    let home = game.addr() + fixture::HOME_CLUB;
    if read_usize(read_usize(home) + club::NATION) == national_team.nation().addr() {
        if exe::club_is_national_team(national_team) != 0 {
            cell = exe::database_get_national_u21_team(game.away_club().nation().addr());
            por(&cell as *const usize as usize);
        } else {
            por(game.addr() + fixture::AWAY_CLUB);
        }
    } else if exe::club_is_national_team(national_team) != 0 {
        cell = exe::database_get_national_u21_team(read_usize(read_usize(home) + club::NATION));
        por(&cell as *const usize as usize);
    } else {
        por(home);
    }
    cell = (cell & !0xff) | game.home_side_mark() as usize;
    let (mut d0, mut d1, mut d2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (p0, p1, p2) = (d0.as_mut_ptr() as usize, d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize);
    let day_year = |p: usize| (read_usize(p) as u32, read_u32(p + 2));
    exe::new_date(p0, game.day() as u32, game.year() as u32);
    exe::date_add_days(p0, p1, 1);
    exe::date_add_days(p1, p2, 0xd);
    let (a1, a2) = day_year(p1);
    let (a3, a4) = day_year(p2);
    (exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), a1, a2, a3, a4, read_usize(national_team.addr()) as u32, l, (-1) as i32 as u32, (-2) as i32 as u32, cell as u32) as u32);
    l.set_cursor(l.head());
    let finais = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP];
    let mut item = exe::llist_get_next(l);
    while item != 0 {
        let c = read_usize(item + fixture::COMPETITION);
        if c == 0 || !finais.iter().any(|&k| read_usize(c) == read_usize(k)) {
            if read_usize(read_usize(item + fixture::HOME_CLUB) + club::NATION) == national_team.nation().addr() {
                por(item + fixture::AWAY_CLUB);
            } else {
                por(item + fixture::HOME_CLUB);
            }
        }
        item = exe::llist_get_next(l);
    }
    exe::drop_llist(l);
    nl
}

/// The position codes of the original's slots: squad of 26 (0x76c771..0x76c84c), of 24 (0x76c870..0x76c92c) and of any
/// other size (0x76c939..0x76c9de, 22 slots), with the common ones written afterwards (0x76c860/0x76c865 and slots 0,
/// 1, 3, 5, 6 and 21). Code 0xc of slot 12/11/10 comes from ebp (0xc since 0x76c2e4/0x76c6a9).
pub(crate) const SQUAD_26_POSITIONS: [u16; 26] = [1, 1, 1, 0x202, 0x202, 0x204, 0x204, 0x204, 0x84, 0x804, 0x88, 0x808, 0xc, 0x210, 0x210, 0x210,
    0x220, 0x90, 0x810, 0xa0, 0x820, 0x240, 0x240, 0x240, 0x240, 0x240];
pub(crate) const SQUAD_24_POSITIONS: [u16; 24] = [1, 1, 0x202, 0x202, 0x204, 0x204, 0x204, 0x84, 0x804, 0x88, 0x808, 0xc, 0x210, 0x210, 0x210,
    0x220, 0x90, 0x810, 0xa0, 0x820, 0x240, 0x240, 0x240, 0x240];
pub(crate) const SQUAD_SLOTS_22: [u16; 22] = [1, 1, 1, 0x202, 0x204, 0x204, 0x204, 0x204, 0x84, 0x804, 0xc, 0x210, 0x210, 0x220, 0x90, 0x810,
    0xa0, 0x820, 0x240, 0x240, 0x240, 0x240];
/// As vagas que a tabela de 24 tem a mais que a de 22, na ordem em que aparecem nela: 0x202 (líbero/centro, a vaga
/// 3), 0x88 e 0x808 (volantes pelos lados, 9 e 10) e 0x210 (o 3º meia central, 14).
pub(crate) const SQUAD_SLOTS_24_EXTRAS: [u16; 3] = [0x202, 0x88, 0x808];

/// The position codes of the `tam` squad slots (byte +0x42 of the type's record), from the tables for 22/24/26; an
/// approved fix fills the extra slots of the 23- and 25-player squads from the extras of the 24 table.
///
/// Faithful to the original routine for 26 and 24 and for the first 22 slots of the other sizes (the 22 table).
///
/// Approved fix of the original (01/11/2026): the original only fills the slots for 22, 24 or 26 players; in squads of
/// 23 (the finals, `national_team_ages`: the 2002 World Cup, on 29/05/2002) and 25, the slots from the 22nd onwards keep stack
/// residue (local_86c), which goes to the team selection (0x6e2000) and to the filter of selected players (`position_mask`),
/// and the residue changes on every run (halves of heap pointers).
///
/// Here the slots beyond the 22 of the original's table receive, in order, the ones the 24 table adds to the 22 table
/// (SQUAD_SLOTS_24_EXTRAS): the 23 squad = the 22 one + 0x202 (the first slot the 24 one adds); the 25 squad = the 22
/// one + 0x202, 0x88 and 0x808.
///
/// Why this way: the first 22 slots remain those the original writes for these sizes (the comparison checks all of
/// them), and each extra slot moves towards the next larger table. The 25 squad starts from the 22 table and not from
/// the 24 one because that is the one the original uses for it (only 24 and 26 have their own table).
///
/// Other sizes: as the original for the first 22; the rest with code 0 (nobody passes the filter). Simplification: the
/// game only asks for 23, 24 and 26.
pub(crate) unsafe fn squad_position_codes(size: u8) -> [u16; 50] {
    let mut c = [0u16; 50];
    match size {
        0x1a => c[..26].copy_from_slice(&SQUAD_26_POSITIONS),
        0x18 => c[..24].copy_from_slice(&SQUAD_24_POSITIONS),
        _ => c[..22].copy_from_slice(&SQUAD_SLOTS_22),
    }
    if size == 23 || size == 25 {
        covered!(if size == 23 { "cqc_12 correção do elenco de 23" } else { "cqc_12 correção do elenco de 25" });
        for i in 22..size as usize {
            c[i] = SQUAD_SLOTS_24_EXTRAS[i - 22]; // cqc12: the correction places
        }
    }
    c
}

/// Nation's record in the national-teams table: [this+4] + 0x6e x nation id.
pub(crate) unsafe fn squad_nation_record(this: usize, nation: usize) -> usize {
    read_usize(this + national_teams_fields::NATION_RECORDS).wrapping_add(read_usize(nation).wrapping_mul(0x6e))
}

/// The accepted reputation bands (+9 of player sheet +0x61) for selection: (exclusive minimum, maximum).
///
/// −1 and 0x2711 outside Australia ([0x9cf224]); for Australia, by the match's competition (friendly `exe::friendly_is_friendly_competition` of
/// [0xae2c74], or the Oceania one: 0x8ca/0x2134; Confederations Cup 0xabe/0x222e; Oceania qualifiers 0x8ca and 9000,
/// 8000 or 10001 by byte +0x42; World Cup 0xabe/10000).
pub(crate) unsafe fn reputation_limits(nation: usize, p: Fixture) -> (i16, i16) {
    // 0x5c6710: push [p + 0x14]; mov ecx, [0xae2c74] -> thiscall ret 4
    if read_usize(nation) != read_usize(KEY_NATION_AUSTRALIA) {
        return (-1, 0x2711);
    }
    if p.is_null() || (read_usize(G_FRIENDLY) != 0 && exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), p.competition()) != 0) {
        return (0x8ca, 0x2134);
    }
    let k = read_usize(p.competition().addr());
    if k == read_usize(KEY_CLUB_COMP_OCEANIA_NATIONS_CUP) {
        (0x8ca, 0x2134)
    } else if k == read_usize(KEY_CLUB_COMP_CONFEDERATIONS_CUP) {
        (0xabe, 0x222e)
    } else if k == read_usize(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) {
        (0x8ca, match p.stage_index() { 1 => 0x1f40, 2 => 0x2711, _ => 0x2328 })
    } else if k == read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP) {
        (0xabe, 0x2710)
    } else {
        (-1, 0x2711)
    }
}

/// For selection type: national team (types 0/1) or U21, the squad mark (+0x5e: 8 or 4) and tactics team flag.
pub(crate) unsafe fn squad_team_for_type(nation: usize, t: i8) -> (usize, u8, u32) {
    // 0x53b3d0 e 0x53b4f0: push nacao; add esp, 4 -> cdecl
    match t {
        1 => (exe::database_get_national_team(nation), 8, 1),
        0 => (exe::database_get_national_team(nation), 4, 0),
        _ => (exe::database_get_national_u21_team(nation), 4, 0),
    }
}

/// Range of reserve types scanned: only type itself, except U21 (2) which scans 0..1 at the Olympics and 1 otherwise.
pub(crate) unsafe fn reserve_type_range(t: i8, p: usize) -> (usize, usize) {
    if t != 2 {
        (0, 0)
    } else if p != 0 && read_usize(p) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) {
        (0, 1)
    } else {
        (1, 1)
    }
}

/// The reserve entries of type `k`, group `g`: (vector [this + 0xc + 4k] from the start, how many).
///
/// The start is the record's dword +8 + 0x15k plus the bytes +0xe + 0x15k + 4j of the previous groups; the count, byte
/// +0xd + 0x15k + 4g (as the original reads it: the sum and the count come from neighbouring bytes).
pub(crate) unsafe fn reserve_group(this: usize, reg: usize, k: usize, g: usize) -> (usize, i32, i32) {
    let base = reg + national_squad_record::RESERVE_GROUPS + 0x15 * k;
    let mut start = read_usize(base - 5) as i32;
    for j in 0..g {
        start = start.wrapping_add(read_u8(base + 1 + 4 * j) as i32);
    }
    let end = start.wrapping_add(read_u8(base + 4 * g) as i32);
    (read_usize(this + national_teams_fields::RESERVE_POOLS + 4 * k).wrapping_add((start as usize).wrapping_mul(4)), start, end)
}

/// Does reserve player j qualify for the squad list (job, injury, retirement, dislikes, club/nation, competition rules,
/// reputation band)? (0x76cb84..0x76cf6c)
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn is_squad_candidate(this: usize, nation: Nation, t: i8, data: usize, p: Fixture, squad: usize, reg: usize, lim: (i16, i16), j: usize) -> bool {
    let j = Staff(j);
    let bits: unsafe extern "stdcall" fn(usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_GET_RETIREMENT_STATUS);
    // 0x53b620: push byte +0x3d; add esp, 4 -> cdecl; 0x62c400: push j; mov ecx, [0xb63d28] -> thiscall ret 4 (al);
    // 0x7a7eb0: push j; mov ecx, [0xb7434c] -> stdcall ret 4 (al); 0x547d30: push club; push j; add esp, 0xc (together
    // with that of 0x53b3d0) -> cdecl; 0x4cf5a0: push j; mov ecx, [0xae1050] -> thiscall ret 4; 0x55d270: push 0; push 1;
    // push [p + 0x14]; push j; mov ecx, [0xae28f0] -> thiscall ret 0x10; 0x76db00: 6 pushes (nation, 0, j, 0, 0, 0);
    // mov ecx, this -> thiscall ret 0x18
    if j.is_null() || exe::database_is_player_job((j.job_for_club() as u32) as i8) == 0 || exe::injury_get_player_injury_status(read_usize(G_INJURY), j) != 0 {
        return false;
    }
    if read_usize(G_PLAYER_REGEN) != 0 && bits(j.addr()) != 0 {
        return false;
    }
    if j.club_job().is_null() || exe::staff_dislikes_club(j, exe::database_get_national_team(j.nation())) != 0 {
        return false;
    }
    let tj = j.national_job();
    if !tj.is_null() && (j.job_for_nation() != job_kind::PLAYER || tj.nation() != nation) {
        return false;
    }
    if !p.is_null() {
        let k = read_usize(p.addr());
        let c = j.club_job();
        let other_nation = |c: Club| !c.is_null() && c.nation().addr() != 0 && c.nation().continent() != nation.continent();
        let mut contract_ok = true;
        if k == read_usize(KEY_CLUB_COMP_FRIENDLY) || k == read_usize(KEY_CLUB_COMP_B_INTERNATIONAL) || k == read_usize(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) {
            if other_nation(c) {
                // international friendlies, B and under-21: a club from another nation only with reputation (+0x80) up to 0.8 of the
                // opponent (the home team +0x1c is the team itself: the away team +0x20); outside friendlies, no
                if k != read_usize(KEY_CLUB_COMP_FRIENDLY) {
                    return false;
                }
                let s = p.home_club();
                let other = Club(if s == Club(squad) { p.away_club().addr() } else { s.addr() });
                if c.reputation() as f64 * 0.8 > other.reputation() as f64 {
                    return false;
                }
            }
            // club with a match (table +0x24 by club id) and contract with +0x3d above −10: no
            if read_u8(read_usize(this + national_teams_fields::CLUB_MARKS).wrapping_add(read_usize(c.addr()))) != 0 {
                let x = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), j);
                if x != 0 && (read_u8(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), j) + contract::HAPPINESS) as i8) > -10 {
                    contract_ok = false;
                }
            }
            if !contract_ok {
                return false;
            }
        } else if other_nation(c) && c.division().addr() != 0
            && (p.competition().reputation() as f64) < c.division().reputation() as f64 * 0.7
        {
            // in the other competitions, a club from another nation whose league (+0x57) has +0x69 above the competition’s / 0.7
            return false;
        }
        if exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), j, p.competition(), 1, 0) != 0 {
            return false;
        }
        if t == 0 {
            let m = p.squad_rules() & 0x300;
            let tj = j.national_job();
            if m != 0x100 && tj == Club(squad) && j.playing_squad() & 8 != 0 {
                return false;
            }
            if m != 0x200 && !tj.is_null() && tj != Club(squad) {
                return false;
            }
        }
    }
    if j.national_job() == Club(exe::database_get_national_team(nation)) && j.playing_squad() & playing_squad::NATIONAL_SQUAD != 0 {
        if exe::national_teams_can_change_squad(this, nation, 0, j, 0, 0, 0) == 0 {
            return false;
        }
        if !(!p.is_null() && read_usize(p.addr()) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES)) && t != 0 {
            return false;
        }
    }
    // with the flag, only after the squad date of the type (year +2 unsigned, day +0 signed)
    let after = |a: usize| {
        let (ya, da) = (read_u16(data + 2), read_i16(data));
        let (yr, dr) = (read_u16(a + 2), read_i16(a));
        ya > yr || (ya == yr && da > dr)
    };
    if j.playing_squad() & playing_squad::NATIONAL_B_SQUAD != 0 && !after(reg + national_squad_record::B_SQUAD_DATE) {
        return false;
    }
    let tj = j.national_job();
    if tj == Club(exe::database_get_national_u21_team(nation)) && j.playing_squad() & 4 != 0 && !after(reg + national_squad_record::U21_SQUAD_DATE) {
        return false;
    }
    if t == 2 && (j.age() as u8) as i8 > 0x17 { // cqc12: the under-21 age
        return false;
    }
    let nat = j.nation();
    if nat != nation {
        // from another nation: no team, no +0x22 mark, no bit 2 of +0xc in the 79-byte record, with the second
        // nation (+0x1e) in this one; it gets in if this one has a higher +0x8e, or (club in the same confederation +0x71 or no club)
        // with +0x85 of 5 or more
        if !tj.is_null() || j.int_apps() != 0 || read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(j.addr()).wrapping_mul(staff_state::SIZE)) + staff_state::EXTRA_FLAGS) & extra_flags::PLAYED_U21_INTERNATIONAL != 0
            || j.second_nation() != nation || nat.is_null()
        {
            return false;
        }
        let c = j.club_job();
        let n2 = Nation(if !c.is_null() { c.nation().addr() } else { 0 });
        if nation.reputation() <= nat.reputation() {
            if !n2.is_null() && nation.continent() != n2.continent() {
                return false;
            }
            if (nation.league_standard() as i8) < 5 {
                return false;
            }
        }
    }
    let v = read_i16(j.player().addr() + 9);
    v > lim.0 && v <= lim.1
}

/// Builds the candidate list from reserves of the type range group by group; returns the count (byte) and leaves the
/// last cursor + 4.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn collect_squad_candidates(this: usize, nation: usize, t: i8, data: usize, p: usize, squad: usize, reg: usize, lim: (i16, i16), cand: &mut [usize; 200], cursor: &mut usize) -> u8 {
    let mut n = 0u8;
    let (lo, hi) = reserve_type_range(t, p);
    for k in lo..=hi {
        for g in 0..4 {
            let (v, ini, fim) = reserve_group(this, reg, k, g);
            *cursor = v;
            for _ in ini..fim {
                let j = read_usize(*cursor);
                if is_squad_candidate(this, Nation(nation), t, data, Fixture(p), squad, reg, lim, j) {
                    // simplification: the original writes to local_32c[(char) n]; from 128 on it would write before the vector
                    if (n as i8) >= 0 {
                        cand[n as usize] = j;
                    }
                    n = n.wrapping_add(1);
                }
                *cursor += 4;
            }
        }
    }
    n
}

/// The national squad selection (senior, B/Olympic or under-21): in the finals the list item; the squad dates, the ages
/// and the size; with a human manager, the request to him; otherwise those from another side and the flagged ones
/// leave, the pool list goes to the slot assignment and each picked player who fits the slot joins the squad (leaving
/// the previous side and the tactic slots); the sorting, the news and the tactic slots. Approved fix of the original
/// (01/11/2026): position codes also for 23- and 25-man squads (the original left stack leftovers in the slots from the
/// 22nd on): the original 22 plus, in order, 0x202, 0x88 and 0x808, the slots the 24 table has beyond the 22 table.
///
/// Ret 0x14, about 4.9 KB; SEH frame only for the local dates, no destructor: `NationalTeams::select_squad(nation,
/// type, date, fixture, forced)`, from the call-up (`select_squads_for_fixture`, in Rust: nation +0x53 of the side, type, 0xae2c90,
/// fixture, 0), from building the tables (0x76a840: fixture 0) and from the news (`exe::newsitem_auto_respond`: forced 1). Type 0 senior, 1
/// B/Olympic (flag 8, tactic team 1), 2 under-21 (`exe::database_get_national_u21_team`). Without `forced`: in the final stages (+0x14 of the
/// fixture in one of the 9 keys) an item (team, competition, match date − 10, 1000) goes into the list at +0 (0x68d0d0,
/// created with `exe::new_llist` if missing); the squad date for the type (+0x3a of the record: today − 1 without a fixture,
/// the match date in type 0 and above 2, the match date − 1 in types 1 and 2, which first mark both nations in vector
/// `[this + 0x36]` at the Olympic Games or release the flagged under-21 player, `release_overage_u21_players`) and today's (+0x32); the ages
/// and the size (`national_team_ages` into +0x42/+0x43). With a human manager who decides (`exe::human_manager_is_human_club`, `exe::human_manager_controls_all_national_teams`; or through the main
/// national side for a B team without a manager): players injured for more than 10 days (0x62c400 = 2, `exe::injury_get_player_injury` +0x2e)
/// leave the squad; if it is still his, the request to the manager (`exe::national_teams_post_squad_request_news`) and stop. Otherwise: the slot codes
/// (squad_position_codes, with the 23 and 25 fix); those from another team leave the squad, those with the flag and, in
/// type 0 with bit 1 of +0x4e of the fixture, those without it (`remove_player_from_selection` and +0x5e/+0x24/+0x28); the pool list
/// (collect_squad_candidates) goes to the slot assignment (0x6e2000, mode 0x2100914 in type 1, 0x2000814 in 0 and 0x814
/// in 2); each picked player who fits the slot (`position_mask`) joins the squad: he leaves the previous team (slot, `national_teams_can_change_squad`,
/// `remove_player_from_selection`) or his own tactic slots (`tactic_slot_person`/`clear_selection_slot`, `national_teams_can_change_squad`), gets the flag (`staff_set_squad_flags`), the team and +0x28 (0xc
/// from 5 or 0xc, otherwise 0xb) and +9 of the 79-byte record goes to 0..100. Finally the squad is sorted (`exe::crt_qsort`
/// with `compare_by_club`), the news (`national_teams_post_call_up_news`) and the tactic slots with those not in the team are cleared. The error boxes
/// (lines 0x748 and 0x76b) as the original.
/// Original: `NationalTeams::select_squad` (0x76c120, national_teams.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn select_squad(this: usize, nacao: usize, kind: u32, data: usize, p: usize, forced: u32) {
    let p = Fixture(p);
    let pick: unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, u32, u32, u32, u32, u32, u32, u32) -> u8 =
        core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    // 0x549ef0: push +0x28; push +0x2a; lea ecx, saida -> thiscall ret 8 (palavras); 0x54c8e0: push n; push saida; mov
    // ecx, data -> thiscall ret 8; 0x417c10: push origem; push saida; lea ecx, destino -> thiscall ret 8; 0x54a110: lea
    // ecx -> thiscall; 0x944cf1 (new): push 0xe; add esp, 4 -> cdecl; 0x68d020: mov ecx, bloco -> thiscall ret 0;
    // 0x68d0d0: push 0x12; push item; mov ecx, [this] -> thiscall ret 8; 0x76d880: push nation; mov ecx, this ->
    // thiscall ret 4; 0x76d430: 4 pushes (match, type, +0x42, +0x43) -> stdcall ret 0x10; 0x600a50: 5 pushes (team,
    // 0, 1, 0, 0); add esp -> cdecl; 0x5fd6b0: push 0; push manager; mov ecx, [0xb63c98] -> thiscall ret 8 (al);
    // 0x5409b0: push; add esp, 4 -> cdecl; 0x62ed70: push j; mov ecx, [0xb63d28] -> thiscall ret 4; 0x896c60: push 1;
    // push time; push j; push elenco; mov ecx, [0xde1d08] -> thiscall ret 0x10; 0x76f1a0: 3 pushes (elenco, time,
    // partida); mov ecx, this -> ret 0xc; 0x6e2000: 15 pushes; add esp, 0x3c -> cdecl; 0x5b7d20: push +0x61; add esp,
    // 4 -> cdecl (ax); 0x8968f0 and 0x896e90: push team; push place; push squad; mov ecx, [0xde1d08] -> thiscall ret 0xc;
    // 0x540980: push marca; push j; add esp, 8 -> cdecl; 0x945333 (qsort): 4 pushes; add esp, 0x10 -> cdecl; 0x76e0b0:
    // 7 pushes; mov ecx, this -> ret 0x1c
    let t = kind as u8 as i8;
    let reg = squad_nation_record(this, nacao);
    let lim = reputation_limits(nacao, p);
    let (elenco, marca, time) = squad_team_for_type(nacao, t);
    let keys = [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP];
    let finais = !p.is_null() && !p.competition().is_null() && keys.iter().any(|&k| read_usize(p.competition().addr()) == read_usize(k));
    let r = reg.wrapping_add((t as i32 * 0x14) as usize);
    let tactic = || read_usize(G_TACTICS);
    if forced == 0 {
        let (mut d88c, mut d87c, mut d8a0) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let (p88c, p87c, p8a0) = (d88c.as_mut_ptr() as usize, d87c.as_mut_ptr() as usize, d8a0.as_mut_ptr() as usize);
        if finais {
            covered!("cqc_12 finals squad");
            let d = exe::new_date(p87c, p.day() as u32, p.year() as u32);
            exe::date_sub_days(d, p88c, 10);
            let comp = p.competition();
            if read_usize(this) == 0 {
                let m = exe::crt_operator_new(0xe);
                write_usize(this, if m != 0 { exe::new_llist(m) } else { 0 });
            }
            let mut item = [0u8; 0x12];
            let pi = item.as_mut_ptr() as usize;
            exe::date_date_54a110(pi + 8);
            write_usize(pi + 4, read_usize(comp.addr()));
            write_usize(pi, read_usize(elenco));
            exe::date_operator_3d(pi + 8, p87c, p88c);
            write_u16(pi + call_up_entry::DUTY_END_DAY, 1000);
            exe::llist_add_tail(read_usize(this), pi, 0x12);
        }
        if p.is_null() {
            let x = exe::date_sub_days(data, p88c, 1);
            exe::date_operator_3d(r + national_squad_record::SQUAD_DATE, p87c, x);
        } else if t != 1 && t != 2 {
            let x = exe::new_date(p88c, p.day() as u32, p.year() as u32);
            exe::date_operator_3d(r + national_squad_record::SQUAD_DATE, p87c, x);
        } else {
            // each side’s nation (+0x53 of the team) and its cell in the vector [this + 0x36]
            let nation = |o: usize| read_usize(read_usize(p.addr() + o) + club::NATION);
            let home = |o: usize| read_usize(nation(o)).wrapping_add(read_usize(this + national_teams_fields::OLYMPIC_NATION_MARKS));
            if read_usize(p.addr()) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) {
                write_u8(home(0x1c), 1);
                write_u8(home(0x20), 1);
            } else {
                if read_u8(home(0x1c)) != 0 {
                    exe::nationalteams_release_overage_u21_players(this, nation(fixture::HOME_CLUB));
                }
                if read_u8(home(0x20)) != 0 {
                    exe::nationalteams_release_overage_u21_players(this, nation(fixture::AWAY_CLUB));
                }
                write_u8(home(0x1c), 0);
                write_u8(home(0x20), 0);
            }
            let x = exe::new_date(p88c, p.day() as u32, p.year() as u32);
            let y = exe::date_sub_days(x, p87c, 1);
            exe::date_operator_3d(r + national_squad_record::SQUAD_DATE, p8a0, y);
        }
        exe::date_operator_3d(r + national_squad_record::SQUAD_STAMP_DATE, p88c, data);
        exe::national_teams_get_squad_size_limits(p, kind, r + 0x42, r + national_squad_record::CALLED_UP_MARK);
        let manager = read_usize(exe::database_get_national_team(nacao) + club::MANAGER);
        let decides = || {
            (exe::human_manager_is_human_club(elenco, 0, 1, 0, 0) != 0 && (t == 0 || exe::human_manager_controls_all_national_teams(read_usize(G_HUMAN_MANAGER), manager, (0) as *mut u32) != 0))
                || (exe::club_is_national_team(elenco) != 0 && read_usize(elenco + club::MANAGER) == 0
                    && exe::human_manager_is_human_club(exe::database_get_national_team(read_usize(elenco + club::NATION)), 0, 1, 0, 0) != 0
                    && exe::human_manager_controls_all_national_teams(read_usize(G_HUMAN_MANAGER), manager, (0) as *mut u32) != 0)
        };
        if decides() {
            for k in 0..50 {
                let s = elenco + club::SQUAD + 4 * k;
                let j = read_usize(s);
                if j == 0 || exe::injury_get_player_injury_status(read_usize(G_INJURY), j) != 2 || read_u8(j + staff::PLAYING_SQUAD) & marca == 0 {
                    continue;
                }
                let l = InjuryEntry(exe::injury_get_player_injury(read_usize(G_INJURY), j));
                if !l.is_null() && l.training_days_left() > 10 {
                    exe::tactics_remove_player_from_selection(tactic(), elenco, j, time, 1);
                    write_u8(j + staff::PLAYING_SQUAD, read_u8(j + staff::PLAYING_SQUAD) & 3);
                    write_usize(j + staff::NATIONAL_JOB, 0);
                    write_u8(j + staff::JOB_FOR_NATION, if read_u8(j + staff::JOB_FOR_NATION) == 0xc { 5 } else { 0 });
                    write_usize(s, 0);
                }
            }
        }
        if decides() {
            covered!("cqc_12 squad by the human manager");
            write_u8(r + 0x44, 0);
            exe::national_teams_post_squad_request_news(this, elenco, time, p);
            return;
        }
    }
    let size = r + 0x42;
    let codes = squad_position_codes(read_u8(size));
    // leave the squad: those from another team; those with the mark; in type 0 with bit 1 of the match’s +0x4e, those without it
    for k in 0..50 {
        let s = elenco + club::SQUAD + 4 * k;
        let j = read_usize(s);
        if j == 0 {
            continue;
        }
        if read_usize(j + staff::NATIONAL_JOB) != elenco {
            exe::tactics_remove_player_from_selection(tactic(), elenco, j, time, 1);
        } else {
            let old = if read_u8(j + staff::PLAYING_SQUAD) & marca != 0 {
                5
            } else if t == 0 && p.flags() & 1 != 0 {
                0xb
            } else {
                continue;
            };
            exe::tactics_remove_player_from_selection(tactic(), elenco, j, time, 1);
            write_u8(j + staff::PLAYING_SQUAD, read_u8(j + staff::PLAYING_SQUAD) & 3);
            write_usize(j + staff::NATIONAL_JOB, 0);
            write_u8(j + staff::JOB_FOR_NATION, if read_u8(j + staff::JOB_FOR_NATION) == 0xc { old } else { 0 });
        }
        write_usize(s, 0); // cqc12: the squad slot becomes free
    }
    let mut cand = [0usize; 200];
    let mut cursor = 0usize;
    let n = collect_squad_candidates(this, nacao, t, data, p.addr(), elenco, reg, lim, &mut cand, &mut cursor);
    let mode: u32 = if t == 1 { 0x2100914 } else if t != 2 { 0x2000814 } else { 0x814 };
    let mut out = [0usize; 50];
    covered!("cqc_12 squad lineup");
    pick(cand.as_ptr() as usize, n as u32, codes.as_ptr() as usize, (mode & !0xff) | read_u8(size) as u32, 0,
        out.as_mut_ptr() as usize, mode, elenco, 0, 0, 0, 0, 0, 0, 0);
    let mut i = 0usize;
    // simplification: the original does not limit to 50 (the game asks for up to 26)
    while (i as i32) < read_u8(size) as i8 as i32 && i < 50 {
        let j = out[i];
        if j != 0 && codes[i] & exe::formation_get_position_mask(read_usize(j + staff::PLAYER)) != 0 {
            if let Some(k) = (0..50).find(|&k| read_usize(elenco + club::SQUAD + 4 * k) == 0) {
                write_usize(elenco + club::SQUAD + 4 * k, j);
            }
            let tj = read_usize(j + staff::NATIONAL_JOB);
            if tj != elenco && tj != 0 {
                // from another team: leaves the slot there (with permission, the call-up mark of its type is zeroed) and the tactic
                let v = ((read_u8(j + staff::PLAYING_SQUAD) as i8 as i32 & 8) >> 3) as u32;
                let c3: u8 = if exe::club_is_national_team(tj) == 0 { (v != 0) as u8 } else if v != 0 { 1 } else { 2 };
                let mut c = read_usize(j + staff::NATIONAL_JOB) + club::SQUAD;
                for _ in 0..50 {
                    if read_usize(c) == j {
                        write_usize(c, 0);
                        if exe::national_teams_can_change_squad(this, read_usize(read_usize(j + staff::NATIONAL_JOB) + club::NATION), c3 as u32, 0, 0, 0, 0) != 0 {
                            write_u8(reg + national_squad_record::CALLED_UP_MARK + c3 as usize * 0x14, 0);
                            break;
                        }
                        show_error_at(0xa5b2a4, 0x748);
                    }
                    c += 4;
                }
                cursor = c;
                exe::tactics_remove_player_from_selection(tactic(), read_usize(j + staff::NATIONAL_JOB), j, v, 1);
            } else if tj == elenco && read_u8(j + staff::PLAYING_SQUAD) & marca == 0 {
                // already of the team without the mark: leaves the team’s tactic slots (tactic team reversed)
                let bz = (time == 0) as u32;
                let c3: u8 = if exe::club_is_national_team(elenco) == 0 { (bz != 0) as u8 } else if bz != 0 { 1 } else { 2 };
                for v in 0..20 {
                    if exe::tactics_get_selected_player(tactic(), read_usize(j + staff::NATIONAL_JOB), v, (bz) as i32) != j {
                        continue;
                    }
                    exe::tactics_clear_selection_slot(tactic(), read_usize(j + staff::NATIONAL_JOB), v, bz);
                    // the original passes as the player the dword of the last traversal pointer (cursor): reproduced
                    if c3 != 0 && exe::national_teams_can_change_squad(this, read_usize(read_usize(j + staff::NATIONAL_JOB) + club::NATION), c3 as u32, read_usize(cursor), 0, 0, 0) != 0 {
                        write_u8(reg + national_squad_record::CALLED_UP_MARK + c3 as usize * 0x14, 0);
                        break;
                    }
                    show_error_at(0xa5b2a4, 0x76b);
                }
            }
            exe::staff_set_squad_flags(j, marca as u32);
            write_usize(j + staff::NATIONAL_JOB, elenco); // cqc12: the called-up player gets the team
            let x = read_u8(j + staff::JOB_FOR_NATION);
            write_u8(j + staff::JOB_FOR_NATION, if x == 5 || x == 0xc { 0xc } else { 0xb });
            let e = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(j).wrapping_mul(staff_state::SIZE)) + 9;
            write_u8(e, (read_u8(e) as i8).clamp(0, 100) as u8);
        }
        i += 1;
    }
    exe::crt_qsort(elenco + club::SQUAD, 0x32, 4, 0x778ef0);
    let key = if p.is_null() { u32::MAX } else { read_usize(p.competition().addr()) as u32 };
    exe::national_teams_post_call_up_news(this, elenco, time, p, key, finais as u32, 0, 0);
    for v in 0..20 {
        let x = exe::tactics_get_selected_player(tactic(), elenco, v, (time) as i32);
        if x != 0 && read_usize(x + staff::NATIONAL_JOB) != elenco {
            exe::tactics_clear_selection_slot(tactic(), elenco, v, time);
        }
    }
}

/// Memory regions the squad selection (`select_squad`) touches: nation record and each scanned reserve player's +0x24..+0x28
/// and +0x5e.
pub(crate) unsafe fn squad_selection_regions(this: usize, nation: usize, t: i8, p: usize) -> Vec<(usize, usize)> {
    let reg = squad_nation_record(this, nation);
    let mut v = vec![(reg, 0x6e)];
    let (lo, hi) = reserve_type_range(t, p);
    for k in lo..=hi {
        for g in 0..4 {
            let (a, ini, fim) = reserve_group(this, reg, k, g);
            for i in 0..(fim - ini).max(0) as usize {
                let j = Staff(read_usize(a + 4 * i));
                if !j.is_null() {
                    v.push((j.addr() + staff::NATIONAL_JOB, 5));
                    v.push((j.addr() + staff::PLAYING_SQUAD, 1));
                }
            }
        }
    }
    v
}

/// Club record of all-star team g ([0x9d00a4] or [0x9d00a8]) in the club table (0x245 bytes).
pub(crate) unsafe fn all_star_club(g: usize) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add(read_usize(g).wrapping_mul(club::SIZE))
}

/// Returns the national side holding the player from the day before to the day after (a national side fixture in the
/// window, against the given opponent if any, or a call-up already started) or the MLS all-star squad on the game date;
/// writes the comment; otherwise 0 (and 0 for a national side or all-star club).
///
/// Ret 0x14: `NationalTeams::get_international_duty_club(player, date, club, comment, opponent)`, from the line-up
/// (`starters`, `select_automatically`), the scouts, the transfers and the screens. With a temporary list (`exe::new_llist`, destroyed at the
/// end by `exe::drop_llist`). Club is a national side (`exe::database_is_national_team`) or one of the all-star teams (id in `[0x9d00a4]`/`[0x9d00a8]`):
/// 0. The window goes from the day before (0x54c8e0) to the day after (`exe::date_add_days`). Called-up player (+0x24, the national
/// side): the national side's matches in the window (`find_fixtures`, side by bit 4 of +0x5e: set 1, otherwise 0; the days and
/// years as the original's dwords); with a match and no opponent, or with the opponent (+0x14 of the match) among them,
/// he is on duty (text 0xa62ff4 into `comment`, 0x66f4e0) and returns the national side. With bit 4 set, the call-ups
/// of list `[this]` (+0 the national side, +4 the opponent, +8 the date) with his national side (and the opponent, if
/// given) and a date before the requested one: he is in the team (text 0xa62fc8) and returns the national side.
/// Otherwise the all-star team (competition `[[0xadadfc] + 4·[0x9cf690]]`: the match date through record +0xa3, day +3
/// and year +5 + year +0x40, 0x549ef0) for a club player (+0x39) of league `[0x9cf590]` (+0x57), with the date in the
/// window: the 50 names (+0xd7) of both teams (first `[0x9d00a8]`; text 0xa62fa4 into 0xde1f64 if there is a
/// `comment`). Otherwise 0.
/// Original: `NationalTeams::get_international_duty_club` (0x7757d0, national_teams.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn get_international_duty_club(this: usize, player: usize, data: usize, club: usize, comentario: usize,
                                                      opponent: usize) -> usize {
    let player = Staff(player);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut list = [0u8; 0x10];
    let pl = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(pl);
    let sai = |r: usize| -> usize {
        exe::drop_llist(pl);
        r
    };
    if club != 0 {
        if exe::database_is_national_team(club) != 0 {
            return sai(0);
        }
        let id = read_usize(club);
        if id == read_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST) || id == read_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST) {
            return sai(0);
        }
    }
    let (mut lo, mut hi) = ([0u8; 8], [0u8; 8]);
    let (plo, phi) = (lo.as_mut_ptr() as usize, hi.as_mut_ptr() as usize);
    exe::date_sub_days(data, plo, 1); // cpz6: a véspera
    exe::date_add_days(data, phi, 1); // cpz6: o dia seguinte
    let nacional = player.national_job();
    if !nacional.is_null() {
        let without_bit = (player.playing_squad() & 4 == 0) as u32;
        // simplification: the original pushes the side with the high 24 bits of ecx left over; 0x5acbe0 only reads the byte
        let games: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
            core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
        let n = games(read_usize(G_FIXTURE_MANAGER), read_usize(plo) as u32, read_usize(plo + 2) as u32, read_usize(phi) as u32, read_usize(phi + 2) as u32,
            read_usize(nacional.addr()) as u32, pl.addr(), u32::MAX, 0xffff_fffe, (without_bit == 0) as u32);
        let mut servico = false;
        if n as u16 != 0 {
            if opponent == 0 {
                servico = true;
            } else {
                pl.set_cursor(pl.head());
                loop {
                    let j = exe::llist_get_next(pl);
                    if j == 0 {
                        break;
                    }
                    if read_usize(j + fixture::COMPETITION) == opponent {
                        servico = true;
                        break;
                    }
                }
            }
        }
        if servico {
            if comentario != 0 {
                text(comentario, 0xa62ff4, 0xad9c64usize);
            }
            covered!("cpz_6 on national team duty");
            return sai(player.national_job().addr()); // cpz6: on national team duty
        }
        let conv = Llist(read_usize(this));
        if !conv.is_null() && without_bit == 0 {
            conv.set_cursor(conv.head());
            loop {
                let e = exe::llist_get_next(read_usize(this));
                if e == 0 {
                    break;
                }
                if read_usize(e) != read_usize(player.national_job().addr()) {
                    continue;
                }
                if opponent != 0 && read_usize(e + 4) != read_usize(opponent) {
                    continue;
                }
                let (dia, year) = (read_u16(e + 8) as i16, read_u16(e + 0xa));
                let today = read_u16(data + 2);
                if year < today || (year == today && dia < read_u16(data) as i16) {
                    if comentario != 0 {
                        text(comentario, 0xa62fc8, 0xad9c64usize);
                    }
                    covered!("cpz_6 in the national team");
                    return sai(player.national_job().addr()); // cpz6: already in the national team
                }
            }
        }
    }
    let comp = Cup(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS).wrapping_mul(4))));
    if comp.is_null() {
        return sai(0);
    }
    let reg = comp.round_table();
    let year = (read_u8(reg + 5) as i8 as i16).wrapping_add((comp.year() as u16) as i16) as u16;
    let mut game = [0u8; 8];
    let pj = game.as_mut_ptr() as usize;
    exe::new_date(pj, read_u16(reg + 3) as u32, year as u32);
    let cj = player.club_job();
    if cj.is_null() {
        return sai(0);
    }
    let league = cj.division().addr();
    if league == 0 || read_usize(league) != read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
        return sai(0);
    }
    let (jd, ja) = (read_u16(pj) as i16, read_u16(pj + 2));
    let (ld, la) = (read_u16(plo) as i16, read_u16(plo + 2));
    if la > ja || (la == ja && ld > jd) {
        return sai(0);
    }
    let (hd, ha) = (read_u16(phi) as i16, read_u16(phi + 2));
    if ha < ja || (ha == ja && hd < jd) {
        return sai(0);
    }
    for i in 0..0x32usize {
        for g in [0x9d00a8usize, G_KEY_CLUB_MLS_ALL_STARS_EAST] {
            if read_usize(all_star_club(g) + club::SQUAD + 4 * i) == player.addr() {
                if comentario != 0 {
                    text(0xde1f64, 0xa62fa4, 0xad9c64usize);
                }
                covered!("cpz_6 all-star team");
                return sai(all_star_club(g));
            }
        }
    }
    sai(0)
}

/// Tells whether the player may play for the manager's national side: nationality, under-21 age, current call-up and
/// the second-nationality rule.
///
/// Stdcall, ret 8: `national_teams_player_is_eligible(player, manager)`, from the player search (0x7afa70, 0x807450)
/// and from the comparison screen. No player sheet (+0x61), no national side for the manager (+0x24) or it not being a
/// national side (`exe::database_is_national_team`): 0. Already called up for it: 1. A national side of the second set (`exe::club_is_national_team`, the under-21)
/// and age (+0x18) above 21: 0. With +0x22 or bit 2 of +0xc of the person's 79-byte record (`[0xae2d0c]`), the
/// nationality (+0x1a) must be that of the national side (+0x53); called up for another national side of another
/// nation: 0. The national side's nation must be the first or the second (+0x1e) nationality; if the nation is
/// `[0x9cf434]` and so is the second: 0. Otherwise 1.
/// Original: `national_teams_player_is_eligible` (0x76d9b0, national_teams.cpp, stdcall, ret 8).
pub(crate) unsafe extern "stdcall" fn national_teams_player_is_eligible(player: usize, manager: usize) -> u32 {
    let manager = Staff(manager);
    let player = Staff(player);
    let sel = manager.national_job();
    if player.player().is_null() || sel.is_null() {
        return 0;
    }
    if exe::database_is_national_team(sel) == 0 {
        return 0;
    }
    if player.national_job() == sel {
        return 1;
    }
    if exe::club_is_national_team(sel) != 0 && (player.age() as u8) as i8 > 0x15 {
        return 0;
    }
    let nation = sel.nation().addr();
    let first = player.nation();
    if player.int_apps() != 0 && first != Nation(nation) {
        return 0;
    }
    let p = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(player.addr()).wrapping_mul(staff_state::SIZE));
    if read_u8(p + staff_state::EXTRA_FLAGS) & extra_flags::PLAYED_U21_INTERNATIONAL != 0 && player.nation() != Nation(nation) {
        return 0;
    }
    let other = player.national_job();
    if !other.is_null() && other.nation().addr() != nation {
        return 0;
    }
    let first = player.nation();
    if first != Nation(nation) && player.second_nation() != Nation(nation) {
        return 0;
    }
    let x = read_usize(KEY_NATION_PORTUGAL);
    if read_usize(nation) == x {
        let s = player.second_nation();
        if !s.is_null() && read_usize(s.addr()) == x {
            return 0;
        }
    }
    1
}

/// Adds to a list each duty competition it does not yet hold and returns how many were added.
///
/// Ret 4: `NationalTeams::collect_duty_competitions(out)`, from the scouts (0x7f7010). Without the call-up list
/// `[this]` or without `out`: 0. For each call-up, its competition (+4) goes into `out` (0x68d0d0, 4 bytes) if it is
/// not there yet; returns how many were added (byte).
/// Original: `NationalTeams::collect_duty_competitions` (0x775580, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn collect_duty_competitions(this: usize, out: usize) -> u8 {
    let out = Llist(out);
    let conv = Llist(read_usize(this));
    if conv.is_null() || out.is_null() {
        return 0;
    }
    let mut n: u8 = 0;
    conv.set_cursor(conv.head());
    let mut e = exe::llist_get_next(read_usize(this));
    while e != 0 {
        out.set_cursor(out.head());
        let mut ja = false;
        loop {
            let x = exe::llist_get_next(out);
            if x == 0 {
                break;
            }
            if read_usize(x) == read_usize(e + 4) {
                ja = true;
                break;
            }
        }
        if !ja {
            (exe::llist_add_tail(out, e + 4, 4) as u32);
            n = n.wrapping_add(1);
        }
        e = exe::llist_get_next(read_usize(this));
    }
    n
}

/// Node values of a linked list (head +2, item +0, next +6) without touching the cursor (+0xa).
pub(crate) unsafe fn national_teams_list_items(list: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut in_the = read_usize(list + 2);
    while in_the != 0 {
        v.push(read_usize(in_the));
        in_the = read_usize(in_the + 6);
    }
    v
}

// simplification: 0x775580 appends nodes (allocates): only the original runs. The plan, made earlier through the nodes (without touching the
// cursors): the call-up competitions not yet in `saida`, in order, without repeats; then, the return value
// is how many and the last items of `saida` (4 bytes each) are them.

/// O ensaio do comparador da `select_squads_for_fixture`: a cópia roda sem escolher elenco (`select_squad`), sem dispensar os acima da idade do
/// sub-21 (`release_overage_u21_players`: no ensaio, só o byte +0x6b que ela zera depois da data) e sem apagar repetidos (`national_teams_remove_duplicate_squad_entries`).
pub(crate) static SELECT_SQUADS_DRY_RUN: AtomicBool = AtomicBool::new(false);
/// In the dry run: the copy requested a squad (the comparison is not valid).
pub(crate) static SELECT_SQUADS_REQUESTED_SQUAD: AtomicBool = AtomicBool::new(false);

/// One squad of nation record `reg` (type 0 main, 1 B/Olympic, 2 under-21).
///
/// Today's mark is at `+0x32`/`+0x46`/`+0x5a`, the squad date at `+0x3a`/`+0x4e`/`+0x62`, the called-up mark at
/// `+0x43`/`+0x57`/`+0x6b`. With the squad up to 14 days old (`0x54cdb0` against today) and the mark (and `extra`), it
/// stamps today (`0x417c10`) and moves the squad date up to the match date (`+0x2a`/`+0x28`) if it is earlier;
/// otherwise it requests the squad (`0x76c120` with the nation, the type, today, the match and 0).
pub(crate) unsafe fn refresh_or_select_squad(this: usize, p: Fixture, club: Club, reg: usize, kind: u32, extra: impl FnOnce() -> bool) {
    let base = reg + [0x32usize, 0x46, 0x5a][kind as usize];
    let data = base + 8;
    let d = exe::date_days_since(G_CURRENT_DATE, read_usize(data) as u32, read_usize(data + 4) as u32);
    if d <= 0xe && read_u8(base + 0x11) != 0 && extra() {
        let mut r = [0u8; 8];
        exe::date_operator_3d(base, (r.as_mut_ptr()) as usize, G_CURRENT_DATE);
        let mut game = [0u8; 8];
        exe::new_date((game.as_mut_ptr()) as *mut u8 as usize, (p.day() as i16) as u16 as u32, (p.year() as i16) as u16 as u32);
        let (ra, rd) = (read_i16(data + 2) as u16, read_i16(data) as i16);
        let (ja, jd) = (u16::from_le_bytes([game[2], game[3]]), i16::from_le_bytes([game[0], game[1]]));
        if ra < ja || (ra == ja && rd < jd) {
            core::ptr::copy_nonoverlapping(data as *const u8, r.as_mut_ptr(), 8);
            exe::date_operator_3d(data, (r.as_mut_ptr()) as usize, game.as_ptr() as usize);
        }
        return;
    }
    if SELECT_SQUADS_DRY_RUN.load(Relaxed) {
        SELECT_SQUADS_REQUESTED_SQUAD.store(true, Relaxed);
        return;
    }
    covered!("cpx_13 squad requested");
    exe::nationalteams_select_squad(this, club.nation().addr(), kind, G_CURRENT_DATE, p, 0); // cpx13: o elenco
}

/// Refreshes the B/Olympic (match +0x4e bit 0x100) or U21 (bit 0x200 or B national team) squad for the club side.
pub(crate) unsafe fn refresh_minor_squads(this: usize, p: Fixture, club: Club, reg: usize) {
    let m = (p.squad_rules() as i16) as u16;
    if m & squad_rules::B_SQUAD_MATCH != 0 {
        refresh_or_select_squad(this, p, club, reg, 1, || true);
    } else if m & squad_rules::U21_SQUAD_MATCH != 0 || exe::club_is_national_team(club) != 0 {
        refresh_or_select_squad(this, p, club, reg, 2, || {
            read_usize(p.addr()) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) || read_u8(read_usize(club.nation().addr()).wrapping_add(read_usize(this + national_teams_fields::OLYMPIC_NATION_MARKS))) == 0
        });
    }
}

/// Calls release_overage_u21_players, or in dry-run clears the U21 call-up mark (+0x6b) when today is past the U21
/// date.
pub(crate) unsafe fn release_overage_u21(this: usize, club: Club) {
    if !SELECT_SQUADS_DRY_RUN.load(Relaxed) {
        return exe::nationalteams_release_overage_u21_players(this, club.nation().addr());
    }
    let reg = club_nation_record(this, club);
    if exe::date_operator_3e(G_CURRENT_DATE, read_usize(reg + national_squad_record::U21_SQUAD_DATE) as u32, read_usize(reg + national_squad_record::U21_SQUAD_DATE + 4) as u32) != 0 {
        write_u8(reg + national_squad_record::U21_CALLED_UP_MARK, 0);
    }
}

/// Selects the squads for a fixture: from two weeks before the match, for each side, a senior, B/Olympic or under-21
/// squad at most 14 days old and flagged only moves its date up to the match; otherwise asks for a new one (`select_squad`);
/// releases the over-age under-21 players and clears duplicates.
///
/// Ret 4; SEH frame without a destructor: `NationalTeams::select_squads_for_fixture(fixture)`: without flag +0x40 with
/// bit 1 of +0x4e, or with a B national side at home (`exe::club_is_national_team`) with bit 2, nothing. Only from two weeks before the
/// match (`exe::date_sub_weeks` over date +0x2a/+0x28). Home side: outside a B national side, the senior squad
/// (refresh_or_select_squad type 0) and, without bit 2, the release of the under-21 players (`release_overage_u21_players`) of both nations;
/// the smaller squads (refresh_minor_squads). Away side: the same without the release. Finally deletes the duplicates
/// (`national_teams_remove_duplicate_squad_entries`) of both.
/// Original: `NationalTeams::select_squads_for_fixture` (0x76bab0, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn select_squads_for_fixture(this: usize, p: usize) {
    let p = Fixture(p);
    if p.home_side_mark() == 0 && p.flags() & 1 != 0 {
        return;
    }
    if exe::club_is_national_team(p.home_club()) != 0 && p.flags() & 2 != 0 {
        return;
    }
    let mut game = [0u8; 8];
    exe::new_date((game.as_mut_ptr()) as *mut u8 as usize, (p.day() as i16) as u16 as u32, (p.year() as i16) as u16 as u32);
    let (hd, ha) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    let mut r = [0u8; 8];
    let q = exe::date_sub_weeks((game.as_mut_ptr()) as usize, (r.as_mut_ptr()) as *mut [u8; 8], 2); // cpx13: a janela
    let (qa, qd) = (read_i16(q as usize + 2) as u16, read_i16(q as usize));
    if ha < qa || (ha == qa && hd < qd) {
        return;
    }
    let (casa, fora) = (p.home_club(), p.away_club());
    let reg = club_nation_record(this, casa);
    if exe::club_is_national_team(casa) == 0 {
        refresh_or_select_squad(this, p, casa, reg, 0, || true);
        if p.flags() & 2 == 0 {
            release_overage_u21(this, p.home_club());
            release_overage_u21(this, p.away_club());
        }
    }
    refresh_minor_squads(this, p, casa, reg);
    let reg = club_nation_record(this, fora);
    if exe::club_is_national_team(fora) == 0 {
        refresh_or_select_squad(this, p, fora, reg, 0, || true);
    }
    refresh_minor_squads(this, p, fora, reg);
    if !SELECT_SQUADS_DRY_RUN.load(Relaxed) {
        exe::national_teams_remove_duplicate_squad_entries(p.home_club().nation().addr());
        exe::national_teams_remove_duplicate_squad_entries(p.away_club().nation().addr());
    }
}

/// Club of the person's contract (`exe::contract_manager_get_contract`, stored in [0xae1080]; club = table + 0x245 x contract +4), or 0.
pub(crate) unsafe fn club_of_person_contract(person: Staff) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 {
        return 0;
    }
    read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE))
}

/// O ensaio do comparador da `national_teams_post_player_called_up_news`: a cópia roda sem entregar a notícia.
pub(crate) static CALLED_UP_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// With a human manager at the player's club, sends news item 0x21 (the player, the national side and the slot) to the
/// manager of club +0x39 and of the contract club.
///
/// Stdcall, ret 0xc; SEH frame without a destructor: `national_teams_post_player_called_up_news(player, nation, slot)`:
/// only with a human manager (`news_club_has_human_manager`) at the player's club +0x39 or at the contract club (which must exist). News
/// 0x21 (local): 0 the id, 1..3 the +0x33 of +4, +8, +0xc; slot 0: 4 the senior national side (`exe::database_get_national_team`), 5 = 0; slot
/// 1: the senior side and 1; other: the B team (`exe::database_get_national_u21_team`) and 0. Delivered to the manager (`send_to_club_manager`) of +0x39, if any,
/// and to that of the contract club when it is another one.
/// Original: `national_teams_post_player_called_up_news` (0x76fd10, national_teams.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn national_teams_post_player_called_up_news(player: usize, nation: usize, slot: u32) {
    let player = Staff(player);
    let human: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let c39 = player.club_job();
    if c39.is_null() || human(c39.addr()) == 0 {
        if club_of_person_contract(player) == 0 {
            return;
        }
        if human(club_of_person_contract(player)) == 0 {
            return;
        }
    }
    covered!("cpx_13 call-up news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x21, 0);
    exe::newsitem_set_slot(n, 0, read_usize(player.addr()) as u32);
    exe::newsitem_set_slot(n, 1, read_usize(player.first_name() + names::ID) as u32);
    exe::newsitem_set_slot(n, 2, read_usize(player.second_name() + names::ID) as u32);
    exe::newsitem_set_slot(n, 3, read_usize(player.common_name() + names::ID) as u32);
    match slot as u8 {
        0 => {
            exe::newsitem_set_slot(n, 4, read_usize(exe::database_get_national_team(nation)) as u32);
            exe::newsitem_set_slot(n, 5, 0);
        }
        1 => {
            exe::newsitem_set_slot(n, 4, read_usize(exe::database_get_national_team(nation)) as u32);
            exe::newsitem_set_slot(n, 5, 1);
        }
        _ => {
            exe::newsitem_set_slot(n, 4, read_usize(exe::database_get_national_u21_team(nation)) as u32);
            exe::newsitem_set_slot(n, 5, 0);
        }
    }
    let news = read_usize(G_NEWS);
    let c39 = player.club_job();
    if !c39.is_null() && !CALLED_UP_NEWS_DRY_RUN.load(Relaxed) {
        exe::news_send_to_club_manager(news, n, c39);
    }
    if club_of_person_contract(player) == player.club_job().addr() {
        return;
    }
    let c = club_of_person_contract(player);
    if !CALLED_UP_NEWS_DRY_RUN.load(Relaxed) {
        exe::news_send_to_club_manager(news, n, c);
    }
}

// simplification: the cup’s virtual +0x44 appends records to people and the news goes to the managers’ inboxes (they allocate): only
// the original; the copy takes the branch.

/// The comparer's dry run of `record_international_result`: the copy runs without the FIFA ranking (`exe::fifa_rankings_record_match_result`, adds points to the history),
/// without the first call-up (`add_first_cap_record`, goes to the file), without the national-team match in the transfer record
/// (`record_international_appearance`) and without the news item to the clubs (`send_club_international_report_news`, allocates).
pub(crate) static INTERNATIONAL_RESULT_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Lineup slot k (0..20) of side in match summary m: m + 0x625 x side + 0x190 + 0x49 x k.
pub(crate) fn lineup_slot(m: usize, side: usize, k: usize) -> usize {
    let side = MatchRecord(side);
    m.wrapping_add(match_record::SIDE_SIZE * side.addr() + match_record::ROW_STAFF_ID + match_record_row::SIZE * k)
}

/// Person in a lineup slot if it counts (id != -1 and starter, or sub who came on) and is in range of the staff table.
pub(crate) unsafe fn lineup_slot_person(slot: usize, k: usize) -> Option<usize> {
    let slot = MatchRecordRow(slot);
    let id = read_usize(slot.addr()) as i32;
    if id == -1 || (k >= 11 && (slot.minute_on() as u8) == 0xff) {
        return None;
    }
    let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if p != 0 && read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return None;
    }
    Some(p)
}

/// The person's 79-byte general state record in [0xae2d0c].
pub(crate) unsafe fn national_teams_staff_state_record(p: usize) -> usize {
    read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(staff_state::SIZE))
}

/// National-teams record of the club's nation: [this+4] + 0x6e x id.
pub(crate) unsafe fn club_nation_record(this: usize, club: Club) -> usize {
    read_usize(this + national_teams_fields::NATION_RECORDS).wrapping_add(read_usize(club.nation().addr()).wrapping_mul(0x6e))
}

/// If the person's second nation (+0x1e) is the club's nation, makes it the first (+0x1a) and the old first the second.
pub(crate) unsafe fn swap_to_club_nation(p: Staff, club: Club) {
    if club.nation() == p.second_nation() {
        p.set_second_nation(p.nation());
        p.set_nation(club.nation());
    }
}

/// For both sides: increments the nation's match count (+2 of its record) and calls f(club, slot, k, person) for each
/// counting lineup person.
pub(crate) unsafe fn for_each_lineup_person(this: usize, p: Fixture, m: usize, mut f: impl FnMut(usize, usize, usize, usize)) {
    for lado in 0..2usize {
        let club = read_usize(p.addr() + if lado == 0 { 0x1c } else { 0x20 });
        let games = (club_nation_record(this, Club(club)) + 2) as *mut i16;
        core::ptr::write_unaligned(games, core::ptr::read_unaligned(games).wrapping_add(1));
        for k in 0..20usize {
            let slot = lineup_slot(m, lado, k);
            if let Some(pessoa) = lineup_slot_person(slot, k) {
                f(club, slot, k, pessoa);
            }
        }
    }
}

/// Records a result between two national sides: counts each nation's match; by competition, marks the general record,
/// swaps the nationality or, in real internationals, scores the FIFA ranking and adds caps and goals to everyone who
/// played; clears the type's +0x44 on its date and sends the news to the clubs.
///
/// Ret 8: `NationalTeams::record_international_result(fixture, summary)`: only between two national sides (`exe::database_is_national_team`).
/// By the fixture's competition (`[+0x14]`): that of `[0x9cf794]` marks +0xd = 1 in the general record of those who
/// played; that of `[0x9cf798]` sets bit 2 of +0xc and swaps the nation (+0x1a/+0x1e); that of `[0x9cf7a0]` nothing;
/// the others: the FIFA ranking (`exe::fifa_rankings_record_match_result` on `[this+0x1c]`) and, for those who played, the first call-up (`add_first_cap_record` with
/// the opponent, if +0x22 = 0, after the nation swap), a cap (+0x22) and the goals (+0x23 += +0x21 of the slot) for the
/// national side and the match in the record (`record_international_appearance` with the nation's number of matches). In all of them, the
/// nation's match count (+2) goes up. Then, per side: the type (1 if +0x40 = 0; otherwise 2 for the second team
/// `exe::club_is_national_team`, otherwise 0); on the type's date (+0x3a + 20 × type, `exe::date_operator_3d_3d` with today's) zeroes +0x44 + 20 × type; the
/// news to the clubs (`send_club_international_report_news`) with the star player (slot +0x153) and the list of those who played. No SEH, no box, no
/// random draw.
/// Original: `NationalTeams::record_international_result` (0x76b600, national_teams.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn record_international_result(this: usize, p: usize, m: usize) {
    let p = Fixture(p);
    if exe::database_is_national_team(p.home_club()) == 0 || exe::database_is_national_team(p.away_club()) == 0 {
        return;
    }
    let comp = read_usize(p.competition().addr());
    if comp == read_usize(KEY_CLUB_COMP_B_INTERNATIONAL) {
        for_each_lineup_person(this, p, m, |_, _, _, person| *((national_teams_staff_state_record(person) + staff_state::B_INTERNATIONAL_PLAYED) as *mut u8) = 1);
    } else if comp == read_usize(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) {
        for_each_lineup_person(this, p, m, |club, _, _, person| {
            *((national_teams_staff_state_record(person) + staff_state::EXTRA_FLAGS) as *mut u8) |= extra_flags::PLAYED_U21_INTERNATIONAL;
            swap_to_club_nation(Staff(person), Club(club));
        });
    } else if comp != read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES) {
        if !INTERNATIONAL_RESULT_DRY_RUN.load(Relaxed) {
            exe::fifa_rankings_record_match_result(read_usize(this + national_teams_fields::FIFA_RANKING), p);
        }
        for lado in 0..2usize {
            let club = read_usize(p.addr() + if lado == 0 { 0x1c } else { 0x20 });
            let games = (club_nation_record(this, Club(club)) + 2) as *mut i16;
            core::ptr::write_unaligned(games, core::ptr::read_unaligned(games).wrapping_add(1));
            for k in 0..20usize {
                let slot = MatchRecordRow(lineup_slot(m, lado, k));
                let id = read_usize(slot.addr()) as i32;
                if id == -1 {
                    continue;
                }
                if k >= 11 && (slot.minute_on() as u8) == 0xff { // cpw12: the substitute who did not come on
                    covered!("cpw_12 reserve not coming on");
                    continue;
                }
                let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
                if person != 0 && read_usize(person) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
                    continue;
                }
                if read_u8(person + staff::INT_APPS) == 0 {
                    swap_to_club_nation(Staff(person), Club(club));
                    let opponent = if club == p.home_club().addr() { p.away_club() } else { p.home_club() };
                    if !INTERNATIONAL_RESULT_DRY_RUN.load(Relaxed) {
                        exe::staff_records_add_first_cap_record(read_usize(G_STAFF_RECORDS), person, opponent);
                    }
                }
                covered!("cpw_12 national team match");
                write_u8(person + staff::INT_APPS, read_u8(person + staff::INT_APPS).wrapping_add(1)); // cpw12: one match
                write_u8(person + staff::INT_GOALS, read_u8(person + staff::INT_GOALS).wrapping_add(slot.goals())); // cpw12: os gols
                if !INTERNATIONAL_RESULT_DRY_RUN.load(Relaxed) {
                    // the original pushes edx with the short of the matches below and the high part of 55 × nation id
                    let id_nation = read_usize(read_usize(club + club::NATION)) as u32;
                    let n = core::ptr::read_unaligned(games) as u16 as u32;
                    exe::transfer_manager_record_international_appearance(read_usize(G_TRANSFER_MANAGER), person, (id_nation.wrapping_mul(0x37) & 0xffff_0000) | n);
                }
            }
        }
    }
    for lado in 0..2usize {
        let club = read_usize(p.addr() + if lado == 0 { 0x1c } else { 0x20 });
        let b = p.home_side_mark() == 0;
        let kind: usize = if exe::club_is_national_team(club) == 0 { if b { 1 } else { 0 } } else if b { 1 } else { 2 };
        let reg = club_nation_record(this, Club(club)) + 20 * kind;
        if exe::date_operator_3d_3d(reg + national_squad_record::SQUAD_DATE, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) != 0 {
            write_u8(reg + 0x44, 0);
        }
        let mut list = [0usize; 20];
        let mut q = 0usize;
        for k in 0..20usize {
            if let Some(person) = lineup_slot_person(lineup_slot(m, lado, k), k) {
                list[q] = person;
                q += 1;
            }
        }
        let highlight_k = read_i8(m + match_record::SIDE_SIZE * lado + match_record::HOME_CAPTAIN) as isize;
        let highlight_id = read_usize((m + match_record::SIDE_SIZE * lado + match_record::ROW_STAFF_ID) .wrapping_add(match_record_row::SIZE * highlight_k as usize));
        let highlight = read_usize(G_STAFF_TABLE).wrapping_add(highlight_id.wrapping_mul(staff::SIZE));
        if !INTERNATIONAL_RESULT_DRY_RUN.load(Relaxed) {
            // simplification: the original pushes the count in a dword of the local with only the low byte written; 0x7706c0 reads the byte
            exe::nationalteams_send_club_international_report_news(this, club, p, m, highlight, list.as_mut_ptr(), q as u32);
        }
    }
}

/// Regions `record_international_result` writes: each side's nation record fields and, per lineup person in range, staff +0x1a..+0x24 and
/// state record +0xc/+0xd.
pub(crate) unsafe fn caps_update_regions(this: usize, p: Fixture, m: usize) -> Vec<(usize, usize)> {
    let mut reg = Vec::new();
    for lado in 0..2usize {
        let club = read_usize(p.addr() + if lado == 0 { 0x1c } else { 0x20 });
        let r = club_nation_record(this, Club(club));
        reg.extend([(r + 2, 2), (r + national_squad_record::SQUAD_SELECTED, 1), (r + national_squad_record::B_SQUAD_SELECTED, 1), (r + national_squad_record::U21_SQUAD_SELECTED, 1)]);
        for k in 0..20usize {
            let slot = lineup_slot(m, lado, k);
            let id = read_usize(slot) as i32;
            if id == -1 {
                continue;
            }
            let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
            if !person.is_null() && (read_usize(person.addr()) as i32) < read_usize(G_STAFF_COUNT) as i32 {
                reg.push((person.addr() + staff::NATION, 10));
                reg.push((national_teams_staff_state_record(person.addr()) + staff_state::EXTRA_FLAGS, 2));
            }
        }
    }
    reg
}

/// Writes the squad name (Main, B, Under 21 or Under 23), in lower case when asked.
///
/// Ret 0x14; `this` is not read: `NationalTeams::format_squad_label(type, under23, out, size, lowercase)`: size below
/// 20: the box (line 0x1c98). Writes (0x66f4e0) the squad name: type 0 "Main", 1 "'B'", others "Under 23" (with
/// under23) or "Under 21"; with lowercase = 1, in lower case.
/// Original: `NationalTeams::format_squad_label` (0x775610, national_teams.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn format_squad_label(this: usize, kind: u32, sub23: u32, out: usize, size: i32, minusculo: u32) -> u32 {
    if size < 0x14 {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, i32, u32) -> u32>(IDX_FORMAT_SQUAD_LABEL)(this, kind, sub23, out, size, minusculo);
    }
    let text: unsafe extern "C" fn(usize, usize, ...) -> u32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s = match (minusculo as u8 == 1, kind as u8, sub23 != 0) {
        (true, 0, _) => 0x9cd9b4,
        (true, 1, _) => 0xa62fa0,
        (true, _, true) => 0xa62f94,
        (true, _, false) => 0xa62f88,
        (false, 0, _) => 0xa62f80,
        (false, 1, _) => 0xa62fa0,
        (false, _, true) => 0xa62f74,
        (false, _, false) => 0xa62f68,
    };
    text(out, s)
}

/// Nation's record in the national-teams manager: [this+4] + 0x6e x id (signed).
pub(crate) unsafe fn nation_record(this: usize, nation: usize) -> usize {
    read_usize(this + national_teams_fields::NATION_RECORDS).wrapping_add((read_usize(nation) as i32).wrapping_mul(0x6e) as usize)
}

/// Call-up type of the player in the national side: 2 for second national teams (`exe::club_is_national_team`), else bit 3 of squad marks
/// (+0x5e).
pub(crate) unsafe fn call_up_type(club: Club, p: Staff) -> u8 {
    if exe::club_is_national_team(club) != 0 {
        2
    } else {
        (p.playing_squad() >> 3) & 1 // cpv11: o tipo pelas marcas
    }
}

/// Writes news slots 0..3: the player id and the +0x33 keys of his +4, +8 and +0xc pointers.
pub(crate) unsafe fn set_news_player_slots(n: usize, p: usize) {
    exe::newsitem_set_slot(n, 0, read_usize(p) as u32);
    for k in 0..3usize {
        exe::newsitem_set_slot(n, 1 + k as i8, read_usize(read_usize(p + 4 + 4 * k) + names::ID) as u32);
    }
}

/// Writes news slot 6: severity category of the player's injury (`exe::injury_get_type_category` over the injury type), or 0.
pub(crate) unsafe fn set_news_injury_severity(n: usize, p: usize) {
    let v = if exe::injury_get_player_injury(read_usize(G_INJURY), p) != 0 {
        let t = read_u8(exe::injury_get_player_injury(read_usize(G_INJURY), p)) as i8;
        exe::injury_get_type_category(read_usize(G_INJURY), t) as i8 as i32
    } else {
        0
    };
    exe::newsitem_set_slot(n, 6, v as u32);
}

/// Club of the player's contract (`exe::contract_manager_get_contract`, cached in [0xae1080]; 0x245-byte records), or 0.
pub(crate) unsafe fn club_of_player_contract(p: Staff) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 {
        0
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE))
    }
}

/// Withdraws an injured player: with 14 or more injury days, removes the call-up from the national tactics and squad;
/// before the squad date, tells his club, opens the slot and calls up a replacement or asks the human manager.
///
/// Ret 8: `NationalTeams::withdraw_injured_player(p, days)`: with 14 or more days of injury, the called-up player
/// (+0x28 = 0xb) of the national side's club (+0x24) leaves: with an injury of type 0x51 he only leaves the tactic
/// (`remove_player_from_selection`, twice) and the squad (`national_teams_remove_player_from_squad`). Otherwise, only before the type's date (+0x3a of the type record:
/// `[this+4]` + 0x6e × nation id + 20 × type; `exe::date_operator_3e` over `[0xae2c90]`) and if list `[this]` does not have the club
/// with a date already past: the news to his club (`national_teams_post_player_withdrawn_news`), the removal, the type's open slots (+0x45) go up 1, the
/// level (`national_team_level`) and, if the national side's manager is human and decides (`exe::human_manager_is_human_club`, `exe::human_manager_controls_all_national_teams`), the request for a
/// replacement (`national_teams_post_replacement_needed_news`); otherwise the replacement (`call_up_replacement`) and the repeated call-ups deleted (`national_teams_remove_duplicate_squad_entries`). Without a
/// club: the original routine (box and 0).
/// Original: `NationalTeams::withdraw_injured_player` (0x775cd0, national_teams.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn withdraw_injured_player(this: usize, p: usize, days_arg: u32) {
    let p = Staff(p);
    let days = days_arg as u16 as i16;
    if days < 0xe { // cpv11: the day limit
        return;
    }
    let club = p.national_job();
    if club.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_WITHDRAW_INJURED_PLAYER)(this, p.addr(), days_arg);
    }
    if p.job_for_nation() != job_kind::PLAYER {
        return;
    }
    let nation = club.nation().addr();
    let rec = nation_record(this, nation);
    // simplification: the original pushes the local’s dword with the type only in the low byte (the others are stack leftovers); the
    // calls read only the byte
    let kind = call_up_type(club, p);
    let inj = exe::injury_get_player_injury(read_usize(G_INJURY), p);
    if inj != 0 && read_u8(inj) == 0x51 {
        covered!("cpv_11 illness");
        exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, p, 1, 1);
        exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, p, 0, 1);
        exe::nationalteams_remove_player_from_squad(this, nation, kind as u32, p);
        return;
    }
    let slot = rec.wrapping_add((kind as i8 as i32 * 20) as usize);
    if exe::date_operator_3e(slot + national_squad_record::SQUAD_DATE, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) == 0 {
        return;
    }
    let list = Llist(read_usize(this));
    if !list.is_null() {
        list.set_cursor((list.head() as u32) as usize);
        let mut e = exe::llist_get_next(read_usize(this));
        while e != 0 {
            if read_usize(e) == read_usize(club.addr()) && exe::date_operator_3e(G_CURRENT_DATE, read_usize(e + 8) as u32, read_usize(e + 0xc) as u32) != 0 {
                return;
            }
            e = exe::llist_get_next(read_usize(this));
        }
    }
    if days < 0x1c {
        covered!("cpv_11 injury of 14 to 27 days");
    }
    exe::national_teams_post_player_withdrawn_news(this, p, nation, kind as u32);
    exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, p, 1, 1);
    exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, p, 0, 1);
    if kind != 2 {
        covered!("cpv_11 type by flags");
    }
    exe::nationalteams_remove_player_from_squad(this, nation, kind as u32, p);
    let a = (slot + 0x45) as *mut u8;
    *a = (*a).wrapping_add(1);
    // the base goes in the original’s eax: the slot address with the low byte replaced by (tipo == 2)
    let level = exe::national_teams_get_position_group(p, rec, (slot as u32 & !0xff) | (kind == 2) as u32);
    // the level’s local is the record’s with the low byte replaced (only the byte is read)
    let level = (rec as u32 & !0xff) | level as u32;
    let pede = 'decide: {
        if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
            if club.manager().addr() != 0 || exe::human_manager_is_human_club(exe::database_get_national_team(nation), 0, 1, 0, 0) == 0 {
                break 'decide false;
            }
        }
        let t = read_usize(exe::database_get_national_team(nation) + club::MANAGER);
        kind == 0 || exe::human_manager_controls_all_national_teams(read_usize(G_HUMAN_MANAGER), t, core::ptr::null_mut()) != 0
    };
    if pede {
        covered!("cpv_11 replacement request");
        exe::national_teams_post_replacement_needed_news(this, p, nation, kind as u32, level);
        return;
    }
    covered!("cpv_11 replacement");
    exe::nationalteams_call_up_replacement(this, nation, kind as u32, level, p); // cpv11: o substituto
    exe::national_teams_remove_duplicate_squad_entries(nation);
}

/// Read-only plan of what `withdraw_injured_player` will do for the player: 0 nothing, 1 illness (just leaves), 2 leaves with news.
pub(crate) unsafe fn release_plan(this: usize, p: Staff, days: u32) -> u8 {
    if (days as u16 as i16) < 0xe || p.job_for_nation() != job_kind::PLAYER {
        return 0;
    }
    let club = p.national_job();
    let kind = call_up_type(club, p);
    let inj = exe::injury_get_player_injury(read_usize(G_INJURY), p);
    if inj != 0 && read_u8(inj) == 0x51 {
        return 1;
    }
    let slot = nation_record(this, club.nation().addr()).wrapping_add((kind as i8 as i32 * 20) as usize);
    if exe::date_operator_3e(slot + national_squad_record::SQUAD_DATE, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) == 0 {
        return 0;
    }
    let list = read_usize(this);
    if list != 0 {
        let mut in_the = read_usize(list + 2);
        while in_the != 0 {
            let e = read_usize(in_the);
            if e == 0 {
                break;
            }
            if read_usize(e) == read_usize(club.addr()) && exe::date_operator_3e(G_CURRENT_DATE, read_usize(e + 8) as u32, read_usize(e + 0xc) as u32) != 0 {
                return 0;
            }
            in_the = read_usize(in_the + 6);
        }
    }
    2
}

/// Builds news item 0x20 (the player left the national squad injured) and delivers it to his club and his contract club
/// when human-managed.
///
/// Ret 0xc; SEH frame without a destructor; `this` is not read: `national_teams_post_player_withdrawn_news(p, nation,
/// type)`: if the player's club (+0x39) or the contract club (`exe::contract_manager_get_contract`) has a human manager (`news_club_has_human_manager`), builds news
/// 0x20 in a local (`construct_news_item` with 0): data 0..3 the player and those of +4/+8/+0xc, 4 the national side's id (senior
/// `exe::database_get_national_team` in types 0 and 1, otherwise `exe::database_get_national_u21_team`), 5 = (type == 1), 6 the injury severity; delivers it to club +0x39
/// and, if it is another one, to the contract club (`send_to_club_manager`).
/// Original: `national_teams_post_player_withdrawn_news` (0x76f7d0, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn national_teams_post_player_withdrawn_news(_this: usize, p: usize, nation: usize, kind: u32) {
    let p = Staff(p);
    let human: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    if p.club_job().is_null() || human(p.club_job().addr()) == 0 {
        if club_of_player_contract(p) == 0 {
            return;
        }
        if human(club_of_player_contract(p)) == 0 {
            return;
        }
    }
    covered!("cpv_11 departure news");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x20, 0);
    set_news_player_slots(n, p.addr());
    let (sel, um) = match kind as u8 {
        0 => (exe::database_get_national_team(nation), 0),
        1 => (exe::database_get_national_team(nation), 1),
        _ => (exe::database_get_national_u21_team(nation), 0),
    };
    exe::newsitem_set_slot(n, 4, read_usize(sel) as u32);
    exe::newsitem_set_slot(n, 5, um);
    set_news_injury_severity(n, p.addr());
    let news = read_usize(G_NEWS);
    if !p.club_job().is_null() {
        exe::news_send_to_club_manager(news, n, p.club_job());
    }
    if club_of_player_contract(p) != p.club_job().addr() {
        exe::news_send_to_club_manager(news, n, club_of_player_contract(p));
    }
}

/// Builds news item 0x22 (a replacement is needed) with the team, squad count, level and type, and delivers it to the
/// national side's human manager.
///
/// Ret 0x10; SEH frame without a destructor; `this` is not read: `national_teams_post_replacement_needed_news(p,
/// nation, type, level)`: news 0x22 to the national side's manager: data 0..3 the player, 4 the team id (type 1: senior
/// with flag 8 and 5 = 1; type 0: senior, otherwise the under-21 `exe::database_get_national_u21_team`, flag 4 and 5 = 0), 6 the injury severity, 7
/// = 0, 8 those in the squad with the flag (`database_count_club_squad`), 9 the level, 0xa the type and 0xb the nation id. Delivered to
/// the team if its manager is human (`exe::human_manager_is_human_club`), otherwise to the senior side if its manager is.
/// Original: `national_teams_post_replacement_needed_news` (0x7701f0, national_teams.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn national_teams_post_replacement_needed_news(_this: usize, p: usize, nation: usize, kind: u32, level: u32) {
    covered!("cpv_11 request news");
    let (time, um, marca) = match kind as u8 {
        0 => (exe::database_get_national_team(nation), 0u32, 4u32),
        1 => (exe::database_get_national_team(nation), 1, 8),
        _ => (exe::database_get_national_u21_team(nation), 0, 4),
    };
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x22, 0);
    set_news_player_slots(n, p);
    exe::newsitem_set_slot(n, 4, read_usize(time) as u32);
    exe::newsitem_set_slot(n, 5, um);
    set_news_injury_severity(n, p);
    exe::newsitem_set_slot(n, 7, 0);
    // simplification: the mark goes in a dword of the local with only the low byte written; 0x53c590 reads only the byte
    exe::newsitem_set_slot(n, 8, exe::database_count_club_squad(time, marca, 0) as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 9, level as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 0xa, kind as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 0xb, read_usize(nation) as u32);
    let news = read_usize(G_NEWS);
    if exe::human_manager_is_human_club(time, 0, 1, 0, 0) != 0 {
        exe::news_send_to_club_manager(news, n, time);
    } else if exe::human_manager_is_human_club(exe::database_get_national_team(nation), 0, 1, 0, 0) != 0 {
        exe::news_send_to_club_manager(news, n, exe::database_get_national_team(nation));
    }
}

/// Writes the national team name (`club_get_national_team_name`) into buf and returns the grammar byte (+0x52).
pub(crate) unsafe fn national_team_name(team: Club, second: u32, buf: &mut [u8; 0x48]) -> i32 {
    let name: unsafe extern "C" fn(usize, u32, *mut u8, u32, u32, u32) -> u32 = core::mem::transmute(FN_CLUB_GET_NATIONAL_TEAM_NAME);
    name(team.addr(), second, buf.as_mut_ptr(), 0x47, 2, 1);
    team.short_gender_name() as i8 as i32
}

/// Tells whether the player may join or leave the squad now (retired, other nation, full squad, injured, squad window,
/// date and match day, human manager) and writes the reason.
///
/// Ret 0x18; SEH frame with the local list of the match search: `national_teams_can_change_squad(nation, type, p,
/// joining, msg, size)`: a player (if any) retired from the national side (`exe::staff_dislikes_club`) or from a club of another nation
/// cannot change (0). The team of the type (0 and 1 the senior side, type 1 with flag 8 and the second team; otherwise
/// the under-21 `exe::database_get_national_u21_team`, flag 4). Joining: a full squad (`database_count_club_squad` against +0x42 of the type record) or an injured
/// player (0x62c400 = 2) cannot change. Can change (1): joining with open slots (+0x45), the type with no window (+0x44
/// = 0), the type's date (+0x3a) already past or, on the day itself, a team match today (0x5acee0 into a local list);
/// in types 1 and 2 with a human manager who decides (`exe::human_manager_is_human_club`), also when list `[this]` does not have the team with a
/// past date and he does not allow it (`exe::human_manager_controls_all_national_teams`). The reasons go to `msg` (0x66f4e0). Without a nation, or `msg` with
/// fewer than 0x32 bytes: the original routine (box and 1).
/// Original: `national_teams_can_change_squad` (0x76db00, national_teams.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn national_teams_can_change_squad(this: usize, nation: usize, kind: u32, p: usize, entrando: u32, msg: usize, size: i32) -> u32 {
    let p = Staff(p);
    if nation == 0 || (msg != 0 && size < 0x32) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, i32) -> u32>(IDX_NATIONAL_TEAMS_CAN_CHANGE_SQUAD)(this, nation, kind, p.addr(), entrando, msg, size);
    }
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if !p.is_null() {
        if exe::staff_dislikes_club(p, exe::database_get_national_team(p.nation())) != 0 { // cpv12: aposentado
            covered!("cpv_12 retired");
            if msg != 0 {
                let mut name = [0u8; 0x12c];
                exe::database_format_staff_name(p, (name.as_mut_ptr()) as *mut u8 as usize, 0x12c, 1);
                text(msg, 0xa5b4c8, name.as_ptr());
            }
            return 0;
        }
        let c = p.national_job();
        if !c.is_null() && c.nation().addr() != nation {
            if msg != 0 {
                let mut name = [0u8; 0x12c];
                exe::database_format_staff_name(p, (name.as_mut_ptr()) as *mut u8 as usize, 0x12c, 1);
                let other = Nation(p.national_job().nation().addr());
                let g = other.short_gender_name() as i8 as i32;
                text(msg, 0xa5b460, g, g, name.as_ptr(), other.addr() + nation::SHORT_NAME);
            }
            return 0;
        }
    }
    let s = kind as u8;
    // the flag goes in the local [esp+0x1c] only in the low byte (the rest is stack leftover); 0x53c590 reads only the byte
    let (time, segundo, marca) = match s {
        0 => (exe::database_get_national_team(nation), 0u32, 4u32),
        1 => (exe::database_get_national_team(nation), 1, 8),
        _ => (exe::database_get_national_u21_team(nation), 0, 4),
    };
    let mut manager = read_usize(exe::database_get_national_team(nation) + club::MANAGER);
    if exe::human_manager_is_human_club(exe::database_get_national_team(nation), 0, 1, 0, 0) == 0 {
        manager = 0;
    }
    let rec = read_usize(this + national_teams_fields::NATION_RECORDS).wrapping_add((read_usize(nation) as i32).wrapping_mul(0x6e) as usize);
    let e = rec.wrapping_add((s as i8 as i32 * 20) as usize);
    if entrando != 0 {
        let ceiling = read_u8(e + 0x42) as i8;
        if (exe::database_count_club_squad(time, marca, 0) as i8) >= ceiling {
            if msg != 0 {
                let mut buf = [0u8; 0x48];
                let g = national_team_name(Club(time), segundo, &mut buf);
                text(msg, 0xa5b3e0, g, g, ceiling as i32, buf.as_ptr());
            }
            return 0;
        }
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
            if msg != 0 {
                text(msg, 0xa5b3bc);
            }
            return 0;
        }
    }
    if read_u8(e + 0x45) != 0 && entrando != 0 {
        return 1;
    }
    if read_u8(e + 0x44) == 0 {
        covered!("cpv_12 without window");
        return 1; // cpv12: no window
    }
    let today = read_usize(G_CURRENT_DATE) as u32;
    let (ano, dia) = ((today >> 16) as u16, today as u16 as i16);
    if s != 0 && manager != 0 {
        let mut passed = false;
        let list = Llist(read_usize(this));
        if !list.is_null() {
            list.set_cursor((list.head() as u32) as usize);
            let mut it = exe::llist_get_next(read_usize(this));
            while it != 0 {
                if read_usize(it) == read_usize(time) {
                    let d = read_usize(it + 8) as u32;
                    let (a, dd) = ((d >> 16) as u16, d as u16 as i16);
                    if a < ano || (a == ano && dd < dia) {
                        passed = true;
                        break;
                    }
                }
                it = exe::llist_get_next(read_usize(this));
            }
        }
        if !passed && exe::human_manager_controls_all_national_teams(read_usize(G_HUMAN_MANAGER), manager, core::ptr::null_mut()) == 0 {
            return 1;
        }
    }
    let d = read_usize(e + national_squad_record::SQUAD_DATE) as u32;
    let (a, dd) = ((d >> 16) as u16, d as u16 as i16);
    if a < ano || (a == ano && dd < dia) {
        covered!("cpv_12 past date");
        return 1; // cpv12: data passada
    }
    if a == ano && dd as u16 == dia as u16 {
        covered!("cpv_12 match today");
        let search: unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, i32, i32, i32, u32, i32, u32, u32, u32, u32, usize, u32, i32, u32, u32) -> u16 =
            core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
        // the date slots the search reads (the year word, the day dword, the slice byte), as the original builds them
        let mut v_year: u32 = ano as u32;
        let mut v_day: u32 = today;
        let mut slice: u8 = 0;
        let mut l = [0u32; 4];
        let lp = l.as_mut_ptr() as usize;
        exe::new_llist(lp);
        let found = search(read_usize(G_FIXTURE_MANAGER), &mut v_year, &mut v_day, &mut slice, 1, -1, -1, -1, read_usize(time) as u32, -1, 0, 0, 0, 1, lp, 1, -2,
            (segundo == 0) as u32, 0);
        exe::drop_llist(lp);
        if found != 0 {
            return 1;
        }
    }
    if msg != 0 {
        let mut buf = [0u8; 0x48];
        let g = national_team_name(Club(time), segundo, &mut buf);
        text(msg, if entrando != 0 { 0xa5b34c } else { 0xa5b2d8 }, g, g, buf.as_ptr());
    }
    0
}

/// Tells whether the player is in the squad of the type (senior, second or under-21) with the type's flag.
///
/// Stdcall, ret 0xc: `national_teams_player_in_squad(p, nation, type)`: 1 if the club of the player's national side
/// (+0x24) is the team of the type (0 and 1 the senior side `exe::database_get_national_team`, 2 the under-21 `exe::database_get_national_u21_team`) and he has the type's
/// flag (+0x5e: 4 in types 0 and 2, 8 in 1). No club or another type: 0.
/// Original: `national_teams_player_in_squad` (0x76da80, national_teams.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn national_teams_player_in_squad(p: usize, nation: usize, kind: u32) -> u32 {
    let p = Staff(p);
    if p.national_job().is_null() {
        return 0;
    }
    let (time, marca) = match kind as u8 {
        0 => (exe::database_get_national_team(nation), 4u8),
        1 => (exe::database_get_national_team(nation), 8),
        2 => (exe::database_get_national_u21_team(nation), 4),
        _ => return 0,
    };
    (p.national_job() == Club(time) && p.playing_squad() & marca != 0) as u32
}

/// Day (signed word) and year (unsigned word) of an 8-byte date.
pub(crate) unsafe fn national_teams_date_day_year(p: usize) -> (i16, u16) {
    (read_i16(p), read_u16(p + 2))
}

/// Tells whether a regular club has too many players called up (more than 2) between dates `a` and `b`.
///
/// Applies to clubs with id < [0xae235c] − 2 × [0xae2350] (regular club, not a national team). The club's list is at
/// [this + 0x20] + 6 × id (+0 the 9-byte records in date order, +5 the count as a signed byte); it skips those before
/// `a`, stops at the first after `b` and sums bytes +8 (as a byte, with wraparound). More than 2: too many.
pub(crate) unsafe fn has_too_many_called_up(this: usize, club: Club, a: usize, b: usize) -> bool {
    if club.is_null() {
        return false;
    }
    let limit = read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT).wrapping_mul(2));
    let id = read_i32(club.addr());
    if id >= limit {
        return false;
    }
    let rec = read_usize(this + national_teams_fields::CLUB_CALL_UP_RECORDS).wrapping_add((id as usize).wrapping_mul(6));
    let n = read_i8(rec + 5);
    let list = read_usize(rec);
    let (a_dia, a_ano) = national_teams_date_day_year(a);
    let (b_dia, b_ano) = national_teams_date_day_year(b);
    let mut sum: i8 = 0;
    let mut k: i8 = 0;
    while k < n {
        let e = list.wrapping_add(k as isize as usize * 9);
        let (dia, ano) = national_teams_date_day_year(e);
        k = k.wrapping_add(1);
        if a_ano > ano || (a_ano == ano && a_dia > dia) {
            continue; // before the window
        }
        if b_ano < ano || (b_ano == ano && b_dia < dia) {
            break; // after the window: the list is in order
        }
        covered!("cpp_12 called up in the window");
        sum = sum.wrapping_add(read_i8(e + 8));
    }
    sum > 2 // more than two called up
}

/// friendly fixture: 1; otherwise 0 when the home or away club (a plain club) has more than 2 players on call-up from a
/// day before to a day after the fixture date, else 1; turned off (DESLIGADAS) from 2026-10-30 to 2026-10-31, when the test criterion changed and the list was emptied: the detour alone (without
/// running the copy) made the career in original mode leave the reference be54a138 (bisection of the 39 routines of
/// batch 252): the detour changes the stack residue a later original reads; the copy stays in the code and does not run
/// Original: `NationalTeams::can_schedule_club_fixture` (0x776fc0, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn can_schedule_club_fixture(this: usize, game: usize) -> u32 {
    let game = Fixture(game);
    let (mut d, mut a, mut b) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    // simplification: the original pushes the words +0x2a and +0x28 with the high part of the register (garbage); 0x549ef0 only reads the
    // low words: the copy passes the clean words.
    exe::new_date((d.as_mut_ptr()) as *mut u8 as usize, game.day() as u32, game.year() as u32);
    exe::date_sub_days((d.as_mut_ptr()) as *mut u8 as usize, (a.as_mut_ptr()) as *mut u8 as usize, 1);
    exe::date_add_days((d.as_mut_ptr()) as usize, (b.as_mut_ptr()) as usize, 1);
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), game.competition().addr()) != 0 {
        return 1;
    }
    let (pa, pb) = (a.as_ptr() as usize, b.as_ptr() as usize);
    if has_too_many_called_up(this, game.home_club(), pa, pb) {
        return 0;
    }
    if has_too_many_called_up(this, game.away_club(), pa, pb) {
        covered!("cpp_12 away side called up");
        return 0; // the away club with too many called up
    }
    1
}

/// qsort comparer of records by the date at +0 (`exe::date_operator_3c`/`exe::date_operator_3e`), earliest first.
///
/// `compare_call_up_records_by_date(a, b)`: the qsort of records with the date at +0 (the 9-byte call-ups, the
/// injuries, the transfers): −1 if that of `a` is earlier (`exe::date_operator_3c`), 1 if later (`exe::date_operator_3e`), otherwise 0. Hot (qsort):
/// no counter.
/// Original: `compare_call_up_records_by_date` (0x778e90, national_teams.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_call_up_records_by_date(a: usize, b: usize) -> i32 {
    let (d0, d1) = (read_usize(b) as u32, read_usize(b + 4) as u32);
    let r = if exe::date_operator_3c(a, d0, d1) != 0 { -1 } else { (exe::date_operator_3e(a, d0, d1) != 0) as i32 };
    r // the earliest call-up first
}

/// With nation and finals both non-null, appends the pair to the new qualifiers list (+0x28).
///
/// Ret 8: `NationalTeams::add_qualified_nation(nation, finals)`: with both non-null, appends the 8-byte record (nation,
/// finals) to the list of new qualifiers at this + 0x28 (LLIST, 0x68d0d0).
/// Original: `NationalTeams::add_qualified_nation` (0x7779b0, national_teams.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn add_qualified_nation(this: usize, nation: usize, stage_final: usize) {
    if nation == 0 || stage_final == 0 {
        return;
    }
    let mut reg = [0u8; 8];
    reg[..4].copy_from_slice(&(nation as u32).to_le_bytes());
    reg[4..].copy_from_slice(&(stage_final as u32).to_le_bytes());
    exe::llist_add_tail(this + national_teams_fields::NEW_QUALIFIERS, reg.as_mut_ptr() as usize, 8);
}

/// Is competition id one of the major finals (World Cup, Olympics, Euro, Copa America, ACN, Asian Cup, OFC Nations Cup,
/// Asian Games, Confederations Cup)?
pub(crate) unsafe fn is_major_finals(id: u32) -> bool {
    [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]
        .iter()
        .any(|&g| read_usize(g) as u32 == id)
}

/// What `0x775220` writes.
///
/// The nation's current competition ([this+4] + id × 110 + 4), the queue's ([this+8] + id × 4) and whether it notifies
/// the managers' manager.
pub(crate) struct NationCurrentCompUpdate {
    pub(crate) slot: usize,
    pub(crate) new: Option<u32>,
    pub(crate) queue: usize,
    pub(crate) in_queue: Option<u32>,
    pub(crate) avisa: bool,
}

/// Read-only plan of `set_current_competition`: whether the new competition becomes current or is queued, what goes to the queue, and
/// whether the manager manager is notified.
pub(crate) unsafe fn current_comp_plan(this: usize, nation: usize, comp: usize) -> NationCurrentCompUpdate {
    let id = read_usize(nation);
    let vaga = read_usize(this + national_teams_fields::NATION_RECORDS).wrapping_add(id.wrapping_mul(110)) + 4;
    let fila = read_usize(this + national_teams_fields::NEXT_COMPETITIONS).wrapping_add(id.wrapping_mul(4));
    let current = read_usize(vaga) as u32;
    let nova = read_usize(comp) as u32;
    let mut p = NationCurrentCompUpdate { slot: vaga, new: Some(nova), queue: fila, in_queue: None, avisa: read_usize(G_MANAGER_MANAGER) != 0 };
    if current != 0xffff_ffff {
        let r = read_usize(G_CLUB_COMP_TABLE).wrapping_add((current as usize).wrapping_mul(107)); // 107-byte records
        if r != 0 && is_major_finals(read_usize(r) as u32) {
            return NationCurrentCompUpdate { slot: vaga, new: None, queue: fila, in_queue: Some(nova), avisa: false };
        }
        if is_major_finals(nova) {
            p.in_queue = Some(current);
        }
    }
    p
}

/// makes the competition the nation's current one; with a major finals under way, the new one goes to the queue; when
/// the new one is a major finals, the current one goes to the queue; notifies the manager handler (`set_season_expectations`)
/// Original: `NationalTeams::set_current_competition` (0x775220, national_teams.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_current_competition(this: usize, nation: usize, comp: usize) {
    if nation == 0 || comp == 0 {
        show_error_at(0xa5b2a4, 0x1bb2);
        return;
    }
    let p = current_comp_plan(this, nation, comp);
    if let Some(f) = p.in_queue {
        write_u32(p.queue, f);
    }
    let Some(nova) = p.new else {
        covered!("cpo_13 final stage stays");
        return;
    };
    covered!("cpo_13 current competition");
    write_u32(p.slot, nova);
    let g = read_usize(G_MANAGER_MANAGER);
    if g != 0 {
        exe::manager_manager_set_season_expectations(g, exe::database_get_national_team(nation), 1);
    }
}

/// Writes code for club c (if present and id < n) into the table at [this+0x24].
pub(crate) unsafe fn mark_club_code(this: usize, c: usize, n: i32, code: u8) {
    if c != 0 && (read_usize(c) as i32) < n {
        write_u8(read_usize(this + national_teams_fields::CLUB_MARKS).wrapping_add(read_usize(c)), code);
    }
}

/// Does the club have a nation (+0x53) with byte +0x11c set?
pub(crate) unsafe fn has_flagged_nation(c: Club) -> bool {
    let n = c.nation();
    !n.is_null() && n.league_selected() != 0
}

/// in the one-byte-per-club table, marks the clubs of the non-friendly fixtures on the date: 1 the club's league game,
/// 2 another, 3 a club of a flagged nation (entry with no nation); turned off (DESLIGADAS) from 2026-10-29 to 2026-10-31, when the test criterion changed and the list was emptied: with the detour the
/// career in Rust mode left the reference be54a138 (bisection of batch 251's 39 routines; with `construct_news_item`, the news
/// constructor, the game later crashed in `player_stats_reset_player` on a broken index); the cause (stack residue or a difference in the
/// copy) is not isolated yet; the copy stays in the code and does not run
/// Original: `NationalTeams::mark_clubs_with_fixtures` (0x7775c0, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn mark_clubs_with_fixtures(this: usize, data: usize) {
    let n = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    if read_usize(this + national_teams_fields::CLUB_MARKS) == 0 {
        let p = exe::crt_calloc((n) as u32, 1);
        write_u32(this + national_teams_fields::CLUB_MARKS, p as u32);
        if p == 0 {
            show_error_at(0xa5b2a4, 0x2229);
            return;
        }
    }
    let get_fixture_list_for_date: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_FIX_MAN_GET_FIXTURE_LIST_FOR_DATE);
    for i in 0..0x21usize {
        let e = G_LEAGUE_WORLD_CLUB_NATION + i * 0x48;
        let nation = Nation(read_usize(e));
        if !nation.is_null() && nation.league_selected() == 0 {
            continue;
        }
        for fatia in 0..3u32 {
            // the original pushes the slice in a stack dword with a garbage high part and the group in al: 0x5abf10 only reads the bytes
            let list = Llist(get_fixture_list_for_date(data, fatia, read_u8(e + key_nation_record::ACTIVE_INDEX) as u32));
            if list.is_null() {
                continue;
            }
            list.set_cursor((list.head() as u32) as usize);
            let mut game = exe::llist_get_next(list);
            while game != 0 {
                if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(game + fixture::COMPETITION)) == 0 {
                    let (casa, fora) = (read_usize(game + fixture::HOME_CLUB), read_usize(game + fixture::AWAY_CLUB));
                    if read_usize(e) == 0 {
                        if casa != 0 && has_flagged_nation(Club(casa)) {
                            mark_club_code(this, casa, n, 3);
                        }
                        if fora != 0 && has_flagged_nation(Club(fora)) {
                            mark_club_code(this, fora, n, 3);
                        }
                    } else {
                        let comp = read_usize(game + fixture::COMPETITION);
                        // the away side +0x60 is read without checking the pointer (like the original)
                        let code = if casa != 0 && (read_usize(casa + club::DIVISION) == comp || read_usize(fora + club::RESERVE_DIVISION) == comp) { 1 } else { 2 };
                        covered!("cpo_13 clubs with a match");
                        mark_club_code(this, casa, n, code); // o mandante
                        mark_club_code(this, fora, n, code); // o visitante
                    }
                }
                game = exe::llist_get_next(list);
            }
        }
    }
}

/// Items of list l (head +2, item +0, next +6, capped at 100000) without touching the cursor.
pub(crate) unsafe fn list_items_capped(l: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut in_the = read_usize(l + 2);
    while in_the != 0 && v.len() < 100_000 {
        v.push(read_usize(in_the));
        in_the = read_usize(in_the + 6);
    }
    v
}

/// Ends call-ups in list [this]: resets the cursor and sets +0x10 to tomorrow's day for each record passing the filter.
pub(crate) unsafe fn end_call_ups(this: usize, passes: impl Fn(usize) -> bool) {
    let l = Llist(read_usize(this));
    if l.is_null() {
        return;
    }
    l.set_cursor(l.head());
    let mut r = exe::llist_get_next(read_usize(this));
    while r != 0 {
        if passes(r) {
            let mut data = [0u8; 8];
            let day = read_u16(exe::date_add_days(G_CURRENT_DATE, (data.as_mut_ptr()) as usize, 1));
            write_u16(r + call_up_entry::DUTY_END_DAY, day); // tomorrow’s day
        }
        r = exe::llist_get_next(read_usize(this));
    }
}

/// In the call-up record list, those of the competition (+4) end tomorrow (+0x10, the day from `exe::date_add_days`).
///
/// Ret 4: `end_duty_for_competition(comp)`: in the call-up record list `[this]`, those of the competition (+4 equal to
/// *comp) end tomorrow (+0x10). No SEH, no box, no random draw; the local is the date that `exe::date_add_days` writes.
/// Original: `end_duty_for_competition` (0x774f30, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn end_duty_for_competition(this: usize, comp: usize) {
    end_call_ups(this, |r| read_usize(r + 4) == read_usize(comp) /* a competição */)
}

/// like `end_duty_for_competition`, only the records of the competition (+4) and the nation (+0)
/// Original: `end_duty_for_nation_in_competition` (0x774f90, national_teams.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn end_duty_for_nation_in_competition(this: usize, nation: usize, comp: usize) {
    end_call_ups(this, |r| read_usize(r + 4) == read_usize(comp) && read_usize(r) == read_usize(nation) /* a nação */)
}

/// Regions touched by end_call_ups: the list cursor and +0x10 of each record.
pub(crate) unsafe fn end_call_ups_regions(this: usize) -> Vec<(usize, usize)> {
    let l = Llist(read_usize(this));
    if l.is_null() {
        return Vec::new();
    }
    let mut reg = vec![(l.addr() + llist::CURSOR, 4)];
    reg.extend(list_items_capped(l.addr()).into_iter().filter(|&r| r != 0).map(|r| (r + call_up_entry::DUTY_END_DAY, 2)));
    reg
}

/// Number of per-club call-up records of `prune_club_call_up_records`: club count - 2 x nation count.
pub(crate) unsafe fn club_call_up_record_count() -> i32 {
    read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT).wrapping_mul(2))
}

/// In each club record (+0x20, 6 bytes), drops the expired entries (`exe::date_operator_3c`) at the start, moves the rest down
/// (`exe::call_up_record_assign`) and frees the emptied buffer.
///
/// `prune_club_call_up_records()`: each club record (+0x20, 6 bytes: +0 vector of 9-byte entries, +4 flag, +5 count,
/// signed byte) counts the entries already expired (`exe::date_operator_3c`: their date before `[0xae2c90]`; the routine assumes they
/// are at the start), moves the others down over them (`exe::call_up_record_assign`) and decreases the count; zeroed with a vector, frees
/// the vector (`exe::crt_free`) and zeroes +4 and +0. No SEH, no box, no random draw; the local (the expired count) is zeroed
/// before being read.
/// Original: `prune_club_call_up_records` (0x7774b0, national_teams.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn prune_club_call_up_records(this: usize) {
    let n = club_call_up_record_count();
    let mut rec = read_usize(this + national_teams_fields::CLUB_CALL_UP_RECORDS);
    if n <= 0 {
        return;
    }
    let count = |rec: usize| read_i8(rec + 5);
    for _ in 0..n {
        let mut vencidas: i8 = 0;
        if count(rec) > 0 {
            let mut k: i8 = 0;
            loop {
                let e = read_usize(rec).wrapping_add(9usize.wrapping_mul(k as i32 as usize));
                if exe::date_operator_3c(e, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) != 0 {
                    vencidas = vencidas.wrapping_add(1); // one won
                }
                k = k.wrapping_add(1);
                if k >= count(rec) {
                    break;
                }
            }
        }
        if vencidas < count(rec) {
            let mut k = vencidas;
            loop {
                covered!("call-ups shift down the remaining");
                let v = read_usize(rec);
                let of = v.wrapping_add(9usize.wrapping_mul(k as i32 as usize));
                let para = v.wrapping_add(9usize.wrapping_mul((k as i32 - vencidas as i32) as usize));
                exe::call_up_record_assign(para, of);
                k = k.wrapping_add(1);
                if k >= count(rec) {
                    break;
                }
            }
        }
        let rest = count(rec).wrapping_sub(vencidas);
        write_i8(rec + 5, rest);
        if rest == 0 && read_usize(rec) != 0 {
            covered!("call-ups free the array");
            exe::crt_free(read_usize(rec));
            write_u8(rec + 4, 0);
            write_usize(rec, 0);
        }
        rec += 6;
    }
}

/// Would record rec of `prune_club_call_up_records` end with no entries but a vector (so the original frees it)? Counts entries before
/// today.
pub(crate) unsafe fn would_free_call_up_record(rec: usize) -> bool {
    let n = read_i8(rec + 5);
    let v = read_usize(rec);
    let vencidas = (0..n.max(0) as usize).filter(|&k| exe::date_operator_3c(v + 9 * k, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) != 0).count();
    v != 0 && n.wrapping_sub(vencidas as i8) == 0
}

/// National team for call-up type: main team (`exe::database_get_national_team`) for 0 and 1, U21 (`exe::database_get_national_u21_team`) otherwise.
pub(crate) unsafe fn national_team_for_type(nation: usize, kind: u8) -> Club {
    if kind == 0 || kind == 1 { Club(exe::database_get_national_team(nation)) } else { Club(exe::database_get_national_u21_team(nation)) }
}

/// Removes person p from the team's tactics selection (`remove_player_from_selection` with 1 then 0), as the originals do before clearing the
/// slot.
pub(crate) unsafe fn remove_from_tactics(team: Club, p: usize) {
    exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), team, p, 1, 1);
    exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), team, p, 0, 1);
}

/// Person leaves the national team: +0x24 = 0, call-up mark +0x28 = 0, +0x5e keeps only club bits (& 3).
pub(crate) unsafe fn clear_national_call_up(p: Staff) {
    p.set_national_job(Club(0));
    p.set_job_for_nation(job_kind::UNEMPLOYED); // the call-up mark
    p.set_playing_squad(p.playing_squad() & 3);
}

/// the first of the 50 free squad slots of the team (+0xd7; type 0/1 the senior team, else the under-21) takes the
/// person, who gets the team (+0x24), today's date (+0x29), mark 0xb (+0x28) and squad bit 4 or 8 (`staff_set_squad_flags`)
/// Original: `add_player_to_squad` (0x776250, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn add_player_to_squad(_this: usize, nation: usize, kind: u32, p: usize) {
    let p = Staff(p);
    let kind = kind as u8;
    let team = Club((national_team_for_type(nation, kind)).addr());
    let squad: u32 = if kind == 0 || kind == 2 { 4 } else { 8 };
    let slot = match (0..50usize).find(|&c| team.squad(c) == 0) {
        Some(c) => team.addr() + club::SQUAD + 4 * c,
        None => return,
    };
    write_usize(slot, p.addr()); // the place
    p.set_national_job(team);
    let mut data = [0u8; 8];
    exe::date_operator_3d(p.addr() + staff::DATE_JOINED_NATION, (data.as_mut_ptr()) as usize, G_CURRENT_DATE);
    p.set_job_for_nation(job_kind::PLAYER);
    exe::staff_set_squad_flags(p, squad);
}

/// Regions of add/remove from squad: the team's 50 slots (+0xd7) and the person's +0x24..+0x5e.
pub(crate) unsafe fn squad_change_regions(team: Club, p: Staff) -> [(usize, usize); 2] {
    [(team.addr() + club::SQUAD, 200), (p.addr() + staff::NATIONAL_JOB, 0x3b)]
}

/// Removes the person from the team tactics (`remove_player_from_selection`) and, if found in a slot, empties it and clears +0x24, +0x28 and
/// the national bits of +0x5e.
///
/// Ret 0xc: `remove_player_from_squad(nation, type, p)`: takes `p` out of the team's tactic (`remove_player_from_selection`, twice) and, if
/// he is in one of the 50 slots (+0xd7), empties the slot and clears the person (+0x24, +0x28, +0x5e & 3). No SEH, no
/// random draw, no locals; a box only with a null team or person (`remove_player_from_selection`).
/// Original: `remove_player_from_squad` (0x7762f0, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn national_teams_remove_player_from_squad(_this: usize, nation: usize, kind: u32, p: usize) {
    let team = Club((national_team_for_type(nation, kind as u8)).addr());
    remove_from_tactics(team, p);
    if let Some(c) = (0..50usize).find(|&c| team.squad(c) == p) {
        team.set_squad(c, 0);
        clear_national_call_up(Staff(p));
    }
}

/// Nation's record in the national-teams manager: [this+4] + 0x6e x id (+0 of the nation).
pub(crate) unsafe fn nation_record_by_id(this: usize, nation: usize) -> usize {
    read_usize(this + national_teams_fields::NATION_RECORDS).wrapping_add(read_i32(nation).wrapping_mul(0x6e) as usize)
}

/// Once the nation record date (+0x62) has passed, removes from the under-21 squad everyone older than 21 (+0x18) and
/// clears record +0x6b.
///
/// Ret 4: `release_overage_u21_players(nation)`: after the date of the nation record (+0x62/+0x66; `exe::date_operator_3e` over
/// `[0xae2c90]`), each of the 50 slots of the under-21 side (`exe::database_get_national_u21_team`) with a person aged (+0x18, signed byte) above 21
/// leaves the tactic (`remove_player_from_selection`, twice), is cleared (+0x24, +0x28, +0x5e & 3) and the slot is emptied; finally zeroes
/// byte +0x6b of the record. No SEH, no random draw; the local is `this`.
/// Original: `release_overage_u21_players` (0x76d880, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn release_overage_u21_players(this: usize, nation: usize) {
    let rec = nation_record_by_id(this, nation);
    if exe::date_operator_3e(G_CURRENT_DATE, read_usize(rec + national_squad_record::U21_SQUAD_DATE) as u32, read_usize(rec + national_squad_record::U21_SQUAD_DATE + 4) as u32) == 0 {
        return;
    }
    covered!("under-21 release over age");
    let team = Club(exe::database_get_national_u21_team(nation));
    for c in 0..50usize {
        let slot = team.addr() + club::SQUAD + 4 * c;
        let p = Staff(read_usize(slot));
        if !p.is_null() && (p.age() as i8) > 0x15 { // over 21 years old
            remove_from_tactics(team, p.addr());
            clear_national_call_up(p);
            write_usize(slot, 0);
        }
    }
    write_u8(nation_record_by_id(this, nation) + national_squad_record::U21_CALLED_UP_MARK, 0);
}

/// Notifies the manager manager ([0xb67a44], if any) about the nation's main team (set_season_expectations with 1).
pub(crate) unsafe fn notify_manager_manager(nation: usize) {
    let g = read_usize(G_MANAGER_MANAGER);
    if g != 0 {
        exe::manager_manager_set_season_expectations(g, exe::database_get_national_team(nation), 1);
    }
}

/// The nation's current competition (record +4) becomes −1 and, if there is a next one (`[this+8][id]`), that one takes
/// its place, notifying the managers (`set_season_expectations`).
///
/// Ret 4: `clear_current_competition(nation)`: the current competition of the nation record (`[this+4]` + 0x6e × id,
/// +4) becomes −1; if there is a next one (`[this+8][id]` ≠ −1), it becomes the current one, the next becomes −1 and
/// the manager manager is notified (`set_season_expectations`). No SEH, no box, no locals.
/// Original: `clear_current_competition` (0x775000, national_teams.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn clear_current_competition(this: usize, nation: usize) {
    let id = read_i32(nation);
    let current = read_usize(this + national_teams_fields::NATION_RECORDS).wrapping_add(id.wrapping_mul(0x6e) as usize) + 4;
    write_i32(current, -1);
    let next = read_usize(this + national_teams_fields::NEXT_COMPETITIONS).wrapping_add((id as usize).wrapping_mul(4));
    let v = read_i32(next);
    if v != -1 {
        covered!("national team the next competition");
        write_i32(current, v); // the next one becomes the current one
        write_i32(next, -1);
        notify_manager_manager(nation); // the notice of the next competition
    }
}

/// Has competition v finished for `release_competitions_of_continent`? Without end-of-stage: comp +0x59 is null; with it: it equals the European
/// continent stage.
pub(crate) unsafe fn is_comp_finished(v: i32, end_of_stage: bool) -> bool {
    let c = read_usize(read_usize(G_CLUB_COMP_TABLE).wrapping_add(v.wrapping_mul(107) as usize) + competition::CONTINENT);
    if end_of_stage {
        c == read_usize(G_CONTINENT_TABLE).wrapping_add(read_i32(KEY_CONTINENT_EUROPE).wrapping_mul(0xc6) as usize)
    } else {
        c == 0
    }
}

/// in the nations of the continent (+0x71), the current and next competitions that ended (competition +0x59 null, or
/// stage [0x9cfa10]) become −1; empty current and a next one: the next moves up, with the notice (`set_season_expectations`)
/// Original: `release_competitions_of_continent` (0x775070, national_teams.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn release_competitions_of_continent(this: usize, continent: usize, end_of_stage: u32) {
    let end = end_of_stage as u8 != 0;
    let mut i = 0i32;
    while i < read_i32(G_NATION_COUNT) {
        let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add(0x122 * i as usize));
        if nation.continent() == continent {
            let current = read_usize(this + national_teams_fields::NATION_RECORDS) + 0x6e * i as usize + 4;
            let v = read_i32(current);
            if v != -1 && is_comp_finished(v, end) {
                write_i32(current, -1); // a atual acabou
            }
            let next = read_usize(this + national_teams_fields::NEXT_COMPETITIONS) + 4 * i as usize;
            let v = read_i32(next);
            if v != -1 && is_comp_finished(v, end) {
                write_i32(next, -1);
            }
            let v = read_i32(read_usize(this + national_teams_fields::NEXT_COMPETITIONS) + 4 * i as usize);
            if v != -1 && *((read_usize(this + national_teams_fields::NATION_RECORDS) + 0x6e * i as usize + 4) as *const i32) == -1 {
                covered!("national team continent next becomes current");
                write_i32(read_usize(this + national_teams_fields::NATION_RECORDS) + 0x6e * i as usize + 4, v);
                write_i32(read_usize(this + national_teams_fields::NEXT_COMPETITIONS) + 4 * i as usize, -1);
                notify_manager_manager(nation.addr()); // the continent warning
            }
        }
        i += 1;
    }
}

/// Comparer: the warning `set_season_expectations` draws at random and touches the managers (it cannot run twice), so only the original
/// routine runs; the proof is the career and the copy's branch in Rust mode.
// simplification: original only (4 calls in the career); comparing would require separating the decision from the warning.

/// Calls up a replacement: in the pool of the level, the first eligible player of the nation who plays a position of
/// the one leaving (`position_mask`) is called up (`add_player_to_squad`, leaving another squad first through `national_teams_remove_player_from_squad`) with the news
/// (`national_teams_post_player_called_up_news`); nobody: clears the type's slot count (+0x45).
///
/// Ret 0x10: `call_up_replacement(nation, type, level, leaving)`: in the section of the level (`national_team_level`) in the type's
/// pool (`[this+0xc]`, under-21 `[this+0x10]`; start +8/+0x1d of the record plus the slots of the previous levels,
/// count +0xd/+0x22), the first one who is not `leaving`, has a player sheet (+0x61), likes the club (`exe::staff_dislikes_club`), is fit
/// (`get_player_injury_status`), is not a flagged regen (`record_bits`) outside the senior side, is from the nation (team +0x24, nationalities
/// +0x1a/+0x1e, +0x22, bit 2 of the 79-byte record), is not suspended (`is_player_suspended`) and plays one of the positions of
/// `leaving` (`position_mask`). Without a team he goes straight in; called up (+0x28 = 0xb) for another type (2 with `exe::club_is_national_team`,
/// otherwise bit 3 of +0x5e) only after the type's date (+0x3a; `exe::date_operator_3c`) and, for type 0, if the club allows
/// (`exe::human_manager_is_human_club`): he leaves there (`national_teams_remove_player_from_squad`). He joins (`add_player_to_squad`), the type's slot (+0x45) goes down 1 and the news goes
/// out (`national_teams_post_player_called_up_news`). Nobody: zeroes +0x45. No SEH, no random draw; the local for the type of the other squad is written
/// before being read (only the byte counts).
/// Original: `call_up_replacement` (0x775f90, national_teams.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn call_up_replacement(this: usize, nation: usize, kind: u32, level: u32, saindo: usize) {
    let saindo = Staff(saindo);
    let regen: unsafe extern "stdcall" fn(usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_GET_RETIREMENT_STATUS);
    let mask = exe::formation_get_position_mask(saindo.player()) as u8;
    let t = kind as u8 as i8;
    let level = level as u8 as i8;
    let rec = read_usize(this + national_teams_fields::NATION_RECORDS).wrapping_add(read_i32(nation).wrapping_mul(0x6e) as usize);
    let sub = (t == 2) as usize;
    let o = 0x15 * sub;
    let mut start = read_usize(rec + o + 8) as i32;
    for j in 0..level.max(0) as usize {
        start = start.wrapping_add(read_u8(rec + o + national_squad_record::RESERVE_LEVEL_CAPACITY + 4 * j) as i32);
    }
    let end = start.wrapping_add(read_u8(rec.wrapping_add(o).wrapping_add((4 * level as i32) as usize) + national_squad_record::RESERVE_GROUPS) as i32);
    let reserve = read_usize(this + national_teams_fields::NATION_RECORDS * sub + national_teams_fields::RESERVE_POOLS);
    let slot = rec.wrapping_add((20 * t as i32) as usize) + 0x45;
    let mut k = start;
    while k < end {
        let q = Staff(read_usize(reserve.wrapping_add((4 * k) as usize)));
        k += 1;
        if q == saindo || q.player().is_null() || exe::staff_dislikes_club(q, exe::database_get_national_team(q.nation())) != 0 {
            continue;
        }
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), q) != 0 {
            continue;
        }
        if read_usize(G_PLAYER_REGEN) != 0 && regen(q.addr()) != 0 && t != 0 {
            continue;
        }
        let team = q.national_job();
        if !team.is_null() && team.nation().addr() != nation {
            continue;
        }
        if q.nation() != Nation(nation) {
            let record = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_i32(q.addr()).wrapping_mul((staff_state::SIZE as i32)) as usize);
            if !team.is_null() || q.int_apps() != 0 || read_u8(record + staff_state::EXTRA_FLAGS) & extra_flags::PLAYED_U21_INTERNATIONAL != 0 || q.second_nation() != Nation(nation) {
                continue;
            }
        }
        let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_i32(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(107) as usize);
        if exe::discipline_is_player_banned(read_usize(G_DISCIPLINE), q, comp, 1, 0) != 0 {
            continue;
        }
        if exe::formation_get_position_mask(q.player()) as u8 & mask & 0x7f == 0 { // the positions of those leaving
            continue;
        }
        let team = q.national_job();
        if !team.is_null() {
            if q.job_for_nation() != job_kind::PLAYER {
                continue;
            }
            let other: i8 = if exe::club_is_national_team(team) != 0 { 2 } else { ((q.playing_squad() >> 3) & 1) as i8 };
            if other == t {
                continue;
            }
            let data = rec.wrapping_add((20 * other as i32) as usize) + 0x3a;
            if exe::date_operator_3c(data, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) == 0 {
                continue;
            }
            if other == 0 && exe::human_manager_is_human_club(team, 0, 1, 0, 0) == 0 {
                continue;
            }
            covered!("replacement leaves the other squad");
            exe::nationalteams_remove_player_from_squad(this, nation, other as u8 as u32, q);
        }
        covered!("replacement called up");
        exe::nationalteams_add_player_to_squad(this, nation, kind, q);
        write_u8(slot, read_u8(slot).wrapping_sub(1)); // one slot fewer
        exe::national_teams_post_player_called_up_news(this, q, nation, kind);
        return;
    }
    write_u8(slot, 0);
}

/// Comparador: a entrada no elenco sai com a notícia (`national_teams_post_player_called_up_news`) e a saída do outro mexe na tática: só a original
/// (a `add_player_to_squad` e a `national_teams_remove_player_from_squad` que ela chama passam pelos comparadores delas).
// simplification: original only; the proof is the career (140 calls) and the copy’s branch in Rust mode.

/// Rating of the person in reserve slot q: word +9 of his player record (+0x61).
pub(crate) unsafe fn slot_person_rating(q: usize) -> i16 {
    read_i16(read_usize(read_usize(q) + staff::PLAYER) + 9)
}

/// Start index of level `nivel` in the reserve (main or U21 fields): record +8 plus the slot counts of previous levels.
pub(crate) unsafe fn reserve_level_start(rec: usize, o: usize, level: i8) -> i32 {
    let mut v = read_usize(rec + o + 8) as i32;
    for j in 0..level.max(0) as usize {
        v = v.wrapping_add(read_u8(rec + o + national_squad_record::RESERVE_LEVEL_CAPACITY + 4 * j) as i32);
    }
    v
}

/// Puts person `p` in the reserve of record `rec` (main or U21) in his level's block, keeping it sorted by rating.
///
/// `sub` 0: the main team at [this+0xc]; 1: the U21 at [this+0x10], fields 0x15 further on; the block is that of his
/// level (`national_team_level`). With a free slot (+0xd < +0xe), at the end of the block, and the lowest rating (+0xf) goes down if
/// needed; when full, only with a rating above the lowest: he enters at the position given by the rating (descending,
/// after equal ones), pushing the last one out, and the lowest rating becomes the last one's. Block full: it is sorted
/// (qsort `exe::crt_qsort` with `compare_by_rating`).
pub(crate) unsafe fn add_to_reserves(this: usize, p: Staff, rec: usize, sub: u32) {
    let o = 0x15 * sub as usize;
    let level = exe::national_teams_get_position_group(p, rec, sub) as i8;
    let c = rec.wrapping_add(o).wrapping_add((4 * level as i32) as usize);
    let reserve = || read_usize(this + national_teams_fields::RESERVE_POOLS + 4 * sub as usize);
    let slot = |k: i32| reserve().wrapping_add((4 * k) as usize);
    let rating = read_i16(p.player().addr() + 9);
    if read_u8(c + national_squad_record::RESERVE_GROUPS) < read_u8(c + national_squad_record::RESERVE_LEVEL_CAPACITY) {
        let start = reserve_level_start(rec, o, level);
        write_usize(slot(start.wrapping_add(read_u8(c + national_squad_record::RESERVE_GROUPS) as i32)), p.addr());
        write_u8(c + national_squad_record::RESERVE_GROUPS, read_u8(c + national_squad_record::RESERVE_GROUPS).wrapping_add(1));
        if rating < read_i16(c + national_squad_record::RESERVE_LEVEL_MIN_REPUTATION) {
            write_i16(c + national_squad_record::RESERVE_LEVEL_MIN_REPUTATION, rating); // a menor nota
        }
    } else if rating > read_i16(c + national_squad_record::RESERVE_LEVEL_MIN_REPUTATION) {
        covered!("reserves full enters by rating");
        let start = reserve_level_start(rec, o, level);
        let last = (read_u8(c + national_squad_record::RESERVE_LEVEL_CAPACITY) as i32).wrapping_add(start).wrapping_sub(2);
        let mut pos = start;
        let mut j = last;
        while j >= start {
            if rating < slot_person_rating(slot(j)) {
                pos = j + 1; // after the ones with a higher rating
                break;
            }
            j -= 1;
        }
        let mut j = last;
        while j >= pos {
            write_usize(slot(j + 1), read_usize(slot(j)));
            j -= 1;
        }
        write_usize(slot(pos), p.addr());
        let ult = (read_u8(c + national_squad_record::RESERVE_LEVEL_CAPACITY) as i32).wrapping_add(start).wrapping_sub(1);
        write_i16(c + national_squad_record::RESERVE_LEVEL_MIN_REPUTATION, slot_person_rating(slot(ult)));
    }
    let slots = read_u8(c + national_squad_record::RESERVE_LEVEL_CAPACITY);
    if read_u8(c + national_squad_record::RESERVE_GROUPS) == slots {
        exe::crt_qsort(slot(reserve_level_start(rec, o, level)), slots as u32, 4, 0x778ed0);
    }
}

/// Builds the selection pools: clears the 4 levels of each nation record and puts every eligible person in the nation
/// pool (and the under-21 one, up to 21 years and under 5 caps), also by the second nationality; full section: enters
/// by rating, and sorts (qsort with `compare_by_rating`).
///
/// `build_selection_pools()`: zeroes the counts (+0xd, +0x22) and puts 10000 in the lowest ratings (+0xf, +0x24) of the
/// 4 levels of each nation record (`[this+4]`, 0x6e bytes); then each person (`[0xae23bc]`, 0x6e bytes) with a nation
/// (+0x1a), a player sheet (+0x61), over 16 years old (+0x18), without a national side or already called up (+0x24,
/// +0x28 = 0xb), in a player position (`exe::database_is_player_job` of +0x3d) and with a club (+0x39) joins the nation's pool
/// (add_to_reserves) and, up to 21 years with fewer than 5 caps (+0x22), the under-21 one; and again by the second
/// nationality (+0x1e: with no caps for the first, a different one and not `[0x9cf434]`). No SEH, no box, no random
/// draw; the locals are written before being read (of the rating stored in 32 bits only the short counts).
/// Original: `build_selection_pools` (0x76abf0, national_teams.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn build_selection_pools(this: usize) {
    let mut i = 0i32;
    while i < read_i32(G_NATION_COUNT) {
        for k in 0..4usize {
            let r = read_usize(this + national_teams_fields::NATION_RECORDS) + 0x6e * i as usize + 4 * k;
            write_u8(r + national_squad_record::RESERVE_GROUPS, 0);
            write_i16(r + national_squad_record::RESERVE_LEVEL_MIN_REPUTATION, 10000);
            write_u8(r + national_squad_record::U21_RESERVE_LEVEL_COUNT, 0);
            write_i16(r + national_squad_record::U21_RESERVE_LEVEL_MIN_REPUTATION, 10000);
        }
        i += 1;
    }
    let mut p = read_usize(G_STAFF_TABLE);
    let mut n = 0i32;
    while n < read_i32(G_STAFF_COUNT) {
        let can = read_usize(p + staff::NATION) != 0
            && read_usize(p + staff::PLAYER) != 0
            && *((p + staff::AGE) as *const i8) > 0x10 // over 16 years old
            && (read_usize(p + staff::NATIONAL_JOB) == 0 || read_u8(p + staff::JOB_FOR_NATION) == 0xb)
            && exe::database_is_player_job((read_u8(p + staff::JOB_FOR_CLUB) as u32) as i8) != 0
            && read_usize(p + staff::CLUB_JOB) != 0;
        if can {
            for k in 0..2 {
                let id = if k == 0 {
                    read_i32(read_usize(p + staff::NATION))
                } else {
                    let n2 = read_usize(p + staff::SECOND_NATION);
                    if n2 == 0 || read_u8(p + staff::INT_APPS) != 0 || n2 == read_usize(p + staff::NATION) || read_i32(n2) == read_i32(KEY_NATION_PORTUGAL) {
                        continue;
                    }
                    covered!("reserves second nationality");
                    read_i32(n2)
                };
                let rec = read_usize(this + national_teams_fields::NATION_RECORDS).wrapping_add(id.wrapping_mul(0x6e) as usize);
                add_to_reserves(this, Staff(p), rec, 0);
                if read_i8(p + staff::AGE) > 0x15 || read_u8(p + staff::INT_APPS) >= 5 { // o sub-21
                    continue;
                }
                add_to_reserves(this, Staff(p), rec, 1);
            }
        }
        n += 1;
        p += 0x6e;
    }
}

/// the pool qsort order: rating (+9 of the player data) descending
/// Original: `compare_by_rating` (0x778ed0, national_teams.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_by_rating(a: usize, b: usize) -> i32 {
    (slot_person_rating(b) as i32) - slot_person_rating(a) as i32 // decrescente
}

/// the order by club (+0x39): no club last, clubs by id, within a club squad bit 4 before 8 (+0x5e)
/// Original: `compare_by_club` (0x778ef0, national_teams.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_by_club(a: usize, b: usize) -> i32 {
    let (x, y) = (read_usize(a), read_usize(b));
    let cx = if x != 0 { read_usize(x + staff::CLUB_JOB) } else { 0 };
    let cy = if y != 0 { read_usize(y + staff::CLUB_JOB) } else { 0 };
    if cx == 0 {
        return (cy != 0) as i32;
    }
    if cy == 0 {
        return -1;
    }
    if cx != cy {
        return read_i32(cx).wrapping_sub(read_i32(cy)); // the club id
    }
    let (mx, my) = (read_u8(x + staff::PLAYING_SQUAD), read_u8(y + staff::PLAYING_SQUAD));
    if mx & 4 != 0 && my & 8 != 0 {
        return -1;
    }
    if mx & 8 != 0 && my & 4 != 0 {
        return 1;
    }
    0
}

/// Takes the player out of his level pool in the nation and out of the squad, clearing his national team fields.
///
/// Ret 4: takes `player` out of the national side. With a nation (+0x1a) and a player sheet (+0x61): in the nation's
/// 110-byte record in `[this+4]`, the level (`national_team_level` with the record and 0); if the level's rating (word +0xf + level
/// × 4) is lower than the player sheet's (+9), looks for the player in the level's section of list `[this+0xc]` (start
/// = dword +8 + bytes +0xe of the previous levels, size in byte +0xd + level × 4): found, the last of the section goes
/// into his slot and the size decreases. Then, if called up (+0x24): his slot among the 50 (+0xd7) is zeroed, +0x24 is
/// zeroed and byte +0x5e keeps only bits 0..1.
/// Original: `NationalTeams::remove_player` (0x775ba0, national_teams.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn national_teams_remove_player(this: usize, player: usize) {
    let player = Staff(player);
    let u = |p: usize| read_usize(p);
    let nation = player.nation();
    if nation.is_null() || player.player().is_null() {
        return;
    }
    let level: unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_POSITION_GROUP);
    let reg = read_usize(this + national_teams_fields::NATION_RECORDS).wrapping_add((read_u32(nation.addr()) as usize).wrapping_mul(110));
    let n = level(player.addr(), reg as *const i16, 0) as i8 as isize;
    let field = |o: isize| (reg as isize).wrapping_add(n * 4 + o) as usize;
    if read_i16(field(0xf)) < read_i16(player.player().addr() + 9) {
        let mut start = u(reg + 8) as i32;
        for k in 0..n.max(0) as usize {
            start = start.wrapping_add(read_u8(reg + national_squad_record::RESERVE_LEVEL_CAPACITY + 4 * k) as i32);
        }
        let size = field(0xd) as *mut u8;
        let end = start.wrapping_add(*size as i32);
        let list = |i: i32| (read_usize(this + national_teams_fields::RESERVE_POOLS) as isize).wrapping_add(i as isize * 4) as *mut usize;
        let mut i = start;
        while i < end {
            if core::ptr::read_unaligned(list(i)) == player.addr() {
                *size = (*size).wrapping_sub(1);
                core::ptr::write_unaligned(list(i), core::ptr::read_unaligned(list(end - 1)));
                break;
            }
            i += 1;
        }
    }
    let national_team = player.national_job();
    if national_team.is_null() {
        return;
    }
    if let Some(k) = (0..50).find(|&k| u(national_team.addr() + club::SQUAD + 4 * k) == player.addr()) {
        national_team.set_squad(k, 0);
    }
    player.set_national_job(Club(0));
    player.set_playing_squad(player.playing_squad() & 3);
}

/// Addresses of the 50 call-up slots (+0xd7) of the nation's main team and U21/B team.
pub(crate) unsafe fn national_squad_slots(nation: usize) -> [usize; 2] {
    [exe::database_get_national_team(nation).wrapping_add(0xd7), exe::database_get_national_u21_team(nation).wrapping_add(0xd7)]
}

/// clears repeated squad entries from the senior and B national sides of a nation
/// Original: `national_teams_remove_duplicate_squad_entries` (0x76c060, national_teams.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn national_teams_remove_duplicate_squad_entries(nation: usize) {
    for vagas in national_squad_slots(nation) {
        let slot = |i: usize| (vagas + 4 * i) as *mut u32;
        for i in 0..50 {
            let v = core::ptr::read_unaligned(slot(i));
            if v == 0 {
                continue;
            }
            for j in i + 1..50 {
                if core::ptr::read_unaligned(slot(j)) == v {
                    core::ptr::write_unaligned(slot(j), 0);
                }
            }
        }
    }
}

/// Returns the player's level 1..3 for the national side by reputation and position ratings.
///
/// Stdcall, ret 0xc: the level 1..3 of player `j` for the national side (0 with reputation `*rep` below 100, or 50 with
/// `base`, or a goalkeeper rated 15): 1 for sweeper or central defender, 2 for defensive midfielder, midfielder or
/// attacking midfielder (rating 15 in +0x10..+0x14 of the player sheet), otherwise 3. Returns only `al`.
/// Original: `national_team_level` (0x76d810, national_teams.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn national_team_level(j: usize, rep: *const i16, base: u8) -> u8 {
    let j = Staff(j);
    if *rep < if base != 0 { 0x32 } else { 0x64 } {
        return 0;
    }
    let sheet = j.player();
    let n = |o: usize| read_i8(sheet.addr() + o) >= 0xf;
    if n(player_sheet::GOALKEEPER) {
        0
    } else if n(player_sheet::SWEEPER) || n(player_sheet::DEFENDER) {
        1
    } else if n(player_sheet::DEFENSIVE_MIDFIELDER) || n(player_sheet::MIDFIELDER) || n(player_sheet::ATTACKING_MIDFIELDER) {
        2
    } else {
        3
    }
}

/// the national teams day: the table of clubs with a match is freed; on 1 January, 15 April and 1 August the pools are
/// rebuilt; for the matches two weeks ahead, the clubs playing around the date are marked, the national teams call up
/// and the call-ups per club are recorded (outside the finals); with the table, the matches of clubs with called-up
/// players are rescheduled; the day's items leave, the FIFA ranking moves, every 70 days the records are pruned and the
/// qualified nations news goes out
/// Original: `NationalTeams::daily_update` (0x76b100, national_teams.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn national_teams_daily_update(this: usize, today: usize) {
    // 0x54c770: push 2; push saida; mov ecx, hoje -> thiscall ret 8; free: add esp, 4 -> cdecl; 0x549f70: 4 pushes;
    // lea ecx, output -> thiscall ret 0x10; 0x76abf0, 0x7774b0, 0x7779e0: mov ecx, edi without pushes -> thiscall;
    // 0x5abf10: 4 pushes; mov ecx, [0xae2a58] -> thiscall ret 0x10; 0x7777e0: push &data; mov ecx, edi -> thiscall
    // ret 4; 0x68d1f0: push item; mov ecx, [edi] -> thiscall ret 4; 0x58d460: push hoje; mov ecx, [edi + 0x1c] ->
    // thiscall ret 4
    let mut dd = [0u8; 8];
    let d = dd.as_mut_ptr() as usize;
    exe::date_add_weeks(today, (d) as *mut [u8; 8], 2);
    if read_usize(this + national_teams_fields::CLUB_MARKS) != 0 {
        exe::crt_free(read_usize(this + national_teams_fields::CLUB_MARKS));
        write_usize(this + national_teams_fields::CLUB_MARKS, 0);
    }
    // simplification: the original pushes the year (word) and the week with register leftovers on top; 0x549f70 reads the words
    let year = read_u16(today + 2) as u32;
    let mut t = [0u8; 8];
    let pt = t.as_mut_ptr() as usize;
    if is_today(exe::date_date_549f70((pt) as *mut [u16; 4], 1, 0, year, u32::MAX), today) // cqc12: 1º de janeiro
        || is_today(exe::date_date_549f70((pt) as *mut [u16; 4], 0xf, 3, year, u32::MAX), today)
        || is_today(exe::date_date_549f70((pt) as *mut [u16; 4], 1, 7, year, u32::MAX), today)
    {
        covered!("cqc_12 redoes the reserves");
        exe::nationalteams_build_selection_pools(this);
    }
    let fixman = read_usize(G_FIXTURE_MANAGER);
    for fatia in 0u32..3 {
        // the original’s dwords: the year (with the next 2 bytes of the date) and the day (with the year on top); simplification: the
        // slice goes without the stack leftover in the upper bytes (0x5abf10 does and 0xff)
        let l0 = exe::fixture_manager_get_fixture_list(fixman, read_usize(d + 2) as u32, read_usize(d) as u32, fatia, 0);
        let l1 = exe::fixture_manager_get_fixture_list(fixman, read_usize(d + 2) as u32, read_usize(d) as u32, fatia, 1);
        if ((l0 != 0 && read_i16(l0) != 0) || (l1 != 0 && read_i16(l1) != 0)) && read_usize(this + national_teams_fields::CLUB_MARKS) == 0 {
            mark_clubs_three_days(this, d); // cqc12: the clubs with a match around the date
        }
        if l0 != 0 {
            select_squads_for_fixtures(this, Llist(l0));
        }
        if l1 != 0 {
            select_squads_for_fixtures(this, Llist(l1));
        }
    }
    if read_usize(this + national_teams_fields::CLUB_MARKS) != 0 {
        exe::nationalteams_reschedule_clashing_club_fixtures(this, d);
    }
    if read_usize(this) != 0 {
        let day = read_i16(today);
        let l = Llist(read_usize(this));
        l.set_cursor(l.head());
        let mut p = exe::llist_get_next(read_usize(this));
        while p != 0 {
            if read_i16(p + call_up_entry::DUTY_END_DAY) == day {
                exe::llist_remove(read_usize(this), p);
            }
            p = exe::llist_get_next(read_usize(this));
        }
    }
    if read_usize(this + national_teams_fields::FIFA_RANKING) != 0 {
        exe::fifa_rankings_update_rankings(read_usize(this + national_teams_fields::FIFA_RANKING), today);
    }
    if read_i16(today) as i32 % 0x46 == 0 {
        exe::nationalteams_prune_club_call_up_records(this);
    }
    exe::nationalteams_post_qualified_nations_news(this);
}

/// Returns the age limits (23/23 in youth competitions, 24/22, 26/22).
///
/// Stdcall, ret 0x10: the age limits (`*minimum`, `*maximum`) of a national side in competition `comp`: 23/23 in the
/// Olympic and youth competitions (`[comp+0x14]` with an id among the 9 constants 0x9cf78c…0x9cf964); 24/22 with `type`
/// 1 or above 2; otherwise 26/22. Null pointers: the original routine (error box).
/// Original: `national_team_ages` (0x76d430, national_teams.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn national_team_ages(comp: usize, kind: i8, minimum: *mut u8, maxima: *mut u8) {
    let comp = Fixture(comp);
    if minimum.is_null() || maxima.is_null() {
        return original_fn::<unsafe extern "stdcall" fn(usize, i8, *mut u8, *mut u8)>(IDX_NATIONAL_TEAM_AGES)(comp.addr(), kind, minimum, maxima);
    }
    if !comp.is_null() {
        let c = comp.competition();
        if !c.is_null() {
            let id = read_i32(c.addr());
            if [KEY_CLUB_COMP_FIFA_WORLD_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP, KEY_CLUB_COMP_COPA_AMERICA, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS, KEY_CLUB_COMP_ASIAN_CUP, KEY_CLUB_COMP_OCEANIA_NATIONS_CUP, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_ASIAN_GAMES, KEY_CLUB_COMP_CONFEDERATIONS_CUP]
                .iter()
                .any(|&a| read_i32(a) == id)
            {
                *minimum = 0x17;
                *maxima = 0x17;
                return;
            }
        }
    }
    if kind != 0 && kind != 2 {
        *minimum = 0x18;
        *maxima = 0x16;
    } else {
        *minimum = 0x1a;
        *maxima = 0x16;
    }
}

/// Sorts the national squad by club and adds, on the match date, how many players each club gives up (the record that
/// stops club fixtures near that date).
///
/// Ret 0xc; SEH frame only for the state, the dates have no destructor: `NationalTeams::record_club_call_ups(team, b,
/// fixture)`: sorts the team's squad (+0xd7, 50 slots) by club (qsort with `compare_by_club`). With the fixture: its date
/// (+0x2a, +0x28; a day earlier for the second team `exe::club_is_national_team` or with `b`); counts the consecutive called-up players of
/// each club (+0x39), skipping the empty slots, the persons out of range (`[0xae2364]`) and those of another squad
/// (+0x5e bit 3 without `b`, bit 2 with it), and when the club changes adds the previous club's count to its record
/// (add_club_callups). The last club of the loop gets no record (as in the original).
/// Original: `NationalTeams::record_club_call_ups` (0x7771c0, national_teams.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn record_club_call_ups(this: usize, team: usize, b: u32, p: usize) {
    let p = Fixture(p);
    let team = Club(team);
    let mut d = [0u8; 8];
    exe::date_date_54a110((d.as_mut_ptr()) as usize);
    exe::crt_qsort(team.addr() + club::SQUAD, 0x32, 4, 0x778ef0);
    if p.is_null() {
        return;
    }
    let (mut t1, mut t2) = ([0u8; 8], [0u8; 8]);
    // simplification: the original pushes the shorts +0x2a and +0x28 with the high part of the register; 0x549ef0 only reads the shorts
    let r = exe::new_date((t1.as_mut_ptr()) as *mut u8 as usize, p.day() as u32, p.year() as u32);
    exe::date_operator_3d((d.as_mut_ptr()) as usize, (t2.as_mut_ptr()) as usize, r);
    if exe::club_is_national_team(team) != 0 || b != 0 {
        let r = exe::date_sub_days((d.as_mut_ptr()) as *mut u8 as usize, (t2.as_mut_ptr()) as *mut u8 as usize, 1);
        exe::date_operator_3d((d.as_mut_ptr()) as usize, (t1.as_mut_ptr()) as usize, r);
    }
    let mut previous = 0usize;
    let mut count: u8 = 0;
    for i in 0..0x32usize {
        let s = Staff(team.squad(i));
        if s.is_null() || read_usize(s.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
            continue;
        }
        let marks = s.playing_squad();
        if (b == 0 && marks & 8 != 0) || (b != 0 && marks & 4 != 0) {
            continue;
        }
        let c = s.club_job();
        if c == Club(previous) && previous != 0 {
            count = count.wrapping_add(1);
            continue;
        }
        if previous != 0 && !add_club_callups(this, previous, count, d.as_mut_ptr()) {
            return;
        }
        previous = c.addr();
        count = 1;
    }
    if previous != 0 && club_callup_record(this, Club(previous)).is_some() {
        covered!("cpw_13 last club without record"); // cpw13: the last club is left without a record
    }
}

