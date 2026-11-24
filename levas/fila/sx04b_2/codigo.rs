// ---------- sx04b_2: the constructors of a nation's awards, the month ratings and the month award (nation_awards.cpp, month_ratings.cpp, month_award.cpp) ----------

/// `NationAwards`' base vtable.
pub(crate) const NATION_AWARDS_BASE_VTABLE: u32 = 0x96da10;
/// `MONTH_AWARD`'s vtable.
pub(crate) const MONTH_AWARD_VTABLE: u32 = 0x96da00;
/// The award type byte (`award_base::TYPE`) of a month award.
pub(crate) const MONTH_AWARD_KIND: u8 = 2;
/// `MONTH_AWARD::load_from_disk(file)` (0x766f50, month_award.cpp, thiscall, ret 4).
const FN_MONTH_AWARD_LOAD_FROM_DISK: usize = 0x766f50;
/// `MONTH_RATINGS::load_from_disk(file)` (0x767b20, month_ratings.cpp, thiscall, ret 4).
const FN_MONTH_RATINGS_LOAD_FROM_DISK: usize = 0x767b20;

/// Builds the base of every nation's awards object: the base vtable, no award table (`nation_awards_fields::TABLE`),
/// no awards (`COUNT`, i16) and no month ratings (`MONTH_RATINGS`). Called first by each nation's constructor, when the game is
/// created or loaded. Returns `this`.
/// Original: `NationAwards::NationAwards` (0x768290, nation_awards.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn construct_nation_awards(this: usize) -> usize {
    covered!("sx04b_2 nation awards built");
    write_u32(this, NATION_AWARDS_BASE_VTABLE); // sx04b_2: the base vtable
    write_u32(this + nation_awards_fields::TABLE, 0);
    write_u16(this + nation_awards_fields::COUNT, 0);
    write_u32(this + nation_awards_fields::MONTH_RATINGS, 0); // sx04b_2: no month ratings yet
    this
}

/// Comparer for `construct_nation_awards`: side by side over the 0xe bytes of the base.
pub(crate) unsafe extern "thiscall" fn compare_construct_nation_awards(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0xe)], || o(this), || construct_nation_awards(this));
    SCOREBOARD.record(equal && r == n, || format!("construct_nation_awards original={:x?} rust={:x?}", mo, mr));
    r
}

/// Builds an empty month ratings table: no entries (+0 pointer, +4 count, +8 capacity), month −1
/// (`month_ratings_fields::MONTH`) and year 0 (`month_ratings_fields::YEAR`), so that the first day of play closes no
/// month; with a save file, the table is then read from it (`MONTH_RATINGS::load_from_disk`). Returns `this`.
/// Original: `MONTH_RATINGS::MONTH_RATINGS` (0x767af0, month_ratings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_month_ratings(this: usize, file: usize) -> usize {
    covered!("sx04b_2 month ratings built");
    write_u8(this + month_ratings_fields::MONTH, 0xff); // sx04b_2: no month yet
    write_u16(this + month_ratings_fields::YEAR, 0);
    write_u32(this, 0); // the entries
    write_u32(this + 4, 0); // the count
    write_u32(this + 8, 0); // the capacity
    if file != 0 {
        covered!("sx04b_2 month ratings loaded");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MONTH_RATINGS_LOAD_FROM_DISK);
        load(this, file);
    }
    this
}

/// Comparer for `construct_month_ratings`: side by side over the 0xf bytes of the table; with a save file (the load
/// allocates) only the original.
pub(crate) unsafe extern "thiscall" fn compare_construct_month_ratings(this: usize, file: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    if file != 0 {
        return o(this, file);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0xf)], || o(this, file), || construct_month_ratings(this, file));
    SCOREBOARD.record(equal && r == n, || format!("construct_month_ratings original={:x?} rust={:x?}", mo, mr));
    r
}

/// Builds a month award: the AWARD base (`AWARD::AWARD` with the nation index, the award index, the staff competition,
/// the reputations, the club competition, the file and the player flag), the `MONTH_AWARD_VTABLE`, the type
/// `MONTH_AWARD_KIND`, and one month slot (no winner, no club: −1) for each month from (`start_month`,
/// `start_year`) up to (`end_month`, `end_year`), at most twelve: the slot count (`MONTH_AWARD_SLOT_COUNT`) goes up
/// with each slot; month 11 is followed by month 0 of the next year. The first slot is always written; the end month
/// is compared signed, the years unsigned (16 bits). With a save file the slots are then read from it
/// (`MONTH_AWARD::load_from_disk`). Returns `this`.
/// Original: `MONTH_AWARD::MONTH_AWARD` (0x766e10, month_award.cpp, thiscall, ret 0x30).
pub(crate) unsafe extern "thiscall" fn construct_month_award(this: usize, nation: u32, index: u32, staff_comp: u32, national_reputation: u32, world_reputation: u32, club_comp: u32, start_month: u32, start_year: u32, end_month: u32, end_year: u32, file: usize, for_players: u32) -> usize {
    award_construct_base(this, [nation, index, staff_comp, national_reputation, world_reputation, club_comp, file as u32, for_players]);
    covered!("sx04b_2 month award built");
    write_u32(this, MONTH_AWARD_VTABLE);
    write_u8(this + award_base::TYPE, MONTH_AWARD_KIND);
    write_u8(this + MONTH_AWARD_SLOT_COUNT, 0);
    let (mut month, mut year) = (start_month as u8 as i8, start_year as u16);
    let end_year = end_year as u16;
    loop {
        let slot = this.wrapping_add((read_i8(this + MONTH_AWARD_SLOT_COUNT) as isize as usize).wrapping_mul(MONTH_AWARD_SLOT_SIZE));
        write_u8(slot + month_award_fields::SLOT_MONTH, month as u8);
        write_u16(slot + month_award_fields::SLOT_YEAR, year);
        write_u32(slot + month_award_fields::SLOT_WINNER_ID, u32::MAX);
        write_u32(slot + month_award_fields::SLOT_CLUB_ID, u32::MAX);
        let count = read_u8(this + MONTH_AWARD_SLOT_COUNT).wrapping_add(1);
        month = month.wrapping_add(1);
        if month > 11 { // sx04b_2: December is followed by January
            month = 0;
            year = year.wrapping_add(1);
        }
        write_u8(this + MONTH_AWARD_SLOT_COUNT, count);
        if (year == end_year && month > end_month as u8 as i8) || year > end_year || count as i8 >= 12 {
            break;
        }
    }
    if file != 0 {
        covered!("sx04b_2 month award loaded");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MONTH_AWARD_LOAD_FROM_DISK);
        load(this, file);
    }
    this
}

/// Comparer for `construct_month_award`: side by side over the 0xa2 bytes of the award (base, slots and count); with a
/// save file (the load allocates) only the original.
pub(crate) unsafe extern "thiscall" fn compare_construct_month_award(this: usize, nation: u32, index: u32, staff_comp: u32, national_reputation: u32, world_reputation: u32, club_comp: u32, start_month: u32, start_year: u32, end_month: u32, end_year: u32, file: usize, for_players: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32) -> usize>(%%I2%%);
    let original = || o(this, nation, index, staff_comp, national_reputation, world_reputation, club_comp, start_month, start_year, end_month, end_year, file, for_players);
    if file != 0 {
        return original();
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, MONTH_AWARD_SLOT_COUNT + 1)], original, || {
        construct_month_award(this, nation, index, staff_comp, national_reputation, world_reputation, club_comp, start_month, start_year, end_month, end_year, file, for_players)
    });
    SCOREBOARD.record(equal && r == n, || format!("construct_month_award(from {}/{} to {}/{}) original={:x?} rust={:x?}", start_month as u8, start_year as u16, end_month as u8, end_year as u16, mo, mr));
    r
}
