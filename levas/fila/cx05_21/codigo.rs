// ---------- cx05_21: the friendly tournament invitations and the replacement of a declined invitee (friendly.cpp) ----------

/// Queues a friendly tournament on the pending list and sends its invitations.
///
/// Without the pending list (`friendly::PENDING_TOURNAMENT_LIST`) a new one (`alloc_empty_list`; out of memory: the box
/// at line 0x944 and nothing more). The tournament pointer is taken off the list (`exe::llist_remove` with a pointer to
/// it) and added at its end (`exe::llist_add_tail`, 4 bytes). Then each invited club of places 1 to
/// `tournament_slot::TEAM_COUNT` − 1 (place 0 is the host) still pending (`tournament_slot::INVITE_PENDING` 1) and run
/// by a human manager (`club_has_human_manager`) gets the pending mark 2 and the invitation news (`exe::new_newsitem`
/// type 9; slots 0 the host's id, 1 the club's id, 2 the slot index (`tournament_slot::SLOT_INDEX`, signed), 3 the
/// pending mark, 5 the start day (signed), 6 the start year; `exe::news_send_to_club_manager` to the club).
/// Original: `Friendly::send_tournament_invitations` (0x5c5310, friendly.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn send_tournament_invitations(this: usize, tournament: usize) {
    let friendly = Friendly(this);
    if friendly.pending_tournament_list().is_null() {
        let list = alloc_empty_list();
        write_usize(this + friendly::PENDING_TOURNAMENT_LIST, list);
        if list == 0 {
            return show_error_at(0x9cd0a0, 0x944);
        }
    }
    let pointer = tournament;
    exe::llist_remove(friendly.pending_tournament_list(), (&pointer) as *const usize as usize);
    exe::llist_add_tail(friendly.pending_tournament_list(), (&pointer) as *const usize as usize, 4);
    let mut k = 1i8;
    while (k as i16) < read_i16(tournament + tournament_slot::TEAM_COUNT) {
        let place = k as usize;
        let club = read_usize(tournament + tournament_slot::INVITED_CLUBS + place * 4);
        if club != 0 && read_u8(tournament + tournament_slot::INVITE_PENDING + place) == 1 && club_has_human_manager(club) {
            covered!("cx05_21 tournament invitation sent");
            write_u8(tournament + tournament_slot::INVITE_PENDING + place, 2); // cx05_21: the invitation is out
            let mut news = [0u32; 0x38];
            let n = news.as_mut_ptr() as usize;
            exe::new_newsitem(n, 9, 0);
            exe::newsitem_set_slot(n, 0, read_u32(read_usize(tournament + tournament_slot::HOST)));
            exe::newsitem_set_slot(n, 1, read_u32(club));
            exe::newsitem_set_slot(n, 2, read_i8(tournament + tournament_slot::SLOT_INDEX) as i32 as u32);
            exe::newsitem_set_slot(n, 3, read_i8(tournament + tournament_slot::INVITE_PENDING + place) as i32 as u32);
            exe::newsitem_set_slot(n, 5, read_i16(tournament + tournament_slot::START_DATE) as i32 as u32);
            exe::newsitem_set_slot(n, 6, read_u16(tournament + tournament_slot::START_DATE + 2) as u32);
            exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
        }
        k = k.wrapping_add(1);
    }
}

/// Comparer for `send_tournament_invitations`: the list moves and the news go out (they cannot run twice); only the
/// original, then the plan: the tournament is on the pending list and no human-run invitee is left with mark 1.
pub(crate) unsafe extern "thiscall" fn compare_send_tournament_invitations(this: usize, tournament: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, tournament);
    let list = Friendly(this).pending_tournament_list();
    let queued = list_item_ids(list.addr()).contains(&(tournament as u32));
    let n = read_i16(tournament + tournament_slot::TEAM_COUNT).clamp(0, 5) as usize;
    let left = (1..n).any(|k| {
        let club = read_usize(tournament + tournament_slot::INVITED_CLUBS + k * 4);
        club != 0 && read_u8(tournament + tournament_slot::INVITE_PENDING + k) == 1 && club_has_human_manager(club)
    });
    SCOREBOARD.record(queued && !left, || format!("send_tournament_invitations({:#x}): queued {}, human invitee left pending {}", tournament, queued, left));
}

/// A club's reputation band for the friendly tournaments: the reputation (`club::REPUTATION`, signed) / 500.
fn reputation_band(reputation: i16) -> i32 {
    reputation as i32 / 500
}

/// Picks the replacement for place `place` of `tournament` (its invited clubs) after a refusal.
///
/// With a national team as host (`exe::database_is_national_team`): among the nations of the host nation's region
/// (`nation::CONTINENT` of the host's `club::NATION`) whose reputation (`nation::REPUTATION`) is not above the host's
/// band (its reputation / 500 + 1), each one's score is a random 0 to 4 (`exe::utils_random`(5)) plus its reputation
/// − 2 (bytes); the best score so far takes the place (its national team, `exe::database_get_national_team`), and the
/// search ends once the best score is above the host's band − 4. With a club as host: among the clubs of the host's
/// nation, each one's score is a random 0 to 4 plus its own band (its reputation / 500) − 2; a score not above the
/// host's band and above the best so far takes the place, the best becoming the club's band (not the score), with the
/// same early end. With a candidate, it goes into `tournament_slot::INVITED_CLUBS`[place]; otherwise nothing changes.
/// Original: `Friendly::pick_replacement_invitee` (0x5c6350, friendly.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn pick_replacement_invitee(_this: usize, tournament: usize, place: u32) {
    let host = Club(read_usize(tournament + tournament_slot::HOST));
    let host_band = (reputation_band(read_i16(host.addr() + club::REPUTATION)) + 1) as i8;
    let (mut best, mut best_band) = (0usize, 0i8);
    if exe::database_is_national_team(host) != 0 {
        let region = host.nation().continent();
        let mut i = 0i16;
        while (i as i32) < read_i32(G_NATION_COUNT) {
            let nation = Nation(read_usize(G_NATION_TABLE) + i as isize as usize * nation::SIZE);
            if nation.continent() == region && read_i16(nation.addr() + nation::REPUTATION) <= host_band as i16 {
                let score = (exe::utils_random(5) as u8).wrapping_add(read_u8(nation.addr() + nation::REPUTATION).wrapping_sub(2)) as i8; // cx05_21: the nation's draw
                if score > best_band {
                    best = exe::database_get_national_team(nation);
                    best_band = score;
                }
                if best_band as i32 > host_band as i32 - 4 {
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
    } else {
        let nation = host.nation().addr();
        let mut i = 0i16;
        while (i as i32) < read_i32(G_CLUB_COUNT) {
            let club = club_at(i as isize);
            if club.nation().addr() == nation {
                let band = reputation_band(read_i16(club.addr() + club::REPUTATION)) as u8;
                let score = (exe::utils_random(5) as u8).wrapping_add(band.wrapping_sub(2)) as i8;
                if score <= host_band {
                    if score > best_band {
                        best = club.addr();
                        best_band = band as i8; // cx05_21: the club's band, not its score
                    }
                    if best_band as i32 > host_band as i32 - 4 {
                        break;
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if best != 0 {
        covered!("cx05_21 replacement invitee");
        write_usize(tournament + tournament_slot::INVITED_CLUBS + (place as u8 as i8 as isize as usize) * 4, best);
    }
}

/// Comparer for `pick_replacement_invitee`: it draws at random, so only the original runs; then the plan: the place
/// holds a club or national team that is not the host.
pub(crate) unsafe extern "thiscall" fn compare_pick_replacement_invitee(this: usize, tournament: usize, place: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%)(this, tournament, place);
    let picked = read_usize(tournament + tournament_slot::INVITED_CLUBS + (place as u8 as i8 as isize as usize) * 4);
    SCOREBOARD.record(picked != read_usize(tournament + tournament_slot::HOST) || picked == 0, || format!("pick_replacement_invitee({:#x}, {}): the host invited itself", tournament, place as u8));
}
