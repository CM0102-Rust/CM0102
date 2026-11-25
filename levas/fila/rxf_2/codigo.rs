// ---------- batch %%LEVA%%: the award manager: the award objects of each nation and the save (award_manager.cpp) ----------

/// When a slot of the award manager's table gets its award object: always, when a European nation is chosen to be
/// played, or when the nation of a key is.
#[derive(Clone, Copy)]
pub(crate) enum AwardSlotGate {
    Always,
    Europe,
    Nation(usize),
}

/// The 30 slots of the award manager's table, in order: the gate, the class's constructor (`Class::Class(file)` on
/// 0x17 bytes, thiscall, ret 4) and, for the classes with an old variant, the staff competition key that picks the new
/// one (key at -1: the old constructor).
pub(crate) const AWARD_SLOTS: [(AwardSlotGate, usize, Option<(usize, usize)>); 30] = [
    (AwardSlotGate::Always, 0x92fc10, None), // WorldCupAwards
    (AwardSlotGate::Always, 0x632ac0, Some((KEY_STAFF_COMP_IRISH_PLAYER_OF_THE_MONTH, 0x79e580))), // InternationalAwards / OldInternationalAwards
    (AwardSlotGate::Europe, 0x583400, None), // EuropeanAwards
    (AwardSlotGate::Nation(KEY_NATION_ARGENTINA), 0x40a350, None), // ArgentinaAwards
    (AwardSlotGate::Nation(KEY_NATION_BELGIUM), 0x424480, None), // BelgiumAwards
    (AwardSlotGate::Nation(KEY_NATION_BRAZIL), 0x43d2b0, None), // BrazilAwards
    (AwardSlotGate::Nation(KEY_NATION_DENMARK), 0x554dc0, None), // DenmarkAwards
    (AwardSlotGate::Nation(KEY_NATION_ENGLAND), 0x578780, None), // EnglandAwards
    (AwardSlotGate::Nation(KEY_NATION_FINLAND), 0x5a8f70, Some((KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_PLAYER_OF_THE_MON, 0x79d540))), // FinlandAwards / OldFinlandAwards
    (AwardSlotGate::Nation(KEY_NATION_FRANCE), 0x5c0980, Some((KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_TEAM_OF_THE_YEAR, 0x79dcf0))), // FranceAwards / OldFranceAwards
    (AwardSlotGate::Nation(KEY_NATION_GERMANY), 0x5df650, None), // GermanyAwards
    (AwardSlotGate::Nation(KEY_NATION_HOLLAND), 0x5f8fe0, None), // HollandAwards
    (AwardSlotGate::Nation(KEY_NATION_REPUBLIC_OF_IRELAND), 0x63b640, Some((KEY_STAFF_COMP_IRISH_PLAYER_OF_THE_MONTH, 0x79ea00))), // IrelandAwards / OldIrelandAwards
    (AwardSlotGate::Nation(KEY_NATION_ITALY), 0x65fe30, None), // ItalyAwards
    (AwardSlotGate::Nation(KEY_NATION_JAPAN), 0x666690, None), // JapanAwards
    (AwardSlotGate::Nation(KEY_NATION_NORWAY), 0x793940, None), // NorwayAwards
    (AwardSlotGate::Nation(KEY_NATION_PORTUGAL), 0x7d3630, None), // PortugalAwards
    (AwardSlotGate::Nation(KEY_NATION_SCOTLAND), 0x7f5170, None), // ScotlandAwards
    (AwardSlotGate::Nation(KEY_NATION_SPAIN), 0x858ea0, None), // SpainAwards
    (AwardSlotGate::Nation(KEY_NATION_SWEDEN), 0x893720, None), // SwedenAwards
    (AwardSlotGate::Nation(KEY_NATION_UNITED_STATES), 0x906980, None), // UsaAwards
    (AwardSlotGate::Nation(KEY_NATION_POLAND), 0x7cd130, None), // PolandAwards
    (AwardSlotGate::Nation(KEY_NATION_GREECE), 0x5ec850, None), // GreeceAwards
    (AwardSlotGate::Nation(KEY_NATION_CROATIA), 0x517f10, None), // CroatiaAwards
    (AwardSlotGate::Nation(KEY_NATION_AUSTRALIA), 0x412540, None), // AustraliaAwards
    (AwardSlotGate::Nation(KEY_NATION_NORTHERN_IRELAND), 0x792d80, None), // NorthernIrelandAwards
    (AwardSlotGate::Nation(KEY_NATION_TURKEY), 0x901c70, None), // TurkeyAwards
    (AwardSlotGate::Nation(KEY_NATION_RUSSIA), 0x7ec210, None), // RussiaAwards
    (AwardSlotGate::Nation(KEY_NATION_WALES), 0x90ead0, None), // WalesAwards
    (AwardSlotGate::Nation(KEY_NATION_SOUTH_KOREA), 0x66e030, None), // KoreaAwards
];

/// Whether a slot of the award manager's table gets its object: always; Europe when the continent of
/// `KEY_CONTINENT_EUROPE` has a nation chosen to be played (`exe::key_nation_continent_has_selected_nation`); a nation
/// when its record (`G_NATION_TABLE` at the key's index, not checked) is chosen to be played (`.league_selected()`).
pub(crate) unsafe fn award_slot_open(gate: AwardSlotGate) -> bool {
    match gate {
        AwardSlotGate::Always => true,
        AwardSlotGate::Europe => {
            let europe = read_usize(G_CONTINENT_TABLE) + read_u32(KEY_CONTINENT_EUROPE) as usize * CONTINENT_SIZE;
            exe::key_nation_continent_has_selected_nation(europe) != 0
        }
        AwardSlotGate::Nation(key) => Nation(read_usize(G_NATION_TABLE) + read_u32(key) as usize * nation::SIZE).league_selected() != 0,
    }
}

/// Builds the award manager's table of award objects: the World Cup's and the international ones always, the
/// European ones with a European nation chosen, and those of each chosen nation, each built new or from the save.
///
/// `AWARD_MANAGER::create_nation_awards(file)`: the table (`NATION_AWARDS`) is a new zeroed block of 30 pointers
/// (`calloc`, not checked). Each slot of `AWARD_SLOTS` in order whose gate opens (`award_slot_open`) gets its object:
/// `operator new(0x17)` and the class's constructor with `file` (0 on a new game, the open "awards.dat" item on a load;
/// a failed allocation leaves 0). The international, Finnish, French and Irish awards use their old class when the
/// staff competition key of the new one is -1 (a database without those awards). A slot whose gate stays shut keeps
/// 0. Nothing is returned.
/// Original: `AWARD_MANAGER::create_nation_awards` (0x4135b0, award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn award_manager_create_nation_awards(this: usize, file: usize) {
    write_usize(this + award_manager_consts::NATION_AWARDS, exe::crt_calloc(award_manager_consts::NATION_AWARD_SLOTS as u32, 4));
    covered!("rxf_2 create the award objects");
    for (k, &(gate, ctor, old)) in AWARD_SLOTS.iter().enumerate() {
        if !award_slot_open(gate) {
            continue;
        }
        let ctor = match old {
            Some((key, old_ctor)) if read_i32(key) == -1 => old_ctor,
            _ => ctor,
        };
        let construct: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(ctor);
        let p = exe::crt_operator_new(0x17);
        let awards = if p != 0 { construct(p, file) } else { 0 };
        write_usize(read_usize(this + award_manager_consts::NATION_AWARDS) + 4 * k, awards);
    }
}

/// Comparer for `award_manager_create_nation_awards`: it allocates the table and every award object (and reads the
/// save item on a load), so it cannot run twice. Only the original runs; the copy is proven by the new career's .qst
/// ("awards.dat") and by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_award_manager_create_nation_awards(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, file)
}

/// `NationAwards::save(file)` (0x76a380, thiscall, ret 4) and `YEAR_RATINGS::save_to_disk(file)` (0x9314e0, thiscall,
/// ret 4).
pub(crate) const FN_NATION_AWARDS_SAVE: usize = 0x76a380;
pub(crate) const FN_YEAR_RATINGS_SAVE_TO_DISK: usize = 0x9314e0;

/// Writes the award manager to the save's "awards.dat": each award object, then the world and the selected nations'
/// year ratings.
///
/// `AWARD_MANAGER::save(save)`: no save: the award_manager.cpp "Error" box (line 0x246). The "awards.dat" item (`zopen`
/// with 1, 0x16, 4; missing: the box, line 0x24e). Each of the 30 award objects present (`NATION_AWARDS`) writes itself
/// (`NationAwards::save`), then the world year ratings (`AWARD_MANAGER_WORLD_YEAR_RATINGS`) and the selected nations'
/// ones (`AWARD_MANAGER_SELECTED_YEAR_RATINGS`) when present (`YEAR_RATINGS::save_to_disk`); their answers are not read.
/// The item closed. The original returns no value (eax is what the last call left); the caller (the save, 0x82ee6e)
/// does not read it.
/// Original: `AWARD_MANAGER::save` (0x4141a0, award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn award_manager_save(this: usize, save: usize) {
    if save == 0 {
        show_error_box(award_manager_consts::AWARD_MANAGER_CPP, 0x246);
        return;
    }
    let mut z = exe::zopen(save, 0x987e18usize, 1, 0x16, 4); // "awards.dat"
    if z == 0 {
        show_error_box(award_manager_consts::AWARD_MANAGER_CPP, 0x24e);
        return;
    }
    covered!("rxf_2 save awards.dat");
    let save_awards: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATION_AWARDS_SAVE);
    for k in 0..award_manager_consts::NATION_AWARD_SLOTS {
        let awards = read_usize(read_usize(this + award_manager_consts::NATION_AWARDS) + 4 * k);
        if awards != 0 {
            save_awards(awards, z);
        }
    }
    let save_ratings: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_YEAR_RATINGS_SAVE_TO_DISK);
    for field in [AWARD_MANAGER_WORLD_YEAR_RATINGS, AWARD_MANAGER_SELECTED_YEAR_RATINGS] {
        let ratings = read_usize(this + field);
        if ratings != 0 {
            save_ratings(ratings, z); // rxf_2: the year ratings
        }
    }
    exe::zclose(&mut z);
}

/// Comparer for `award_manager_save`: it writes into the save container, so it cannot run twice; only the original
/// runs. The copy is proven by the save proof (the .qst written with and without the batch, item by item).
pub(crate) unsafe extern "thiscall" fn compare_award_manager_save(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, save)
}
