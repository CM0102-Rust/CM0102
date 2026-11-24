// ---------- sx10_1: the fog of war in the save: building it and reading it (fog_of_war.cpp) ----------

/// The source path of fog_of_war.cpp, for the error boxes ("E:\dev\CM3\cm3 00-01\cm3\code\fog_of_war.cpp").
pub(crate) const FOG_OF_WAR_CPP: usize = 0x9cc27c;
/// The name of the fog of war item in the save archive: "fog.dat".
pub(crate) const FOG_DAT: usize = 0x9cc274;
/// The fog of war object holds one map pointer per human manager: 16 slots, the manager with id `id` at slot
/// `G_STAFF_COUNT - id - 1` (the human managers are the last people of the table).
pub(crate) const FOG_MAP_SLOTS: usize = 16;
/// The first data version whose saves carry fog.dat; older saves get fresh maps for their human managers.
const FOG_DAT_VERSION: i32 = 0x3e036;
/// FOG_OF_WAR::load (thiscall(fog, save) ret 4, 1 on success).
pub(crate) const FN_FOG_OF_WAR_LOAD: usize = 0x5af6e0;
/// FOG_OF_WAR::add_manager_map (thiscall(fog, manager) ret 4).
pub(crate) const FN_FOG_OF_WAR_ADD_MANAGER_MAP: usize = 0x5afd70;
/// FOG_OF_WAR_MAP::load (thiscall(map, file) ret 4, 1 on success).
pub(crate) const FN_FOG_OF_WAR_MAP_LOAD: usize = 0x5b0010;

/// A new fog of war map for `manager` (0: none), as the original's `new FOG_OF_WAR_MAP(manager)`: 8 bytes from
/// `operator new` and the map constructor (FOG_OF_WAR_MAP::FOG_OF_WAR_MAP 0x5aff00, thiscall ret 4); 0 when the
/// allocation fails.
unsafe fn fog_map_new(manager: usize) -> usize {
    const FOG_OF_WAR_MAP_NEW: usize = 0x5aff00; // FOG_OF_WAR_MAP::FOG_OF_WAR_MAP, thiscall(map, manager) ret 4
    let map = exe::crt_operator_new(8);
    if map == 0 {
        return 0;
    }
    let new: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FOG_OF_WAR_MAP_NEW);
    new(map, manager)
}

/// The person at `k` places from the end of the people table, where the 16 human managers live.
unsafe fn fog_human_slot_person(k: usize) -> Staff {
    staff_at((read_i32(G_STAFF_COUNT) as isize).wrapping_sub(k as isize).wrapping_sub(1))
}

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
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
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
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, save)
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
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, file)
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
