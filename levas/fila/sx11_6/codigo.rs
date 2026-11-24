// ---------- batch %%LEVA%%: the game settings object, part 3 (game_config.cpp) ----------

/// `Config::~Config()`: a copy (with a source) does nothing; the live Config frees the background configuration tables,
/// stops, destroys and frees the sound player, shuts the display down and the language library.
/// Original: `Config::~Config` (0x5d7b70, game_config.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn config_destruct(this: usize) {
    if read_usize(this + config_offsets::SOURCE) != 0 {
        return;
    }
    config_call0(config_life_fns::BACKGROUND_FREE_CONFIG_TABLES);
    if read_usize(SOUND_PLAYER) != 0 {
        exe::soundplayer_stop_all(read_usize(SOUND_PLAYER));
        config_free_sound_player();
        write_usize(SOUND_PLAYER, 0);
    }
    config_call0(config_life_fns::GRAPHICS_SHUTDOWN);
    config_call0(config_life_fns::LANGLIB_SHUTDOWN);
}

/// `Config::set_defaults()`: the option word becomes 0x212552a9, then by the machine (`OS_GET_CPU_SPEED_CLASS` 0 fast
/// .. 2 slow, `OS_GET_TOTAL_PHYSICAL_MEMORY_MB`): option 1 when the speed class is not 2 and the memory is at least 63
/// MB, otherwise 2; speed 0: 0x20000 and 0x40000; otherwise 0x10000 and, speed 2, 0x80000, else 0x40000; then 0x1000000
/// with a printer installed, otherwise 0x2000000 (all through `Config::set_option`). Returns 1.
/// Original: `Config::set_defaults` (0x5d7bc0, game_config.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn config_set_defaults(this: usize) -> u32 {
    let speed = config_call0(config_life_fns::OS_GET_CPU_SPEED_CLASS) as u8;
    let memory = config_call0(config_life_fns::OS_GET_TOTAL_PHYSICAL_MEMORY_MB) as i32;
    write_u32(this + config_offsets::OPTIONS, config_life_fns::DEFAULT_OPTIONS);
    exe::config_set_option(this, if speed != 2 && memory >= 0x3f { 1 } else { 2 });
    if speed == 0 {
        exe::config_set_option(this, 0x20000);
        exe::config_set_option(this, 0x40000);
    } else {
        exe::config_set_option(this, 0x10000);
        exe::config_set_option(this, if speed == 2 { 0x80000 } else { 0x40000 });
    }
    let printer: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(config_life_fns::PRINTER_IS_INSTALLED);
    exe::config_set_option(this, if printer(config_life_fns::G_PRINTER) != 0 { 0x100_0000 } else { 0x200_0000 });
    1
}

/// Comparer for `config_destruct`: it shuts the display and the language down, so only the original runs; a live Config
/// leaves no sound player.
pub(crate) unsafe extern "thiscall" fn compare_config_destruct(this: usize) {
    let live = read_usize(this + config_offsets::SOURCE) == 0;
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    if live {
        SCOREBOARD.record(read_usize(SOUND_PLAYER) == 0, || format!("Config::~Config: sound player {:#x} left", read_usize(SOUND_PLAYER)));
    }
}

/// Comparer for `config_set_defaults`: side by side over the Config (the options it sets are not the sound's).
pub(crate) unsafe extern "thiscall" fn compare_config_set_defaults(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_config_side_by_side("Config::set_defaults", this, &[], || o(this), || config_set_defaults(this))
}
