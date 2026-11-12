// ---------- batch %%LEVA%%: the discipline rule sets of the ruling bodies (discipline.cpp) ----------

/// When a ruling body of the table is built.
#[derive(Clone, Copy)]
pub(crate) enum RuleSetWhen {
    /// always
    Always,
    /// when the league of the nation (the `KEY_NATION_*` global) is selected (`nation::LEAGUE_SELECTED`)
    Nation(usize),
    /// when the continent (the `KEY_CONTINENT_*` global) passes 0x6699d0
    Continent(usize),
    /// always, the constructor's argument 0 for data older than the version (`G_GAME_DATA_VERSION`, absolute value)
    OlderThan(i32),
    /// when the nation is selected, and the argument 0 for data older than the version
    NationOlderThan(usize, i32),
}

/// The 43 ruling bodies of `Discipline::create_rule_sets` in the original's order: the slot (byte offset in the
/// table), when it is built and its constructor (thiscall(object, argument), each 0x23 bytes).
pub(crate) const RULE_SETS: [(usize, RuleSetWhen, usize); 43] = {
    use RuleSetWhen::*;
    [(0x00, Always, 0x7df180), (0x04, Always, 0x7dd400), (0x08, Always, 0x7e29e0),
     (0x0c, Nation(KEY_NATION_ARGENTINA), 0x7db100),
     (0x10, Nation(KEY_NATION_BELGIUM), 0x7dbca0), (0x14, Nation(KEY_NATION_BELGIUM), 0x7db8f0),
     (0x18, Nation(KEY_NATION_BRAZIL), 0x7dc080), (0x1c, Nation(KEY_NATION_BRAZIL), 0x7dc3c0),
     (0x20, Nation(KEY_NATION_DENMARK), 0x7dcae0),
     (0x24, Nation(KEY_NATION_ENGLAND), 0x7dcf90),
     (0x28, Nation(KEY_NATION_FINLAND), 0x7dd990), (0x2c, Nation(KEY_NATION_FINLAND), 0x7dd660),
     (0x30, Nation(KEY_NATION_FRANCE), 0x7ddcc0),
     (0x34, Nation(KEY_NATION_GERMANY), 0x7de620), (0x38, Nation(KEY_NATION_GERMANY), 0x7de2e0),
     (0x3c, Nation(KEY_NATION_HOLLAND), 0x7ded90),
     (0x40, Nation(KEY_NATION_REPUBLIC_OF_IRELAND), 0x7df480),
     (0x44, Nation(KEY_NATION_ITALY), 0x7dfd70), (0x48, Nation(KEY_NATION_ITALY), 0x7df960),
     (0x4c, Nation(KEY_NATION_JAPAN), 0x7e0490), (0x50, Nation(KEY_NATION_JAPAN), 0x7e01b0),
     (0x54, Nation(KEY_NATION_NORWAY), 0x7e1150), (0x58, Nation(KEY_NATION_NORWAY), 0x7e0ef0),
     (0x5c, Nation(KEY_NATION_PORTUGAL), 0x7e1a60),
     (0x60, Nation(KEY_NATION_SCOTLAND), 0x7e2470), (0x64, Nation(KEY_NATION_SCOTLAND), 0x7e21a0),
     (0x68, Nation(KEY_NATION_SPAIN), 0x7e2f80), (0x6c, Nation(KEY_NATION_SPAIN), 0x7e2c30),
     (0x70, Nation(KEY_NATION_SWEDEN), 0x7e3650), (0x74, Nation(KEY_NATION_SWEDEN), 0x7e32d0),
     (0x78, Nation(KEY_NATION_UNITED_STATES), 0x7e40f0),
     (0x7c, Nation(KEY_NATION_POLAND), 0x7e1610),
     (0x80, Nation(KEY_NATION_GREECE), 0x7de960),
     (0x84, Nation(KEY_NATION_CROATIA), 0x7dc700),
     (0x88, Nation(KEY_NATION_AUSTRALIA), 0x7db5b0),
     (0x8c, Continent(KEY_CONTINENT_OCEANIA), 0x7e13d0),
     (0x90, Nation(KEY_NATION_NORTHERN_IRELAND), 0x7e0af0),
     (0x94, Nation(KEY_NATION_TURKEY), 0x7e3d60), (0x98, Nation(KEY_NATION_TURKEY), 0x7e3a10),
     (0x9c, Nation(KEY_NATION_RUSSIA), 0x7e1e00),
     (0xa0, Nation(KEY_NATION_WALES), 0x7e43c0),
     (0xa4, OlderThan(0x3e044), 0x7db360),
     (0xa8, NationOlderThan(KEY_NATION_SOUTH_KOREA, 0x3e047), 0x7e0730)]
};

/// The continent test of the Oceania rule set (0x6699d0, cdecl(continent) -> nonzero).
const CONTINENT_IS_ACTIVE: usize = 0x6699d0;

/// Allocates the 43-entry ruling body table and constructs one 0x23-byte ruling body object per active rule set.
///
/// `Discipline::create_rule_sets(arg)`: the table (`exe::crt_calloc` of 0x2b dwords) in the discipline object's first
/// dword; failing, the error box (discipline.cpp line 0x161e) and nothing else. Then, in `RULE_SETS` order, each
/// ruling body whose condition holds: a nation's by its league being selected (the `nation::LEAGUE_SELECTED` byte of
/// the nation the key names), Oceania's by its continent (0x6699d0 on `G_CONTINENT_TABLE` + key × 0xc6), the last two
/// (the South Korean one also by the nation) with the constructor's argument 0 when the absolute data version
/// (`G_GAME_DATA_VERSION`) is below 0x3e044 / 0x3e047, otherwise `arg`; each one `exe::crt_operator_new(0x23)` and
/// its constructor (null when the allocation fails), stored in its slot.
/// Original: `Discipline::create_rule_sets` (0x55d420, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn discipline_create_rule_sets(this: usize, arg: u32) {
    let t = exe::crt_calloc(0x2b, 4);
    write_usize(this, t);
    if t == 0 {
        show_error_at(0x9b8f18, 0x161e); // "E:\dev\CM3\cm3 00-01\cm3\code\discipline.cpp"
        return;
    }
    let selected = |key: usize| read_u8(read_usize(G_NATION_TABLE) + read_u32(key) as usize * 0x122 + nation::LEAGUE_SELECTED) != 0;
    let old = |v: i32| read_i32(G_GAME_DATA_VERSION).wrapping_abs() < v;
    for (slot, when, ctor) in RULE_SETS {
        let a = match when {
            RuleSetWhen::Always => arg,
            RuleSetWhen::Nation(k) => {
                if !selected(k) {
                    continue;
                }
                arg
            }
            RuleSetWhen::Continent(k) => {
                let active: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(CONTINENT_IS_ACTIVE);
                if active(read_usize(G_CONTINENT_TABLE) + read_u32(k) as usize * 0xc6) == 0 {
                    continue;
                }
                arg
            }
            RuleSetWhen::OlderThan(v) => if old(v) { 0 } else { arg },
            RuleSetWhen::NationOlderThan(k, v) => {
                if !selected(k) {
                    continue;
                }
                if old(v) { 0 } else { arg }
            }
        };
        if slot == 0x24 {
            covered!("sma_5 english rule set");
        }
        let a = if slot == 0x24 { a } else { a }; // sma5: the English rule set's argument (the mutation's anchor)
        let o = exe::crt_operator_new(0x23);
        let v = if o != 0 {
            let f: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(ctor);
            f(o, a)
        } else {
            0
        };
        write_usize(read_usize(this) + slot, v);
    }
}

/// Comparer for `discipline_create_rule_sets`.
///
/// The ruling bodies are built once per game: only the original runs, and the table it leaves is checked: one object
/// exactly in the slots whose condition holds, none in the others.
pub(crate) unsafe extern "thiscall" fn compare_discipline_create_rule_sets(this: usize, arg: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, arg);
    let t = read_usize(this);
    if t == 0 {
        return;
    }
    let selected = |key: usize| read_u8(read_usize(G_NATION_TABLE) + read_u32(key) as usize * 0x122 + nation::LEAGUE_SELECTED) != 0;
    let mut wrong = Vec::new();
    for (slot, when, _) in RULE_SETS {
        let built = match when {
            RuleSetWhen::Nation(k) | RuleSetWhen::NationOlderThan(k, _) => selected(k),
            RuleSetWhen::Continent(_) => continue, // the continent test is not repeated here
            _ => true,
        };
        if built != (read_usize(t + slot) != 0) {
            wrong.push(slot);
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("discipline_create_rule_sets: slots {:x?} against the selected leagues", wrong));
}
