// ---------- batch %%LEVA%%: the daily fog of war update, the due note reminders and the ruling body's base constructor (fog_of_war.cpp, notes.cpp, ruling_body.cpp) ----------

/// FOG_OF_WAR_MAP::daily_update (thiscall, ret 0): one human manager's map's daily knowledge update.
pub(crate) const FN_FOG_OF_WAR_MAP_DAILY_UPDATE: usize = 0x5b0260;
/// Notes::post_reminder_news (thiscall, ret 8): the reminder news of one note of manager slot k.
pub(crate) const FN_NOTES_POST_REMINDER_NEWS: usize = 0x795e20;
/// The note's due date (+0x10b of a note record).
pub(crate) const NOTE_DUE_DATE: usize = 0x10b;
/// RulingBody's own vtable, RulingBody::load (thiscall, ret 4) and the ruling body's list (+7, a 0xe-byte LLIST).
pub(crate) const RULING_BODY_VTABLE: u32 = 0x96f104;
pub(crate) const FN_RULING_BODY_LOAD: usize = 0x7e78a0;
pub(crate) const RULING_BODY_LIST: usize = 0x7;
/// "E:\dev\CM3\cm3 00-01\cm3\code\ruling_body\ruling_body.cpp" (the error box).
pub(crate) const RULING_BODY_CPP: usize = 0xa7a858;

/// The daily fog of war update: each of the sixteen human manager maps that exists runs its daily knowledge update.
///
/// `this` holds 16 map pointers; each non-null one gets FOG_OF_WAR_MAP::daily_update (`FN_FOG_OF_WAR_MAP_DAILY_UPDATE`,
/// thiscall, ret 0), in slot order.
/// Original: `FOG_OF_WAR::daily_update` (0x5afce0, fog_of_war.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fog_of_war_daily_update(this: usize) {
    let update: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FOG_OF_WAR_MAP_DAILY_UPDATE);
    for k in 0..16 {
        let map = read_usize(this + k * 4);
        if map != 0 {
            covered!("mwg_17 fog of war map");
            update(map);
        }
    }
}

// simplification: each map's update changes its knowledge records: it does not run twice. Original only; the plan: the 16
// slots unchanged (the update does not replace the maps).
/// Comparer for `fog_of_war_daily_update`.
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_daily_update(this: usize) {
    let before: Vec<usize> = (0..16).map(|k| read_usize(this + k * 4)).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let same = (0..16).all(|k| read_usize(this + k * 4) == before[k]);
    SCOREBOARD.record(same, || format!("fog_of_war_daily_update({:#x}): the maps changed", this));
}

/// The due note reminders: every note of the sixteen manager slots whose due date is today gets its reminder news.
///
/// `this` holds 16 slots of 8 bytes whose +0 is the slot's note list (LLIST). For each slot k: the list's cursor
/// (`llist::CURSOR`) back to its head (`llist::HEAD`); then, as many times as the list's count (`llist::COUNT`, a signed
/// word read again on each pass), the next note (LLIST::get_next `exe::llist_get_next`, thiscall) and, when its due date
/// (`NOTE_DUE_DATE`) equals today (Date::operator== `exe::date_operator_3d_3d` with [`G_CURRENT_DATE`] by value),
/// Notes::post_reminder_news(k, note) (`FN_NOTES_POST_REMINDER_NEWS`, thiscall, ret 8).
/// Original: `Notes::post_due_reminders` (0x795360, notes.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn notes_post_due_reminders(this: usize) {
    let post: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_NOTES_POST_REMINDER_NEWS);
    for k in 0..16usize {
        let list = read_usize(this + k * 8);
        write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
        let mut i = 0i32;
        while i < read_i16(read_usize(this + k * 8) + llist::COUNT) as i32 {
            let note = exe::llist_get_next(read_usize(this + k * 8));
            if exe::date_operator_3d_3d(note + NOTE_DUE_DATE, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) != 0 {
                covered!("mwg_17 note due today");
                post(this, k as u32, note);
            }
            i += 1;
        }
    }
}

// simplification: the reminders are news: they do not run twice. Original only; the plan: each list's cursor ends at
// the end of the list (or at the head of an empty one).
/// Comparer for `notes_post_due_reminders`.
pub(crate) unsafe extern "thiscall" fn compare_notes_post_due_reminders(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let wrong = (0..16usize).find(|&k| {
        let list = read_usize(this + k * 8);
        read_i16(list + llist::COUNT) > 0 && read_usize(list + llist::CURSOR) != 0
    });
    SCOREBOARD.record(wrong.is_none(), || format!("notes_post_due_reminders({:#x}): slot {:?} not walked to the end", this, wrong));
}

/// The base constructor of every ruling body: its vtable, a new list, no entries; from a saved game, its saved state.
///
/// [this] = `RULING_BODY_VTABLE`; `RULING_BODY_LIST` = a new LLIST (`exe::crt_operator_new(0xe)` and LLIST::LLIST
/// `exe::new_llist`, or 0 out of memory); no entries (`ruling_body::ENTRIES`, `ENTRY_CAPACITY` and `ENTRY_COUNT` = 0).
/// With a saved game (`file`), RulingBody::load(file) (`FN_RULING_BODY_LOAD`, thiscall, ret 4): 0 shows the error box
/// (line 0x36). Returns this (the original builds the list under an SEH frame; nothing here throws).
/// Original: `RulingBody::RulingBody` (0x7e7760, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_ruling_body(this: usize, file: usize) -> usize {
    write_u32(this, RULING_BODY_VTABLE);
    let p = exe::crt_operator_new(0xe);
    let list = if p != 0 { exe::new_llist(p) } else { 0 };
    write_usize(this + RULING_BODY_LIST, list);
    covered!("mwg_17 ruling body built");
    write_u32(this + ruling_body::ENTRIES, 0);
    write_u32(this + ruling_body::ENTRY_CAPACITY, 0);
    write_u32(this + ruling_body::ENTRY_COUNT, 0);
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_RULING_BODY_LOAD);
        if load(this, file) == 0 {
            show_error_at(RULING_BODY_CPP, 0x36);
        }
    }
    this
}

// simplification: allocates the list: it does not run twice. Original only; the plan: the vtable, a list and no entries
// (without a saved game).
/// Comparer for `construct_ruling_body`.
pub(crate) unsafe extern "thiscall" fn compare_construct_ruling_body(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    let ok = file != 0 || (r == this && read_u32(this) == RULING_BODY_VTABLE && read_usize(this + RULING_BODY_LIST) != 0
        && read_u32(this + ruling_body::ENTRIES) == 0 && read_u32(this + ruling_body::ENTRY_COUNT) == 0);
    SCOREBOARD.record(ok, || format!("construct_ruling_body({:#x}): vtable {:#x} list {:#x}", this, read_u32(this), read_usize(this + RULING_BODY_LIST)));
    r
}
