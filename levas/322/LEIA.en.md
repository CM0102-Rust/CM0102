# Batch 322 — the profile's last pieces: the squad and history menu, the rows order, the nickname dialog and the scout request; the screens' small routines: the background picture settings, a Text copy, a list's deleting destructor, the contract news buttons, the waiting screen, a static date's destructor and the search filter record; the screen manager's players and network packets: the player records, the network session, the click and scroll packets and the status message; the network object's life and its packets: destroying a TcpIp and a Network, closing a player's connection, sending a packet and building and destroying a network buffer; the Display's network side: a text entry's text back to its caller and a remote player's click sent to the server; the human managers of a restored game: who takes the screens and who goes to the computer; the club history lists of the screens: league seasons, cup runs, the competitions of each list and the current league record; the written history files (.his): reading one into the history screen, existence, category, title, the -verifyhist check and the order by title

**Routines:**
- `staff_screens_add_squad_and_history_menu()` = 0x88b910 (staff_screens.cpp, cdecl): the profile's menu button: the squad of the person's club (or national side) to jump to another player (up to 28 names, 26 with a history) and "History" when the person has a written history (the key "first second d.m.yyyy");
- `staff_screens_compare_text_rows()` = 0x88bc50 (staff_screens.cpp, cdecl): the order of two text rows (utils_string_compare mode 1), for the qsort of the profile lists;
- `staff_screens_build_nickname_dialog()` = 0x88bc70 (staff_screens.cpp, cdecl): builds the nickname dialog: the name as the title, the nickname entry and Cancel, Remove and Ok (Remove and Ok greyed with no nickname);
- `staff_screens_handle_nickname_dialog_action()` = 0x88bee0 (staff_screens.cpp, cdecl): handles the nickname dialog: typing redraws; Remove takes the nickname away; Ok stores it (changing or adding, unless it equals the common name); Cancel closes; the buffer is freed on closing; the rest goes to the menu bar;
- `staff_screens_build_scout_request_dialog()` = 0x88c100 (staff_screens.cpp, cdecl): builds the scout request: assess the player now (scouts free) or watch him when scouts are available, in the manager's club colours, with No and Yes;
- `staff_screens_handle_scout_request_action()` = 0x88c360 (staff_screens.cpp, cdecl): handles the scout request: No closes, Yes asks for the scout report and closes; the rest goes to the menu bar;
- `background_free_config_tables()` = 0x41bad0 (background.cpp, cdecl): frees the pics.cfg tables: of the 11 sections, each subsection (its two picture lists and its name) and the subsection array, clearing pointers and counts;
- `background_set_pictures_disabled()` = 0x41bf80 (background.cpp, cdecl): sets the fixed picture option (0xdd6bda) to the inverse of the argument;
- `background_pictures_disabled()` = 0x41bfa0 (background.cpp, cdecl): returns the inverse of the fixed picture option (1 when 0xdd6bda is 0);
- `llist_scalar_deleting_destructor()` = 0x497910 (comp_screens.cpp, thiscall): destroys a linked list and frees its memory when bit 0 of the flags is set;
- `text_construct_copy()` = 0x4ae5c0 (comp_text.cpp, thiscall): constructs a Text as a copy of another: its string and each link;
- `contract_screens_news_offer_contract_action()` = 0x4dc9b0 (contract_screens.cpp, cdecl): the news item's "Offer Contract" button: opens the contract offer to the player of slot 2;
- `contract_screens_news_negotiate_action()` = 0x4e5770 (contract_screens.cpp, cdecl): the contract news item's "Negotiate" button: opens the negotiation with the person of slot 0; after an accepted transfer offer (type 0xfa4), on the offer of slot 7;
- `match_screens_draw_waiting_screen()` = 0x727fa0 (match_screens.cpp, cdecl): the waiting for players screen (network game): only clears the background picture;
- `match_stats_static_date_atexit()` = 0x72aa70 (match_screens.cpp, cdecl): the empty destructor of 0x729ff0's static date;
- `search_filter_record_init()` = 0x81b800 (search_screens.cpp, thiscall): constructs the player search's filter record (0x11 bytes at PLAYER_SEARCH +0x293) with the default values;
- `screen_manager_get_local_addresses()` = 0x7fc7c0 (scrman.cpp, thiscall, ret 4): the machine's addresses, through the network session (forwards to Network::get_local_addresses);
- `screen_manager_start_network_session()` = 0x7ff500 (scrman.cpp, thiscall, ret 8): keeps the join and leave hooks, closes the old session and opens a new one whose connections call the player records; returns whether it opened (defect: out of memory it returns "opened" with no session);
- `scrman_add_player_record()` = 0x7ff630 (scrman.cpp, cdecl): a player who joins gets a fresh record in his channel (the one whose first record is his, or the first empty one): up to 16 per channel, the others move up; with a join hook the hook runs and the channel is marked for the monitor; without one the new record goes behind the current one; nothing on a network client;
- `scrman_remove_player_records()` = 0x7ff7b0 (scrman.cpp, cdecl): a player who leaves loses every record of the channels that are his; only on the host;
- `screen_manager_add_manager_record()` = 0x7ff8f0 (scrman.cpp, thiscall, ret 0xc): a manager gets a new record in the current channel; with his key the record goes behind the current one and gets the key, and the session hooks stay as they were;
- `screen_manager_send_clear_click()` = 0x800000 (scrman.cpp, thiscall): clears the current screen's queued click (command 0x11), on the local Display or the remote player's;
- `screen_manager_set_pending_click_widget()` = 0x800160 (scrman.cpp, thiscall, ret 4): the next left click on the current screen returns the given widget (command 0x13);
- `screen_manager_send_area_scroll_row()` = 0x800330 (scrman.cpp, thiscall, ret 8): when an area scrolls its first visible row becomes a packet (command 0xf), sent to the remote screen when channel 0 shows the remote Display; a local game only fills the buffer;
- `screen_manager_send_network_status_message()` = 0x802f80 (scrman.cpp, thiscall, ret 0x10): a status message (a text and two numbers, the progress bar) goes to the given channel's screen, or to every channel's with -1: applied to the local Display for a local record, sent to the player for a remote one;
- `tcpip_destructor()` = 0x77a870 (national_teams_screens.cpp, thiscall): closes every socket and frees the connection table (only through the unwinding of an exception while the Network is built);
- `network_destructor()` = 0x77a890 (national_teams_screens.cpp, thiscall): when hosting, says goodbye to the list servers ("CM3 LIST SERVER BYE" with the address, broadcast; defect: 50 more bytes of whatever the stack held); then closes the sockets and frees the table;
- `network_release_slot_socket()` = 0x77ac00 (national_teams_screens.cpp, thiscall, ret 4): closes the connection of player 1 to 16 (any other number nothing); the slot is not marked free;
- `network_send_packet()` = 0x77ac30 (national_teams_screens.cpp, thiscall, ret 0xc): sends a packet, preceded by its 4-byte length, to one player's connection or, when hosting, to all of them for player 0; a connection found closed is freed (and, when hosting, the leave hook told); a length of 0 or less gives the error box, 50000 or more returns 0; returns whether the last send worked;
- `network_buffer_construct()` = 0x77b630 (national_teams_screens.cpp, thiscall, ret 4): a network buffer starts empty with the given capacity and its own storage; with no memory the game stops (error box, cleanup, exit);
- `network_buffer_destruct()` = 0x77b6e0 (national_teams_screens.cpp, thiscall): frees the buffer's storage, when it has one, and clears the pointer;
- `guio_copy_text_to_caller()` = 0x567b00 (display.cpp, thiscall): a text entry widget with a caller's buffer gets its text copied back, with the terminator;
- `display_send_widget_click()` = 0x568560 (display.cpp, thiscall, ret 8): a remote player's click goes to the server: command 6 with the widget and all its current fields and, when it was dropped on another widget, the drop rectangle and that widget's fields; failing, the "Network Error" box and 0;
- `setup_assign_restored_network_managers()` = 0x8387e0 (Setup.cpp, cdecl): when a saved game is restored, each human manager with no password (in a game with no network name) takes the screens: the first one the current record, the others a record each; one with a password not on holiday goes to computer control until he resumes; returns 1 (open Manager Status) with a manager locked or one with a password on holiday;
- `club_history_get_league_history()` = 0x443380 (club_history.cpp, thiscall, ret 4): builds the list of the club's league seasons, newest first: the current season in front when the league is not over, then the stored ones, from the last to the first;
- `club_history_get_cup_history()` = 0x443520 (club_history.cpp, thiscall, ret 4): builds the list of the club's cup runs: first its progress in each running cup (not a league, a division nor a friendly) where it reached a stage, then the stored ones, from the last to the first;
- `club_history_get_league_comp_ids()` = 0x443770 (club_history.cpp, thiscall, ret 4): collects the distinct competitions of a league season list and returns them in a new list, "sorted" by a comparison that never orders;
- `club_history_get_cup_comp_ids()` = 0x443a20 (club_history.cpp, thiscall, ret 4): the same for a cup run list;
- `club_history_get_current_league_record()` = 0x444190 (club_history.cpp, cdecl): finds the club in the current table of its division (in Brazil, February to June, the state league when asked), from the current stage back, only league stages with more than 7 rounds (any in Poland; in Argentina only the league), and fills the record with division, year, position and totals; also tells whether the season is over and in which stage it was found;
- `compare_int_ids()` = 0x444500 (club_history.cpp, cdecl): qsort comparison that never orders: 0 equal, 1 different;
- `history_load_his_file()` = 0x5f1e50 (History.cpp, cdecl): reads the .his: {title:} the title (parameter 1), {section:} a new section, {pic:} a picture used by the following paragraphs; the text becomes paragraphs (one line break = a space, two = end of paragraph); stores sections, pictures and counts in parameters 2 to 6;
- `history_his_file_exists()` = 0x5f2500 (History.cpp, cdecl): tells whether cm3_history\<name>.his exists, only in a single-player game on the local screen;
- `history_get_his_file_category()` = 0x5f2580 (History.cpp, cdecl): reads the {cat:} or {category:} tag and returns its index among CLUB, NATION, COMP, LEAGUE and PLAYER, or -1;
- `history_get_his_file_title()` = 0x5f2700 (History.cpp, cdecl): reads the {title:} tag into a static text and returns it, or 0;
- `verify_history_files()` = 0x5f2840 (History.cpp, cdecl): with -verifyhist, checks each .his against the database by category (club and nation against the club names, competition and league against the competitions, player against "first second day.month.year") and shows a box for each one with no match;
- `history_compare_entries_by_title()` = 0x5f3410 (History.cpp, cdecl): order of the written history list by title, a null entry last;

**How it was compared:** real comparison on every routine (original and Rust from the same state, with the regions the routine touches).

**Result** (`cadeia.txt`):
- **Mutations (0):** 0 caught.
- **Real game comparing to 2002-01-01:** 0 divergences (881a6c2e).
- **Rust ×2 and original careers to 2002-06-01:** Rust ×2 6edd502b, original 388c7666 (recorded; does not block the batch since 31/10/2026). Garbage detector: equal.

**Redo**, on top of batch 321's commit:
1. `python integrar.py`
2. `cargo build --release` and copy the DLL to `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 322`
