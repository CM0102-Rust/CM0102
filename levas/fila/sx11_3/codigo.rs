// ---------- batch %%LEVA%%: the game settings object, part 1 (game_config.cpp) ----------

/// The game settings object `Config` (0x31b bytes; the live one at `[G_CONFIG]`, copies made by the Game Settings box).
#[allow(dead_code)]
pub(crate) mod config_offsets {
    /// The live Config a copy was made from (0 in the live one).
    pub(crate) const SOURCE: usize = 0x0;
    /// The option bits, in groups of which `config_set_option` keeps one bit each (display, sound, backgrounds, ...).
    pub(crate) const OPTIONS: usize = 0x4;
    pub(crate) const LANGUAGE: usize = 0x8;
    /// The currency index 0..25 (byte).
    pub(crate) const CURRENCY: usize = 0xc;
    /// The auto save interval in days, 7..365 (word).
    pub(crate) const AUTO_SAVE_DAYS: usize = 0xd;
    /// The data folder path (0x104 bytes) [inference: the user folder of the game data].
    pub(crate) const DATA_FOLDER: usize = 0xf;
    /// 1 once game.cfg was read or written.
    pub(crate) const SAVED: usize = 0x213;
    /// The font index (-2: the default font).
    pub(crate) const FONT_INDEX: usize = 0x217;
    /// The font name (0x100 bytes).
    pub(crate) const FONT_NAME: usize = 0x21b;
    pub(crate) const SIZE: usize = 0x31b;
}

/// The allowed fonts table read from allowed_fonts.txt: 10 entries of 0x68 bytes (+0 language, −1 unused; +4 the font
/// name, 100 bytes).
#[allow(dead_code)]
pub(crate) mod allowed_fonts {
    pub(crate) const TABLE: usize = 0xae2d28;
    pub(crate) const COUNT: usize = 10;
    pub(crate) const ENTRY: usize = 0x68;
    pub(crate) const NAME: usize = 0x4;
}

/// Exe routines and texts of game_config.cpp without an `exe::` wrapper (cm0102.exe 3.9.68).
#[allow(dead_code)]
pub(crate) mod config_fns {
    pub(crate) const SET_DEFAULT_CURRENCY: usize = 0x5d8310; // Config::set_default_currency thiscall(this)
    pub(crate) const FONT_GET_NAME: usize = 0x5b2040; // cdecl(index, out, size): the font file's name
    pub(crate) const FONT_GET_COUNT: usize = 0x5b1fc0; // cdecl(): the number of font files
    pub(crate) const FONT_SELECT: usize = 0x5e4a60; // cdecl(index): makes the font the display's
    pub(crate) const LANGLIB_LOAD_LANGUAGE: usize = 0x66eb40; // cdecl(language, flag)
    pub(crate) const SOUND_PLAYER_CONSTRUCT: usize = 0x84ca80; // SoundPlayer::SoundPlayer thiscall(this)
    pub(crate) const SOUND_PLAYER_DESTRUCT: usize = 0x84caf0; // SoundPlayer::~SoundPlayer thiscall(this)
    pub(crate) const SOUND_PLAYER_DEVICE: usize = 0x2c; // the SoundPlayer's DirectSound device (0: no sound)
    pub(crate) const GAME_CONFIG_CPP: usize = 0x9cd3a0; // "E:\dev\CM3\cm3 00-01\cm3\code\game_config.cpp"
    pub(crate) const TEXT_ERROR: usize = 0x9870e0; // "Error"
    pub(crate) const TEXT_NO_SOUND: usize = 0x9cd2cc; // "Unable to initialise the sound - please ensure that DirectX ..."
    pub(crate) const TEXT_CM3_SOUNDS: usize = 0x9c382c; // "CM3_SOUNDS"
    pub(crate) const TEXT_INTRO2_WAV: usize = 0x9cd25c; // "intro2.wav"
    /// The lowercased command line (`game_init_from_command_line`; 100 bytes up to the game phase byte 0xae2cfc).
    pub(crate) const G_COMMAND_LINE: usize = 0xae2c98;
}

/// The font file name of font `index` into `out` (`size` bytes).
unsafe fn config_font_name(index: i32, out: usize, size: u32) -> usize {
    let f: unsafe extern "C" fn(i32, usize, u32) -> usize = core::mem::transmute(config_fns::FONT_GET_NAME);
    f(index, out, size)
}

/// Whether the allowed fonts table has an entry for the language.
unsafe fn language_has_font_rules(language: i32) -> bool {
    (0..allowed_fonts::COUNT).any(|k| read_i32(allowed_fonts::TABLE + k * allowed_fonts::ENTRY) == language)
}

/// `is_font_allowed_for_language(language, name)`: 1 when the allowed fonts table has no entry for the language, or
/// when one of the language's entries names the font `name` (exact text); otherwise 0.
/// Original: `is_font_allowed_for_language` (0x5d8b80, game_config.cpp, cdecl).
pub(crate) unsafe extern "C" fn is_font_allowed_for_language(language: i32, name: usize) -> u32 {
    if !language_has_font_rules(language) {
        return 1;
    }
    let e = |k: usize| allowed_fonts::TABLE + k * allowed_fonts::ENTRY;
    (0..allowed_fonts::COUNT).any(|k| read_i32(e(k)) == language && guio_same_c_string(e(k) + allowed_fonts::NAME, name)) as u32
}

/// The first of the first 20 fonts the language allows, −2 (the default font) when the language has no font rules or
/// none of them is allowed. The original repeats this loop inline in `Config::Config` (twice) and `set_language`.
unsafe fn config_first_allowed_font(language: i32) -> i32 {
    if !language_has_font_rules(language) {
        return -2;
    }
    let count: unsafe extern "C" fn() -> i32 = core::mem::transmute(config_fns::FONT_GET_COUNT);
    let n = count().min(20);
    let mut name = [0u8; 100];
    for k in 0..n {
        config_font_name(k, name.as_mut_ptr() as usize, 100);
        if is_font_allowed_for_language(language, name.as_ptr() as usize) != 0 {
            return k;
        }
    }
    -2
}

/// The font that the Config gets when its font is not allowed: index, name (0x100 bytes) and the display's font.
unsafe fn config_switch_font(this: usize, index: i32) {
    write_i32(this + config_offsets::FONT_INDEX, index);
    config_font_name(index, this + config_offsets::FONT_NAME, 0x100);
    let select: unsafe extern "C" fn(i32) = core::mem::transmute(config_fns::FONT_SELECT);
    select(index);
}

/// Destroys and frees the sound player, if there is one, and clears `SOUND_PLAYER`.
unsafe fn config_free_sound_player() {
    let p = read_usize(SOUND_PLAYER);
    if p != 0 {
        let d: unsafe extern "thiscall" fn(usize) = core::mem::transmute(config_fns::SOUND_PLAYER_DESTRUCT);
        d(p);
        exe::crt_operator_delete(p);
        write_usize(SOUND_PLAYER, 0);
    }
}

/// Creates the sound player (`operator new` 0x30, `SoundPlayer::SoundPlayer`, into `SOUND_PLAYER`). With a DirectSound
/// device, `play_intro` plays intro2.wav (CM3_SOUNDS, slot 2); without one, the "Error" box, the option back to 0x100
/// (sound off, through `Config::set_option`) and the player destroyed and freed.
unsafe fn config_start_sound(this: usize, play_intro: bool) {
    let p = exe::crt_operator_new(0x30);
    let p = if p != 0 {
        let c: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(config_fns::SOUND_PLAYER_CONSTRUCT);
        c(p)
    } else {
        0
    };
    write_usize(SOUND_PLAYER, p);
    if p != 0 && read_usize(p + config_fns::SOUND_PLAYER_DEVICE) != 0 {
        if play_intro {
            covered!("sx11_3 sound on");
            let path = exe::utils_build_file_path(config_fns::TEXT_CM3_SOUNDS, config_fns::TEXT_INTRO2_WAV);
            exe::soundplayer_play_file(read_usize(SOUND_PLAYER), 2, path, 0);
        }
        return;
    }
    exe::gui_show_message_box(config_fns::TEXT_ERROR, config_fns::TEXT_NO_SOUND, 0);
    exe::config_set_option(this, 0x100);
    config_free_sound_player();
}

/// The sound player of the option bits `options` just set: sound on (bit 0x80) without a player starts one
/// (`config_start_sound`); sound off (bit 0x100) with a player stops it (`exe::soundplayer_stop_all`), destroys it and
/// frees it.
unsafe fn config_switch_sound(this: usize, options: u32, play_intro: bool) {
    if options & 0x80 != 0 && read_usize(SOUND_PLAYER) == 0 {
        config_start_sound(this, play_intro);
    } else if options & 0x100 != 0 && read_usize(SOUND_PLAYER) != 0 {
        covered!("sx11_3 sound off");
        exe::soundplayer_stop_all(read_usize(SOUND_PLAYER));
        config_free_sound_player();
        write_usize(SOUND_PLAYER, 0);
    }
}

/// The bit group of option `value` (the bits `config_set_option` clears before setting `value`), None for a value
/// that is not an option.
fn config_option_group(value: u32) -> Option<u32> {
    Some(match value {
        1 | 2 | 4 => 0x7,
        8 | 0x10 => 0x18,
        0x20 | 0x40 => 0x60,
        0x80 | 0x100 => 0x180,
        0x200 | 0x400 | 0x800 => 0xe00,
        0x1000 | 0x2000 => 0x3000,
        0x4000 | 0x8000 => 0xc000,
        0x10000 | 0x20000 => 0x30000,
        0x40000 | 0x80000 | 0x100000 => 0x1c_0000,
        0x200000 | 0x400000 | 0x800000 => 0xe0_0000,
        0x100_0000 | 0x200_0000 | 0x400_0000 => 0x700_0000,
        0x2000_0000 | 0x4000_0000 => 0x6000_0000,
        _ => return None,
    })
}

/// `Config::Config(source)`, the copy constructor the Game Settings box uses: keeps `source` at
/// `config_offsets::SOURCE`, copies the options, language, currency and auto save interval, the data folder (0x103
/// bytes, `strncpy`), the save name (`config::SAVE_NAME`, 0xff bytes), the saved flag, the font index and the font name
/// (0xff bytes), each text cut by a terminator at its end. Returns `this`.
/// Original: `Config::Config` (0x5d7ad0, game_config.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn config_construct_copy(this: usize, source: usize) -> usize {
    use config_offsets as c;
    write_usize(this + c::SOURCE, source);
    write_u32(this + c::OPTIONS, read_u32(source + c::OPTIONS));
    write_u32(this + c::LANGUAGE, read_u32(source + c::LANGUAGE));
    write_u8(this + c::CURRENCY, read_u8(source + c::CURRENCY));
    write_u16(this + c::AUTO_SAVE_DAYS, read_u16(source + c::AUTO_SAVE_DAYS));
    exe::at_9466d0(this + c::DATA_FOLDER, (source + c::DATA_FOLDER) as *const u8, 0x103); // strncpy
    write_u8(this + 0x112, 0); // the data folder's last byte
    exe::at_9466d0(this + config::SAVE_NAME, (source + config::SAVE_NAME) as *const u8, 0xff);
    write_u8(this + 0x212, 0); // the save name's last byte
    write_u32(this + c::SAVED, read_u32(source + c::SAVED));
    write_u32(this + c::FONT_INDEX, read_u32(source + c::FONT_INDEX));
    exe::at_9466d0(this + c::FONT_NAME, (source + c::FONT_NAME) as *const u8, 0xff);
    write_u8(this + 0x31a, 0); // the font name's last byte
    this
}

/// `Config::set_option(value)`: sets one option bit, clearing the other bits of its group in the option word
/// (`config_option_group`: 1/2/4, 8/0x10 windowed or full screen, 0x20/0x40 backgrounds, 0x80/0x100 sound on or off,
/// 0x200/0x400/0x800, 0x1000/0x2000 auto save, 0x4000/0x8000, 0x10000/0x20000, 0x40000/0x80000/0x100000,
/// 0x200000/0x400000/0x800000, 0x1000000/0x2000000/0x4000000 printing, 0x20000000/0x40000000); the sound group then
/// starts or stops the sound player (`config_switch_sound`). Any other value: the game_config.cpp line 0x255 "Error" box
/// (`show_error_box`) and 0. Returns 1.
/// Original: `Config::set_option` (0x5d7c60, game_config.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn config_set_option(this: usize, value: u32) -> u32 {
    let Some(group) = config_option_group(value) else {
        show_error_box(config_fns::GAME_CONFIG_CPP, 0x255);
        return 0;
    };
    let options = read_u32(this + config_offsets::OPTIONS) & !group | value;
    write_u32(this + config_offsets::OPTIONS, options);
    if group == 0x180 {
        config_switch_sound(this, options, true);
    }
    1
}

/// `Config::set_language(language)`: a language the language library does not have (`exe::langlib_is_language_available`)
/// returns 0. Otherwise the Config takes the language and its default currency (`Config::set_default_currency`); when
/// the current font (`FONT_INDEX`) is not allowed for the language, the first allowed font of the first 20 (−2 when
/// none) becomes the font (`config_switch_font`); then the language's texts are loaded (`LANGLIB_LOAD_LANGUAGE`(language,
/// 1)) and the screen redrawn (`exe::screenmanager_redraw_screen`(1, 1)). Returns 1.
/// Original: `Config::set_language` (0x5d8020, game_config.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn config_set_language(this: usize, language: i32) -> u32 {
    if exe::langlib_is_language_available(language as u32) == 0 {
        return 0;
    }
    write_i32(this + config_offsets::LANGUAGE, language);
    let currency: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(config_fns::SET_DEFAULT_CURRENCY);
    currency(this);
    let mut name = [0u8; 100];
    config_font_name(read_i32(this + config_offsets::FONT_INDEX), name.as_mut_ptr() as usize, 100);
    if is_font_allowed_for_language(language, name.as_ptr() as usize) == 0 {
        covered!("sx11_3 language changes the font");
        config_switch_font(this, config_first_allowed_font(language));
    }
    let load: unsafe extern "C" fn(i32, u32) = core::mem::transmute(config_fns::LANGLIB_LOAD_LANGUAGE);
    load(read_i32(this + config_offsets::LANGUAGE), 1);
    exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 1, 1);
    1
}

/// `Config::set_currency(currency)`: a currency 0..25 goes to `CURRENCY` and returns 1; another value shows the
/// game_config.cpp line 0x293 "Error" box and returns 0.
/// Original: `Config::set_currency` (0x5d8270, game_config.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn config_set_currency(this: usize, currency: u32) -> u32 {
    let c = currency as u8 as i8;
    if !(0..0x1a).contains(&c) {
        show_error_box(config_fns::GAME_CONFIG_CPP, 0x293);
        return 0;
    }
    write_u8(this + config_offsets::CURRENCY, c as u8);
    1
}

/// The default currency of a language: 1 → 0x18, 2..5, 7 and 9 → 7, 6 → 6, 8 → 0x10, 10 → 0x15, 11 → 0xf, 12 → 0x19,
/// any other (0 included) → 0.
fn default_currency_of(language: i32) -> u8 {
    match language {
        1 => 0x18,
        2..=5 | 7 | 9 => 7,
        6 => 6,
        8 => 0x10,
        10 => 0x15,
        11 => 0xf,
        12 => 0x19,
        _ => 0,
    }
}

/// `Config::set_default_currency()`: `CURRENCY` becomes the default currency of the Config's language
/// (`default_currency_of`). Returns 1.
/// Original: `Config::set_default_currency` (0x5d8310, game_config.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn config_set_default_currency(this: usize) -> u32 {
    write_u8(this + config_offsets::CURRENCY, default_currency_of(read_i32(this + config_offsets::LANGUAGE)));
    1
}

/// `Config::set_auto_save_interval(days)`: 7 to 365 days go to `AUTO_SAVE_DAYS` and return 1; another value shows the
/// game_config.cpp line 0x2d5 "Error" box and returns 0.
/// Original: `Config::set_auto_save_interval` (0x5d83e0, game_config.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn config_set_auto_save_interval(this: usize, days: u32) -> u32 {
    let d = days as u16 as i16;
    if !(7..=0x16d).contains(&d) {
        show_error_box(config_fns::GAME_CONFIG_CPP, 0x2d5);
        return 0;
    }
    write_i16(this + config_offsets::AUTO_SAVE_DAYS, d);
    1
}

/// `Config::get_font_index()`: the font index (`FONT_INDEX`).
/// Original: `Config::get_font_index` (0x5d8b40, game_config.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn config_get_font_index(this: usize) -> u32 {
    read_u32(this + config_offsets::FONT_INDEX)
}

/// `Config::set_font(index)`: the font index and its file name (0x100 bytes) in the Config. Returns what the name
/// routine returns.
/// Original: `Config::set_font` (0x5d8b50, game_config.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn config_set_font(this: usize, index: i32) -> usize {
    write_i32(this + config_offsets::FONT_INDEX, index);
    config_font_name(index, this + config_offsets::FONT_NAME, 0x100)
}

/// Side by side over a Config object (and what `extra` adds), with the routine's name in the message.
unsafe fn compare_config_side_by_side<R: PartialEq + core::fmt::Debug>(name: &str, this: usize, extra: &[(usize, usize)], o: impl FnOnce() -> R, n: impl FnOnce() -> R) -> R {
    let mut regions = vec![(this, config_offsets::SIZE)];
    regions.extend_from_slice(extra);
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&regions, o, n);
    SCOREBOARD.record(equal && r == c, || format!("{}: original={:?} rust={:?} differences at {:?}", name, r, c, first_differences(&mo, &mr)));
    r
}

/// Comparer for `config_construct_copy`: side by side over the new object.
pub(crate) unsafe extern "thiscall" fn compare_config_construct_copy(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    compare_config_side_by_side("Config::Config (copy)", this, &[], || o(this, source), || config_construct_copy(this, source))
}

/// Comparer for `config_set_option`: side by side over the Config, except the sound group (it creates, plays or frees
/// the sound player) and a value that is not an option (the error box), which run only in the original.
pub(crate) unsafe extern "thiscall" fn compare_config_set_option(this: usize, value: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%);
    match config_option_group(value) {
        Some(g) if g != 0x180 => compare_config_side_by_side(&format!("Config::set_option({:#x})", value), this, &[], || o(this, value), || config_set_option(this, value)),
        _ => o(this, value),
    }
}

/// Comparer for `config_set_language`: it loads the language's texts and redraws the screen, so only the original runs;
/// then the plan: the language, its default currency, and a font allowed for it (or −2).
pub(crate) unsafe extern "thiscall" fn compare_config_set_language(this: usize, language: i32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(%%I2%%)(this, language);
    if r != 0 {
        let mut name = [0u8; 0x100];
        let font = read_i32(this + config_offsets::FONT_INDEX);
        config_font_name(font, name.as_mut_ptr() as usize, 0x100);
        let ok = read_i32(this + config_offsets::LANGUAGE) == language
            && read_u8(this + config_offsets::CURRENCY) == default_currency_of(language)
            && (font == -2 || is_font_allowed_for_language(language, name.as_ptr() as usize) != 0);
        SCOREBOARD.record(ok, || format!("Config::set_language({}): language {}, currency {}, font {}", language,
            read_i32(this + config_offsets::LANGUAGE), read_u8(this + config_offsets::CURRENCY), font));
    }
    r
}

/// Comparer for `config_set_currency`: side by side over the Config; an invalid currency (the error box) only in the
/// original.
pub(crate) unsafe extern "thiscall" fn compare_config_set_currency(this: usize, currency: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I3%%);
    if !(0..0x1a).contains(&(currency as u8 as i8)) {
        return o(this, currency);
    }
    compare_config_side_by_side(&format!("Config::set_currency({})", currency as u8 as i8), this, &[], || o(this, currency), || config_set_currency(this, currency))
}

/// Comparer for `config_set_default_currency`: side by side over the Config.
pub(crate) unsafe extern "thiscall" fn compare_config_set_default_currency(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I4%%);
    compare_config_side_by_side("Config::set_default_currency", this, &[], || o(this), || config_set_default_currency(this))
}

/// Comparer for `config_set_auto_save_interval`: side by side over the Config; an invalid interval (the error box) only
/// in the original.
pub(crate) unsafe extern "thiscall" fn compare_config_set_auto_save_interval(this: usize, days: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I5%%);
    if !(7..=0x16d).contains(&(days as u16 as i16)) {
        return o(this, days);
    }
    compare_config_side_by_side(&format!("Config::set_auto_save_interval({})", days as u16 as i16), this, &[], || o(this, days), || config_set_auto_save_interval(this, days))
}

/// Comparer for `config_get_font_index`.
pub(crate) unsafe extern "thiscall" fn compare_config_get_font_index(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I6%%)(this);
    let n = config_get_font_index(this);
    SCOREBOARD.record(r == n, || format!("Config::get_font_index original={} rust={}", r as i32, n as i32));
    r
}

/// Comparer for `config_set_font`: side by side over the Config (the returned pointer is not compared).
pub(crate) unsafe extern "thiscall" fn compare_config_set_font(this: usize, index: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(%%I7%%);
    let mut kept = 0usize;
    compare_config_side_by_side(&format!("Config::set_font({})", index), this, &[], || { kept = o(this, index); 0u8 }, || { config_set_font(this, index); 0u8 });
    kept
}

/// Comparer for `is_font_allowed_for_language`.
pub(crate) unsafe extern "C" fn compare_is_font_allowed_for_language(language: i32, name: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(i32, usize) -> u32>(%%I8%%)(language, name);
    let n = is_font_allowed_for_language(language, name);
    SCOREBOARD.record(r == n, || format!("is_font_allowed_for_language({}, {:?}) original={} rust={}", language,
        (0..100).map(|k| read_u8(name + k)).take_while(|&b| b != 0).map(|b| b as char).collect::<String>(), r, n));
    r
}
