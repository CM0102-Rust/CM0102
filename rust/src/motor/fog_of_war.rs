use super::*;

/// Computes a 1..20 value from the person's word +4 and `nivel`: x = (w + 2 x nivel) x 0.05 + 10, then (x + 0.1 x^2) /
/// 3 + 0.5, clamped to 1..20 and truncated [inference: how well the person's attribute is known, fog of war].
pub(crate) unsafe extern "C" fn chance_to_know(person: usize, level: i8) -> i32 {
    let x = (read_i16(person + 4) as f64 + (level as f64 + level as f64)) * 0.05 + 10.0;
    let y = (x * x * 0.1 + x) * read_f64(rdata::F64_0_3333333333) + 0.5;
    ftol(if !(y > 1.0) { 1.0 } else if !(y < 20.0) { 20.0 } else { y })
}

/// Fog of war: records knowledge of `pessoa` (9-byte records in [this], by id): stamps today's date at +1 and raises
/// the knowledge level (+0). Without a scout/observer (`fonte` +0x69) the level becomes 3 (fully known); with one,
/// below level 3 a roll on the observer's judging rating decides whether the level goes up by one, jumps to 2 or to 3.
pub(crate) unsafe extern "thiscall" fn know_person(this: usize, person: usize, font: usize) {
    let font = Staff(font);
    if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let reg = read_usize(this).wrapping_add((read_i32(person) as usize).wrapping_mul(9));
    let mut data = [0u8; 8];
    exe::date_operator_3d(reg + 1, data.as_mut_ptr() as usize, G_CURRENT_DATE);
    let obs = NonPlayer(if font.is_null() { 0 } else { font.non_player().addr() });
    let level = reg as *mut u8;
    if obs.is_null() {
        *level = 3;
        return;
    }
    if *level == 3 {
        return;
    }
    let c = exe::non_player_attribute_rating(obs, obs.judgement()) as i8;
    let r = random(c as i32) as i8;
    *level = if c < 15 && r == 0 {
        (*level).wrapping_add(1)
    } else if *level == 0 && r < 6 {
        2
    } else {
        3
    };
}

/// After a match (attribute masking / fog of war): for each side whose club is controlled by a human manager that has a
/// knowledge map, every valid player among the 20 lineup slots of the match data (substitutes only if used, +0x32 !=
/// 0xff) becomes known to that manager (`exe::fog_of_war_map_raise_knowledge`).
pub(crate) unsafe extern "stdcall" fn fog_after_match(fixture: usize, data: usize) {
    let n = || read_i32(G_STAFF_COUNT);
    for lado in 0..2usize {
        let club = Club(read_usize(fixture + if lado == 0 { 0x20 } else { 0x1c }));
        if exe::human_manager_is_human_club(club, 0, (1) as u32, (0) as u32, 0) == 0 {
            continue;
        }
        let q = club.manager().addr();
        if q == 0 {
            continue;
        }
        let k = n().wrapping_sub(read_i32(q)) as usize;
        let obj = read_usize(read_usize(G_FOG_OF_WAR).wrapping_add(k.wrapping_mul(4)).wrapping_sub(4));
        if obj == 0 {
            continue;
        }
        for vaga in 0..20usize {
            let e = MatchRecordRow(data + lado * match_record::SIDE_SIZE + match_record::ROW_STAFF_ID + vaga * match_record_row::SIZE);
            let i = read_i32(e.addr());
            if i == -1 || read_i32(e.addr() + 4) == -1 || (vaga > 10 && (e.minute_on() as u8) == 0xff) {
                continue;
            }
            let p = read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
            if p == 0 || read_i32(p) < n() {
                exe::fog_of_war_map_raise_knowledge(obj, p, 0);
            }
        }
    }
}

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

/// Comparer for `fog_of_war_daily_update`.
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_daily_update(this: usize) {
    let before: Vec<usize> = (0..16).map(|k| read_usize(this + k * 4)).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FOG_OF_WAR_DAILY_UPDATE)(this);
    let same = (0..16).all(|k| read_usize(this + k * 4) == before[k]);
    SCOREBOARD.record(same, || format!("fog_of_war_daily_update({:#x}): the maps changed", this));
}

/// FOG_OF_WAR_MAP::daily_update (thiscall, ret 0): one human manager's map's daily knowledge update.
pub(crate) const FN_FOG_OF_WAR_MAP_DAILY_UPDATE: usize = 0x5b0260;

/// Rule: the fog of war starts with no maps; with attribute masking on and a save to load, the maps come from the save,
/// and when that fails each human manager gets a fresh map.
///
/// `FOG_OF_WAR::FOG_OF_WAR(save)`: clears the 16 map slots. With attribute masking on (`G_ATTRIBUTE_MASKING_ENABLED`)
/// and a save, `FOG_OF_WAR::load(save)`; when it returns 0, each of the 16 people at the end of the table
/// (`G_STAFF_COUNT - k - 1`, k = 0..15) whose classification is 1 (a human manager) gets a map through
/// `FOG_OF_WAR::add_manager_map`. A new game passes no save and starts with no maps (the human managers get theirs when
/// they are added). Returns `this`.
/// Original: `FOG_OF_WAR::FOG_OF_WAR` (0x5af630, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_new(this: usize, save: usize) -> usize {
    for k in 0..FOG_MAP_SLOTS {
        write_usize(this + 4 * k, 0);
    }
    if read_u8(G_ATTRIBUTE_MASKING_ENABLED) == 0 || save == 0 {
        return this;
    }
    let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FOG_OF_WAR_LOAD);
    if load(this, save) != 0 { // sx10_1: the maps came from the save
        return this;
    }
    covered!("sx10_1 fresh maps after a failed load");
    let add: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FOG_OF_WAR_ADD_MANAGER_MAP);
    for k in 0..FOG_MAP_SLOTS {
        let person = fog_human_slot_person(k);
        if person.classification() == 1 {
            add(this, person.addr());
        }
    }
    this
}

/// Comparer for `fog_of_war_new`.
///
/// It reads the save item and allocates the maps: it cannot run twice. Only the original runs; the copy is proven by
/// the save proof (the holiday save loaded with and without the routines, levas/N/CARGA) and by the career, which
/// builds the object at the game creation.
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_new(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FOG_OF_WAR_NEW)(this, save)
}

/// Rule: reads the fog of war from the save; saves older than fog.dat give each human manager a fresh map.
///
/// `FOG_OF_WAR::load(save)`: with a data version (`G_GAME_DATA_VERSION`, absolute value) below 0x3e036, for each of the
/// 16 people at the end of the table that is a human manager (classification 1), a new map for that manager
/// (`new FOG_OF_WAR_MAP(manager)`) in the manager's slot, whatever attribute masking says; returns 1. Otherwise: no save:
/// the fog_of_war.cpp 0x99 box and 0. The "fog.dat" item of the save (`zopen` with 0, 0x16, 4; missing: the 0xa1 box and
/// 0). The masking flag (1 byte into `G_ATTRIBUTE_MASKING_ENABLED`; short: the 0xa7 box and 0). Masking off: the item
/// closed and 1. Masking on: for each slot a presence word (short: the 0xb8 box and 0); 0 empties the slot; otherwise a
/// new map without a manager goes in the slot (allocation failed: the 0xc4 box, every map freed as in the destructor,
/// and 0) and reads itself (`FOG_OF_WAR_MAP::load`; failed: the 0xd1 box, every map freed, the item closed and 0). Then
/// the item is closed and 1.
///
/// Defects of the original: the 0xa7, 0xb8 and 0xc4 failures return without closing the save item; freeing the maps
/// after a failure leaves the slots pointing at the freed maps; maps already in the slots are overwritten without being
/// freed.
/// Original: `FOG_OF_WAR::load` (0x5af6e0, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_load(this: usize, save: usize) -> u32 {
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() < FOG_DAT_VERSION {
        covered!("sx10_1 save without fog.dat");
        for k in 0..FOG_MAP_SLOTS {
            let person = fog_human_slot_person(k);
            if person.classification() != 1 || person.is_null() {
                continue;
            }
            let slot = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(person.addr())).wrapping_sub(1);
            write_usize(this.wrapping_add((slot as usize).wrapping_mul(4)), fog_map_new(person.addr()));
        }
        return 1;
    }
    if save == 0 {
        show_error_at(FOG_OF_WAR_CPP, 0x99);
        return 0;
    }
    let mut z = exe::zopen(save, FOG_DAT, 0, 0x16, 4);
    if z == 0 {
        show_error_at(FOG_OF_WAR_CPP, 0xa1);
        return 0;
    }
    if exe::zread(G_ATTRIBUTE_MASKING_ENABLED, 1, 1, z) != 1 {
        show_error_at(FOG_OF_WAR_CPP, 0xa7);
        return 0;
    }
    if read_u8(G_ATTRIBUTE_MASKING_ENABLED) == 0 {
        exe::zclose(&mut z);
        return 1; // sx10_1: masking off, nothing more in fog.dat
    }
    covered!("sx10_1 maps read from the save");
    let map_load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FOG_OF_WAR_MAP_LOAD);
    for k in 0..FOG_MAP_SLOTS {
        let slot = this + 4 * k;
        let mut present: u32 = 0;
        if exe::zread(&mut present as *mut u32 as usize, 4, 1, z) != 1 {
            show_error_at(FOG_OF_WAR_CPP, 0xb8);
            return 0;
        }
        if present == 0 {
            write_usize(slot, 0);
            continue;
        }
        let map = fog_map_new(0);
        write_usize(slot, map);
        if map == 0 {
            show_error_at(FOG_OF_WAR_CPP, 0xc4);
            fog_free_maps(this);
            return 0;
        }
        if map_load(map, z) == 0 {
            show_error_at(FOG_OF_WAR_CPP, 0xd1);
            fog_free_maps(this);
            exe::zclose(&mut z);
            return 0;
        }
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `fog_of_war_load`.
///
/// It reads the save item and allocates the maps: it cannot run twice. Only the original runs; the copy is proven by
/// loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_FOG_OF_WAR_LOAD)(this, save)
}

/// Rule: a map is read back from fog.dat in the order it was written: its manager, then the knowledge of every person.
///
/// `FOG_OF_WAR_MAP::load(file)`: the manager's id (4 bytes; short: the fog_of_war.cpp 0x230 box and 0); an id below 0
/// or above `G_STAFF_COUNT`: the 0x235 box and 0; otherwise the map's manager (+4) is that person. Then the knowledge
/// grid (+0): `G_STAFF_COUNT` entries of 9 bytes (the level and the date it was last raised); short: the 0x23e box and
/// 0. Returns 1.
///
/// Defect of the original: an id equal to `G_STAFF_COUNT` is accepted and points one record past the table.
/// Original: `FOG_OF_WAR_MAP::load` (0x5b0010, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_map_load(this: usize, file: usize) -> u32 {
    let mut owner: i32 = 0;
    if exe::zread(&mut owner as *mut i32 as usize, 4, 1, file) != 1 {
        show_error_at(FOG_OF_WAR_CPP, 0x230);
        return 0;
    }
    if owner < 0 || owner > read_i32(G_STAFF_COUNT) {
        show_error_at(FOG_OF_WAR_CPP, 0x235);
        return 0;
    }
    covered!("sx10_1 map read");
    write_usize(this + 4, staff_at(owner as isize).addr()); // +4: the map's manager
    let count = read_u32(G_STAFF_COUNT);
    if exe::zread(read_usize(this), 9, count, file) != read_u32(G_STAFF_COUNT) { // +0: the knowledge grid
        show_error_at(FOG_OF_WAR_CPP, 0x23e);
        return 0;
    }
    1
}

/// Comparer for `fog_of_war_map_load`.
///
/// It reads the open save item: it cannot run twice. Only the original runs; the copy is proven by loading a save made
/// with attribute masking on.
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_map_load(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_FOG_OF_WAR_MAP_LOAD)(this, file)
}

/// Rule: the fog of war goes into the save as the attribute masking flag and, when masking is on, a presence word and
/// the map of each of the 16 slots.
///
/// `FOG_OF_WAR::save(save)`: no save: the fog_of_war.cpp 0xf5 box. The "fog.dat" item of the save (`zopen` with 1,
/// 0x16, 4; none: the 0xfd box). The masking flag (`G_ATTRIBUTE_MASKING_ENABLED`, 1 byte; short: the 0x103 box).
/// Masking off: the item closed. Masking on: for each slot, a presence word 1 and the map (`FOG_OF_WAR_MAP::save`) when
/// it holds one, otherwise a word 0 (a short write: the 0x116 or 0x11f box); then the item is closed.
///
/// Defect of the original: after a failed write (0x103, 0x116, 0x11f) the item is left open.
/// Original: `FOG_OF_WAR::save` (0x5afad0, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_save(this: usize, save: usize) {
    let (present, absent): (u32, u32) = (1, 0);
    if save == 0 {
        show_error_at(FOG_OF_WAR_CPP, 0xf5);
        return;
    }
    let mut z = exe::zopen(save, FOG_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_at(FOG_OF_WAR_CPP, 0xfd);
        return;
    }
    if exe::zwrite(G_ATTRIBUTE_MASKING_ENABLED, 1, 1, z) != 1 { // sx10_2: the masking flag
        show_error_at(FOG_OF_WAR_CPP, 0x103);
        return;
    }
    if read_u8(G_ATTRIBUTE_MASKING_ENABLED) == 0 { // sx10_2: masking off, nothing more
        exe::zclose(&mut z);
        return;
    }
    covered!("sx10_2 maps written");
    let map_save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FOG_OF_WAR_MAP_SAVE);
    for k in 0..FOG_MAP_SLOTS {
        let map = read_usize(this + 4 * k);
        if map != 0 {
            if exe::zwrite(&present as *const u32 as usize, 4, 1, z) != 1 {
                show_error_at(FOG_OF_WAR_CPP, 0x116);
                return;
            }
            map_save(map, z);
        } else if exe::zwrite(&absent as *const u32 as usize, 4, 1, z) != 1 {
            show_error_at(FOG_OF_WAR_CPP, 0x11f);
            return;
        }
    }
    exe::zclose(&mut z);
}

/// Comparer for `fog_of_war_save`.
///
/// It writes the save item: it cannot run twice. Only the original runs; the copy is proven by the save proof (the
/// fog.dat item of the .qst written with and without the routines).
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_save(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FOG_OF_WAR_SAVE)(this, save)
}

/// Rule: a map goes into fog.dat as its manager's id and the knowledge of every person.
///
/// `FOG_OF_WAR_MAP::save(file)`: the id of the map's manager (+4; 4 bytes; short: the fog_of_war.cpp 0x254 box), then
/// the knowledge grid (+0): `G_STAFF_COUNT` entries of 9 bytes (short: the 0x25b box). The caller does not look at the
/// result.
/// Original: `FOG_OF_WAR_MAP::save` (0x5b0170, fog_of_war.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fog_of_war_map_save(this: usize, file: usize) {
    let owner: i32 = read_i32(read_usize(this + 4)); // +4: the map's manager
    if exe::zwrite(&owner as *const i32 as usize, 4, 1, file) != 1 {
        show_error_at(FOG_OF_WAR_CPP, 0x254);
        return;
    }
    covered!("sx10_2 map written");
    let count = read_u32(G_STAFF_COUNT);
    if exe::zwrite(read_usize(this), 9, count, file) != read_u32(G_STAFF_COUNT) { // +0: the knowledge grid
        show_error_at(FOG_OF_WAR_CPP, 0x25b);
    }
}

/// Comparer for `fog_of_war_map_save`.
///
/// It writes the open save item: it cannot run twice. Only the original runs; the copy is proven by saving a game with
/// attribute masking on.
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_map_save(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FOG_OF_WAR_MAP_SAVE)(this, file)
}

/// Rule: destroying the fog of war frees every map and its knowledge grid.
///
/// `FOG_OF_WAR::~FOG_OF_WAR()`: for each of the 16 slots that holds a map, its knowledge grid (+0) is freed and zeroed
/// and the map deleted (`fog_free_maps`). The slots are not cleared; the caller (free_game_data 0x82ffd4) deletes the
/// object right after.
/// Original: `FOG_OF_WAR::~FOG_OF_WAR` (0x5af6a0, fog_of_war.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fog_of_war_destructor(this: usize) {
    fog_free_maps(this);
}

/// Comparer for `fog_of_war_destructor`.
///
/// It frees the maps: it cannot run twice. Only the original runs; the copy runs when a game is freed (leaving it,
/// loading another save).
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_destructor(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FOG_OF_WAR_DESTRUCTOR)(this)
}

/// A new fog of war map for `manager` (0: none), as the original's `new FOG_OF_WAR_MAP(manager)`: 8 bytes from
/// `operator new` and the map constructor (FOG_OF_WAR_MAP::FOG_OF_WAR_MAP 0x5aff00, thiscall ret 4); 0 when the
/// allocation fails.
pub(crate) unsafe fn fog_map_new(manager: usize) -> usize {
    const FOG_OF_WAR_MAP_NEW: usize = 0x5aff00; // FOG_OF_WAR_MAP::FOG_OF_WAR_MAP, thiscall(map, manager) ret 4
    let map = exe::crt_operator_new(8);
    if map == 0 {
        return 0;
    }
    let new: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FOG_OF_WAR_MAP_NEW);
    new(map, manager)
}

/// The person at `k` places from the end of the people table, where the 16 human managers live.
pub(crate) unsafe fn fog_human_slot_person(k: usize) -> Staff {
    staff_at((read_i32(G_STAFF_COUNT) as isize).wrapping_sub(k as isize).wrapping_sub(1))
}

/// Frees every map of the fog of war: for each of the 16 slots that holds a map, its knowledge grid (+0, `free`, the
/// field zeroed) and the map itself (`operator delete`). The slots keep pointing at the freed maps. The body of the
/// destructor, also run inline by `FOG_OF_WAR::load` after a failure.
pub(crate) unsafe fn fog_free_maps(this: usize) {
    for k in 0..FOG_MAP_SLOTS {
        let map = read_usize(this + 4 * k);
        if map == 0 {
            continue;
        }
        let grid = read_usize(map);
        if grid != 0 {
            exe::crt_free(grid);
            write_usize(map, 0);
        }
        exe::crt_operator_delete(map);
    }
}

/// The source path of fog_of_war.cpp, for the error boxes ("E:\dev\CM3\cm3 00-01\cm3\code\fog_of_war.cpp").
pub(crate) const FOG_OF_WAR_CPP: usize = 0x9cc27c;

/// The name of the fog of war item in the save archive: "fog.dat".
pub(crate) const FOG_DAT: usize = 0x9cc274;

/// The fog of war object holds one map pointer per human manager: 16 slots, the manager with id `id` at slot
/// `G_STAFF_COUNT - id - 1` (the human managers are the last people of the table).
pub(crate) const FOG_MAP_SLOTS: usize = 16;

/// The first data version whose saves carry fog.dat; older saves get fresh maps for their human managers.
pub(crate) const FOG_DAT_VERSION: i32 = 0x3e036;

/// FOG_OF_WAR::load (thiscall(fog, save) ret 4, 1 on success).
pub(crate) const FN_FOG_OF_WAR_LOAD: usize = 0x5af6e0;

/// FOG_OF_WAR_MAP::load (thiscall(map, file) ret 4, 1 on success).
pub(crate) const FN_FOG_OF_WAR_MAP_LOAD: usize = 0x5b0010;

/// FOG_OF_WAR_MAP::save (thiscall(map, file) ret 4; its result is not used).
pub(crate) const FN_FOG_OF_WAR_MAP_SAVE: usize = 0x5b0170;
