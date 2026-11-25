use super::*;

/// Builds a year award: the award base (with the file and the player flag as its last two arguments), type 1, the
/// kind (`kind`) and option (`option`) bytes, no shortlist and no history (−1, −1). With a file,
/// `YEAR_AWARD::load_from_disk(scope, file)`; when there is none or the load fails, a scope gets a new shortlist
/// (`construct_award_shortlist`(scope, no file) in 0x40 bytes of `exe::crt_operator_new`; 0 when out of memory).
/// Returns this.
/// Original: `YEAR_AWARD::YEAR_AWARD` (0x92fe80, year_award.cpp, thiscall, ret 0x2c).
pub(crate) unsafe extern "thiscall" fn year_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, kind: u32, scope: usize, option: u32, file: usize, for_players: u32) -> usize {
    use year_award_obj as ya;
    award_construct_base(this, [nation, index, staff_comp, national_rep, world_rep, club_comp, file as u32, for_players]);
    write_usize(this, ya::VTABLE);
    write_usize(this + ya::SHORTLIST, 0);
    write_u8(this + ya::KIND, kind as u8);
    write_u8(this + ya::OPTION, option as u8);
    write_i32(this + ya::HISTORY_FIRST, -1);
    write_i32(this + ya::HISTORY_LAST, -1);
    write_u8(this + award_base::TYPE, 1);
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_YEAR_AWARD_LOAD_FROM_DISK);
        if load(this, scope, file) != 0 {
            return this;
        }
    }
    if scope != 0 {
        covered!("sx05b_5 year award shortlist");
        let p = exe::crt_operator_new(0x40);
        write_usize(this + ya::SHORTLIST, if p != 0 { construct_award_shortlist(p, scope, 0) } else { 0 });
    }
    this
}

pub(crate) unsafe extern "thiscall" fn compare_year_award_construct(this: usize, nation: u32, index: u32, staff_comp: u32, national_rep: u32,
    world_rep: u32, club_comp: u32, kind: u32, scope: usize, option: u32, file: usize, for_players: u32) -> usize {
    use year_award_obj as ya;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, u32, usize, u32) -> usize>(IDX_YEAR_AWARD_CONSTRUCT);
    let r = o(this, nation, index, staff_comp, national_rep, world_rep, club_comp, kind, scope, option, file, for_players);
    compare_award_built(r, this, file, || {
        read_usize(this) == ya::VTABLE && read_u8(this + ya::KIND) == kind as u8 && read_u8(this + ya::OPTION) == option as u8
            && read_i32(this + ya::HISTORY_FIRST) == -1 && read_i32(this + ya::HISTORY_LAST) == -1 && read_u8(this + award_base::TYPE) == 1
            && (read_usize(this + ya::SHORTLIST) != 0) == (scope != 0)
    }, || format!("year_award_construct(nation {}, index {}, kind {}, scope {:#x}): vtable {:#x} shortlist {:#x}", nation, index, kind, scope,
        read_usize(this), read_usize(this + ya::SHORTLIST)))
}

/// `YEAR_AWARD::load_from_disk(scope, file)` (0x92fff0, thiscall, ret 8): 1 when read.
pub(crate) const FN_YEAR_AWARD_LOAD_FROM_DISK: usize = 0x92fff0;
