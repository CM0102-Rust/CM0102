// ---------- sx18_2: the three general ruling bodies built in every game: Europe, International (FIFA) and Asia (ruling_body/rb_europe.cpp, rb_international.cpp, rb_asia.cpp) ----------

/// The European ruling body: vtable 0x96eb34, record slot 2 (the confederations'), rule set 1, ban scale 10, no
/// nation, Europe, no league or cup rule, itself as the continental rule.
pub(crate) const EUROPE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eb34, record_slot: 2, rule_set: 1, scale: 0xa,
    nation: None, continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0xff, cup_rule: 0xff, continental_rule: 1 };

/// The international (FIFA) ruling body: vtable 0x96ec34, record slot 3, rule set 0, ban scale 10, no nation, no
/// continent, no league, cup or continental rule.
pub(crate) const INTERNATIONAL_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ec34, record_slot: 3, rule_set: 0, scale: 0xa,
    nation: None, continent: None, league_rule: 0xff, cup_rule: 0xff, continental_rule: 0xff };

/// The Asian ruling body: vtable 0x96ea14, record slot 2, rule set 0x29, ban scale 10, no nation, Asia, no league or
/// cup rule, itself as the continental rule.
pub(crate) const ASIA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ea14, record_slot: 2, rule_set: 0x29, scale: 0xa,
    nation: None, continent: Some(KEY_CONTINENT_ASIA), league_rule: 0xff, cup_rule: 0xff, continental_rule: 0x29 };

/// Builds the European ruling body (`ruling_body_class_construct` with `EUROPE_RULING_BODY`). Returns this.
/// Original: `RbEurope::RbEurope` (0x7dd400, rb_europe.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_europe_construct(this: usize, file: usize) -> usize {
    covered!("sx18_2 european ruling body built");
    ruling_body_class_construct(this, file, &EUROPE_RULING_BODY)
}

/// Builds the international ruling body (`ruling_body_class_construct` with `INTERNATIONAL_RULING_BODY`). Returns this.
/// Original: `RbInternational::RbInternational` (0x7df180, rb_international.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_international_construct(this: usize, file: usize) -> usize {
    covered!("sx18_2 international ruling body built");
    ruling_body_class_construct(this, file, &INTERNATIONAL_RULING_BODY)
}

/// Builds the Asian ruling body (`ruling_body_class_construct` with `ASIA_RULING_BODY`); with data older than version
/// 0x3e044 the discipline passes it no file. Returns this.
/// Original: `RbAsia::RbAsia` (0x7db360, rb_asia.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_asia_construct(this: usize, file: usize) -> usize {
    covered!("sx18_2 asian ruling body built");
    ruling_body_class_construct(this, file, &ASIA_RULING_BODY)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_europe_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_ruling_body_class_built(r, this, &EUROPE_RULING_BODY, "rb_europe_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_international_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, file);
    compare_ruling_body_class_built(r, this, &INTERNATIONAL_RULING_BODY, "rb_international_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_asia_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, file);
    compare_ruling_body_class_built(r, this, &ASIA_RULING_BODY, "rb_asia_construct")
}
