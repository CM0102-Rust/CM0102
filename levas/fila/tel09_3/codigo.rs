// ---------- batch %%LEVA%%: the staff profile's input handler (staff_screens.cpp, part 3) ----------

/// The notice or confirm box (0x591840, cdecl): title, subtitle, text, kind (1 asks to confirm), the two kit colour
/// words and the command id the answer comes back with (`G_PENDING_COMMAND_CODE`).
unsafe fn profile_input_box(title: usize, subtitle: usize, text: usize, kind: u32, c1: u32, c2: u32, id: u32) {
    let f: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x591840usize);
    f(title, subtitle, text, kind, c1, c2, id);
}

/// A notice box about the person (titled with his name `name`, kind 0): the text `fmt` with `args` into the screens'
/// buffer, the subtitle `subtitle` into `buf`, the colours and the command id.
unsafe fn profile_input_notice(name: usize, buf: usize, subtitle: usize, text: usize, c1: u32, c2: u32, id: u32) {
    let r = staff_tab_lang()(buf, subtitle, text);
    profile_input_box(name, r as u32 as usize, text, 0, c1, c2, id);
}

/// `TRANSFER_MANAGER` 0x8ce990 (thiscall, ret 0x10): whether the person can be approached by `club`; the reason code
/// goes to `out` (0 = he can, the reason is then the offer kind of 0x8f0e40).
unsafe fn profile_input_approach(p: Staff, club: u32, out: &mut u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, *mut u32, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    f(read_usize(G_TRANSFER_MANAGER), p.addr(), club, out, 0)
}

/// A cdecl action of the profile on the person with up to three more dword arguments.
unsafe fn profile_input_action(f: usize, p: Staff, a: u32, b: u32, c: u32) {
    let g: unsafe extern "C" fn(usize, u32, u32, u32) = core::mem::transmute(f);
    g(p.addr(), a, b, c);
}

/// `Date` 0x549f70 (thiscall, ret 0x10): builds a date into `out` from (1, `a`, `b`, -1) and returns `out`.
unsafe fn profile_input_make_date(out: &mut [u16; 4], a: u32, b: u32) -> *const u16 {
    let f: unsafe extern "thiscall" fn(*mut u16, u32, u32, u32, i32) -> *const u16 = core::mem::transmute(0x549f70usize);
    f(out.as_mut_ptr(), 1, a, b, -1)
}

/// Answers a click on the staff profile screen (item `item` of the current human's display; its command id, 0 and -1
/// being the default command `G_CURRENT_COMMAND_ID`, and its value).
///
/// On a network game (`[0xb77c84]`), first the pending box answer: 0x18 (the award vote) clears the pending code and
/// argument, and with answer 2 votes (0x417fe0 on the shortlist of screen parameter 0xd, for the slot's manager) and
/// returns -10, clearing parameter 0xd either way; 0x27, 0x42 and 0x3c are only cleared. Then by command: 0x3c / 0x3d
/// parameter 0x14 = 1 / 0 (-11); 1 opens the profile of the item's value on the same tab (`staff_screens_open_staff_profile`);
/// 0x1f 0x4e53e0; 0x1e "first second day.month.year" to 0x5f11c0; 2 the award vote box (the shortlist entry in
/// parameter 0xd, the kit colours of the entry's club, of the award's nation or of the default nation 0x9cf79c,
/// "Vote for ... award ?", "Please Confirm", "Vote For Award", command 0x18); 3 moving the player to the squad
/// (0x4cbd20; when it fails, -11; else "There is no space in the ... squad", command 0x27); 0x1d the shortlist
/// (0x842c00 when there is a transfer manager with a shortlist; -11); 0x2c notes; 0x2e / 0x2d 0x7e5b10 with 1 / 0;
/// 0x2a the club's affiliate or another person of the club's five staff slots (+0x19f) to 0x48bc00 (-11); 0x2b the
/// first of the club's three slots at +0x1cf to 0x7a2a50 (-11); 0x23 0x538bc0 (-11); 5 0x8f6770; 0x24 0x5a8380; 6 an
/// offer of contract (0x8c5450: "Cannot offer contract", command 0x3c; else 0x4dd0f0); 7 and 8 / 0x19 and 0x1a
/// 0x8f8120 with 0 / 1; 9 0x8f8630; 0xa / 0xb 0x8f4160 with 0 / 1; 0x2f / 0x30 0x8f41b0 with 1, 1 / 1, 2; 0x31 the
/// compare players screen (two different players picked: open 0x876520 for the first one or the person, the
/// staff_screens.cpp 428 box without one; else the picking screen 0x88c460); 0xc 0x4dd0f0 with the value; 0xd the
/// value's offer (0x8f2470 for a kind 2 offer, else 0x8f4c70); 0xe 0x8e9350 with the value; 0xf, 0x10, 0x17 and 0x16
/// the approaches of the slot manager's club (0x8ce990 and 0x8f0e40 with kinds 3, 0, 2 and 1; 0x16 first checks the
/// work permit, reason 0x29, and compares the two dates of 0x8d0590 to choose between the approach and 0x8f9690);
/// 0x18 0x8fbcc0; 0x11 the item's value in parameter 1 when it changes (-11); 0x13 0x8e9550 with the value; 0x1b /
/// 0x1c 0x778f60 with the value and 1 / 0; 0x25 and 0x26 the person's name screen (0x88bc70; parameter 0 the person,
/// parameter 1 a 26-byte copy of his name); 0x29 the scouting of the person (0x7f8ff0: assess screen 0x88c100 with
/// parameter 0 his id and 1 = 1 or 0, or the "no scouts", "already scouting" and "no resources" boxes, command 0x42;
/// then 0x7f75f0). Every action returns -4 unless said otherwise. Any other command goes by the view of parameter
/// 1: 1 the squad menu 0x88b2f0, 5 the history menu 0x88b4b0 (their answer when not 0), 4 the scroll row of parameter
/// 0xc into parameter 0xb for a player, another view the staff_screens.cpp 13210 box; then the menu bar's command and
/// object link answers.
///
/// Defects of the original (kept): 0x25 and 0x26 leak the name copy when the screen does not open, and copy the name
/// into 26 bytes without a limit; the error box view path tests an uninitialised local (here 0, the menu bar goes on).
/// Original: `staff_screens_profile_input` (0x888260, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_profile_input(item: u32) -> i32 {
    let p = Staff(get_screen_param(0));
    let view = get_screen_param(1) as u8;
    let mut c1 = get_screen_param(0x11) as u32;
    let mut c2 = get_screen_param(0x12) as u32;
    let manager = tel26_slot_manager();
    if read_u32(0xb77c84) != 0 {
        if read_u32(G_PENDING_COMMAND_CODE) == 0x18 {
            let answer = read_u32(G_PENDING_COMMAND_ARG);
            write_u32(G_PENDING_COMMAND_CODE, 0);
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if answer == 2 {
                covered!("tel09_3 profile input award vote");
                let shortlist = get_screen_param(0xd);
                let vote: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x417fe0usize);
                vote(read_usize(shortlist + 0x1f), manager.addr(), p.addr());
                set_screen_param(0xd, 0);
                return -10;
            }
            set_screen_param(0xd, 0);
        }
        for code in [0x27u32, 0x42, 0x3c] {
            if read_u32(G_PENDING_COMMAND_CODE) == code {
                write_u32(G_PENDING_COMMAND_ARG, 0);
                write_u32(G_PENDING_COMMAND_CODE, 0);
            }
        }
    }
    let id = tel26_item_id(item as i16);
    let value = || tel26_item_value(item as i16);
    let lang = staff_tab_lang();
    let mut name = [0u8; 200];
    let mut subtitle = [0u8; 0x200];
    let mut title = [0u8; 0x200];
    let nm = name.as_mut_ptr() as usize;
    let sub = subtitle.as_mut_ptr() as usize;
    match id {
        0x3c | 0x3d => {
            set_screen_param(0x14, (id == 0x3c) as usize);
            return -11;
        }
        1 => {
            let open: unsafe extern "C" fn(u32, u32, u32, u32, u32) = core::mem::transmute(0x8702a0usize);
            open(value(), view as u32, 1, 1, 0);
            return -4;
        }
        0x1f => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x4e53e0usize);
            f(p.addr());
            return -4;
        }
        0x1e => {
            let second = if read_usize(p.addr() + 8) != 0 { read_usize(p.addr() + 8) } else { 0x989ad8 };
            let first = if read_usize(p.addr() + 4) != 0 { read_usize(p.addr() + 4) } else { 0x989ad8 };
            let dob = p.addr() + staff::DATE_OF_BIRTH;
            let year = read_u16(dob + 2) as u32;
            let month = exe::date_get_month(dob) as i8 as i32 + 1;
            let day = exe::date_get_day_of_month(dob) as i16 as i32;
            // the month and the year are the two arguments the original left on the stack for the date calls
            staff_tab_format()(JOB_TEXT, 0x9ced08usize, first, second, day, month, year); // "%s %s %d.%d.%d"
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x5f11c0usize);
            f(JOB_TEXT);
            return -4;
        }
        2 => {
            covered!("tel09_3 profile input vote box");
            let entry = value() as usize;
            let award = read_usize(read_usize(read_usize(G_AWARD_MANAGER) + 8) + (read_i32(entry + 0xd) as usize).wrapping_mul(4));
            set_screen_param(0xd, entry);
            exe::database_format_staff_name(p, nm, 200, 1);
            let colours: unsafe extern "C" fn(u32, *mut u32, *mut u32) = core::mem::transmute(0x53b7c0usize);
            let club = read_u32(entry + 9);
            if club != 0 {
                colours(club, &mut c1, &mut c2);
            } else if read_u8(award + 0x12) == 1 {
                let nation: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b3d0usize);
                exe::database_get_club_kit_colours(nation(read_u32(award + 0xe)) as usize, 1, &mut c1, &mut c2);
            } else {
                colours((read_usize(G_CLUB_COMP_TABLE) as u32).wrapping_add((read_i32(0x9cf79c) as u32).wrapping_mul(0x6b)), &mut c1, &mut c2);
            }
            // "Vote for <%s - player> for the <%s - award> award ?"
            let text = lang(JOB_TEXT, 0xa8b320usize, nm, read_u32(entry + 5).wrapping_add(4), 1u32, c1, c2, 0x18u32) as u32 as usize;
            let s = lang(sub, 0x9909b8usize, text) as u32 as usize; // "Please Confirm"
            let t = lang(title.as_mut_ptr() as usize, 0xa8562cusize, s) as u32 as usize; // "Vote For Award"
            profile_input_box(t, s, text, 1, c1, c2, 0x18);
            return -4;
        }
        3 => {
            let mut club_out: usize = 0;
            let club = manager.club_job().addr() as u32;
            let squad: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize) -> u32 = core::mem::transmute(0x4cbd20usize);
            if squad(read_usize(G_CONTRACT_MANAGER), p.addr(), club, &mut club_out) != 0 {
                return -11;
            }
            covered!("tel09_3 profile input no squad space");
            exe::database_format_staff_name(p, nm, 200, 1);
            exe::database_get_club_kit_colours(club as usize, 1, &mut c1, &mut c2);
            let g = read_i8(club_out + 0x52) as i32;
            // "There is no space in the {}<%s - Club Name(e.g.Barcelona)>{} squad ..."
            let text = lang(JOB_TEXT, 0xa8b2ccusize, g, g, club_out + 0x38, 0u32, c1, c2, 0x27u32) as u32 as usize;
            profile_input_notice(nm, sub, 0xa8b2b0, text, c1, c2, 0x27); // "Cannot move player to squad"
            return -4;
        }
        0x1d => {
            let tm = read_usize(G_TRANSFER_MANAGER);
            if tm != 0 && read_usize(tm + 0x856) != 0 {
                let shortlist: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842c00usize);
                shortlist(read_usize(tm + 0x856), p.addr(), 1);
            }
            return -11;
        }
        0x2c => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x7961b0usize);
            f(p.addr());
            return -4;
        }
        0x2e | 0x2d => {
            let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x7e5b10usize);
            f(p.addr(), (id == 0x2e) as u32);
            return -4;
        }
        0x2a => {
            let mut club = p.club_job().addr();
            if club == 0 {
                return -11;
            }
            let mut out = view as u32;
            let affiliate: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
            let a = affiliate(club, &mut out, 1);
            if a != 0 && out == 0 {
                club = a;
            }
            let person: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53cb80usize);
            let mut other = person(club);
            if other == p.addr() {
                other = 0;
            }
            if other == 0 {
                for k in 0..5 {
                    other = read_usize(club + 0x19f + 4 * k);
                    if other == p.addr() {
                        other = 0;
                        continue;
                    }
                    if other != 0 {
                        break;
                    }
                }
                if other == 0 {
                    return -11;
                }
            }
            let f: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x48bc00usize);
            f(p.addr(), other);
            return -11;
        }
        0x2b => {
            let club = p.club_job().addr();
            if club == 0 {
                return -11;
            }
            let mut other = 0;
            for k in 0..3 {
                other = read_usize(club + 0x1cf + 4 * k);
                if other != 0 {
                    break;
                }
            }
            if other != 0 {
                let f: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x7a2a50usize);
                f(p.addr(), other);
            }
            return -11;
        }
        0x23 => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x538bc0usize);
            f(p.addr());
            return -11;
        }
        5 => {
            profile_input_action(0x8f6770, p, 0, u32::MAX, 0);
            return -4;
        }
        0x24 => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x5a8380usize);
            f(p.addr());
            return -4;
        }
        6 => {
            let mut club_out: usize = 0;
            let offer: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> u32 = core::mem::transmute(0x8c5450usize);
            if offer(read_usize(G_TRANSFER_MANAGER), p.addr(), &mut club_out) != 0 {
                covered!("tel09_3 profile input cannot offer contract");
                exe::database_format_staff_name(p, nm, 200, 1);
                let g = read_i8(club_out + 0x52) as i32;
                // "<%s - Staff Name(e.g. Kevin James)> ..."
                let text = lang(JOB_TEXT, 0xa8b23cusize, g, g, nm, club_out + 0x38, 0u32, c1, c2, 0x3cu32) as u32 as usize;
                profile_input_notice(nm, sub, 0xa8b224, text, c1, c2, 0x3c); // "Cannot offer contract"
            } else {
                profile_input_action(0x4dd0f0, p, 0, 0, 0);
            }
            return -4;
        }
        7 | 8 | 0x19 | 0x1a => {
            let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x8f8120usize);
            f(p.addr(), (id >= 0x19) as u32);
            return -4;
        }
        9 => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x8f8630usize);
            f(p.addr());
            return -4;
        }
        0xa | 0xb => {
            let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x8f4160usize);
            f(p.addr(), (id == 0xb) as u32);
            return -4;
        }
        0x2f | 0x30 => {
            let f: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x8f41b0usize);
            f(p.addr(), 1, if id == 0x2f { 1 } else { 2 });
            return -4;
        }
        0x31 => {
            let picked: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x7f7920usize);
            let a = picked(read_usize(G_SCOUT_MANAGER), 1);
            let b = picked(read_usize(G_SCOUT_MANAGER), 2);
            if a == 0 || b == 0 || a == b {
                exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x88c460usize, 0x88c660usize, 1, 0usize, 0);
                return -4;
            }
            let first = picked(read_usize(G_SCOUT_MANAGER), 1);
            let who = if first != 0 { first } else { p.addr() };
            if who == 0 {
                tel26_error_box(0xa84aac, 0x1ac); // "E:\dev\CM3\cm3 00-01\cm3\code\staff_screens.cpp"
                return -4;
            }
            exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x876520usize, 0x889c30usize, 0, 0x88b8a0usize, 0);
            return -4;
        }
        0xc => {
            profile_input_action(0x4dd0f0, p, value(), 1, 0);
            return -4;
        }
        0xd => {
            let v = value() as usize;
            let offer = read_usize(v);
            if read_u8(offer + 0x2c) == 2 {
                let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x8f2470usize);
                f(offer, read_u32(offer + 0x24));
            } else {
                let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x8f4c70usize);
                f(v);
            }
            return -4;
        }
        0xe => {
            let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x8e9350usize);
            f(p.addr(), value());
            return -4;
        }
        0xf => {
            let mut reason = 0u32;
            if profile_input_approach(p, manager.club_job().addr() as u32, &mut reason) == 0 {
                profile_input_action(0x8f0e40, p, manager.club_job().addr() as u32, reason, 3);
            } else {
                profile_input_action(0x4dd0f0, p, 0, 1, 0);
            }
            return -4;
        }
        0x10 => {
            let mut reason = 0u32;
            if profile_input_approach(p, manager.club_job().addr() as u32, &mut reason) == 0 {
                profile_input_action(0x8f0e40, p, manager.club_job().addr() as u32, reason, 0);
                return -4;
            }
            if !p.club_job().is_null() {
                let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x8e9350usize);
                f(p.addr(), 0);
                return -4;
            }
            covered!("tel09_3 profile input approach without club");
        }
        0x17 => {
            let mut reason = 0u32;
            if profile_input_approach(p, manager.club_job().addr() as u32, &mut reason) == 0 {
                profile_input_action(0x8f0e40, p, manager.club_job().addr() as u32, reason, 2);
            } else {
                let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x8f8be0usize);
                f(p.addr());
            }
            return -4;
        }
        0x16 => {
            let club = manager.club_job().addr() as u32;
            if exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), p, club as usize, 1) != 0 {
                profile_input_action(0x8f0e40, p, club, 0x29, 1);
                return -4;
            }
            let mut reason = 0u32;
            if profile_input_approach(p, club, &mut reason) == 0 {
                profile_input_action(0x8f0e40, p, club, reason, 1);
                return -4;
            }
            covered!("tel09_3 profile input approach dates");
            let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
            write_usize(G_CONTRACT_LOOKUP_RESULT, contract);
            let current = if contract != 0 { club_at(read_i32(contract + 4) as isize).addr() } else { 0 };
            let (mut a, mut b, mut c, mut d) = (0u32, 0u32, 0u32, 0u32);
            let dates: unsafe extern "thiscall" fn(usize, u32, usize, usize, *mut u32, *mut u32, *mut u32, *mut u32) =
                core::mem::transmute(0x8d0590usize);
            dates(read_usize(G_TRANSFER_MANAGER), club, current, p.addr(), &mut c, &mut d, &mut a, &mut b);
            let (mut t1, mut t2) = ([0u16; 4], [0u16; 4]);
            let d1 = profile_input_make_date(&mut t1, a, b);
            let (day1, year1) = (read_i16(d1 as usize), read_u16(d1 as usize + 2));
            let d2 = profile_input_make_date(&mut t2, c, d);
            let year2 = read_u16(d2 as usize + 2);
            let earlier = if year1 < year2 { true } else if year1 > year2 { false } else { day1 < read_i16(d2 as usize) };
            if earlier {
                profile_input_action(0x8f0e40, p, club, reason, 1);
            } else {
                let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x8f9690usize);
                f(p.addr());
            }
            return -4;
        }
        0x18 => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x8fbcc0usize);
            f(p.addr());
            return -4;
        }
        0x11 => {
            if value() as usize != get_screen_param(1) {
                set_screen_param(1, value() as usize);
                return -11;
            }
        }
        0x13 => {
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(0x8e9550usize);
            f(value(), 1);
            return -4;
        }
        0x1b | 0x1c => {
            let f: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x778f60usize);
            f(p.addr(), value(), (id == 0x1b) as u32);
            return -4;
        }
        0x25 | 0x26 => {
            let copy = exe::crt_malloc(0x1a);
            if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x88bc70usize, 0x88bee0usize, 1, 0usize, 0) == 0 {
                return -4;
            }
            set_screen_param(0, p.addr());
            let hm = read_usize(G_HUMAN_MANAGER);
            let has_name: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x600740usize);
            if has_name(hm, p.addr(), 0) != 0 {
                let name_of: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x6005e0usize);
                staff_tab_strcpy(copy, name_of(hm, p.addr(), 0));
            } else {
                staff_tab_strcpy(copy, read_usize(p.addr() + staff::COMMON_NAME));
            }
            set_screen_param(1, copy);
            return -4;
        }
        0x29 => {
            covered!("tel09_3 profile input scouting");
            let club = manager.club_job().addr();
            exe::database_format_staff_name(p, nm, 200, 1);
            exe::database_get_club_kit_colours(club, 1, &mut c1, &mut c2);
            let scout: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x7f8ff0usize);
            let answer = scout(read_usize(G_SCOUT_MANAGER), p.addr(), 0) as i8;
            let notice = |text: usize, subtitle: usize| {
                let text = lang(JOB_TEXT, text, 0u32, c1, c2, 0x42u32) as u32 as usize;
                profile_input_notice(nm, sub, subtitle, text, c1, c2, 0x42);
            };
            match answer {
                0 | 1 => {
                    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x88c100usize, 0x88c360usize, 1, 0usize, 0) != 0 {
                        set_screen_param(0, read_u32(p.addr()) as usize);
                        set_screen_param(1, (answer == 0) as usize);
                    }
                }
                2 => notice(0xa8b1e4, 0xa8b1c8), // "There are currently no scouts available ..." / "Cannot have player scouted"
                3 => notice(0xa8b144, 0xa8b134), // "You are already scouting this player." / "Already scouted"
                4 => notice(0xa8b16c, 0xa8b1c8), // "Your club does not have the resources ..." / "Cannot have player scouted"
                _ => {}
            }
            let done: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x7f75f0usize);
            done(read_usize(G_SCOUT_MANAGER), p.addr(), 0);
            return -4;
        }
        _ => {}
    }
    let r = match (view as i8 as i32) - 1 {
        0 => {
            let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x88b2f0usize);
            f(item)
        }
        1 | 2 => 0,
        3 => {
            let q = Staff(get_screen_param(0));
            if !q.player().is_null() && (q.is_null() || read_i32(q.addr()) < read_i32(G_STAFF_COUNT)) && item as i16 != -1 {
                let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(0xc));
                set_screen_param(0xb, row);
            }
            0
        }
        4 => {
            let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x88b4b0usize);
            f(item)
        }
        _ => {
            tel26_error_box(0xa84aac, 0x339a); // "E:\dev\CM3\cm3 00-01\cm3\code\staff_screens.cpp"
            0
        }
    };
    if r != 0 {
        return r;
    }
    let r = exe::menubar_handle_command(item);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(item)
}

/// Comparer for `staff_screens_profile_input`: side by side over the screen parameters and the answer only for the
/// clicks that just write a parameter (0x3c, 0x3d, and 0x11 with a new value) with no pending network box answer;
/// every other click opens a box, a screen or acts on the game, and only the original runs.
pub(crate) unsafe extern "C" fn compare_staff_screens_profile_input(item: u32) -> i32 {
    let pending = read_u32(0xb77c84) != 0 && matches!(read_u32(G_PENDING_COMMAND_CODE), 0x18 | 0x27 | 0x42 | 0x3c);
    let id = if item as i16 != -1 { tel26_item_id(item as i16) } else { 0 };
    let stays = !pending
        && match id {
            0x3c | 0x3d => true,
            0x11 => tel26_item_value(item as i16) as usize != get_screen_param(1),
            _ => false,
        };
    compare_resposta("staff profile input", item, original_fn(%%I0%%), staff_screens_profile_input, !stays)
}
