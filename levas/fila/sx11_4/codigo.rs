// ---------- batch %%LEVA%%: the game settings object, part 2 (game_config.cpp) ----------

/// Exe routines and texts of the Config's life (cm0102.exe 3.9.68), besides `config_fns`.
#[allow(dead_code)]
pub(crate) mod config_life_fns {
    pub(crate) const SET_DEFAULTS: usize = 0x5d7bc0; // Config::set_defaults thiscall(this)
    pub(crate) const APPLY_AND_SAVE: usize = 0x5d8480; // Config::apply_and_save thiscall(this) -> 0 or 1
    pub(crate) const LANGLIB_SETUP: usize = 0x66e790; // cdecl(path, flag) -> 0 on failure
    pub(crate) const LANGLIB_SHUTDOWN: usize = 0x66e9c0; // cdecl()
    pub(crate) const DATABASE_GET_LANGUAGE: usize = 0x535920; // cdecl(): the language of the database names
    pub(crate) const DATABASE_LOAD_LANGUAGE_NAMES: usize = 0x5339a0; // cdecl(language, flag)
    pub(crate) const CASH_GET_CURRENCY: usize = 0x4415e0; // cdecl() -> al
    pub(crate) const CASH_SET_CURRENCY: usize = 0x43f7f0; // cdecl(currency byte)
    pub(crate) const UTILS_GET_USER_FOLDER: usize = 0x90bea0; // cdecl(folder name, out)
    pub(crate) const GRAPHICS_INIT_DISPLAY: usize = 0x5e2620; // cdecl(width, height, 2, font, full screen, title) -> 0 on failure
    pub(crate) const GRAPHICS_IS_FULL_SCREEN: usize = 0x5e2ea0; // cdecl() [inference: compared with the option bits 8 and 0x10]
    pub(crate) const GRAPHICS_SHUTDOWN: usize = 0x5e2cc0; // cdecl()
    pub(crate) const SETUP_BACKGROUND_SYSTEM: usize = 0x41b3d0; // cdecl()
    pub(crate) const BACKGROUND_FREE_CONFIG_TABLES: usize = 0x41bad0; // cdecl()
    pub(crate) const BACKGROUND_SET_PICTURES_DISABLED: usize = 0x41bf80; // cdecl(flag)
    pub(crate) const BACKGROUND_PICTURES_DISABLED: usize = 0x41bfa0; // cdecl() -> the flag
    pub(crate) const OS_GET_CPU_SPEED_CLASS: usize = 0x7a1750; // cdecl() -> al: 0 fast .. 2 slow
    pub(crate) const OS_GET_TOTAL_PHYSICAL_MEMORY_MB: usize = 0x7a16d0; // cdecl()
    pub(crate) const PRINTER_IS_INSTALLED: usize = 0x84c7f0; // thiscall(G_PRINTER) -> al
    pub(crate) const G_PRINTER: usize = 0xdd7ee8; // the printer object the default options ask
    pub(crate) const CRT_SSCANF: usize = 0x946fd8; // C runtime: sscanf
    pub(crate) const DEFAULT_OPTIONS: u32 = 0x212552a9;
    pub(crate) const GAME_CFG_MAGIC: u32 = 0x1553117c;
    pub(crate) const TEXT_CM3_DATA: usize = 0x9b5348; // "CM3_DATA"
    pub(crate) const TEXT_LANGUAGE_LDB: usize = 0x9cd3d0; // "language.ldb"
    pub(crate) const TEXT_ALLOWED_FONTS_TXT: usize = 0x9cd38c; // "allowed_fonts.txt"
    pub(crate) const TEXT_MODE_R: usize = 0x9899fc; // "r"
    pub(crate) const TEXT_FONT_LINE: usize = 0x9cd384; // "%d, %s"
    pub(crate) const TEXT_DEFAULT_FOLDER: usize = 0xad9c64; // the user folder name passed to utils_get_user_folder
    pub(crate) const TEXT_CM3_PREFS: usize = 0x9cd36c; // "CM3_PREFS"
    pub(crate) const TEXT_GAME_CFG: usize = 0x9cd378; // "game.cfg"
    pub(crate) const TEXT_MODE_RB: usize = 0x98f928; // "rb"
    pub(crate) const TEXT_MODE_WB: usize = 0x9a7f28; // "wb"
    pub(crate) const TEXT_WINDOWED: usize = 0x9cd360; // "-windowed"
    pub(crate) const TEXT_NOSOUND: usize = 0x9cd354; // "-nosound"
    pub(crate) const TEXT_DEFAULTFONT: usize = 0x9cd344; // "-defaultfont"
    pub(crate) const TEXT_TITLE: usize = 0x9cd328; // "Championship Manager 01/02"
}

/// A cdecl routine of the exe with no argument.
unsafe fn config_call0(f: usize) -> u32 {
    let g: unsafe extern "C" fn() -> u32 = core::mem::transmute(f);
    g()
}

/// A cdecl routine of the exe with one argument.
unsafe fn config_call1(f: usize, a: u32) -> u32 {
    let g: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(f);
    g(a)
}

/// `Config::set_default_currency` through the exe (its detour).
unsafe fn config_default_currency_now(this: usize) {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(config_fns::SET_DEFAULT_CURRENCY);
    f(this);
}

/// Opens the display: 800 × 600, mode 2, the font, full screen or not, the window title.
unsafe fn config_open_display(font: i32, full_screen: u32) -> u32 {
    let f: unsafe extern "C" fn(u32, u32, u32, i32, u32, usize) -> u32 = core::mem::transmute(config_life_fns::GRAPHICS_INIT_DISPLAY);
    f(0x320, 0x258, 2, font, full_screen, config_life_fns::TEXT_TITLE)
}

/// The fatal exit of game_config.cpp: `game_fatal_cleanup`, the shared empty routine 0x5a8f60 and `exit(code)`.
unsafe fn config_fatal_exit(code: i32) {
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(code);
}

/// The allowed fonts table from allowed_fonts.txt ("r"): every entry unused (language −1), then each line, trimmed
/// (`exe::utils_trim_control_chars`), that reads as "%d, %s" fills the next entry (language and font name), up to 10.
/// The original never closes the file.
unsafe fn config_read_allowed_fonts() {
    for k in 0..allowed_fonts::COUNT {
        write_i32(allowed_fonts::TABLE + k * allowed_fonts::ENTRY, -1);
    }
    let f = exe::crt_fopen(config_life_fns::TEXT_ALLOWED_FONTS_TXT, config_life_fns::TEXT_MODE_R);
    if f == 0 {
        return;
    }
    let sscanf: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(config_life_fns::CRT_SSCANF);
    let (mut line, mut name, mut language) = ([0u8; 100], [0u8; 100], 0i32);
    let mut entry = 0usize;
    while exe::at_94700c(line.as_mut_ptr() as usize, 100, f) != 0 && entry < allowed_fonts::COUNT {
        exe::utils_trim_control_chars(line.as_mut_ptr());
        if sscanf(line.as_ptr(), config_life_fns::TEXT_FONT_LINE, &mut language as *mut i32, name.as_mut_ptr()) == 2 {
            let e = allowed_fonts::TABLE + entry * allowed_fonts::ENTRY;
            write_i32(e, language);
            copy_c_string(e + allowed_fonts::NAME, name.as_ptr() as usize);
            entry += 1;
        }
    }
    // the original leaves the file open (DEFEITOS: a handle lost per program start)
}

/// Reads game.cfg (CM3_PREFS, "rb") into the Config: magic 0x1553117c, options, language (taken, with its texts, the
/// database names and the default currency, when the library has it), currency (taken and made current when 0..25),
/// auto save interval, data folder (taken when not empty and an existing folder), font index and font name; a font name
/// that is not the index's own makes the index the language's first allowed font (the name stays). Returns whether
/// everything was read; a short or foreign file is deleted by the caller. None when the file does not open.
unsafe fn config_read_game_cfg(this: usize) -> Option<bool> {
    use config_offsets as c;
    let mut path = [0u8; 0x104];
    copy_c_string(path.as_mut_ptr() as usize, exe::utils_build_file_path(config_life_fns::TEXT_CM3_PREFS, config_life_fns::TEXT_GAME_CFG));
    let f = exe::crt_fopen(path.as_ptr() as usize, config_life_fns::TEXT_MODE_RB);
    if f == 0 {
        return None;
    }
    let read = |p: usize, n: u32| exe::crt_fread(p, n, 1, f) == 1;
    let ok = 'read: {
        let mut magic = 0u32;
        if !read(&mut magic as *mut u32 as usize, 4) || magic != config_life_fns::GAME_CFG_MAGIC {
            break 'read false;
        }
        let mut language = 0i32;
        if !read(this + c::OPTIONS, 4) || !read(&mut language as *mut i32 as usize, 4) {
            break 'read false;
        }
        if exe::langlib_is_language_available(language as u32) != 0 {
            write_i32(this + c::LANGUAGE, language);
            let load: unsafe extern "C" fn(i32, u32) = core::mem::transmute(config_fns::LANGLIB_LOAD_LANGUAGE);
            load(language, 0);
            let names: unsafe extern "C" fn(i32, u32) = core::mem::transmute(config_life_fns::DATABASE_LOAD_LANGUAGE_NAMES);
            names(read_i32(this + c::LANGUAGE), 0);
            config_default_currency_now(this);
        }
        let mut currency = 0u8;
        if !read(&mut currency as *mut u8 as usize, 1) {
            break 'read false;
        }
        if (0..0x1a).contains(&(currency as i8)) {
            write_u8(this + c::CURRENCY, currency);
            config_call1(config_life_fns::CASH_SET_CURRENCY, currency as u32);
        }
        let mut folder = [0u8; 0x104];
        if !read(this + c::AUTO_SAVE_DAYS, 2) || !read(folder.as_mut_ptr() as usize, 0x104) {
            break 'read false;
        }
        if folder[0] != 0 && exe::utils_is_directory(folder.as_ptr() as usize) != 0 {
            copy_c_string(this + c::DATA_FOLDER, folder.as_ptr() as usize);
        }
        if !read(this + c::FONT_INDEX, 4) || !read(this + c::FONT_NAME, 0x100) {
            break 'read false;
        }
        let mut name = [0u8; 0x100];
        config_font_name(read_i32(this + c::FONT_INDEX), name.as_mut_ptr() as usize, 0x100);
        if !guio_same_c_string(this + c::FONT_NAME, name.as_ptr() as usize) {
            covered!("sx11_4 game.cfg font replaced");
            write_i32(this + c::FONT_INDEX, config_first_allowed_font(read_i32(this + c::LANGUAGE)));
        }
        true
    };
    exe::crt_fclose(f);
    Some(ok)
}

/// `Config::Config()`, the live settings at the program start (`game_init_from_command_line`):
///
/// - no source; the default options (`Config::set_defaults`);
/// - the language library from CM3_DATA\language.ldb; failing: the game_config.cpp line 0x83 "Error" box and the fatal
///   exit (−1);
/// - the library's language and its default currency; the allowed fonts table (`config_read_allowed_fonts`) and the
///   language's first allowed font (−2 the default one);
/// - auto save every 30 days, the user folder (`UTILS_GET_USER_FOLDER`) as data folder, no save name, not saved;
/// - game.cfg (`config_read_game_cfg`): read, the Config is saved (`SAVED` = 1); unreadable, the file is deleted;
/// - the command line: "-windowed" sets option 0x10, "-nosound" 0x100 (through `Config::set_option`), "-defaultfont"
///   the font −2;
/// - the display (800 × 600, full screen by option bit 8); failing: the fatal exit (0);
/// - sound on (option 0x80): the sound player (`config_start_sound`, without the intro);
/// - the background system and its pictures off by option bit 0x20.
///
/// Returns `this`.
/// Original: `Config::Config` (0x5d72c0, game_config.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn config_construct(this: usize) -> usize {
    use config_offsets as c;
    write_usize(this + c::SOURCE, 0);
    let defaults: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(config_life_fns::SET_DEFAULTS);
    defaults(this);
    let setup: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(config_life_fns::LANGLIB_SETUP);
    if setup(exe::utils_build_file_path(config_life_fns::TEXT_CM3_DATA, config_life_fns::TEXT_LANGUAGE_LDB), 1) == 0 {
        show_error_box(config_fns::GAME_CONFIG_CPP, 0x83);
        config_fatal_exit(-1);
    }
    write_i32(this + c::LANGUAGE, exe::langlib_get_language());
    config_default_currency_now(this);
    config_read_allowed_fonts();
    write_i32(this + c::FONT_INDEX, config_first_allowed_font(read_i32(this + c::LANGUAGE)));
    write_u16(this + c::AUTO_SAVE_DAYS, 0x1e);
    let folder: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(config_life_fns::UTILS_GET_USER_FOLDER);
    folder(config_life_fns::TEXT_DEFAULT_FOLDER, this + c::DATA_FOLDER);
    write_u8(this + config::SAVE_NAME, 0);
    write_u32(this + c::SAVED, 0);
    match config_read_game_cfg(this) {
        Some(true) => write_u32(this + c::SAVED, 1),
        Some(false) => {
            covered!("sx11_4 game.cfg deleted");
            exe::crt_remove(exe::utils_build_file_path(config_life_fns::TEXT_CM3_PREFS, config_life_fns::TEXT_GAME_CFG) as *const u8);
        }
        None => {}
    }
    let switch = |text: usize| exe::crt_strstr(config_fns::G_COMMAND_LINE, text) != 0;
    if switch(config_life_fns::TEXT_WINDOWED) {
        exe::config_set_option(this, 0x10);
    }
    if switch(config_life_fns::TEXT_NOSOUND) {
        exe::config_set_option(this, 0x100);
    }
    if switch(config_life_fns::TEXT_DEFAULTFONT) {
        write_i32(this + c::FONT_INDEX, -2);
    }
    if config_open_display(read_i32(this + c::FONT_INDEX), read_u32(this + c::OPTIONS) >> 3 & 1) == 0 {
        config_fatal_exit(0);
    }
    if read_u8(this + c::OPTIONS) & 0x80 != 0 {
        config_start_sound(this, false);
    }
    config_call0(config_life_fns::SETUP_BACKGROUND_SYSTEM);
    config_call1(config_life_fns::BACKGROUND_SET_PICTURES_DISABLED, read_u32(this + c::OPTIONS) >> 5 & 1);
    this
}

/// The fields `Config::apply_and_save` copies from a working copy to its source (the three texts with `strcpy`).
unsafe fn config_copy_back(this: usize, source: usize) {
    use config_offsets as c;
    write_u32(source + c::OPTIONS, read_u32(this + c::OPTIONS));
    write_u32(source + c::LANGUAGE, read_u32(this + c::LANGUAGE));
    write_u8(source + c::CURRENCY, read_u8(this + c::CURRENCY));
    write_u16(source + c::AUTO_SAVE_DAYS, read_u16(this + c::AUTO_SAVE_DAYS));
    copy_c_string(source + c::DATA_FOLDER, this + c::DATA_FOLDER);
    copy_c_string(source + config::SAVE_NAME, this + config::SAVE_NAME);
    write_u32(source + c::SAVED, read_u32(this + c::SAVED));
    write_u32(source + c::FONT_INDEX, read_u32(this + c::FONT_INDEX));
    copy_c_string(source + c::FONT_NAME, this + c::FONT_NAME);
}

/// The display after a settings change: a font other than the display's (`G_FONT_MODE`) or a screen mode other than
/// the option's (bit 8 full screen against `GRAPHICS_IS_FULL_SCREEN`, bit 0x10 windowed) opens it again; failing, the
/// other mode is tried (`Config::set_option` 0x10 or 8); failing again, the game_config.cpp line 0x337 "Error" box and
/// the fatal exit (−1).
unsafe fn config_apply_display(this: usize) {
    use config_offsets as c;
    let font_changed = read_i32(G_FONT_MODE) != read_i32(this + c::FONT_INDEX);
    let mode_changed = (read_u8(this + c::OPTIONS) & 8 != 0 && config_call0(config_life_fns::GRAPHICS_IS_FULL_SCREEN) == 0)
        || (read_u8(this + c::OPTIONS) & 0x10 != 0 && config_call0(config_life_fns::GRAPHICS_IS_FULL_SCREEN) != 0);
    if !font_changed && !mode_changed {
        return;
    }
    let full = read_u32(this + c::OPTIONS) & 8;
    if config_open_display(read_i32(this + c::FONT_INDEX), full) != 0 {
        return;
    }
    covered!("sx11_4 display mode refused");
    exe::config_set_option(this, if full != 0 { 0x10 } else { 8 });
    if config_open_display(read_i32(this + c::FONT_INDEX), (full == 0) as u32) == 0 {
        show_error_box(config_fns::GAME_CONFIG_CPP, 0x337);
        config_fatal_exit(-1);
    }
}

/// The fields of game.cfg in the order they are written, each (address, size, the game_config.cpp line of its error
/// box): the magic (at `magic`), options, language, currency, auto save interval, data folder, font index and name.
unsafe fn config_game_cfg_fields(this: usize, magic: usize) -> [(usize, u32, u32); 8] {
    use config_offsets as c;
    [(magic, 4, 0x38c), (this + c::OPTIONS, 4, 0x394), (this + c::LANGUAGE, 4, 0x39c), (this + c::CURRENCY, 1, 0x3a4),
        (this + c::AUTO_SAVE_DAYS, 2, 0x3ac), (this + c::DATA_FOLDER, 0x104, 0x3b4), (this + c::FONT_INDEX, 4, 0x3bc),
        (this + c::FONT_NAME, 0x100, 0x3c4)]
}

/// Writes game.cfg (CM3_PREFS, "wb"): magic, options, language, currency, auto save interval, data folder (0x104), font
/// index and font name (0x100). Not opened: the line 0x384 "Error" box and 0; a short write: the line of that field
/// (0x38c .. 0x3c4) "Error" box, the file closed and 0. Written: `SAVED` = 1 and 1.
unsafe fn config_write_game_cfg(this: usize) -> u32 {
    use config_offsets as c;
    let f = exe::crt_fopen(exe::utils_build_file_path(config_life_fns::TEXT_CM3_PREFS, config_life_fns::TEXT_GAME_CFG), config_life_fns::TEXT_MODE_WB);
    if f == 0 {
        show_error_box(config_fns::GAME_CONFIG_CPP, 0x384);
        return 0;
    }
    let magic = config_life_fns::GAME_CFG_MAGIC;
    for (p, n, line) in config_game_cfg_fields(this, &magic as *const u32 as usize) {
        if exe::crt_fwrite(p, n, 1, f) != 1 {
            show_error_box(config_fns::GAME_CONFIG_CPP, line);
            exe::crt_fclose(f);
            return 0;
        }
    }
    exe::crt_fclose(f);
    write_u32(this + c::SAVED, 1);
    1
}

/// `Config::apply_and_save()`: a working copy copies its fields back to its source (`config_copy_back`) and applies the
/// source (through `Config::apply_and_save`), returning its answer. The live Config applies itself:
///
/// - a language other than the library's: its texts (`LANGLIB_LOAD_LANGUAGE`(language, 1)) and a redraw; other than the
///   database names' (with the nation table loaded): the names (`DATABASE_LOAD_LANGUAGE_NAMES`(language, 0)) and a
///   redraw;
/// - a currency other than the current one: `CASH_SET_CURRENCY`;
/// - the display (`config_apply_display`);
/// - the sound (`config_switch_sound`, without the intro);
/// - the background pictures: bit 0x20 with them on turns them off; otherwise bit 0x40 with them off turns them on;
/// - game.cfg (`config_write_game_cfg`), whose answer it returns.
/// Original: `Config::apply_and_save` (0x5d8480, game_config.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn config_apply_and_save(this: usize) -> u32 {
    use config_offsets as c;
    let source = read_usize(this + c::SOURCE);
    if source != 0 {
        config_copy_back(this, source);
        let apply: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(config_life_fns::APPLY_AND_SAVE);
        return apply(source);
    }
    let language = read_i32(this + c::LANGUAGE);
    if exe::langlib_get_language() != language {
        let load: unsafe extern "C" fn(i32, u32) = core::mem::transmute(config_fns::LANGLIB_LOAD_LANGUAGE);
        load(language, 1);
        exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 1, 1);
    }
    let language = read_i32(this + c::LANGUAGE);
    if config_call0(config_life_fns::DATABASE_GET_LANGUAGE) as i32 != language && read_usize(G_NATION_TABLE) != 0 {
        let names: unsafe extern "C" fn(i32, u32) = core::mem::transmute(config_life_fns::DATABASE_LOAD_LANGUAGE_NAMES);
        names(language, 0);
        exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 1, 1);
    }
    let currency = read_u8(this + c::CURRENCY);
    if config_call0(config_life_fns::CASH_GET_CURRENCY) as u8 != currency {
        config_call1(config_life_fns::CASH_SET_CURRENCY, currency as u32);
    }
    config_apply_display(this);
    config_switch_sound(this, read_u32(this + c::OPTIONS), false);
    if read_u8(this + c::OPTIONS) & 0x20 != 0 && config_call0(config_life_fns::BACKGROUND_PICTURES_DISABLED) == 0 {
        config_call1(config_life_fns::BACKGROUND_SET_PICTURES_DISABLED, 1);
    } else if read_u8(this + c::OPTIONS) & 0x40 != 0 && config_call0(config_life_fns::BACKGROUND_PICTURES_DISABLED) != 0 {
        config_call1(config_life_fns::BACKGROUND_SET_PICTURES_DISABLED, 0);
    }
    config_write_game_cfg(this)
}

/// Comparer for `config_construct`: it opens the display, the sound and the files, so only the original runs; then the
/// plan: no source, the library's language, a currency 0..25, a font −2 or below the font count, and a sound player
/// exactly when the sound option is on.
pub(crate) unsafe extern "thiscall" fn compare_config_construct(this: usize) -> usize {
    use config_offsets as c;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(this);
    let count: unsafe extern "C" fn() -> i32 = core::mem::transmute(config_fns::FONT_GET_COUNT);
    let font = read_i32(this + c::FONT_INDEX);
    let ok = r == this && read_usize(this + c::SOURCE) == 0 && read_i32(this + c::LANGUAGE) == exe::langlib_get_language()
        && (0..0x1a).contains(&(read_u8(this + c::CURRENCY) as i8)) && (font == -2 || (0..count()).contains(&font))
        && (read_usize(SOUND_PLAYER) != 0) == (read_u8(this + c::OPTIONS) & 0x80 != 0);
    SCOREBOARD.record(ok, || format!("Config::Config: options {:#x}, language {}, currency {}, font {}, sound player {:#x}",
        read_u32(this + c::OPTIONS), read_i32(this + c::LANGUAGE), read_u8(this + c::CURRENCY), font, read_usize(SOUND_PLAYER)));
    r
}

/// Comparer for `config_apply_and_save`: it changes the display, the sound and the language and writes game.cfg, so
/// only the original runs; then the plan: a copy left its fields in the source; a live Config that answered 1 is saved
/// and game.cfg holds exactly the copy's list of fields (`config_game_cfg_fields`).
pub(crate) unsafe extern "thiscall" fn compare_config_apply_and_save(this: usize) -> u32 {
    use config_offsets as c;
    let source = read_usize(this + c::SOURCE);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this);
    if source != 0 {
        let same = |o: usize, n: usize| (0..n).all(|k| read_u8(source + o + k) == read_u8(this + o + k));
        let ok = same(c::OPTIONS, 0xb) && guio_same_c_string(source + c::DATA_FOLDER, this + c::DATA_FOLDER)
            && guio_same_c_string(source + config::SAVE_NAME, this + config::SAVE_NAME) && same(c::FONT_INDEX, 4)
            && guio_same_c_string(source + c::FONT_NAME, this + c::FONT_NAME);
        SCOREBOARD.record(ok, || "Config::apply_and_save (copy): the source does not hold the copy's fields".to_string());
        return r;
    }
    if r == 1 {
        let path = exe::utils_build_file_path(config_life_fns::TEXT_CM3_PREFS, config_life_fns::TEXT_GAME_CFG);
        let path = String::from_utf8_lossy(&c_string_bytes(path)).into_owned();
        let magic = config_life_fns::GAME_CFG_MAGIC;
        let mut plan = Vec::new();
        for (p, n, _) in config_game_cfg_fields(this, &magic as *const u32 as usize) {
            plan.extend((0..n as usize).map(|k| read_u8(p + k)));
        }
        let file = std::fs::read(&path).unwrap_or_default();
        SCOREBOARD.record(file == plan && read_u32(this + c::SAVED) == 1, || format!("Config::apply_and_save: {} has {} bytes, plan {} (first difference at {:?})",
            path, file.len(), plan.len(), file.iter().zip(&plan).position(|(a, b)| a != b)));
    }
    r
}
