// ---------- batch %%LEVA%%: the club screens (6c): the club screen's action button (club_screens.cpp) ----------

/// The action button of the club screen heading (`ScreenManager::add_button_item` at 0x294, 4, 0x311, 0x18): the
/// text (`FN_LANGLIB_SPRINTF` of `format` into `JOB_TEXT`), the action id and its data.
unsafe fn club_action_button(colour: u32, colour_text: u32, format: usize, id: u32, data: usize) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    lang(JOB_TEXT, format);
    tel26_add_button([0x294, 4, 0x311, 0x18, 0x30, colour_text, 0xc, 1, colour, JOB_TEXT as u32, id, data as u32]);
}

/// Rule: the club screen offers one action, the first that applies: "Take Control" while the human is creating his
/// manager, "Scout Club", inviting the club to the human's friendly, dates request or tournament (or "Uninvite Club"),
/// "Apply for Job".
///
/// `add_club_screen_action_button(club, colour, colour_text)`, the button made by `club_action_button`:
/// - while a manager is being created (`is_current_manager_being_created`, 0x838de0): nothing for a human club
///   (`exe::human_manager_is_human_club`(club, 1, 1, 0, 0)) or one that cannot be managed (`exe::club_can_be_managed`),
///   otherwise "Take Control" (0x64);
/// - "Scout Club" (0x6a) when the scouts' current search has no club (`SCOUT_MANAGER::current_search_has_no_club`,
///   0x7f8420), the club is not in the session's searches (0x7f8500) and is not a national team;
/// - for a club the human does not run, from the Friendly object ([G_FRIENDLY]): the open invitation (0x5c68b0) whose
///   club +0xc is a national team as much as this club: "Invite Club" (0x66, the invitation); the open dates request
///   (0x5c6920) of the club's nation (+4) whose club +0 matches likewise: "Invite Club" (0x67); the tournament
///   (0x5c6830) whose club +0 matches likewise: "Uninvite Club" (0x68) when the club is among its invited (+0x13
///   count, +0x15 the clubs; the index a signed byte as the original), otherwise "Invite Club" (0x69);
/// - "Apply for Job" (0x65) for a club without a manager (`exe::database_get_club_manager`) that can be managed.
/// Returns 1 when a button was added, otherwise 0.
/// Original: `add_club_screen_action_button` (0x482a40, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn add_club_screen_action_button(club: usize, colour: u32, colour_text: u32) -> u32 {
    let being_created: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x838de0usize);
    if being_created() != 0 {
        if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 || exe::club_can_be_managed(club) == 0 {
            return 0;
        }
        club_action_button(colour, colour_text, 0x991594, 0x64, 0); // "Take Control"
        return 1;
    }
    let scouts = read_usize(G_SCOUT_MANAGER);
    let no_club: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7f8420usize);
    let searched: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7f8500usize);
    if no_club(scouts) != 0 && searched(scouts, club) == 0 && exe::database_is_national_team(club) == 0 {
        club_action_button(colour, colour_text, 0x991588, 0x6a, 0); // "Scout Club"
        return 1;
    }
    if exe::human_manager_current_controls_club(club, 0, 1) == 0 {
        let find = |f: usize| {
            let g: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(f);
            g(read_usize(G_FRIENDLY))
        };
        let national = |c: usize| exe::database_is_national_team(c) != 0;
        const INVITE: usize = 0x99157c; // "Invite Club"
        let m = find(0x5c68b0);
        if m != 0 && national(read_usize(m + 0xc)) == national(club) {
            club_action_button(colour, colour_text, INVITE, 0x66, m);
            return 1;
        }
        let r = find(0x5c6920);
        if r != 0 && read_u32(club + club::NATION) == read_u32(r + 4) && national(club) == national(read_usize(r)) {
            club_action_button(colour, colour_text, INVITE, 0x67, r);
            return 1;
        }
        let t = find(0x5c6830);
        if t != 0 && national(club) == national(read_usize(t)) {
            covered!("tel06_3 tournament button");
            let n = read_i16(t + 0x13);
            let mut invited = false;
            let mut k = 0i8;
            while n > 0 && !invited {
                if read_usize((t as isize + 0x15 + 4 * k as isize) as usize) == club {
                    invited = true;
                }
                k = k.wrapping_add(1);
                if k as i16 >= n {
                    break;
                }
            }
            if invited {
                club_action_button(colour, colour_text, 0x99156c, 0x68, t); // "Uninvite Club"
            } else {
                club_action_button(colour, colour_text, INVITE, 0x69, t);
            }
            return 1;
        }
    }
    if exe::database_get_club_manager(club) == 0 && exe::club_can_be_managed(club) != 0 {
        club_action_button(colour, colour_text, 0x99155c, 0x65, 0); // "Apply for Job"
        return 1;
    }
    0
}

/// Comparer for `add_club_screen_action_button`: side by side over the display list (`compare_match_screen_build`)
/// and the return.
pub(crate) unsafe extern "C" fn compare_add_club_screen_action_button(club: usize, colour: u32, colour_text: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32) -> u32>(%%I0%%);
    let (mut a, mut b) = (0u32, 0u32);
    compare_match_screen_build("club screen action button", &[], || a = o(club, colour, colour_text), || b = add_club_screen_action_button(club, colour, colour_text));
    SCOREBOARD.record(a == b, || format!("add_club_screen_action_button (club screen) returned {} and Rust {}", a, b));
    a
}
