use super::*;

/// Builds the Asian ruling body (`ruling_body_class_construct` with `ASIA_RULING_BODY`); with data older than version
/// 0x3e044 the discipline passes it no file. Returns this.
/// Original: `RbAsia::RbAsia` (0x7db360, rb_asia.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_asia_construct(this: usize, file: usize) -> usize {
    covered!("sx18_2 asian ruling body built");
    ruling_body_class_construct(this, file, &ASIA_RULING_BODY)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_asia_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RB_ASIA_CONSTRUCT)(this, file);
    compare_ruling_body_class_built(r, this, &ASIA_RULING_BODY, "rb_asia_construct")
}

/// Builds a ruling body of the class: the RulingBody base with the file (`construct_ruling_body`), then the class
/// fields (`ruling_body_class_fields`). Returns this.
pub(crate) unsafe fn ruling_body_class_construct(this: usize, file: usize, class: &RulingBodyClass) -> usize {
    construct_ruling_body(this, file);
    ruling_body_class_fields(this, class)
}

/// Comparer of the ruling body constructors: they are built once per game, so only the original runs and the fields
/// it leaves are checked against the class (the copy's constants).
pub(crate) unsafe fn compare_ruling_body_class_built(r: usize, this: usize, class: &RulingBodyClass, name: &str) -> usize {
    let ok = r == this && read_u32(this) == class.vtable
        && read_u8(this + ruling_body::RECORD_SLOT) == class.record_slot
        && read_u8(this + ruling_body::RULE_SET_INDEX) == class.rule_set
        && read_u8(this + ruling_body::BAN_SCALE) == class.scale
        && read_u32(this + ruling_body::NATION_ID) == ruling_body_key(class.nation)
        && read_u32(this + ruling_body::CONTINENT_ID) == ruling_body_key(class.continent)
        && read_u8(this + 0x1f) == class.league_rule && read_u8(this + 0x20) == class.cup_rule
        && read_u8(this + 0x21) == class.continental_rule && read_u8(this + 0x22) == 0;
    SCOREBOARD.record(ok, || format!("{}({:#x}): vtable {:#x} slot {} rule {} scale {} nation {} continent {} rules {:x} {:x} {:x}",
        name, this, read_u32(this), read_u8(this + ruling_body::RECORD_SLOT), read_u8(this + ruling_body::RULE_SET_INDEX),
        read_u8(this + ruling_body::BAN_SCALE), read_i32(this + ruling_body::NATION_ID), read_i32(this + ruling_body::CONTINENT_ID),
        read_u8(this + 0x1f), read_u8(this + 0x20), read_u8(this + 0x21)));
    r
}

/// The value of an optional key (−1 without one).
pub(crate) unsafe fn ruling_body_key(key: Option<usize>) -> u32 {
    key.map_or(u32::MAX, |k| read_u32(k))
}

/// The Asian ruling body: vtable 0x96ea14, record slot 2, rule set 0x29, ban scale 10, no nation, Asia, no league or
/// cup rule, itself as the continental rule.
pub(crate) const ASIA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ea14, record_slot: 2, rule_set: 0x29, scale: 0xa,
    nation: None, continent: Some(KEY_CONTINENT_ASIA), league_rule: 0xff, cup_rule: 0xff, continental_rule: 0x29 };
