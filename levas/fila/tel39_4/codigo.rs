// ---------- batch %%LEVA%%: the small constructors and destructor of club_screens.cpp: a match record's team pair, a dated record and a human's squad view settings ----------

/// The destructor of a match record's two team blocks: the C runtime's vector destructor over the two 0x625-byte
/// blocks at +0x11f, each with `match_team_block_destructor` (0x453c10: the vector destructor over its twenty
/// 0x49-byte entries at +0x71, whose destructor 0x5a53c0 is a bare `ret`), so nothing is actually written.
///
/// `match_team_pair_destructor(this)`: `exe::crt_vector_destructor_iterator(this + 0x11f, 0x625, 2, 0x453c10)`.
/// Original: `match_team_pair_destructor` (0x453bc0, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn match_team_pair_destructor(this: usize) {
    covered!("tel39_4 team pair destructor");
    exe::crt_vector_destructor_iterator(this + MATCH_TEAM_PAIR, MATCH_TEAM_BLOCK_SIZE as u32, 2, MATCH_TEAM_BLOCK_DESTRUCTOR);
}

/// Where a match record keeps its two team blocks, the size of one and the block's destructor.
const MATCH_TEAM_PAIR: usize = 0x11f;
const MATCH_TEAM_BLOCK_SIZE: usize = 0x625;
const MATCH_TEAM_BLOCK_DESTRUCTOR: usize = 0x453c10;
/// The date field of a dated record (the average rating record and its kin).
const RATING_RECORD_DATE: usize = 0x14;

/// The constructor of a record with a date at +0x14 (the average rating record of the club and competition
/// records): only the date is built.
///
/// `rating_record_construct(this)`: `Date::Date` (0x54a110) on `this + 0x14`; returns `this`.
/// Original: `rating_record_construct` (0x453c50, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn rating_record_construct(this: usize) -> usize {
    covered!("tel39_4 rating record constructor");
    // tel39_4: the date at +0x14
    exe::date_date_54a110(this + RATING_RECORD_DATE);
    this
}

/// The constructor of a human manager's squad screen view settings (15 bytes, in his HUMAN_RECORD): the default
/// view, sort and filter.
///
/// `squad_view_settings_construct(this)`: dword +0 = 0x41, dword +4 = 8, dword +8 = 0x154a21, byte +0xc = 6, bytes
/// +0xd and +0xe = 0xff [inference: the columns shown, the sort column and the filters]; returns `this`.
/// Original: `squad_view_settings_construct` (0x487810, club_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn squad_view_settings_construct(this: usize) -> usize {
    covered!("tel39_4 squad view settings");
    write_u32(this, 0x41);
    write_u32(this + 4, 8);
    write_u8(this + 0xc, 6);
    // tel39_4: the default view word
    write_u32(this + 8, 0x154a21);
    write_u8(this + 0xd, 0xff);
    write_u8(this + 0xe, 0xff);
    this
}

/// Comparer for `match_team_pair_destructor`: only the original runs (a destructor does not run twice; its element
/// destructors write nothing, so there is no outcome to check).
pub(crate) unsafe extern "thiscall" fn compare_match_team_pair_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_MATCH_TEAM_PAIR_DESTRUCTOR)(this)
}

/// Comparer for `rating_record_construct`: side by side over the date and the return.
pub(crate) unsafe extern "thiscall" fn compare_rating_record_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_RATING_RECORD_CONSTRUCT);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, RATING_RECORD_DATE + 8)], || o(this), || rating_record_construct(this));
    SCOREBOARD.record(same && r == n, || format!("rating_record_construct({:#x}) original={:#x} {:x?} rust={:#x} {:x?}", this, r, mo, n, mr));
    r
}

/// Comparer for `squad_view_settings_construct`: side by side over the 15 bytes and the return.
pub(crate) unsafe extern "thiscall" fn compare_squad_view_settings_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_SQUAD_VIEW_SETTINGS_CONSTRUCT);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, 15)], || o(this), || squad_view_settings_construct(this));
    SCOREBOARD.record(same && r == n, || format!("squad_view_settings_construct({:#x}) original {:x?} rust {:x?}", this, mo, mr));
    r
}
