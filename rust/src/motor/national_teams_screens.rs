use super::*;

/// Rule: a TcpIp object is destroyed by closing all its sockets and freeing its connection table.
///
/// `tcpip_close_and_free`. Only reached from the unwind handlers of `Network::Network` (an exception while the TcpIp
/// base is built).
/// Original: `TcpIp::~TcpIp` (0x77a870, national_teams_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tcpip_destructor(this: usize) {
    covered!("tel40_2 TcpIp destroyed");
    tcpip_close_and_free(this);
}

/// Rule: a Network object is destroyed; when it hosts a game it first says goodbye to the list servers ("CM3 LIST SERVER
/// BYE" with its address), then all its sockets are closed and its connection table freed.
///
/// Hosting (`network_game::HOSTING`): the datagram "CM3 LIST SERVER BYE" (`NETWORK_LIST_BYE`, without its terminator)
/// followed by the 50 bytes of server address 0 (`network_game::SERVER_ADDRESSES`), sent by `TcpIp::send_datagram`
/// with the length of the text + 100, the game's port and no address (broadcast). The original's buffer is a stack
/// local, so the last 50 bytes sent are whatever its stack held; the copy sends zeros there. Then
/// `TcpIp::close_all_sockets` and the TcpIp base's destructor (`tcpip_close_and_free`: closing them again and freeing
/// the table).
/// Original: `Network::~Network` (0x77a890, national_teams_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn network_destructor(this: usize) {
    covered!("tel40_2 Network destroyed");
    if read_u32(this + network_game::HOSTING) != 0 {
        let mut buffer = [0u8; 0x7d0];
        let p = buffer.as_mut_ptr() as usize;
        let n = network_strlen(NETWORK_LIST_BYE);
        core::ptr::copy(NETWORK_LIST_BYE as *const u8, p as *mut u8, n);
        core::ptr::copy((this + network_game::SERVER_ADDRESSES) as *const u8, (p + n) as *mut u8, network_game::SERVER_TEXT);
        let send: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> i32 = tcpip_fn(FN_TCPIP_SEND_DATAGRAM);
        send(this, p, (n + 0x64) as u32, NETWORK_PORT, 0, 0, 0); // tel40_2: the goodbye datagram
    }
    let close: unsafe extern "thiscall" fn(usize) = tcpip_fn(FN_TCPIP_CLOSE_ALL_SOCKETS);
    close(this);
    tcpip_close_and_free(this);
}

/// Rule: the connection of one of the sixteen players (numbered 1 to 16) is closed; any other number does nothing.
///
/// For `player` 1..16 (a short): `TcpIp::close_connection` with the connection slot of that player
/// (`NETWORK_PLAYER_SLOTS` + 2 × player, a word); the slot is not marked free here.
/// Original: `Network::release_slot_socket` (0x77ac00, national_teams_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_release_slot_socket(this: usize, player: u32) {
    let k = player as i16;
    if k <= 0 || k > 0x10 {
        return;
    }
    covered!("tel40_2 close a player's connection");
    let close: unsafe extern "thiscall" fn(usize, u32) = tcpip_fn(FN_TCPIP_CLOSE_CONNECTION);
    close(this, read_u16(this + NETWORK_PLAYER_SLOTS + 2 * k as usize) as u32);
}

/// Rule: a packet goes to one player's connection, or to every connection for player 0 when hosting, preceded by its
/// 4-byte length; a connection found closed is freed (and the leave callback told, when hosting). Returns whether the
/// (last) send worked.
///
/// A length of 0 or less: the box `network.CPP 211` and 0. A length of 50000 or more: 0. Hosting
/// (`network_game::HOSTING`): player 0 (the low word) walks the 16 connection slots (`network_game::SOCKETS`); each one
/// in use (not 0xffff) gets `network_send_on`; a closed one is marked free (0xffff) and the leave callback
/// (`network_game::ON_DISCONNECT`, when set) gets the slot + 1. Returns the result of the last slot in use (0 with
/// none). Another player: his slot (`NETWORK_PLAYER_SLOTS` + 2 × player); free, 0; otherwise `network_send_on`, closed
/// -> freed and the callback gets `player`. A client: slot 0 the same way, freed without the callback.
/// Original: `Network::send_packet` (0x77ac30, national_teams_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn network_send_packet(this: usize, data: usize, length: i32, player: u32) -> u32 {
    use network_game as g;
    if length <= 0 {
        tel26_error_box(NETWORK_CPP, 0xd3);
        return 0;
    }
    if length >= 0xc350 {
        return 0;
    }
    covered!("tel40_2 send a packet");
    let callback = |arg: u32| {
        let f = read_usize(this + g::ON_DISCONNECT);
        if f != 0 {
            let f: unsafe extern "C" fn(u32) = core::mem::transmute(f);
            f(arg);
        }
    };
    if read_u32(this + g::HOSTING) == 0 {
        let slot = read_u16(this + g::SOCKETS);
        if slot == 0xffff {
            return 0;
        }
        let ok = network_send_on(this, slot, data, length as u32);
        if !ok {
            write_u16(this + g::SOCKETS, 0xffff);
        }
        return ok as u32;
    }
    if player as u16 != 0 {
        let at = (this as isize + NETWORK_PLAYER_SLOTS as isize + player as i16 as isize * 2) as usize;
        let slot = read_u16(at);
        if slot == 0xffff {
            return 0;
        }
        let ok = network_send_on(this, slot, data, length as u32);
        if !ok {
            write_u16(at, 0xffff);
            callback(player);
        }
        return ok as u32;
    }
    covered!("tel40_2 send a packet to everyone");
    let mut ok = false;
    for k in 0..0x10usize {
        let slot = read_u16(this + g::SOCKETS + 2 * k);
        if slot == 0xffff {
            continue;
        }
        ok = network_send_on(this, slot, data, length as u32);
        if !ok {
            write_u16(this + g::SOCKETS + 2 * k, 0xffff); // tel40_2: the closed connection freed
            callback(k as u32 + 1);
        }
    }
    ok as u32
}

/// Rule: a network buffer starts empty with the given capacity and its own storage; with no memory the game stops.
///
/// Position 0, capacity = `capacity`, the storage `operator new(capacity)`; null: the box `network.CPP 562`,
/// `game_fatal_cleanup` (0x5cc7d0), the empty routine 0x5a8f60 and `exe::crt_exit(-1)`. Returns the buffer.
/// Original: `NetworkBuffer::NetworkBuffer` (0x77b630, national_teams_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_buffer_construct(this: usize, capacity: u32) -> usize {
    write_u32(this + network_buffer::POS, 0);
    write_u32(this + network_buffer::CAPACITY, capacity);
    let data = exe::crt_operator_new(capacity);
    write_usize(this + network_buffer::DATA, data);
    if data == 0 {
        tel26_error_box(NETWORK_CPP, 0x232);
        exe::game_fatal_cleanup();
        exe::shared_noop_ret0_5a8f60();
        exe::crt_exit(-1);
    }
    this
}

/// Rule: a network buffer is destroyed by freeing its storage, when it has one, and clearing the pointer.
/// Original: `NetworkBuffer::~NetworkBuffer` (0x77b6e0, national_teams_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn network_buffer_destruct(this: usize) {
    let data = read_usize(this + network_buffer::DATA);
    if data != 0 {
        exe::crt_operator_delete(data);
    }
    write_usize(this + network_buffer::DATA, 0); // tel40_2: the pointer cleared
}

/// Comparer for `tcpip_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_tcpip_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TCPIP_DESTRUCTOR)(this)
}

/// Comparer for `network_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_network_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NETWORK_DESTRUCTOR)(this)
}

/// Comparer for `network_release_slot_socket`.
pub(crate) unsafe extern "thiscall" fn compare_network_release_slot_socket(this: usize, player: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_NETWORK_RELEASE_SLOT_SOCKET)(this, player)
}

/// Comparer for `network_send_packet`.
pub(crate) unsafe extern "thiscall" fn compare_network_send_packet(this: usize, data: usize, length: i32, player: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32) -> u32>(IDX_NETWORK_SEND_PACKET)(this, data, length, player)
}

/// Comparer for `network_buffer_construct`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_construct(this: usize, capacity: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_NETWORK_BUFFER_CONSTRUCT)(this, capacity);
    SCOREBOARD.record(r == this && read_u32(this + network_buffer::POS) == 0 && read_u32(this + network_buffer::CAPACITY) == capacity
        && read_usize(this + network_buffer::DATA) != 0, || format!("NetworkBuffer::NetworkBuffer({}): returned {:#x}, buffer {:x?}", capacity, r,
        core::slice::from_raw_parts(this as *const u8, 12)));
    r
}

/// Comparer for `network_buffer_destruct`.
pub(crate) unsafe extern "thiscall" fn compare_network_buffer_destruct(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NETWORK_BUFFER_DESTRUCT)(this);
    SCOREBOARD.record(read_usize(this + network_buffer::DATA) == 0, || format!("NetworkBuffer::~NetworkBuffer: storage {:#x} left", read_usize(this + network_buffer::DATA)));
}

/// Rule: opening the dialog that moves a national team player into, out of or between squads.
///
/// `national_teams_screens_open_squad_move_screen(player, kind, joining)`: `open_screen` (builder
/// `national_squad_move_screen_build`, input `national_squad_move_screen_handle_input`, flag 1, no leave, no exit);
/// if it opens, parameter 0 = the player, 1 = the squad kind (its byte, sign-extended), 2 = joining (1) or leaving (0)
/// and 3 = −1 (the squad he is in, set by the build).
/// Original: `national_teams_screens_open_squad_move_screen` (0x778f60, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_teams_screens_open_squad_move_screen(player: usize, kind: u32, joining: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x778fd0usize, 0x779610usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel28_2 open the squad move dialog");
    for (k, v) in national_squad_move_plan(player, kind, joining).iter().enumerate() {
        set_screen_param(k as u32, *v);
    }
}

/// Comparer for `national_teams_screens_open_squad_move_screen`: only the original runs (it opens a screen); the
/// parameters checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_national_teams_screens_open_squad_move_screen(player: usize, kind: u32, joining: u32) {
    let plan = national_squad_move_plan(player, kind, joining);
    original_fn::<unsafe extern "C" fn(usize, u32, u32)>(IDX_NATIONAL_TEAMS_SCREENS_OPEN_SQUAD_MOVE_SCREEN)(player, kind, joining);
    if player != 0 && get_screen_param(0) == player {
        let got = [get_screen_param(0), get_screen_param(1), get_screen_param(2), get_screen_param(3)];
        SCOREBOARD.record(got == plan, || format!("national squad move dialog (open): params {:x?} x plan {:x?}", got, plan));
    }
}

/// Rule: the squad move dialog asks to confirm adding the player to a squad, removing him from one or moving him
/// between two, or says why he cannot be moved.
///
/// The player is parameter 0, the target squad kind parameter 1's byte, joining parameter 2. The current slot's human
/// and his national team (`staff::NATIONAL_JOB`) are needed (none: the national_teams_screens.cpp box of line 0x67 or
/// 0x6f and nothing else). The title: the team's national name (`club_get_national_team_name` with the under-21 form
/// for kinds other than 0 and 2) in its home colours. The squad the player is in (parameter 3): none (−1) without a
/// national job; 2 when that job is a national team (`club_is_national_team`); else 1 unless he is in the national
/// squad (`playing_squad::NATIONAL_SQUAD`), then 0. In a squad, he must be able to leave it
/// (`national_teams_can_change_squad`, leaving); joining, he must be able to join the target. Otherwise the reason
/// (written by those checks), "Could Not Add Player To Squad" or "Could Not Remove Player From Squad" as the subtitle
/// and "Ok". Otherwise, in a squad and joining: "Move Player To <target> Squad" and "Move <player> from <squad> to
/// <target> ?"; in no squad and joining: "Add Player To <target> Squad" and "Add <player> to <target> squad ?";
/// leaving: "Remove Player From <target> Squad" and "Remove <player> from <target> squad ?" (the squad names by
/// `format_squad_label`, the player's by `database_format_staff_name`); the question in a text area and No/Yes.
/// Original: `national_squad_move_screen_build` (0x778fd0, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_move_screen_build() {
    let player = Staff(get_screen_param(0));
    let target = get_screen_param(1) as u8 as u32;
    let joining = get_screen_param(2);
    let (_, club) = national_squad_human_team(0x67, 0x6f);
    if club == 0 {
        return;
    }
    covered!("tel28_2 build the squad move dialog");
    let under = (target as u8 != 0 && target as u8 != 2) as usize;
    national_squad_title(club, JOB_TEXT, || {
        exe::club_get_national_team_name(club, under, JOB_TEXT, 0x7d0, 3, 1);
    });
    let mut current: u8 = 0xff;
    let job = player.national_job().addr();
    if job != 0 {
        current = if exe::club_is_national_team(job) != 0 {
            2
        } else {
            ((!player.playing_squad()) >> 2) & 1
        };
    }
    set_screen_param(3, current as i8 as isize as usize);
    let nation = read_usize(club + club::NATION);
    let mut cannot = false;
    if current != 0xff && exe::national_teams_can_change_squad(read_usize(G_NATIONAL_TEAMS), nation, current as u32, player, 0, JOB_TEXT as u32, 0x7d0) == 0 {
        cannot = true;
    }
    if (joining != 0 && exe::national_teams_can_change_squad(read_usize(G_NATIONAL_TEAMS), nation, target, player, 1, JOB_TEXT as u32, 0x7d0) == 0) || cannot {
        covered!("tel28_2 squad move refused");
        national_squad_message();
        tel28_lang()(JOB_TEXT, if joining != 0 { 0xa645c4 } else { 0xa645a0 }); // "Could Not Add Player To Squad" / "Could Not Remove Player From Squad"
        exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
        national_squad_ok();
        return;
    }
    let (mut to, mut name, mut from) = ([0u8; 0x64], [0u8; 0x12c], [0u8; 0x64]);
    let g = read_u8(club + club::SHORT_GENDER_NAME) as i8 as i32;
    let staff_name = |name: &mut [u8; 0x12c]| {
        exe::database_format_staff_name(player, name.as_mut_ptr() as usize, 0x12c, 1);
    };
    if current != 0xff && joining != 0 {
        national_squad_label(nation, target, to.as_mut_ptr() as usize, 0);
        tel28_lang()(JOB_TEXT, 0xa64564, g, g, to.as_ptr()); // "Move Player To {}<%s>{} Squad"
        exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
        staff_name(&mut name);
        national_squad_label(nation, current as u32, from.as_mut_ptr() as usize, 1);
        national_squad_label(nation, target, to.as_mut_ptr() as usize, 1);
        tel28_lang()(JOB_TEXT, 0xa644e8, name.as_ptr(), from.as_ptr(), to.as_ptr());
    } else if joining != 0 {
        national_squad_label(nation, target, to.as_mut_ptr() as usize, 0);
        tel28_lang()(JOB_TEXT, 0xa644b4, to.as_ptr()); // "Add Player To <%s> Squad"
        exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
        staff_name(&mut name);
        national_squad_label(nation, target, to.as_mut_ptr() as usize, 1);
        tel28_lang()(JOB_TEXT, 0xa64460, name.as_ptr(), to.as_ptr()); // "Add <%s> to <%s> squad ?"
    } else {
        national_squad_label(nation, target, to.as_mut_ptr() as usize, 0);
        tel28_lang()(JOB_TEXT, 0xa64424, to.as_ptr()); // "Remove Player From <%s> Squad"
        exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
        staff_name(&mut name);
        national_squad_label(nation, target, to.as_mut_ptr() as usize, 1);
        tel28_lang()(JOB_TEXT, 0xa643cc, name.as_ptr(), to.as_ptr()); // "Remove <%s> from <%s> squad ?"
    }
    national_squad_message();
    national_squad_yes_no();
}

/// Comparer for `national_squad_move_screen_build`: side by side over the display list; with no human team (the
/// error box) only the original.
pub(crate) unsafe extern "C" fn compare_national_squad_move_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_NATIONAL_SQUAD_MOVE_SCREEN_BUILD);
    if national_squad_has_box() {
        return o();
    }
    compare_match_screen_build("national squad move dialog", &[], || o(), || national_squad_move_screen_build());
}

/// Rule: answering Yes in the squad move dialog takes the player out of the squad he is in and puts him in the target
/// squad, calling up a replacement when needed.
///
/// `national_squad_move_screen_handle_input(widget)`: Yes (id 2): the human team is needed (none: the box of line
/// 0x136 or 0x13e and 0). The nation's 110-byte record (the national teams' vector +4). With a national job the
/// player leaves the squad of parameter 3 (`nationalteams_remove_player_from_squad`); when that squad is not the main
/// one (0), its call-up is active (0x76e010) and the human does not control all national teams
/// (`human_manager_controls_all_national_teams`), a replacement is due: `national_team_level` of the player (with the
/// record, 1 for squad 2) replaces the record dword's low byte. Joining: `nationalteams_add_player_to_squad` into the
/// target (parameter 1). The replacement: `nationalteams_call_up_replacement` (the squad, the dword, the player).
/// Returns −9. Other widgets: `national_squad_other_click`.
/// Original: `national_squad_move_screen_handle_input` (0x779610, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_move_screen_handle_input(widget: u32) -> i32 {
    if national_squad_widget_id(widget) != 2 {
        return national_squad_other_click(widget);
    }
    let player = Staff(get_screen_param(0));
    let target = get_screen_param(1) as u8 as u32;
    let joining = get_screen_param(2);
    let (manager, club) = national_squad_human_team(0x136, 0x13e);
    if club == 0 {
        return 0;
    }
    covered!("tel28_2 squad move confirmed");
    let nt = read_usize(G_NATIONAL_TEAMS);
    let nation = read_usize(club + club::NATION);
    let mut record = read_usize(nt + 4) + read_u32(nation) as usize * NATIONAL_SQUAD_RECORD_SIZE;
    let mut replace = false;
    let mut squad = 0u32;
    if player.national_job().addr() != 0 {
        squad = get_screen_param(3) as u8 as u32;
        exe::nationalteams_remove_player_from_squad(nt, nation, squad, player);
        let active: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_NATIONALTEAMS_IS_SQUAD_SELECTED);
        if active(read_usize(G_NATIONAL_TEAMS), nation, squad) != 0
            && squad != 0
            && exe::human_manager_controls_all_national_teams(read_usize(G_HUMAN_MANAGER), manager, core::ptr::null_mut()) == 0
        {
            covered!("tel28_2 squad move replacement");
            replace = true;
            let level = exe::national_teams_get_position_group(player, record, (squad == 2) as u32);
            record = (record & !0xff) | level as usize;
        }
    }
    if joining != 0 {
        exe::nationalteams_add_player_to_squad(read_usize(G_NATIONAL_TEAMS), nation, target, player);
    }
    if replace {
        exe::nationalteams_call_up_replacement(read_usize(G_NATIONAL_TEAMS), nation, squad, record as u32, player);
    }
    NATIONAL_SQUAD_DONE
}

/// Comparer for `national_squad_move_screen_handle_input`: No side by side (nothing changes); Yes (it moves the
/// player), the menu bar and the object links only the original.
pub(crate) unsafe extern "C" fn compare_national_squad_move_screen_handle_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_NATIONAL_SQUAD_MOVE_SCREEN_HANDLE_INPUT);
    let pure = national_squad_widget_id(widget) == 1;
    tel28_click_compare("national squad move dialog input", widget, pure, || o(widget), || national_squad_move_screen_handle_input(widget))
}

/// Rule: answering a squad selection news item opens the dialog that confirms the selected squad.
///
/// `national_squad_selection_open_from_news(manager, id)`: `open_screen` (builder
/// `national_squad_selection_screen_build`, input `national_squad_selection_screen_handle_input`, flag 1); if it
/// opens, the news item (`News::find_item_by_id`) gives the team (slot 0, an index into the club table) and the squad
/// kind (slot 1); the players of the team's squad (50 slots) not yet flagged for that squad (the national squad flag
/// for a kind other than 0, the B squad flag for 0) are counted. Parameters 0 the team, 1 the kind, 2 the manager, 3
/// the id, 4 the count (its byte), 5 to 7 slots 5 to 7.
/// Original: `national_squad_selection_open_from_news` (0x7798c0, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_selection_open_from_news(manager: usize, id: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x779a50usize, 0x779df0usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel28_2 open the squad selection dialog");
    for (k, v) in national_squad_selection_plan(manager, id).iter().enumerate() {
        set_screen_param(k as u32, *v);
    }
}

/// Comparer for `national_squad_selection_open_from_news`: only the original runs (it opens a screen); the
/// parameters checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_national_squad_selection_open_from_news(manager: usize, id: u32) {
    let plan = national_squad_selection_plan(manager, id);
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_NATIONAL_SQUAD_SELECTION_OPEN_FROM_NEWS)(manager, id);
    if get_screen_param(2) == manager && get_screen_param(3) == id as usize {
        let mut got = [0usize; 8];
        for (k, v) in got.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
        SCOREBOARD.record(got == plan, || format!("national squad selection dialog (open {}): params {:x?} x plan {:x?}", id, got, plan));
    }
}

/// Rule: the squad selection dialog confirms the selected national squad when its size is within the limits.
///
/// See `national_squad_confirm_build` (the selection form: the title in [`JOB_TEXT`], the question with the name in
/// form 2).
/// Original: `national_squad_selection_screen_build` (0x779a50, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_selection_screen_build() {
    national_squad_confirm_build(false);
}

/// Comparer for `national_squad_selection_screen_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_national_squad_selection_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_NATIONAL_SQUAD_SELECTION_SCREEN_BUILD);
    compare_match_screen_build("national squad selection dialog", &[], || o(), || national_squad_selection_screen_build());
}

/// Rule: confirming a squad selection marks the selection as made, posts the call-up news and, when no match is
/// pending in the news, moves the team's club fixtures that clash with the squad date.
///
/// `national_squad_selection_screen_handle_input(widget)`: Yes (id 2): the news item of parameters 2 and 3 gives
/// slots 3 (a pending fixture [inference]), 4 (how many clubs, 1 to 3: slots 8 on are their indexes, kept in a local
/// list) and 5. The squad slot (`national_teams_get_squad_slot` of the team and parameter 1) over the kind's low byte
/// marks the nation (0x76c0f0); the item's slot 2 gets 1 (`News::set_item_slot`); `national_teams_post_call_up_news`
/// (the team, the kind, 0, slot 5, slot 3, the club list, slot 4). With slot 3 at 0: the squad date (parameters 6 and
/// 7) and the day before (`Date::sub_days`), the team's fixtures between them (`fixture_manager_find_fixtures`, every
/// competition, −2, −1); the first one records the clubs' call-ups (`nationalteams_record_club_call_ups`) and
/// reschedules the clashing club fixtures (`nationalteams_reschedule_clashing_club_fixtures` on the squad date).
/// Returns −9. Other widgets: `national_squad_other_click`.
/// Original: `national_squad_selection_screen_handle_input` (0x779df0, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_selection_screen_handle_input(widget: u32) -> i32 {
    if national_squad_widget_id(widget) != 2 {
        return national_squad_other_click(widget);
    }
    covered!("tel28_2 squad selection confirmed");
    let mut clubs = [0u8; 0xe];
    exe::new_llist(clubs.as_mut_ptr() as usize);
    let mut item = [0u8; 0xe0];
    exe::new_newsitem(item.as_mut_ptr() as usize, 0, 0);
    let club = Club(get_screen_param(0));
    let kind = get_screen_param(1) as u32;
    let (manager, id) = (get_screen_param(2), get_screen_param(3) as u32);
    exe::news_find_item_by_id(read_usize(G_NEWS), manager, id, item.as_mut_ptr() as usize);
    let pending = exe::newsitem_get_slot(item.as_ptr() as usize, 3);
    let n = exe::newsitem_get_slot(item.as_ptr() as usize, 4) as u8 as i8;
    let slot5 = exe::newsitem_get_slot(item.as_ptr() as usize, 5);
    if n < 4 && n > 0 {
        let mut i: i8 = 0;
        while i < n {
            let mut c = national_squad_slot_club(&item, (i as u8).wrapping_add(8) as u32);
            exe::llist_add_tail(clubs.as_mut_ptr() as usize, &mut c as *mut usize as usize, 4);
            i += 1;
        }
    }
    let slot = exe::national_teams_get_squad_slot(club, kind);
    exe::nationalteams_set_squad_selected(read_usize(G_NATIONAL_TEAMS), club.nation().addr(), (kind & !0xff) | slot as u32);
    exe::news_set_item_slot(read_usize(G_NEWS), manager, (id) as u32 as usize, 2, 1, 0);
    exe::national_teams_post_call_up_news(read_usize(G_NATIONAL_TEAMS), club, kind, 0usize, slot5, pending, clubs.as_ptr() as u32, n as u8 as u32);
    if pending == 0 {
        covered!("tel28_2 squad selection reschedules");
        let mut fixtures = [0u8; 0xe];
        exe::new_llist(fixtures.as_mut_ptr() as usize);
        let (mut date, mut before) = ([0u8; 8], [0u8; 8]);
        exe::new_date(date.as_mut_ptr() as usize, get_screen_param(6) as u32, get_screen_param(7) as u32);
        exe::date_sub_days(date.as_mut_ptr() as usize, before.as_mut_ptr() as usize, 1);
        let (b, d) = (before.as_ptr() as usize, date.as_ptr() as usize);
        exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u32(b), read_u32(b + 2), read_u32(d), read_u32(d + 2), read_u32(club.addr()),
            fixtures.as_mut_ptr() as usize, u32::MAX, 0xffff_fffe, u32::MAX);
        let list = fixtures.as_mut_ptr() as usize;
        write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
        let fixture = exe::llist_get_next(list);
        if fixture != 0 {
            exe::nationalteams_record_club_call_ups(read_usize(G_NATIONAL_TEAMS), club, kind, fixture);
            exe::nationalteams_reschedule_clashing_club_fixtures(read_usize(G_NATIONAL_TEAMS), date.as_mut_ptr() as usize);
        }
        exe::drop_llist(list);
    }
    exe::drop_llist(clubs.as_mut_ptr() as usize);
    NATIONAL_SQUAD_DONE
}

/// Comparer for `national_squad_selection_screen_handle_input`: No side by side; Yes (news, call-ups, fixtures), the
/// menu bar and the object links only the original.
pub(crate) unsafe extern "C" fn compare_national_squad_selection_screen_handle_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_NATIONAL_SQUAD_SELECTION_SCREEN_HANDLE_INPUT);
    let pure = national_squad_widget_id(widget) == 1;
    tel28_click_compare("national squad selection dialog input", widget, pure, || o(widget), || national_squad_selection_screen_handle_input(widget))
}

/// Rule: answering a call-up news item opens the dialog that confirms the called-up squad.
///
/// `national_squad_call_up_open_from_news(manager, id)`: `open_screen` (builder `national_squad_call_up_screen_build`,
/// input `national_squad_call_up_screen_handle_input`, flag 1); if it opens, the five parameters of
/// `national_squad_call_up_plan` (the team from slot 4, the kind from slot 5, the manager, the id, the squad count).
/// Original: `national_squad_call_up_open_from_news` (0x77a140, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_call_up_open_from_news(manager: usize, id: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x77a270usize, 0x77a610usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel28_2 open the call-up dialog");
    for (k, v) in national_squad_call_up_plan(manager, id).iter().enumerate() {
        set_screen_param(k as u32, *v);
    }
}

/// Comparer for `national_squad_call_up_open_from_news`: only the original runs (it opens a screen); the parameters
/// checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_national_squad_call_up_open_from_news(manager: usize, id: u32) {
    let plan = national_squad_call_up_plan(manager, id);
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_NATIONAL_SQUAD_CALL_UP_OPEN_FROM_NEWS)(manager, id);
    if get_screen_param(2) == manager && get_screen_param(3) == id as usize {
        let got = [get_screen_param(0), get_screen_param(1), get_screen_param(2), get_screen_param(3), get_screen_param(4)];
        SCOREBOARD.record(got == plan, || format!("national squad call-up dialog (open {}): params {:x?} x plan {:x?}", id, got, plan));
    }
}

/// Rule: the call-up dialog confirms the called-up national squad when its size is within the limits.
///
/// See `national_squad_confirm_build` (the call-up form: the title in the local name, the question with the name in
/// form 3).
/// Original: `national_squad_call_up_screen_build` (0x77a270, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_call_up_screen_build() {
    national_squad_confirm_build(true);
}

/// Comparer for `national_squad_call_up_screen_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_national_squad_call_up_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_NATIONAL_SQUAD_CALL_UP_SCREEN_BUILD);
    compare_match_screen_build("national squad call-up dialog", &[], || o(), || national_squad_call_up_screen_build());
}

/// Rule: confirming a call-up posts the news of each squad player called up today and clears the nation's call-up
/// mark.
///
/// `national_squad_call_up_screen_handle_input(widget)`: Yes (id 2): the team is parameter 0, the kind parameter 1;
/// for each of the team's 50 squad slots, a valid player (id below `G_STAFF_COUNT`) flagged for the squad (the B
/// squad flag 8 for a kind other than 0, else 4) who joined the nation today (`staff::DATE_JOINED_NATION` ==
/// `G_CURRENT_DATE`) gets the called-up news (`national_teams_post_player_called_up_news` with the squad slot of
/// `national_teams_get_squad_slot`); the news item of parameters 2 and 3 gets 1 in slot 7 (`News::set_item_slot`) and
/// the nation's mark of the squad slot is cleared (0x76d7e0). Returns −9. Other widgets: `national_squad_other_click`.
/// Original: `national_squad_call_up_screen_handle_input` (0x77a610, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_call_up_screen_handle_input(widget: u32) -> i32 {
    if national_squad_widget_id(widget) != 2 {
        return national_squad_other_click(widget);
    }
    covered!("tel28_2 call-up confirmed");
    let club = Club(get_screen_param(0));
    let kind = get_screen_param(1);
    let (manager, id) = (get_screen_param(2), get_screen_param(3) as u32);
    let flag = if kind != 0 { playing_squad::NATIONAL_B_SQUAD } else { playing_squad::NATIONAL_SQUAD };
    for k in 0..0x32 {
        let p = Staff(club.squad(k));
        if !p.is_null()
            && read_i32(p.addr()) < read_i32(G_STAFF_COUNT)
            && p.playing_squad() & flag != 0
            && exe::date_operator_3d_3d(p.addr() + staff::DATE_JOINED_NATION, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) != 0
        {
            let slot = exe::national_teams_get_squad_slot(club, kind as u32);
            exe::national_teams_post_player_called_up_news(read_usize(G_NATIONAL_TEAMS), p, club.nation().addr(), slot as u32);
        }
    }
    exe::news_set_item_slot(read_usize(G_NEWS), manager, (id) as u32 as usize, 7, 1, 0);
    let slot = exe::national_teams_get_squad_slot(club, kind as u32);
    exe::nationalteams_close_squad(read_usize(G_NATIONAL_TEAMS), club.nation().addr(), slot as u32);
    NATIONAL_SQUAD_DONE
}

/// Comparer for `national_squad_call_up_screen_handle_input`: No side by side; Yes (news), the menu bar and the
/// object links only the original.
pub(crate) unsafe extern "C" fn compare_national_squad_call_up_screen_handle_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_NATIONAL_SQUAD_CALL_UP_SCREEN_HANDLE_INPUT);
    let pure = national_squad_widget_id(widget) == 1;
    tel28_click_compare("national squad call-up dialog input", widget, pure, || o(widget), || national_squad_call_up_screen_handle_input(widget))
}

/// Rule: a network game object starts with no connections and no servers listed; it opens its broadcast sockets on the
/// game's port when Winsock starts, and reports in `ok` whether it did.
///
/// `Network::Network(ok, on_connect, on_disconnect)`: `TcpIp::TcpIp(0x10)`; the server count, the hosting and the
/// connected flags 0; the 16 sockets free; the two callbacks kept; `*ok` = 1; `TcpIp::start_winsock` then
/// `TcpIp::open_broadcast_sockets(6667)`, or `*ok` = 0 when Winsock does not start. Returns the object.
/// Original: `Network::Network` (0x77a7d0, national_teams_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn network_network(this: usize, ok: *mut u32, on_connect: usize, on_disconnect: usize) -> usize {
    covered!("tel36_14 network object");
    let base: unsafe extern "thiscall" fn(usize, u32) -> usize = tcpip_fn(0x8ae180);
    base(this, 0x10);
    write_u16(this + network_game::SERVER_COUNT, 0);
    write_u32(this + network_game::HOSTING, 0);
    write_u32(this + network_game::CONNECTED, 0);
    core::ptr::write_bytes((this + network_game::SOCKETS) as *mut u8, 0xff, 0x20);
    write_usize(this + network_game::ON_DISCONNECT, on_disconnect);
    write_usize(this + network_game::ON_CONNECT, on_connect);
    *ok = 1;
    let start: unsafe extern "thiscall" fn(usize) -> u32 = tcpip_fn(0x8ae260);
    if start(this) != 0 {
        let open: unsafe extern "thiscall" fn(usize, u32) -> u32 = tcpip_fn(0x8aea80);
        open(this, NETWORK_PORT);
    } else {
        *ok = 0;
    }
    this
}

/// Rule: the machine's own addresses, as text, for the network game screen.
///
/// `Network::get_local_addresses(out)`: `tcpip_scan_adapter_addresses` into 20 local entries of 32 bytes and their
/// count; each address formatted (`format_ipv4_address`) into `out` + 20 × k. Returns the count.
/// Original: `Network::get_local_addresses` (0x77a980, national_teams_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn network_get_local_addresses(this: usize, out: usize) -> i32 {
    let mut addresses = [0u8; 0x280];
    let mut count = 0i16;
    let scan: unsafe extern "thiscall" fn(usize, usize, *mut i16) = tcpip_fn(0x8aee20);
    scan(this, addresses.as_mut_ptr() as usize, &mut count);
    let format: unsafe extern "thiscall" fn(usize, usize, usize) = tcpip_fn(0x8ae140);
    for k in 0..count.max(0) as usize {
        format(this, addresses.as_ptr() as usize + k * 0x20, out + k * 0x14);
    }
    count as i32
}

/// Rule: hosting opens the listen sockets, lists this machine under the given name and announces it by broadcast;
/// joining connects to the listed server of that name (or to the name itself as an address) and closes the broadcast
/// sockets. Returns whether it is connected.
///
/// `Network::host_or_join_game(join, name)`: `TcpIp::start_winsock`, 0 when it fails. Hosting (`join` 0): hosting = 1;
/// `TcpIp::open_listen_sockets(6667)` sets connected; server name 0 = `name`; server address 0 = `gethostname` (50
/// bytes); the "CM3 LIST SERVER ACK" datagram with them, broadcast (no address). Joining:
/// `NETWORK_JOIN_BY_ADDRESS` = 1; a listed server whose name equals `name` gives its address (and the flag 0),
/// otherwise the address is `name`; hosting = 0, the state of connection 0 = -4; `parse_ipv4_address`, then
/// `TcpIp::connect_to_host(6667, &address)` into socket 0: when it connects, `TcpIp::close_broadcast_sockets` and
/// connected = 1. Returns the connected flag.
/// Original: `Network::host_or_join_game` (0x77a9f0, national_teams_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn network_host_or_join_game(this: usize, join: u32, name: usize) -> u32 {
    let start: unsafe extern "thiscall" fn(usize) -> u32 = tcpip_fn(0x8ae260);
    if start(this) == 0 {
        return 0;
    }
    if join == 0 {
        covered!("tel36_14 host a network game");
        write_u32(this + network_game::HOSTING, 1);
        let listen: unsafe extern "thiscall" fn(usize, u32) -> u32 = tcpip_fn(0x8ae370);
        if listen(this, NETWORK_PORT) != 0 {
            write_u32(this + network_game::CONNECTED, 1);
        }
        network_copy(this + network_game::SERVER_NAMES, name);
        exe::at_944c28(this + network_game::SERVER_ADDRESSES, 0x32);
        network_send_list(this, NETWORK_LIST_ACK, this + network_game::SERVER_ADDRESSES, this + network_game::SERVER_NAMES, 0, 0, 0);
        return read_u32(this + network_game::CONNECTED);
    }
    covered!("tel36_14 join a network game");
    let mut address = name;
    write_u32(NETWORK_JOIN_BY_ADDRESS, 1);
    for k in 0..read_i16(this + network_game::SERVER_COUNT).max(0) as usize {
        if network_same(name, this + network_game::SERVER_NAMES + k * network_game::SERVER_TEXT) {
            write_u32(NETWORK_JOIN_BY_ADDRESS, 0);
            address = this + network_game::SERVER_ADDRESSES + k * network_game::SERVER_TEXT;
        }
    }
    write_u32(this + network_game::HOSTING, 0);
    write_i32(this + network_game::STATES, -4);
    let mut socket_address = [0u8; 0x10];
    let parse: unsafe extern "thiscall" fn(usize, usize, usize) = tcpip_fn(0x8ae0d0);
    parse(this, socket_address.as_mut_ptr() as usize, address);
    let connect: unsafe extern "thiscall" fn(usize, u32, usize) -> u16 = tcpip_fn(0x8ae640);
    let socket = connect(this, NETWORK_PORT, socket_address.as_ptr() as usize);
    write_u16(this + network_game::SOCKETS, socket);
    if socket != 0xffff {
        let close: unsafe extern "thiscall" fn(usize) = tcpip_fn(0x8aec60);
        close(this);
        write_u32(this + network_game::CONNECTED, 1);
    }
    read_u32(this + network_game::CONNECTED)
}

/// Rule: each turn the network object answers the list requests (hosting) or collects the server list (looking for a
/// game), then reads the connections; a complete packet is handed out with its connection number.
///
/// `Network::receive_packet(out, slot)`: hosting: the first free socket takes `TcpIp::accept_connection` (its header
/// and state -4, `ON_CONNECT(k + 1)`); a datagram (`TcpIp::receive_datagram`, 2000 bytes) "CM3 LIST SERVER REQ" is
/// answered with the "ACK" datagram (the host's address and name) to its sender. Not hosting: an "ACK" datagram with
/// fewer than 9 servers listed adds the sender (its formatted address and the name in the datagram) unless the
/// address is already listed or `TcpIp::can_resolve_address` refuses it, then sorts the list by name; a "BYE"
/// datagram removes the sender's address from the list (moving the rest up); when it is not found the count still goes
/// up by one (a defect of the original, kept). Then for each socket in use: while the state is negative the 4-byte
/// header is read a byte at a time (`TcpIp::receive_from_connection`); when complete a zero length starts it again,
/// otherwise the state becomes the length; with a length the packet is read into the connection's buffer until the
/// header (counting down) reaches 0: then `*slot` = k + 1, the packet is copied to `out`, the state goes back to -4 and
/// 1 comes back. A read of -1 or -2 closes the socket (`ON_DISCONNECT(k + 1)`). Otherwise 0.
/// Original: `Network::receive_packet` (0x77af20, national_teams_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn network_receive_packet(this: usize, out: usize, slot: *mut u16) -> u32 {
    let mut buffer = [0u8; 0x7d0];
    let p = buffer.as_mut_ptr() as usize;
    let mut from = [0u8; 0x10];
    let mut text = [0u8; 0x34];
    let (mut a, mut b) = (0u32, 0u32);
    let receive: unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut u32, *mut u32) -> i32 = tcpip_fn(0x8aed80);
    let format: unsafe extern "thiscall" fn(usize, usize, usize) = tcpip_fn(0x8ae140);
    let callback = |field: usize, k: usize| {
        let f = read_usize(this + field);
        if f != 0 {
            let g: unsafe extern "C" fn(u32) = core::mem::transmute(f);
            g(k as u32 + 1);
        }
    };
    if read_u32(this + network_game::HOSTING) != 0 {
        if let Some(k) = (0..0x10).find(|&k| read_u16(this + network_game::SOCKETS + 2 * k) == 0xffff) {
            let accept: unsafe extern "thiscall" fn(usize) -> u16 = tcpip_fn(0x8ae840);
            let socket = accept(this);
            write_u16(this + network_game::SOCKETS + 2 * k, socket);
            if socket != 0xffff {
                covered!("tel36_14 connection accepted");
                write_i32(this + network_game::HEADERS + 4 * k, -4);
                write_i32(this + network_game::STATES + 4 * k, -4);
                callback(network_game::ON_CONNECT, k);
            }
        }
        if receive(this, p, 0x7d0, from.as_mut_ptr() as usize, &mut a, &mut b) > 0 && network_same(p, NETWORK_LIST_REQ) {
            covered!("tel36_14 list request answered");
            network_send_list(this, NETWORK_LIST_ACK, this + network_game::SERVER_ADDRESSES, this + network_game::SERVER_NAMES, from.as_ptr() as usize, a, b);
        }
    } else if receive(this, p, 0x7d0, from.as_mut_ptr() as usize, &mut a, &mut b) > 0 {
        let count = read_i16(this + network_game::SERVER_COUNT);
        let ack = network_strlen(NETWORK_LIST_ACK) as u32;
        if exe::crt_strncmp(p, NETWORK_LIST_ACK, ack) == 0 && count < 9 {
            let n = count as usize;
            let address = this + network_game::SERVER_ADDRESSES + n * network_game::SERVER_TEXT;
            format(this, from.as_ptr() as usize, address);
            core::ptr::copy((p + ack as usize + network_game::SERVER_TEXT) as *const u8,
                (this + network_game::SERVER_NAMES + n * network_game::SERVER_TEXT) as *mut u8, network_game::SERVER_TEXT);
            let listed = (0..n).any(|k| network_same(address, this + network_game::SERVER_ADDRESSES + k * network_game::SERVER_TEXT));
            let resolve: unsafe extern "thiscall" fn(usize, usize) -> u32 = tcpip_fn(0x8aee00);
            if !listed && resolve(this, from.as_ptr() as usize) != 0 {
                covered!("tel36_14 server listed");
                write_i16(this + network_game::SERVER_COUNT, count + 1);
                network_sort_servers(this);
            }
        } else if exe::crt_strncmp(p, NETWORK_LIST_BYE, network_strlen(NETWORK_LIST_BYE) as u32) == 0 {
            covered!("tel36_14 server gone");
            format(this, from.as_ptr() as usize, text.as_mut_ptr() as usize);
            let mut k = 0i32;
            while k < read_i16(this + network_game::SERVER_COUNT) as i32 {
                let at = |j: i32| this + network_game::SERVER_ADDRESSES + j as usize * network_game::SERVER_TEXT;
                if network_same(text.as_ptr() as usize, at(k)) {
                    k += 1;
                    while k < read_i16(this + network_game::SERVER_COUNT) as i32 {
                        network_copy(at(k - 1), at(k));
                        let name = |j: i32| this + network_game::SERVER_NAMES + j as usize * network_game::SERVER_TEXT;
                        network_copy(name(k - 1), name(k));
                        k += 1;
                    }
                    write_i16(this + network_game::SERVER_COUNT, read_i16(this + network_game::SERVER_COUNT) - 1);
                }
                k += 1;
            }
            let count = read_i16(this + network_game::SERVER_COUNT);
            if k == count as i32 {
                write_i16(this + network_game::SERVER_COUNT, count + 1);
            }
        }
    }
    let read: unsafe extern "thiscall" fn(usize, u32, usize, i32) -> i32 = tcpip_fn(0x8ae9c0);
    for k in 0..0x10usize {
        let socket = this + network_game::SOCKETS + 2 * k;
        let state = this + network_game::STATES + 4 * k;
        let header = this + network_game::HEADERS + 4 * k;
        let packet = this + network_game::BUFFERS + k * network_game::BUFFER_SIZE;
        if read_u16(socket) == 0xffff {
            continue;
        }
        if read_i32(state) < 0 {
            let mut r = read(this, read_u16(socket) as u32, (header as isize + 4 + read_i32(state) as isize) as usize, 1);
            while r > 0 {
                write_i32(state, read_i32(state) + 1);
                if read_i32(state) == 0 {
                    if read_i32(header) != 0 {
                        write_i32(state, read_i32(header));
                        break;
                    }
                    write_i32(state, -4);
                }
                r = read(this, read_u16(socket) as u32, (header as isize + 4 + read_i32(state) as isize) as usize, 1);
            }
            if r == -1 || r == -2 {
                write_u16(socket, 0xffff);
                callback(network_game::ON_DISCONNECT, k);
            }
        }
        if read_i32(state) > 0 {
            let at = || (packet as isize + read_i32(state) as isize - read_i32(header) as isize) as usize;
            let mut r = read(this, read_u16(socket) as u32, at(), read_i32(header));
            while r > 0 {
                write_i32(header, read_i32(header) - r);
                if read_i32(header) <= 0 {
                    covered!("tel36_14 packet received");
                    *slot = k as u16 + 1;
                    core::ptr::copy(packet as *const u8, out as *mut u8, read_i32(state) as u32 as usize);
                    write_i32(state, -4);
                    return 1;
                }
                r = read(this, read_u16(socket) as u32, at(), read_i32(header));
            }
            if r == -1 || r == -2 {
                write_u16(socket, 0xffff);
                callback(network_game::ON_DISCONNECT, k);
            }
        }
    }
    0
}

/// Comparer for `network_network`.
pub(crate) unsafe extern "thiscall" fn compare_network_network(this: usize, ok: *mut u32, on_connect: usize, on_disconnect: usize) -> usize {
    // simplification: Winsock and the sockets do not run twice; proven in Rust mode (a network game)
    original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, usize, usize) -> usize>(IDX_NETWORK_NETWORK)(this, ok, on_connect, on_disconnect)
}

/// Comparer for `network_get_local_addresses`.
pub(crate) unsafe extern "thiscall" fn compare_network_get_local_addresses(this: usize, out: usize) -> i32 {
    // simplification: the adapter scan runs once; proven in Rust mode (a network game)
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_NETWORK_GET_LOCAL_ADDRESSES)(this, out)
}

/// Comparer for `network_host_or_join_game`.
pub(crate) unsafe extern "thiscall" fn compare_network_host_or_join_game(this: usize, join: u32, name: usize) -> u32 {
    // simplification: the sockets and the datagrams do not run twice; proven in Rust mode (a network game)
    original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(IDX_NETWORK_HOST_OR_JOIN_GAME)(this, join, name)
}

/// Comparer for `network_receive_packet`.
pub(crate) unsafe extern "thiscall" fn compare_network_receive_packet(this: usize, out: usize, slot: *mut u16) -> u32 {
    // simplification: reading the sockets does not run twice; proven in Rust mode (a network game)
    original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u16) -> u32>(IDX_NETWORK_RECEIVE_PACKET)(this, out, slot)
}

/// The TcpIp base's destructor body: every socket closed (`TcpIp::close_all_sockets`) and the connection table freed
/// (`operator delete`, when set; the pointer is left as it was).
pub(crate) unsafe fn tcpip_close_and_free(this: usize) {
    let close: unsafe extern "thiscall" fn(usize) = tcpip_fn(FN_TCPIP_CLOSE_ALL_SOCKETS);
    close(this);
    let table = read_usize(this + TCPIP_CONNECTIONS);
    if table != 0 {
        exe::crt_operator_delete(table);
    }
}

/// Sends one packet on a connection slot: its length as 4 single-byte sends (little-endian), then the data; true
/// unless the data send returned −2 (the connection closed).
pub(crate) unsafe fn network_send_on(this: usize, slot: u16, data: usize, length: u32) -> bool {
    let send: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> i32 = tcpip_fn(FN_TCPIP_SEND_TO_CONNECTION);
    let header = length.to_le_bytes();
    for k in 0..4 {
        send(this, slot as u32, header.as_ptr() as usize + k, 1);
    }
    send(this, slot as u32, data, length) != -2 // tel40_2: only a closed connection fails
}

/// `langlib_sprintf` (0x66f4e0, cdecl).
pub(crate) unsafe fn tel28_lang() -> Tel28FormatFn {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// A screen's click comparer: side by side over the screen manager and the current screen's parameter block
/// (`screen_param_regions`) and the answer, when `pure` (the click only reads and writes params); otherwise only the
/// original runs.
pub(crate) unsafe fn tel28_click_compare(name: &str, widget: u32, pure: bool, original: impl FnOnce() -> i32, rust: impl FnOnce() -> i32) -> i32 {
    if !pure {
        // simplification: opening screens, the menu bar and the game changes do not run twice; those clicks are
        // proven in Rust mode
        return original();
    }
    let mut regions = screen_param_regions();
    regions.push((G_SCREEN_MANAGER, 0x3840));
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("{}(widget {}): return {} x {}, differences in {:?}", name, widget as i16, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// The national team the current screen slot's human manages (`staff::NATIONAL_JOB` of `tel26_slot_manager`): 0 with
/// no manager or no national job, after the national_teams_screens.cpp box of `line_manager` or `line_club`.
pub(crate) unsafe fn national_squad_human_team(line_manager: u32, line_club: u32) -> (Staff, usize) {
    let manager = tel26_slot_manager();
    if manager.is_null() {
        tel26_error_box(NATIONAL_TEAMS_SCREENS_CPP, line_manager);
        return (manager, 0);
    }
    let club = manager.national_job().addr();
    if club == 0 {
        tel26_error_box(NATIONAL_TEAMS_SCREENS_CPP, line_club);
    }
    (manager, club)
}

/// Whether the national squad screens' human team is missing (no manager or no national job): those runs show an
/// error box and run the original only.
pub(crate) unsafe fn national_squad_has_box() -> bool {
    let manager = tel26_slot_manager();
    manager.is_null() || manager.national_job().addr() == 0
}

/// The No (id 1) and Yes (id 2) buttons of a national squad dialog, in a two-column area of the current Display.
pub(crate) unsafe fn national_squad_yes_no() {
    let weights = NATIONAL_SQUAD_YES_NO_WEIGHTS;
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    tel28_lang()(JOB_TEXT, 0x98e8cc); // "No"
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
    tel28_lang()(JOB_TEXT, 0x98e8d0); // "Yes"
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]); // tel28_2 Yes
}

/// The message of a national squad dialog: the text in [`JOB_TEXT`] as a pale grey text area (font 4).
pub(crate) unsafe fn national_squad_message() {
    tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0xf, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
}

/// The "Ok" button of a national squad dialog that cannot go on (id 1).
pub(crate) unsafe fn national_squad_ok() {
    tel28_lang()(JOB_TEXT, 0x990f7c); // "Ok"
    tel26_add_button([0x6e, 0x22b, 0x30c, 0x24e, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
}

/// The title of a national squad dialog: the club's picture, the menu bar (4) and `name` in the club's home kit
/// colours (`database_get_club_kit_colours`, kit 1, the defaults pale grey and 0xaebdf0) after `fill_name` wrote it.
pub(crate) unsafe fn national_squad_title(club: usize, name: usize, fill_name: impl FnOnce()) {
    let mut bottom = job_colour(JOB_COLOUR_3184);
    let mut front = read_u16(0xaebdf0) as u32;
    exe::background_set_club_picture(club, 0, 0);
    exe::menubar_build(4, 0);
    fill_name();
    exe::database_get_club_kit_colours(club, 1, &mut front, &mut bottom);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, bottom, front, name);
}

/// `NationalTeams::format_squad_label` (0x775610, thiscall, ret 0x14) by address: the squad name of `kind` (the
/// under-23 form when the nation is marked for the Olympic Games) into `out` (100 bytes), lower case with `lower`.
pub(crate) unsafe fn national_squad_label(nation: usize, kind: u32, out: usize, lower: u32) {
    let marked = (read_u8(read_usize(read_usize(G_NATIONAL_TEAMS) + national_teams_fields::OLYMPIC_NATION_MARKS) + read_u32(nation) as usize) != 0) as u32;
    exe::nationalteams_format_squad_label(read_usize(G_NATIONAL_TEAMS), kind, marked, out, 0x64, lower);
}

/// The four parameters `national_teams_screens_open_squad_move_screen` leaves.
pub(crate) fn national_squad_move_plan(player: usize, kind: u32, joining: u32) -> [usize; 4] {
    [player, kind as u8 as i8 as isize as usize, joining as usize, usize::MAX]
}

/// The widget's id, or the default command id, as the national squad inputs read it inline.
pub(crate) unsafe fn national_squad_widget_id(widget: u32) -> u16 {
    tel26_item_id(widget as i16)
}

/// The end of a national squad dialog's input: No (id 1) closes it (−9); otherwise the menu bar
/// (`menubar_handle_command`) and, when it does nothing, the object link (`menubar_handle_object_link`).
pub(crate) unsafe fn national_squad_other_click(widget: u32) -> i32 {
    if national_squad_widget_id(widget) == 1 {
        return NATIONAL_SQUAD_DONE;
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// The club of a news item slot (an index into the club table).
pub(crate) unsafe fn national_squad_slot_club(item: &[u8; 0xe0], slot: u32) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add((exe::newsitem_get_slot(item.as_ptr() as usize, slot) as usize).wrapping_mul(club::SIZE))
}

/// The eight parameters of the squad selection dialog as `national_squad_selection_open_from_news` leaves them: the
/// team (slot 0), the squad kind (slot 1), the manager record, the news id, the players not yet in that squad (the
/// byte), and slots 5, 6 and 7 (the match day and year of the squad date [inference]).
pub(crate) unsafe fn national_squad_selection_plan(manager: usize, id: u32) -> [usize; 8] {
    let mut item = [0u8; 0xe0];
    national_squad_news_item(&mut item, manager, id);
    let club = Club(national_squad_slot_club(&item, 0));
    let kind = exe::newsitem_get_slot(item.as_ptr() as usize, 1);
    let flag = if kind != 0 { playing_squad::NATIONAL_SQUAD } else { playing_squad::NATIONAL_B_SQUAD };
    let mut count = 0u8;
    for k in 0..0x32 {
        let p = Staff(club.squad(k));
        if !p.is_null() && p.playing_squad() & flag == 0 {
            count = count.wrapping_add(1);
        }
    }
    let slot = |k: u32| exe::newsitem_get_slot(item.as_ptr() as usize, k) as usize;
    [club.addr(), kind as usize, manager, id as usize, count as i8 as isize as usize, slot(5), slot(6), slot(7)]
}

/// The squad confirmation dialogs (the selection and the call-up): the title, then either the confirmation with
/// No/Yes or why the squad is invalid with "Ok".
///
/// The team is parameter 0, the squad kind parameter 1, the players counted by the opener parameter 4's byte. The
/// title: the team's national name (`club_get_national_team_name`, form 3) in its home colours (the call-up writes it
/// into the local name, the selection into [`JOB_TEXT`]). The squad size limits (`NationalTeams::get_max_squad_size`
/// 0x7754c0, `get_min_squad_size` 0x775520, signed bytes). Within them: "Confirm Squad Selection" as the subtitle and
/// "Confirm <team name> squad selection ?" (the name in form 2 for the selection, 3 for the call-up) with No/Yes.
/// Outside: "Squad Invalid" and the reason with the name in form 2: with equal limits "must have <max> player" (count
/// 1) or the limit and the count; else the two limits (and the count when it is not 1); then "Ok".
pub(crate) unsafe fn national_squad_confirm_build(call_up: bool) {
    let club = Club(get_screen_param(0));
    let kind = get_screen_param(1);
    let count = get_screen_param(4) as u8 as i8;
    let mut name = [0u8; 0xc8];
    let title = if call_up { name.as_mut_ptr() as usize } else { JOB_TEXT };
    national_squad_title(club.addr(), title, || {
        if call_up {
            exe::club_get_national_team_name(club, kind, title, 0xc8, 3, 1);
        } else {
            exe::club_get_national_team_name(club, kind, title, 0x7d0, 3, 1);
        }
    });
    let limit = |address: usize| {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(address);
        f(read_usize(G_NATIONAL_TEAMS), club.addr(), kind) as i8
    };
    let max = limit(0x7754c0);
    let min = limit(0x775520);
    let g = club.short_gender_name() as i8 as i32;
    let national_name = |name: &mut [u8; 0xc8], form: u32| {
        exe::club_get_national_team_name(club, kind, name.as_mut_ptr() as usize, 0xc8, form, 1);
    };
    if count <= max && count >= min {
        covered!("tel28_2 squad confirmation");
        if !call_up {
            national_name(&mut name, 2);
        }
        tel28_lang()(JOB_TEXT, 0xa64968); // "Confirm Squad Selection"
        exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
        if call_up {
            national_name(&mut name, 3);
        }
        tel28_lang()(JOB_TEXT, 0xa64918, g, g, name.as_ptr()); // "Confirm {}<%s>{} squad selection ?"
        national_squad_message();
        national_squad_yes_no();
        return;
    }
    covered!("tel28_2 squad invalid");
    tel28_lang()(JOB_TEXT, 0xa64908); // "Squad Invalid"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    national_name(&mut name, 2);
    let (min, max, count) = (min as i32, max as i32, count as i32);
    if min == max {
        if count == 1 {
            tel28_lang()(JOB_TEXT, 0xa6486c, g, g, name.as_ptr(), max);
        } else {
            tel28_lang()(JOB_TEXT, 0xa647b8, g, g, name.as_ptr(), max, count);
        }
    } else if count == 1 {
        tel28_lang()(JOB_TEXT, 0xa646f8, g, g, name.as_ptr(), min, max);
    } else {
        tel28_lang()(JOB_TEXT, 0xa64620, g, g, name.as_ptr(), min, max, count);
    }
    national_squad_message();
    national_squad_ok();
}

/// The five parameters of the call-up dialog as `national_squad_call_up_open_from_news` leaves them: the team (slot
/// 4), the squad kind (slot 5), the manager record, the news id and the squad count (`database_count_club_squad` of
/// the B squad flag 8 for a kind other than 0, else 4; its byte).
pub(crate) unsafe fn national_squad_call_up_plan(manager: usize, id: u32) -> [usize; 5] {
    let mut item = [0u8; 0xe0];
    national_squad_news_item(&mut item, manager, id);
    let club = national_squad_slot_club(&item, 4);
    let kind = exe::newsitem_get_slot(item.as_ptr() as usize, 5);
    let count = exe::database_count_club_squad(club, if kind != 0 { 8 } else { 4 }, 0);
    [club, kind as usize, manager, id as usize, count as i8 as isize as usize]
}

/// A TcpIp method by address (thiscall).
pub(crate) unsafe fn tcpip_fn<F>(address: usize) -> F {
    core::mem::transmute_copy(&address)
}

/// The length of a C string.
pub(crate) unsafe fn network_strlen(s: usize) -> usize {
    let mut n = 0;
    while read_u8(s + n) != 0 {
        n += 1;
    }
    n
}

/// `strcmp` on two C strings, as the original's inline loop (equal or not).
pub(crate) unsafe fn network_same(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        k += 1;
    }
}

/// `strcpy` (the original's inline copy, terminator included).
pub(crate) unsafe fn network_copy(dst: usize, src: usize) {
    let n = network_strlen(src) + 1;
    core::ptr::copy(src as *const u8, dst as *mut u8, n);
}

/// The "list server" datagram: the text (without its terminator), then 50 bytes of an address and 50 bytes of a name,
/// sent by `TcpIp::send_datagram(buffer, length + 100, port, to, a, b)`.
pub(crate) unsafe fn network_send_list(this: usize, text: usize, address: usize, name: usize, to: usize, a: u32, b: u32) {
    let mut buffer = [0u8; 0x7d0];
    let p = buffer.as_mut_ptr() as usize;
    let n = network_strlen(text);
    core::ptr::copy(text as *const u8, p as *mut u8, n);
    core::ptr::copy(address as *const u8, (p + n) as *mut u8, network_game::SERVER_TEXT);
    core::ptr::copy(name as *const u8, (p + n + network_game::SERVER_TEXT) as *mut u8, network_game::SERVER_TEXT);
    let send: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> i32 = tcpip_fn(0x8aec90);
    send(this, p, (n + 0x64) as u32, NETWORK_PORT, to, a, b);
}

/// The server list after an "ACK": sorts the names (bubble sort by `strcmp`, swapping names and addresses as strings)
/// until a pass makes no swap.
pub(crate) unsafe fn network_sort_servers(this: usize) {
    let mut held = [0u8; 0x7d0];
    let tmp = held.as_mut_ptr() as usize;
    loop {
        let mut swapped = false;
        for k in 1..read_i16(this + network_game::SERVER_COUNT) as usize {
            let name = |j: usize| this + network_game::SERVER_NAMES + j * network_game::SERVER_TEXT;
            let address = |j: usize| this + network_game::SERVER_ADDRESSES + j * network_game::SERVER_TEXT;
            if exe_strcmp(name(k - 1), name(k)) > 0 {
                network_copy(tmp, address(k - 1));
                network_copy(address(k - 1), address(k));
                network_copy(address(k), tmp);
                network_copy(tmp, name(k - 1));
                network_copy(name(k - 1), name(k));
                network_copy(name(k), tmp);
                swapped = true;
            }
        }
        if !swapped {
            return;
        }
    }
}

/// The national squad news item behind a dialog opened from the inbox: `NewsItem::NewsItem` on `item` and
/// `News::find_item_by_id` (0x786b70) of the manager record and the id.
pub(crate) unsafe fn national_squad_news_item(item: &mut [u8; 0xe0], manager: usize, id: u32) {
    exe::new_newsitem(item.as_mut_ptr() as usize, 0, 0);
    exe::news_find_item_by_id(read_usize(G_NEWS), manager, id, item.as_mut_ptr() as usize);
}

/// `strcmp` with the sign (the original's inline loop: -1, 0 or 1).
pub(crate) unsafe fn exe_strcmp(a: usize, b: usize) -> i32 {
    let mut k = 0;
    loop {
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        if x != y {
            return if x < y { -1 } else { 1 };
        }
        if x == 0 {
            return 0;
        }
        k += 1;
    }
}

/// The TcpIp methods the Network calls (tcpip.cpp, thiscall): close every socket and shut Winsock down, close one
/// connection slot (short), send on one connection slot (slot short, data, length) -> bytes sent or −2 when closed.
pub(crate) const FN_TCPIP_CLOSE_ALL_SOCKETS: usize = 0x8ae2a0;

pub(crate) const FN_TCPIP_CLOSE_CONNECTION: usize = 0x8ae340;

pub(crate) const FN_TCPIP_SEND_TO_CONNECTION: usize = 0x8ae8d0;

/// "E:\dev\CM3\cm3 00-01\si\code\network.CPP" (the error boxes of this file).
pub(crate) const NETWORK_CPP: usize = 0xa649a8;

/// The TcpIp's connection table pointer (+0, freed by the destructors) and the Network's connection slots as seen from
/// a 1-based player number (slot k is at `network_game::SOCKETS` + 2 × (k − 1)).
pub(crate) const TCPIP_CONNECTIONS: usize = 0x0;

pub(crate) const NETWORK_PLAYER_SLOTS: usize = 0x4b8;

/// "E:\dev\CM3\cm3 00-01\cm3\code\national_teams_screens.cpp", the path the national_teams_screens.cpp error boxes
/// print.
pub(crate) const NATIONAL_TEAMS_SCREENS_CPP: usize = 0xa645e4;

/// The No/Yes area's column weights (3 and 1).
pub(crate) const NATIONAL_SQUAD_YES_NO_WEIGHTS: [u8; 2] = [3, 1];

/// The national teams' per-nation record (110 bytes) in the vector at +4 of the national teams object.
pub(crate) const NATIONAL_SQUAD_RECORD_SIZE: usize = 0x6e;

/// The return value of the national squad dialogs once answered (back to the previous screen) [inference].
pub(crate) const NATIONAL_SQUAD_DONE: i32 = -9;

/// The game's port (6667).
pub(crate) const NETWORK_PORT: u32 = 0x1a0b;

/// "CM3 LIST SERVER REQ", "CM3 LIST SERVER ACK", "CM3 LIST SERVER BYE".
pub(crate) const NETWORK_LIST_REQ: usize = 0xa649d4;

pub(crate) const NETWORK_LIST_ACK: usize = 0xa64994;

pub(crate) const NETWORK_LIST_BYE: usize = 0xa64980;

/// The "connecting to a listed server" flag (1 while the name typed is not a listed server).
pub(crate) const NETWORK_JOIN_BY_ADDRESS: usize = 0xb77c88;
