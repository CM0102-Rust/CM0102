// ---------- sx10_2: the fog of war in the save: writing it and freeing it (fog_of_war.cpp) ----------

/// FOG_OF_WAR_MAP::save (thiscall(map, file) ret 4; its result is not used).
pub(crate) const FN_FOG_OF_WAR_MAP_SAVE: usize = 0x5b0170;

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
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, save)
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
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, file)
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
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
