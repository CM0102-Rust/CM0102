// --- sx20_15: the squad status sentence (shortlist_manager.cpp, 0x84b330), the shoot-out taker slot (tactics.cpp,
// 0x898150) and the staff and club history lists and the club history line (staff_records.cpp, 0x86de70, 0x86df90,
// 0x86f300) ---

/// The squad status sentences, statuses 1 to 8 ("This player is indispensable to the club" .. "This player is on
/// trial"); any other "Uncertain".
const SQUAD_STATUS_SENTENCES: [usize; 8] = [0xa8275c, 0xa8272c, 0xa826fc, 0xa826d0, 0xa826a0, 0xa82678, 0xa82650, 0xa82638];
const SQUAD_STATUS_UNCERTAIN: usize = 0xa8262c;
/// A club's tactics row (`tactics_fields::ROWS`, 0x91 bytes) and the first of its eleven shoot-out taker slots.
const TACTICS_ROW_BYTES: usize = 0x91;
const TACTICS_ROW_SHOOTOUT_TAKERS: usize = 0x86;
/// The size of a club competition record (`G_CLUB_COMP_TABLE`).
const CLUB_COMP_SIZE: usize = 0x6b;
/// The history node sizes of the person and club history lists, and where each keeps its kind byte.
const STAFF_HISTORY_NODE_SIZE: usize = 0x35;
const CLUB_HISTORY_NODE_SIZE: usize = 0x31;
const STAFF_HISTORY_KIND: usize = 0xc;
const CLUB_HISTORY_KIND: usize = 0x8;

/// Writes the description sentence of squad status `status` into `out`; a buffer `size` under 30 shows the error box
/// (line 0x2dae) and writes nothing.
/// Original: `shortlist_manager_squad_status_text` (0x84b330, shortlist_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn shortlist_manager_squad_status_text(_this: usize, status: i8, out: usize, size: i32) {
    if size < 30 {
        show_error_at(0xa825a4, 0x2dae); // shortlist_manager.cpp
        return;
    }
    let sentence = if (1..=8).contains(&status) { SQUAD_STATUS_SENTENCES[status as usize - 1] } else { SQUAD_STATUS_UNCERTAIN };
    langlib_text(out, sentence, &[]); // sx20_15: the squad status sentence
}

/// Stores the selection slot (-1 to 19) of the player who takes shoot-out kick `kick` (0 to 10) in the club's
/// tactics row (`Tactics::get_club_tactics_index` with `human_slot`) and returns 1; no club, a kick or slot out of
/// range show the error box (lines 0x6d4, 0x6dc, 0x6e4) and return 0; a club without a row returns 0.
/// Original: `Tactics::set_shootout_taker_slot` (0x898150, tactics.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn tactics_set_shootout_taker_slot(this: usize, club: usize, kick: i8, slot: i8, human_slot: u32) -> u32 {
    if club == 0 {
        show_error_at(0xa8b908, 0x6d4); // tactics.cpp
        return 0;
    }
    if !(0..=10).contains(&kick) {
        show_error_at(0xa8b908, 0x6dc); // tactics.cpp
        return 0;
    }
    if !(-1..=19).contains(&slot) {
        show_error_at(0xa8b908, 0x6e4); // tactics.cpp
        return 0;
    }
    let row = exe::tactics_get_club_tactics_index(this, club, human_slot);
    if row == -1 {
        return 0;
    }
    let at = read_usize(this + tactics_fields::ROWS).wrapping_add((row as usize).wrapping_mul(TACTICS_ROW_BYTES));
    write_i8(at + TACTICS_ROW_SHOOTOUT_TAKERS + kick as usize, slot); // sx20_15: the taker's slot
    1
}

/// Whether a history node of kind `kind` belongs to one of the categories in `mask`: 1 kinds up to 0x31, 2 from 0x32
/// to 0x63, 4 from 0x64 to 0x95, 8 from 0x96 to 0xc7.
fn history_kind_wanted(kind: u8, mask: u32) -> bool {
    (mask & 1 != 0 && kind <= 0x31)
        || (mask & 2 != 0 && (0x32..=0x63).contains(&kind))
        || (mask & 4 != 0 && (0x64..=0x95).contains(&kind))
        || (mask & 8 != 0 && (0x96..=0xc7).contains(&kind)) // sx20_15: the fourth category
}

/// The history list of entry `id` (`FILE_LLIST::get_nodes` of the file at `file` between the start and end of the
/// entry's pair in the head table at `heads`), without the nodes whose kind byte (at `kind`) falls outside `mask`
/// (none removed when `mask` is 0).
unsafe fn history_list(this: usize, file: usize, heads: usize, id: i32, mask: u32, kind: usize) -> usize {
    let pair = read_usize(this + heads).wrapping_add((id as usize).wrapping_mul(8));
    let list = exe::file_llist_get_nodes(read_usize(this + file), read_u32(pair), read_u32(pair + 4));
    if mask == 0 || list == 0 {
        return list;
    }
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    loop {
        let node = exe::llist_get_next(list);
        if node == 0 {
            break;
        }
        if !history_kind_wanted(read_u8(node + kind), mask) {
            exe::llist_remove(list, node);
        }
    }
    list
}

/// The history list of a person, without the entries whose kind falls outside `mask` (`history_kind_wanted`); without
/// a person the error box (line 0xa74) and no list.
/// Original: `Staff_Records::get_staff_history` (0x86de70, staff_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn staff_records_get_staff_history(this: usize, person: Staff, mask: u32) -> usize {
    if person.is_null() {
        show_error_at(0xa83b78, 0xa74); // staff_records.cpp
        return 0;
    }
    use staff_records_fields as f;
    history_list(this, f::PERSON_HISTORY_FILE, f::PERSON_HISTORY_LISTS, read_i32(person.addr()), mask, STAFF_HISTORY_KIND)
}

/// The history list of a club, without the entries whose kind falls outside `mask`; without a club the error box
/// (line 0xad7) and no list.
/// Original: `Staff_Records::get_club_history` (0x86df90, staff_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn staff_records_get_club_history(this: usize, club: Club, mask: u32) -> usize {
    if club.is_null() {
        show_error_at(0xa83b78, 0xad7); // staff_records.cpp
        return 0;
    }
    use staff_records_fields as f;
    history_list(this, f::CLUB_HISTORY_FILE, f::CLUB_HISTORY_LISTS, read_i32(club.addr()), mask, CLUB_HISTORY_KIND)
}

/// The club competition with id `id` (the record base for -1, as the original's arithmetic gives).
unsafe fn club_comp_record(id: i32) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(CLUB_COMP_SIZE))
}

/// Sets a history line about competition `comp`: the template filled with the competition's gender byte twice and its
/// name (into the shared text 0xde1f64), and, when `linked`, a link to the competition (kind 0x7d0).
unsafe fn club_history_comp_line(out: usize, template: usize, comp: usize, linked: bool) {
    let gender = read_i8(comp + competition::GENDER_NAME) as i32 as usize;
    langlib_text(0xde1f64, template, &[gender, gender, comp + competition::NAME]);
    exe::text_set_string(out, 0xde1f64 as *const u8);
    if linked {
        exe::text_add_link(out, (comp + competition::NAME) as *const u8, 0x7d0, comp);
    }
}

/// Builds the text and link of one club history line by its kind (+8): 0x96 a manager change ("Hired", "resigned" or
/// "Sacked" with the name built from the stored name keys, +0x19 the change, linked to the person while he still has
/// those names); 0 cup winners, 1 runners up, 2 third placed, 5 league champions (linked to the competition); 8
/// promotion from and 9 relegation from a league (linked when there is one); any other kind writes nothing.
/// Defect of the original: a promotion or relegation line without a competition (id -1) reads the gender byte at
/// address 0x37 and crashes the game.
/// Original: `Staff_Records::format_club_history_entry` (0x86f300, staff_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn staff_records_format_club_history_entry(_this: usize, node: usize, out: usize) {
    let kind = read_u8(node + CLUB_HISTORY_KIND);
    match kind {
        0x96 => {
            let (id, first, second, common) = (read_u32(node + 9), read_u32(node + 0xd), read_u32(node + 0x11), read_u32(node + 0x15));
            let change = read_u8(node + 0x19);
            let mut name = [0u8; 0xc8];
            exe::database_format_name_from_ids(first, second, common, name.as_mut_ptr(), 0xc8, 1, id);
            let template = match change {
                1 => 0xa84a34, // "  Hired <%s - Staff Name(e.g.Alex Ferguson)>"
                0 => 0xa84a04, // "  <%s - Staff Name(e.g.Alex Ferguson)> resigned"
                _ => 0xa849d4, // "  Sacked <%s - Staff Name(e.g.Alex Ferguson)>"
            };
            langlib_text(0xde1f64, template, &[name.as_ptr() as usize]);
            exe::text_set_string(out, 0xde1f64 as *const u8);
            let person = staff_at(id as i32 as isize);
            if exe::database_staff_matches_names(person, first, second, common, 0usize) != 0 {
                exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
            }
        }
        0 | 1 | 2 | 5 => {
            let template = match kind {
                0 => 0xa84848, // "  {}<%s - Cup name(e.g.FA Cup)>{} winners"
                1 => 0xa84818, // ".. runners up"
                2 => 0xa847e8, // ".. third placed"
                _ => 0xa847b0, // "  {}<%s - League name(e.g.FA Premiership)>{} champions"
            };
            club_history_comp_line(out, template, club_comp_record(read_i32(node + 9)), true);
        }
        8 | 9 => {
            let id = read_i32(node + 9);
            let comp = if id == -1 { 0 } else { club_comp_record(id) };
            let template = if kind == 8 { 0xa843ac } else { 0xa84368 }; // "  Promotion from {}..{}", "  Relegation from {}..{}"
            club_history_comp_line(out, template, comp, comp != 0);
        }
        _ => {}
    }
}

/// Comparer for `shortlist_manager_squad_status_text`.
pub(crate) unsafe extern "thiscall" fn compare_shortlist_manager_squad_status_text(this: usize, status: i8, out: usize, size: i32) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize, i32)>(%%I0%%);
    if size < 30 {
        return original(this, status, out, size); // the error box: once
    }
    contract_compare("shortlist_manager_squad_status_text", Contract(0), out, |o| original(this, status, o, size),
        |o| shortlist_manager_squad_status_text(this, status, o, size))
}

/// Comparer for `tactics_set_shootout_taker_slot`: side by side over the club's tactics row.
pub(crate) unsafe extern "thiscall" fn compare_tactics_set_shootout_taker_slot(this: usize, club: usize, kick: i8, slot: i8, human_slot: u32) -> u32 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, i8, u32) -> u32>(%%I1%%);
    if club == 0 || !(0..=10).contains(&kick) || !(-1..=19).contains(&slot) {
        return original(this, club, kick, slot, human_slot); // the error box: once
    }
    let row = exe::tactics_get_club_tactics_index(this, club, human_slot);
    if row == -1 {
        return original(this, club, kick, slot, human_slot);
    }
    let at = read_usize(this + tactics_fields::ROWS).wrapping_add((row as usize).wrapping_mul(TACTICS_ROW_BYTES));
    let (r, c, same, mo, mr) = crate::comparison::side_by_side(&[(at, TACTICS_ROW_BYTES)],
        || original(this, club, kick, slot, human_slot), || tactics_set_shootout_taker_slot(this, club, kick, slot, human_slot));
    SCOREBOARD.record(same && r == c, || format!("tactics_set_shootout_taker_slot(kick {}, slot {}) original={} rust={} {:?}", kick, slot, r, c, first_differences(&mo, &mr)));
    r
}

/// Comparer for `staff_records_get_staff_history`.
pub(crate) unsafe extern "thiscall" fn compare_staff_records_get_staff_history(this: usize, person: Staff, mask: u32) -> usize {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, Staff, u32) -> usize>(%%I2%%);
    if person.is_null() {
        return original(this, person, mask); // the error box: once
    }
    club_history_compare_list("staff_records_get_staff_history", STAFF_HISTORY_NODE_SIZE, &[], || original(this, person, mask),
        || staff_records_get_staff_history(this, person, mask))
}

/// Comparer for `staff_records_get_club_history`.
pub(crate) unsafe extern "thiscall" fn compare_staff_records_get_club_history(this: usize, club: Club, mask: u32) -> usize {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, Club, u32) -> usize>(%%I3%%);
    if club.is_null() {
        return original(this, club, mask); // the error box: once
    }
    club_history_compare_list("staff_records_get_club_history", CLUB_HISTORY_NODE_SIZE, &[], || original(this, club, mask),
        || staff_records_get_club_history(this, club, mask))
}

/// Comparer for `staff_records_format_club_history_entry`.
pub(crate) unsafe extern "thiscall" fn compare_staff_records_format_club_history_entry(this: usize, node: usize, out: usize) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I4%%);
    compare_text_writer(out, || original(this, node, out), |t| staff_records_format_club_history_entry(this, node, t),
        || format!("staff_records_format_club_history_entry (club history screen, kind {:#x})", read_u8(node + CLUB_HISTORY_KIND)))
}
