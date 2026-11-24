// ---------- cx01_1: the competition and cup constructors and the fixture's substitute rules (comp.cpp, Cup.cpp) ----------

/// The Comp base vtable.
const COMP_VTABLE: u32 = 0x9689e8;
/// The Cup vtable.
const CUP_VTABLE: u32 = 0x9693c8;

/// Builds the Comp base object: the Comp vtable, no statistics, stages, entrant book or second buffer (+0x10, +0x18),
/// no division above or below and no fixed stadium (−1), no prize fund, no stages and the current stage −1, every count
/// and code word 0 (+0x34 too), played by club sides, active (`ACTIVE` dword 1), 5 substitutes named and 3 substitutions
/// allowed, not closed, the discipline rule 0xff, the importance 20 and no squad flags. Returns the object.
/// Original: `Comp::Comp` (0x48c580, comp.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn new_comp(this: usize) -> usize {
    let comp = League(this);
    write_u32(this, COMP_VTABLE);
    write_u32(this + 0x18, 0); // the second owned buffer, freed by the destructor
    write_u16(this + 0x34, 0); // a code word [inference]
    comp.set_club_sides(1);
    comp.set_lower_division_id(usize::MAX);
    comp.set_upper_division_id(-1);
    comp.set_stage_id(0);
    comp.set_importance(0x14);
    comp.set_prize_fund(0);
    comp.set_season_closed(0);
    write_u32(this + 0x10, 0); // an owned pointer [inference]
    comp.set_comp_stats(0);
    comp.set_stages(0);
    comp.set_current_stage(usize::MAX);
    comp.set_stage_count(0);
    comp.set_active(1);
    comp.set_club_count(0);
    comp.set_squad_flags(0);
    comp.set_substitutes_named(5);
    comp.set_substitutions_allowed(3);
    comp.set_entrant_book(0);
    comp.set_entrant_capacity(0);
    comp.set_entrant_count(0);
    comp.set_entrants_are_qualifiers(0);
    comp.set_fixed_stadium_id(usize::MAX);
    comp.set_discipline_rule(0xff);
    this
}

/// Builds the Cup base object: the Comp base (`new_comp`), then the Cup vtable, the round being set up and the next
/// round to draw both 0 and the competition type cup (1). Returns the object.
/// Original: `Cup::Cup` (0x518640, Cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn new_cup(this: usize) -> usize {
    new_comp(this);
    let cup = Cup(this);
    write_u32(this, CUP_VTABLE);
    cup.set_current_round(0);
    cup.set_next_draw_round(0);
    cup.set_comp_type(1);
    this
}

/// Writes the competition's substitute rules into a fixture: a stage (`STAGE_INDEX` not 0xff) whose parent competition
/// (the object of its competition id in `G_COMP_OBJECT_TABLE`) exists defers to the parent's `SET_FIXTURE_SUBSTITUTES`;
/// otherwise the low nibble of the fixture's `SQUAD_RULES` word becomes the substitutes named and the next nibble the
/// substitutions allowed (each & 0xf), the other bits kept.
/// Original: `Comp::set_fixture_substitutes` (0x48f2d0, comp.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_set_fixture_substitutes(this: usize, fixture: usize) {
    let comp = League(this);
    if comp.stage_index() != 0xff {
        let table = read_usize(G_COMP_OBJECT_TABLE);
        if table != 0 {
            let parent = read_usize(table + read_u32(comp.competition().addr()) as usize * 4);
            if parent != 0 {
                covered!("cx01_1 substitutes from the parent");
                let set: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(parent) + comp_vtable::SET_FIXTURE_SUBSTITUTES));
                set(parent, fixture);
                return;
            }
        }
    }
    covered!("cx01_1 substitutes of the competition");
    let fixture = Fixture(fixture);
    let named = (comp.substitutes_named() & 0xf) as u16;
    let allowed = (comp.substitutions_allowed() & 0xf) as u16;
    fixture.set_squad_rules((fixture.squad_rules() & 0xff00) | allowed << 4 | named);
}

/// Comparer for `new_comp`: the original and the copy build the object from the same bytes; the 0x53 bytes of the
/// Comp base and the return must match.
pub(crate) unsafe extern "thiscall" fn compare_new_comp(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, 0x53)], || o(this), || new_comp(this));
    SCOREBOARD.record(same && r == n, || format!("new_comp({:#x}): original {:x?} rust {:x?}", this, mo, mr));
    r
}

/// Comparer for `new_cup`: as `compare_new_comp`, over the 0xad bytes up to the Cup round fields.
pub(crate) unsafe extern "thiscall" fn compare_new_cup(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, 0xad)], || o(this), || new_cup(this));
    SCOREBOARD.record(same && r == n, || format!("new_cup({:#x}): original {:x?} rust {:x?}", this, mo, mr));
    r
}

/// Comparer for `comp_set_fixture_substitutes`: the fixture's `SQUAD_RULES` word must match.
pub(crate) unsafe extern "thiscall" fn compare_comp_set_fixture_substitutes(this: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(fixture + fixture::SQUAD_RULES, 2)], || o(this, fixture), || comp_set_fixture_substitutes(this, fixture));
    SCOREBOARD.record(same, || format!("comp_set_fixture_substitutes({:#x}, {:#x}): original {:x?} rust {:x?}", this, fixture, mo, mr));
}
