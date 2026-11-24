// --- sxB_6: the board's opinion of the chairman's favoured and disliked players and of the squad size (manager_manager.cpp) ---

/// The chairman's players the board names (`build_board_squad_opinion_text`): the disliked players to release and the
/// favoured ones, valuable members or signing targets; each list holds the staff records, at most three.
struct SquadOpinion {
    /// Favoured players at another club, reputation 4501.. within 750 of the club's and aged under 35.
    targets: Vec<Staff>,
    /// Favoured players at the club, under 36 and with a reputation above the club's − 750.
    valuable: Vec<Staff>,
    /// Disliked players at the club, below the club's reputation, not sold on, listed or leaving on a Bosman.
    disliked: Vec<Staff>,
}

/// A player of the chairman's list (`staff::JOB_FOR_CLUB` 0xb with a player sheet) and his club by his contract
/// (`exe::contract_manager_get_contract`, its answer stored in `G_CONTRACT_LOOKUP_RESULT` as the original does);
/// `None` when he is not such a player, `Some(0)` without a contract or a club.
unsafe fn squad_opinion_player_club(p: Staff) -> Option<usize> {
    if p.is_null() || p.player().is_null() || p.job_for_club() != 0xb {
        return None;
    }
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract);
    if contract == 0 {
        return Some(0);
    }
    Some(club_at(read_i32(contract + contract::CLUB) as isize).addr())
}

/// The chairman's disliked and favoured players (`exe::staff_get_disliked_staff`, `exe::staff_get_favoured_staff`)
/// sorted into the three lists of `SquadOpinion`.
unsafe fn squad_opinion_players(club: Club) -> SquadOpinion {
    let mut o = SquadOpinion { targets: Vec::new(), valuable: Vec::new(), disliked: Vec::new() };
    let mut list = [0usize; 0x226];
    let mut n: i8 = 0;
    let rep = club.reputation() as i16 as i32;
    exe::staff_get_disliked_staff(club.chairman(), list.as_mut_ptr() as usize, &mut n as *mut i8 as usize);
    for k in 0..n.max(0) as usize {
        let p = Staff(list[k]);
        if squad_opinion_player_club(p) != Some(club.addr()) {
            continue;
        }
        if (p.player().current_reputation() as i16 as i32) < rep
            && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) == 0
            && exe::staff_is_transfer_listed_or_requested(p, club) == 0
            && exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club) != 0
            && read_u8(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club) + contract::LEAVING_ON_BOSMAN) == 0
        {
            o.disliked.push(p);
        }
    }
    exe::staff_get_favoured_staff(club.chairman(), list.as_mut_ptr() as usize, &mut n);
    for k in 0..n.max(0) as usize {
        let p = Staff(list[k]);
        let Some(at) = squad_opinion_player_club(p) else { continue };
        let pr = p.player().current_reputation() as i16 as i32;
        let age = p.age() as i8;
        if at != 0 && at == club.addr() && age < 0x24 && pr > rep - 0x2ee {
            o.valuable.push(p);
        } else if pr > 0x1194 && pr > rep - 0x2ee && pr < rep + 0x2ee && age < 0x23 {
            o.targets.push(p);
        }
    }
    o
}

/// The full names (`exe::database_format_staff_name`, 200 bytes, style 1) of up to three players.
unsafe fn squad_opinion_names(players: &[Staff]) -> [[u8; 0xc8]; 3] {
    let mut names = [[0u8; 0xc8]; 3];
    for (k, p) in players.iter().take(3).enumerate() {
        exe::database_format_staff_name(*p, names[k].as_mut_ptr() as usize, 0xc8, 1);
    }
    names
}

/// The sentences on the three lists (`[one, two, three]` names; another count leaves the sentence empty): the signing
/// targets ("would be a useful addition" 0xa02ee0, 0xa02f30, 0xa02fa0), the valuable members (one: "will achieve great
/// things" 0xa02d90 when he joined the club less than 30 days ago (`exe::date_days_since` of `staff::DATE_JOINED_CLUB`),
/// else "an invaluable member" 0xa02d44; 0xa02ddc, 0xa02e50), the disliked ("don't deserve ...": oversized squad and a
/// wealthy club "should be released immediately" 0xa02614 / 0xa02904 / 0xa02c40, oversized "concerns about a club of
/// our stature" 0xa02578 / 0xa02838 / 0xa02b60, few players "can understand your reluctance" 0xa024bc / 0xa02758 /
/// 0xa02a7c, else 0xa02474 / 0xa026e0 / 0xa029f0).
unsafe fn squad_opinion_sentence(out: usize, players: &[Staff], forms: [usize; 3]) {
    let names = squad_opinion_names(players);
    let [a, b, c] = [names[0].as_ptr(), names[1].as_ptr(), names[2].as_ptr()];
    match players.len() {
        1 => story_lang!(out, forms[0], a),
        2 => story_lang!(out, forms[1], a, b),
        3 => story_lang!(out, forms[2], a, b, c),
        _ => write_u8(out, 0),
    }
}

/// Writes the board's opinion of the chairman's favoured and disliked players and of the squad size into `out`, the
/// named players' staff ids into `ids` (when not null: the targets, the valuable members, the disliked) and returns how
/// many it named.
///
/// `MANAGER_MANAGER::build_board_squad_opinion_text(out, club, ids)`: nothing (an empty text, 0) without a club, a
/// chairman, a chairman's non-player sheet, or for a national team (`exe::database_is_national_team`). The squad size
/// (`exe::database_count_club_squad`): under 15 "few players"; "oversized" above 30 under reputation 2000 or above 45
/// under 3250; by the club's wealth band (`exe::finance_get_wealth_band` of `exe::financetable_get_club_finance`):
/// 0xfe "wealthy" (and oversized above 40 under 3500), 0xff "wealthy", any other band clears "oversized"; no finance,
/// not wealthy. The players (`squad_opinion_players`) and their sentences (`squad_opinion_sentence`), joined by the
/// number of sentences (1 to 3): wealthy, oversized and nobody to release "the club cannot sustain such a large squad"
/// (0xa02038, 0xa020fc, 0xa021dc; none 0xa01fe4; a wealthy club otherwise the plain forms); few players and nobody to release "requires a greater squad depth"
/// (0xa01c38, 0xa01cc8, 0xa01da0; none 0xa01be4); else the plain forms (0xa01960, 0xa019a8, 0xa01a34; none: an empty
/// text, 0). The three-sentence forms are overwritten by the two-sentence ones (the original falls through), so the
/// third sentence never shows.
/// Original: `MANAGER_MANAGER::build_board_squad_opinion_text` (0x693850, manager_manager.cpp, thiscall, ret 0xc).
/// Defect of the original: with three sentences the two-sentence form replaces the three-sentence one.
pub(crate) unsafe extern "thiscall" fn build_board_squad_opinion_text(_this: usize, out: usize, club: usize, ids: usize) -> u8 {
    let club = Club(club);
    let chairman = if club.is_null() { Staff(0) } else { club.chairman() };
    if club.is_null() || chairman.is_null() || chairman.non_player().is_null() || exe::database_is_national_team(club) != 0 {
        write_u8(out, 0);
        return 0;
    }
    covered!("sxB_6 squad opinion");
    let count = exe::database_count_club_squad(club, 0, 0) as i8;
    let rep = club.reputation() as i16;
    let few = count < 0xf;
    let mut oversized = !few && ((rep < 0x7d0 && count > 0x1e) || (rep < 0xcb2 && count > 0x2d));
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    let mut wealthy = false;
    if finance != 0 {
        match exe::finance_get_wealth_band(finance) {
            0xfe => {
                wealthy = true;
                if !oversized && rep < 0xdac && count > 0x28 {
                    oversized = true;
                }
            }
            0xff => wealthy = true,
            _ => oversized = false,
        }
    }
    let o = squad_opinion_players(club);
    if ids != 0 {
        for (k, p) in o.targets.iter().chain(o.valuable.iter()).chain(o.disliked.iter()).enumerate() {
            write_u32(ids + 4 * k, read_u32(p.addr())); // the staff id
        }
    }
    let mut targets = [0u8; 0x3e8];
    let mut valuable = [0u8; 0x3e8];
    let mut disliked = [0u8; 0x3e8];
    squad_opinion_sentence(targets.as_mut_ptr() as usize, &o.targets, [0xa02ee0, 0xa02f30, 0xa02fa0]);
    if o.valuable.len() == 1 && exe::date_days_since(G_CURRENT_DATE, read_u32(o.valuable[0].addr() + staff::DATE_JOINED_CLUB),
        read_u32(o.valuable[0].addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR)) < 0x1e {
        squad_opinion_sentence(valuable.as_mut_ptr() as usize, &o.valuable, [0xa02d90, 0xa02ddc, 0xa02e50]);
    } else {
        squad_opinion_sentence(valuable.as_mut_ptr() as usize, &o.valuable, [0xa02d44, 0xa02ddc, 0xa02e50]);
    }
    let release = if oversized {
        if wealthy { [0xa02614, 0xa02904, 0xa02c40] } else { [0xa02578, 0xa02838, 0xa02b60] }
    } else if few {
        [0xa024bc, 0xa02758, 0xa02a7c]
    } else {
        [0xa02474, 0xa026e0, 0xa029f0]
    };
    squad_opinion_sentence(disliked.as_mut_ptr() as usize, &o.disliked, release);
    let mut parts: Vec<usize> = Vec::new();
    for (list, text) in [(o.targets.len(), targets.as_ptr()), (o.valuable.len(), valuable.as_ptr()), (o.disliked.len(), disliked.as_ptr())] {
        if list != 0 {
            parts.push(text as usize);
        }
    }
    // [one, two, three sentences, none]
    let plain = [0xa01960, 0xa019a8, 0xa01a34, 0];
    let forms: [usize; 4] = if wealthy {
        if oversized && o.disliked.is_empty() {
            [0xa02038, 0xa020fc, 0xa021dc, 0xa01fe4] // "... the club cannot sustain such a large squad ..."
        } else {
            plain
        }
    } else if few && o.disliked.is_empty() {
        [0xa01c38, 0xa01cc8, 0xa01da0, 0xa01be4] // "... the club requires a greater squad depth ..."
    } else {
        plain
    };
    let named = (o.targets.len() + o.valuable.len() + o.disliked.len()) as u8;
    match parts.len() {
        1 => story_lang!(out, forms[0], parts[0]),
        2 | 3 => {
            if parts.len() == 3 {
                story_lang!(out, forms[2], parts[0], parts[1], parts[2]);
            }
            story_lang!(out, forms[1], parts[0], parts[1]); // the original falls through from the three-sentence form
        }
        _ if forms[3] != 0 => story_lang!(out, forms[3]),
        _ => {
            write_u8(out, 0);
            return 0;
        }
    }
    named
}

/// Comparer for `build_board_squad_opinion_text`: the original into `out` and `ids`, the copy into a local buffer and
/// a local id list, side by side over `G_CONTRACT_LOOKUP_RESULT`; the text, the returned count and the ids it names
/// must match.
pub(crate) unsafe extern "thiscall" fn compare_build_board_squad_opinion_text(this: usize, out: usize, club: usize, ids: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8>(%%I0%%);
    let mut text = [0u8; 0x1388];
    let mut local_ids = [0u32; 15];
    let tp = text.as_mut_ptr() as usize;
    let lp = if ids != 0 { local_ids.as_mut_ptr() as usize } else { 0 };
    let (r, n, same, _, _) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, out, club, ids),
        || build_board_squad_opinion_text(this, tp, club, lp));
    let cstr = |p: usize| core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().to_vec();
    let (to, tr) = (cstr(out), cstr(tp));
    let k = (r as i8).max(0) as usize;
    let (io, ir): (Vec<u32>, Vec<u32>) = if ids != 0 { ((0..k).map(|i| read_u32(ids + 4 * i)).collect(), local_ids[..k.min(15)].to_vec()) } else { (Vec::new(), Vec::new()) };
    SCOREBOARD.record(same && r == n && to == tr && io == ir, || format!("build_board_squad_opinion_text: original {} {:?} {:?} rust {} {:?} {:?}", r, String::from_utf8_lossy(&to), io, n, String::from_utf8_lossy(&tr), ir));
    r
}
