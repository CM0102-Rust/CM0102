// ---------- batch %%LEVA%%: the treatment confirm screen (record_utils.cpp) ----------

/// The staff record of screen parameter `k` (a staff id): `staff::SIZE` × id into the staff table, in 32 bits like
/// the original.
pub(crate) unsafe fn tel36_staff_param(k: u32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((get_screen_param(k) as u32).wrapping_mul(staff::SIZE as u32) as usize)
}

/// The club record of screen parameter `k` (a club id): `club::SIZE` × id into the club table.
pub(crate) unsafe fn tel36_club_param(k: u32) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add((get_screen_param(k) as u32).wrapping_mul(club::SIZE as u32) as usize)
}

/// The club competition record size (`G_CLUB_COMP_TABLE`, `competition` fields).
pub(crate) const TEL36_CLUB_COMP_SIZE: u32 = 0x6b;

/// `langlib_sprintf(out, format, ...)` (0x66f4e0, cdecl): the language text of `format` into `out`.
pub(crate) unsafe fn tel36_langlib() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// The "Please Confirm" screen of a person and a club (the screens of record_utils.cpp and ultimatum.cpp): the
/// person of parameter 0, the club of parameter 2. The club's background picture, the menu bar (4, 0), the title (the
/// person's name, 2000 bytes, style 1, in the club's kit 1 colours), the subtitle "Please Confirm", the question
/// written by `question` into the screens' text buffer (it gets the person's name), the question as a text area
/// (0x6e, 0x91)-(0x30c, 0x217) (font 4, pale grey) and a two-column area (weights 3/1) with "No" (id 2) and "Yes"
/// (id 3).
pub(crate) unsafe fn tel36_confirm_build(question: impl FnOnce(*const u8)) {
    let text = tel36_langlib();
    let weights: [u8; 2] = [3, 1];
    let person = tel36_staff_param(0);
    let club = tel36_club_param(2);
    exe::background_set_club_picture(club, 0, 0);
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    let mut name = [0u8; 0x7d0];
    exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x7d0, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, name.as_ptr() as usize);
    text(JOB_TEXT, 0x9909b8);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    question(name.as_ptr());
    tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    text(JOB_TEXT, 0x98e8cc);
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
    text(JOB_TEXT, 0x98e8d0);
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 3, 0]);
}

/// The answer of a "Please Confirm" screen: in a network game (`TEL35_NETWORK_FLAG`) the pending command argument is
/// cleared and -9 comes back; "Yes" (id 3) runs `yes` and closes with -9, "No" (id 2) closes with -9, other widgets go
/// to the menu bar.
pub(crate) unsafe fn tel36_confirm_answer(raw: u32, yes: impl FnOnce()) -> i32 {
    if read_u32(TEL35_NETWORK_FLAG) != 0 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    match tel26_item_id(raw as i16) {
        3 => {
            yes();
            -9
        }
        2 => -9,
        _ => manager_menu_bar(raw),
    }
}

/// The treatment of each recurring injury type from 0xb to 0x3b (`Injury::get_recurring_injury_type` − 0xb): 0 a
/// physiotherapy ('N'), 1 a surgery ('O'), 2 a faith healer ('M'); outside the table also the faith healer.
const TREATMENT_CASE: [u8; 0x31] = [0, 1, 1, 2, 2, 2, 0, 1, 2, 1, 2, 2, 0, 2, 1, 1, 1, 1, 2, 0, 1, 2, 2, 2, 0, 1, 1, 2, 2, 2, 0, 2, 2,
    1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0];

/// The treatment kinds of the screen (parameter 1): 'M' faith healer, 'N' physiotherapy, 'O' surgery, 'Q' leave of
/// absence ('P' has no question).
const TREATMENT_FAITH_HEALER: u8 = 0x4d;
const TREATMENT_PHYSIO: u8 = 0x4e;
const TREATMENT_SURGERY: u8 = 0x4f;
const TREATMENT_LEAVE: u8 = 0x51;

/// Rule: the treatment confirm screen opens for a player with a club: a leave of absence when asked, otherwise the
/// treatment of his recurring injury type.
///
/// `open_treatment_confirm_screen(person, leave)`: nothing with no person or no `staff::CLUB_JOB`. `open_screen` with
/// the builder 0x7e5c00, the handler 0x7e5e30, single instance, the no-op leave routine 0x5a8f60 and 0; when it opens,
/// the kind: 'Q' with `leave` (a char), otherwise `Injury::get_recurring_injury_type(person)` on `G_INJURY` − 0xb
/// through `TREATMENT_CASE` ('N', 'O' or 'M'; outside 0..0x30 'M'). Parameter 0 = the person's id, 1 = the kind
/// (sign-extended), 2 = the club's id.
/// Original: `open_treatment_confirm_screen` (0x7e5b10, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_treatment_confirm_screen(person: usize, leave: u32) {
    if person == 0 || read_usize(person + staff::CLUB_JOB) == 0 {
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x7e5c00usize, 0x7e5e30usize, 1, 0x5a8f60usize, 0) == 0 {
        return;
    }
    covered!("tel36_1 open the treatment confirm");
    let kind = if leave as u8 != 0 {
        TREATMENT_LEAVE
    } else {
        let recurring: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62ed30usize);
        let t = (recurring(read_usize(G_INJURY), person) as i8 as i32).wrapping_sub(0xb) as u32;
        match TREATMENT_CASE.get(t as usize) {
            Some(0) => TREATMENT_PHYSIO,
            Some(1) => TREATMENT_SURGERY,
            _ => TREATMENT_FAITH_HEALER,
        }
    };
    set_screen_param(0, read_u32(person) as usize);
    set_screen_param(1, kind as i8 as isize as usize);
    set_screen_param(2, read_u32(read_usize(person + staff::CLUB_JOB)) as usize);
}

/// Rule: the treatment confirm screen asks whether to send the player on a two-week leave of absence, to a faith
/// healer, to physiotherapy or to surgery (by the kind), with No / Yes.
///
/// `draw_treatment_confirm_screen()`: `tel36_confirm_build` with the question by the kind (parameter 1): 'Q' the leave
/// of absence (0xa7a3cc), 'M' the faith healer (0xa7a318), 'N' the physiotherapy (0xa7a27c), 'O' the surgery
/// (0xa7a1d8), each with the player's name; another kind ('P') leaves "Please Confirm" in the text.
/// Original: `draw_treatment_confirm_screen` (0x7e5c00, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn draw_treatment_confirm_screen() {
    covered!("tel36_1 draw the treatment confirm");
    let kind = get_screen_param(1) as u8;
    tel36_confirm_build(|name| {
        let question = match kind {
            TREATMENT_LEAVE => 0xa7a3cc,
            TREATMENT_FAITH_HEALER => 0xa7a318,
            TREATMENT_PHYSIO => 0xa7a27c,
            TREATMENT_SURGERY => 0xa7a1d8,
            _ => return,
        };
        tel36_langlib()(JOB_TEXT, question, name);
    });
}

/// Rule: confirming a treatment pays it from the club's finances (except a leave of absence): 22,500 for
/// physiotherapy, 75,000 for surgery, 7,500 otherwise; then the player gets the treatment as an injury and loses the
/// low flag of his state record.
///
/// `handle_treatment_confirm_screen(item)`: `tel36_confirm_answer`; "Yes": with a club (`staff::CLUB_JOB`) and a kind
/// other than 'Q', `FinanceTable::get_club_finance(club)` on `G_FINANCE_TABLE` and, when there is one,
/// `Finance::pay_other_costs` (0x5999e0) with 22500 ('N'), 75000 ('O') or 7500; then `Injury::add_injury(kind, 0,
/// person, 0, 1, -1, 0, 0)` on `G_INJURY` and bit 0 of byte +0xc of the person's record in `G_STAFF_STATE_TABLE`
/// (0x4f bytes each) cleared.
/// Original: `handle_treatment_confirm_screen` (0x7e5e30, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn handle_treatment_confirm_screen(raw: u32) -> i32 {
    let person = tel36_staff_param(0);
    let kind = get_screen_param(1) as u8;
    tel36_confirm_answer(raw, || {
        covered!("tel36_1 confirm the treatment");
        let club = read_usize(person + staff::CLUB_JOB);
        if club != 0 && kind != TREATMENT_LEAVE {
            let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
            if finance != 0 {
                let cost = match kind {
                    TREATMENT_PHYSIO => 0x57e4,
                    TREATMENT_SURGERY => 0x124f8,
                    _ => 0x1d4c,
                };
                let pay: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x5999e0usize);
                pay(finance, cost);
            }
        }
        exe::injury_add_injury(read_usize(G_INJURY), kind as u32, 0, person, 0, 1, -1, 0, 0);
        let state = read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_u32(person) as usize).wrapping_mul(0x4f)) + 0xc;
        write_u8(state, read_u8(state) & 0xfe);
    })
}

/// Comparer for `open_treatment_confirm_screen`.
pub(crate) unsafe extern "C" fn compare_open_treatment_confirm_screen(person: usize, leave: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%)(person, leave)
}

/// Comparer for `draw_treatment_confirm_screen`: the screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_draw_treatment_confirm_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("treatment confirm screen (draw_treatment_confirm_screen)", &[], || o(), || draw_treatment_confirm_screen());
}

/// Comparer for `handle_treatment_confirm_screen`.
pub(crate) unsafe extern "C" fn compare_handle_treatment_confirm_screen(item: u32) -> i32 {
    // simplification: the payment, the injury and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(item)
}
