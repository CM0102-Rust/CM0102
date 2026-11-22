// ---------- batch %%LEVA%%: the Setup screens, part 5: the Game Settings box (Setup.cpp) ----------

// (uses the Setup helpers of tel11_1)
const FN_CONFIG_COPY: usize = 0x5d7ad0; // Config::Config(this, source) thiscall ret 4
const FN_CONFIG_DESTROY: usize = 0x5d7b70; // Config::~Config thiscall
const FN_CONFIG_SET_DEFAULTS: usize = 0x5d7bc0; // Config::set_defaults thiscall -> 1
const FN_CONFIG_SET_CURRENCY: usize = 0x5d8270; // Config::set_currency(this, currency) thiscall ret 4
const FN_CONFIG_SET_FONT: usize = 0x5d8b50; // Config::set_font(this, font) thiscall ret 4
const FN_CONFIG_SET_AUTO_SAVE_INTERVAL: usize = 0x5d83e0; // Config::set_auto_save_interval(this, days) thiscall ret 4
const FN_CONFIG_GET_FONT_INDEX: usize = 0x5d8b40; // Config::get_font_index thiscall -> int
const FN_CASH_GET_CURRENCY_NAME: usize = 0x4415f0; // cash_get_currency_name(buffer, size, currency) -> 1 or 0
const FN_FONT_GET_MODE_NAME: usize = 0x5b2040; // font_get_mode_name(font, buffer, size)
const FN_FONT_GET_FILE_COUNT: usize = 0x5b1fc0; // font_get_file_count() -> count
const FN_IS_FONT_ALLOWED_FOR_LANGUAGE: usize = 0x5d8b80; // is_font_allowed_for_language(language, name) -> 1 or 0
const G_GUI_COLOUR_SILVER: usize = 0xae3154; // g_gui_colour_silver
const G_SETTINGS_OPTION_COLOUR: usize = 0xdd6ec4; // the Game Settings option buttons' colour (RGB 0, 0xc0, 0), made once
const G_SETTINGS_STATICS_GUARD: usize = 0xdd710f; // bit 0: G_SETTINGS_OPTION_COLOUR made
const SETTINGS_LANGUAGE_NAMES: usize = 0x9eb290; // the 19 language names ("English"...)
const CONFIG_OPTIONS: usize = 0x4; // the Config's option bits
const CONFIG_LANGUAGE: usize = 0x8; // the Config's language
const CONFIG_CURRENCY: usize = 0xc; // the Config's currency (byte)
const CONFIG_AUTO_SAVE_DAYS: usize = 0xd; // the Config's auto save interval in days (short)
const CONFIG_SIZE: usize = 0x31b; // the Config object (operator new size of the settings copy)

/// Opens the Game Settings box (`game_settings_box_build`, `game_settings_box_handle_input`, flag 1) with a copy of the
/// live Config (`operator new` 0x31b bytes, `Config::Config` from `G_CONFIG`; 0 when the allocation fails) in param 0
/// and `origin` (0 first run, 1 Setup Game, 2/3 the side bar) in param 1.
/// Original: `open_game_settings_box` (0x823890, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_game_settings_box(origin: usize) {
    covered!("tel12_1 open game settings");
    let copy: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONFIG_COPY);
    if exe::screenmanager_open_screen(SCREEN_MANAGER, 0x823950usize, 0x8259a0usize, 1, 0usize, 0) == 0 {
        return;
    }
    let p = exe::crt_operator_new(CONFIG_SIZE as u32);
    let config = if p != 0 { copy(p, read_usize(G_CONFIG)) } else { 0 };
    exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, config, 0);
    exe::screenmanager_set_screen_param(SCREEN_MANAGER, 1, origin, 0);
}

/// Opens the Game Settings box from the side bar: origin 2 for item 0, 3 for any other.
/// Original: `open_game_settings_from_side_bar` (0x823930, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_game_settings_from_side_bar(item: u32) {
    covered!("tel12_1 settings from side bar");
    let open: unsafe extern "C" fn(usize) = core::mem::transmute(FN_OPEN_GAME_SETTINGS_BOX);
    open(if item == 0 { 2 } else { 3 });
}

/// A row label of the Game Settings box: `label` (translated, with an empty argument) in column 0, silver, font 3.
unsafe fn settings_label(area: usize, row: u32, label: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let cell: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    text(SETUP_TEXT, label, 0xad9c64usize);
    cell(SCREEN_MANAGER, area, 0, row, 1, 0, 1, 3, setup_colour(G_GUI_COLOUR_SILVER), SETUP_TEXT);
}

/// One choice of an on/off row (already in the shared text) in column `column`: when `bit` is not set in the options,
/// a button (id 0x2c with `bit` as value, the option colour) or, with `disabled`, the same as plain text; when set,
/// yellow text with style 0x221.
unsafe fn settings_toggle(area: usize, column: u32, row: u32, options: u32, bit: u32, disabled: bool) {
    let cell: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let button: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32, u32, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let (option, yellow) = (setup_colour(G_SETTINGS_OPTION_COLOUR), setup_colour(G_GUI_COLOUR_YELLOW));
    if options & bit != 0 {
        cell(SCREEN_MANAGER, area, column, row, 0x221, yellow, 0xc, 2, yellow, SETUP_TEXT);
    } else if disabled {
        cell(SCREEN_MANAGER, area, column, row, 1, 0, 0x2c, 2, option, SETUP_TEXT);
    } else {
        button(SCREEN_MANAGER, area, column, row, 1, 0, 0xc, 2, option, SETUP_TEXT, 0x2c, bit);
    }
}

/// `settings_toggle` with the choice's text `label` translated first (with an empty argument).
unsafe fn settings_toggle_text(area: usize, column: u32, row: u32, options: u32, bit: u32, label: usize, disabled: bool) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(SETUP_TEXT, label, 0xad9c64usize);
    settings_toggle(area, column, row, options, bit, disabled);
}

/// The drop-down of a list row in column 2: read-only, an empty black cell; otherwise an empty button and the menu area
/// it opens (style 0x100030, green), which is returned (0 read-only: the original keeps a leftover there, never used).
unsafe fn settings_dropdown(area: usize, row: u32, read_only: bool) -> usize {
    let cell: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let button: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32, u32, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let (black, pale) = (setup_colour(G_GUI_COLOUR_BLACK), setup_colour(G_GUI_COLOUR_PALE_GREY));
    if read_only {
        cell(SCREEN_MANAGER, area, 2, row, 0x30, black, 0xc, 1, pale, 0);
        return 0;
    }
    let b = button(SCREEN_MANAGER, area, 2, row, 0x30, black, 0xc, 1, pale, 0, 0, 0);
    add_area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x100030, setup_colour(G_GUI_COLOUR_GREEN), b)
}

/// One item of a drop-down menu `menu`: `text`, style `style` (1 normal, 0x21 greyed), id `id` with value `value`.
unsafe fn settings_choice(menu: usize, text: usize, style: usize, id: usize, value: usize) {
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let (green, yellow, black) = (setup_colour(G_GUI_COLOUR_GREEN), setup_colour(G_GUI_COLOUR_YELLOW), setup_colour(G_GUI_COLOUR_BLACK));
    item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, style, 1, black, text, 0, id, value, menu);
}

/// The current choice of a list row in column 1: `name` after two spaces, yellow.
unsafe fn settings_current(area: usize, row: u32, name: &[u8; 0xc8]) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let cell: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    format(SETUP_TEXT, 0x987ff0, name.as_ptr()); // "  %s"
    cell(SCREEN_MANAGER, area, 1, row, 1, 0, 1, 2, setup_colour(G_GUI_COLOUR_YELLOW), SETUP_TEXT);
}

/// Builds the Game Settings box over the screen.
///
/// The option colour `G_SETTINGS_OPTION_COLOUR` is made once (`graphics_rgb_to_pixel`(0, 0xc0, 0, 0), guard bit 0 of
/// `G_SETTINGS_STATICS_GUARD`). Param 0 is the Config copy, param 1 the origin (3: read-only lists; 2 and 3: the
/// database and match store rows cannot change). It counts the languages available (`langlib_is_language_available`,
/// 19), clears the background (`background_set_picture`(0)) and, with an origin, covers the screen with an empty area.
/// The box is 500 pixels high (0x1db with fewer than two languages), centred in 600: its area (0x91 to 0x2e8, colour
/// `graphics_rgb_to_pixel`(0, 0x38, 0, 0)), the title "Game Settings" (black, font 4), the version ("Version <%s>" from
/// `utils_get_version_string`, with the build date and time "Nov 21 2002" "13:10:31" in English), Reset (id 0x27) and,
/// at the bottom, Cancel (0x28) and Ok (0x29) in two columns (3, 1); a pitch drawing (`draw_pitch`) under the rows. The
/// rows go in six areas at the same place (0xaa to 0x2cf), the first, third with the columns 0xc, 9, 1, 5, 1 and the
/// others 3, 1, 1, 1, 1, 15 rows (14 with one language), all row heights 1:
/// - (with two or more languages) Language: the current one ("  %s" of its name, yellow) and the drop-down of the
///   available ones (id 0x2a with the language; greyed and without id when not English, a database is loaded and it has
///   no language file);
/// - Currency: the current one and the 26 currencies (`cash_get_currency_name`, id 0x2b with the index);
/// - Fonts: the current font (`font_get_mode_name` of `Config::get_font_index`, yellow on yellow) and, unless read-only,
///   the drop-down: Traditional (id 0x3d, −1) and Futuristic (−2) unless the language is 0xb or 0xc, a separator and up
///   to 20 font files allowed for the language (`is_font_allowed_for_language`, id 0x3d with the index);
/// - the on/off rows (`settings_toggle`, id 0x2c with the bit): Wages (Weekly 0x200000, Monthly 0x400000, Yearly
///   0x800000), Screen Mode (Full 8, Windowed 0x10), Background Changes (On 0x20, Off 0x40), Sound (On 0x80, Off
///   0x100), Database (Normal 1, Minimum 2, Maximum 4) and Matches Stored (10000 0x200, 5000 0x400, 20000 0x800);
/// - Auto Save: the drop-down Never (id 0x2c, 0x1000) and every 7, 14, 30, 90, 180 and 360 days (id 0x2d with the days)
///   and the current one: the last choice that matches (Never with bit 0x1000; the interval with bit 0x2000 and
///   `CONFIG_AUTO_SAVE_DAYS`); DEFECT of the original: with none, the name buffer keeps its last text (a font name or
///   a currency);
/// - Save Compressed (Yes 0x4000, No 0x8000), Save Safely (Yes 0x20000000, a button only when 0x40000000 is set and
///   0x20000000 is not; No 0x40000000), Flashing Text (Off 0x10000, On 0x20000), Background Matches (Normal 0x40000,
///   Off 0x80000, High 0x100000) and Print To (Printer 0x1000000, File 0x2000000, 0x4000000).
/// Original: `game_settings_box_build` (0x823950, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_settings_box_build() {
    covered!("tel12_1 build game settings");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let set_picture: unsafe extern "C" fn(usize) = core::mem::transmute(FN_BACKGROUND_SET_PICTURE);
    let version: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let label_item: unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, u32, usize, u32, u32, usize, usize, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let button: unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, u32, usize, u32, u32, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let area_button: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32, u32, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let cell: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let pitch: unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, usize, u32) -> u32 = core::mem::transmute(FN_DRAW_PITCH);
    let currency_name: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_CASH_GET_CURRENCY_NAME);
    let font_index: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONFIG_GET_FONT_INDEX);
    let font_name: unsafe extern "C" fn(u32, usize, u32) = core::mem::transmute(FN_FONT_GET_MODE_NAME);
    let font_count: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_FONT_GET_FILE_COUNT);
    let font_allowed: unsafe extern "C" fn(u32, usize) -> u32 = core::mem::transmute(FN_IS_FONT_ALLOWED_FOR_LANGUAGE);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    if read_u8(G_SETTINGS_STATICS_GUARD) & 1 == 0 {
        write_u8(G_SETTINGS_STATICS_GUARD, read_u8(G_SETTINGS_STATICS_GUARD) | 1);
        write_u16(G_SETTINGS_OPTION_COLOUR, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
    let mut buttons: [u8; 7] = [3, 1, 0xc, 9, 1, 5, 1]; // the bottom buttons' 2 columns, then the list columns
    let mut toggles: [u8; 5] = [3, 1, 1, 1, 1];
    let mut heights = [1u8; 15];
    let mut rows = 0xfu32;
    let config = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    let origin = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 1) as i32;
    let mut languages = 0i8;
    for k in 0..0x13 {
        if exe::langlib_is_language_available(k) != 0 {
            languages = languages.wrapping_add(1);
        }
    }
    set_picture(0);
    if origin != 0 {
        add_area(current_human_display(), 0, 0, 0x31f, 0x257, 1, 0, 1, 0, 2, 0, usize::MAX);
    }
    let height = if languages >= 2 { 0x1f4 } else { 0x1db };
    let top = (0x258 - height) / 2;
    let bottom = height + top - 1;
    let background = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0) as usize;
    add_area(current_human_display(), 0x91, top as usize, 0x2e8, bottom as usize, 1, 0, 1, 0, 0x30, background, usize::MAX);
    let (grey, pale, black, silver) = (setup_colour(G_GUI_COLOUR_GREY), setup_colour(G_GUI_COLOUR_PALE_GREY), setup_colour(G_GUI_COLOUR_BLACK), setup_colour(G_GUI_COLOUR_SILVER));
    text(SETUP_TEXT, 0xa5a46c); // "Game Settings"
    let y = top + 0xa;
    label_item(SCREEN_MANAGER, 0x9b, y, 0x2de, y + 0x2d, 0x30, black, 0xc, 4, pale, SETUP_TEXT, 1);
    let y = y + 0x37;
    if exe::langlib_get_language() == 0 {
        let v = version(0xa80e4c, 0xa80e58);
        text(SETUP_TEXT, 0xa80e04, v, 0xa80e4cusize, 0xa80e58usize); // "Version: <%s> - Build: <%s> <%s>", "Nov 21 2002", "13:10:31"
    } else {
        let v = exe::utils_get_version_string() as usize;
        text(SETUP_TEXT, 0xa80dec, v); // "Version <%s - version>"
    }
    label_item(SCREEN_MANAGER, 0xa5, y, 0x22b, y + 0x14, 1, 0, 1, 1, silver, SETUP_TEXT, 1);
    text(SETUP_TEXT, 0xa7e768); // "Reset"
    button(SCREEN_MANAGER, 0x261, y, 0x2de, y + 0x14, 0x30, grey, 0xc, 1, pale, SETUP_TEXT, 0x27, 0);
    let low = bottom - 0xa;
    let bottom_area = add_area(current_human_display(), 0x9b, (low - 0x1e) as usize, 0x2de, low as usize, 2, buttons.as_mut_ptr() as usize, 1, 0, 1, 0, usize::MAX);
    text(SETUP_TEXT, 0x98a1a0); // "Cancel"
    area_button(SCREEN_MANAGER, bottom_area, 0, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, 0x28, 0);
    text(SETUP_TEXT, 0x990f7c); // "Ok"
    area_button(SCREEN_MANAGER, bottom_area, 1, 0, 0x30, grey, 0xc, 3, pale, SETUP_TEXT, 0x29, 0);
    let (mut left, mut upper, mut right, mut lower) = (0i32, 0i32, 0i32, 0i32);
    pitch(0xa5, y + 0x1e, 0x2d4, low - 0x32, &mut left, &mut upper, &mut right, &mut lower, 1, bottom_area, 0x40800000);
    let (list_top, list_bottom) = ((y + 0x23) as usize, (low - 0x37) as usize);
    if languages == 1 {
        rows = 0xe;
    }
    let list_area = |columns: *mut u8, rows: u32, heights: *mut u8| add_area(current_human_display(), 0xaa, list_top, 0x2cf, list_bottom, 5, columns as usize, rows as usize, heights as usize, 1, 0, usize::MAX);
    let cfg = |offset: usize| read_u32(config + offset);
    let read_only = origin == 3;
    let fixed = origin == 2 || origin == 3;
    let mut name = [0u8; 0xc8];
    let mut area = list_area(buttons.as_mut_ptr().add(2), rows, heights.as_mut_ptr());
    let mut row = 0u32;
    // Language
    if languages > 1 {
        settings_label(area, 0, 0xa7ffcc); // " Language"
        let menu = settings_dropdown(area, 0, read_only);
        for k in 0..0x13u32 {
            if exe::langlib_is_language_available(k) == 0 {
                continue;
            }
            copy_text(name.as_mut_ptr(), read_usize(SETTINGS_LANGUAGE_NAMES + 4 * k as usize) as *const u8);
            if k == cfg(CONFIG_LANGUAGE) {
                settings_current(area, 0, &name);
            }
            if read_only {
                continue;
            }
            if k == 0 || read_usize(G_NATION_TABLE) == 0 || exe::database_language_file_exists(k) != 0 {
                settings_choice(menu, name.as_ptr() as usize, 1, 0x2a, k as usize);
            } else {
                settings_choice(menu, name.as_ptr() as usize, 0x21, 0, 0);
            }
        }
        row = 1;
    }
    // Currency
    settings_label(area, row, 0xa80de0); // " Currency"
    let menu = settings_dropdown(area, row, read_only);
    for k in 0..0x1au8 {
        currency_name(name.as_mut_ptr() as usize, 0xc8, k as u32);
        if k == read_u8(config + CONFIG_CURRENCY) {
            settings_current(area, row, &name);
        }
        if !read_only {
            settings_choice(menu, name.as_ptr() as usize, 1, 0x2b, k as usize);
        }
    }
    // Fonts
    settings_label(area, row + 1, 0xa80db8); // " Fonts<%s - COMMENT - game settings>"
    let b = area_button(SCREEN_MANAGER, area, 2, row + 1, 0x30, black, 0xc, 1, pale, 0, 0, 0);
    let menu = add_area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x100030, setup_colour(G_GUI_COLOUR_GREEN), b);
    let language = cfg(CONFIG_LANGUAGE);
    font_name(font_index(config), SETUP_TEXT, 0x7d0);
    let yellow = setup_colour(G_GUI_COLOUR_YELLOW);
    cell(SCREEN_MANAGER, area, 1, row + 1, 1, yellow, 1, 2, yellow, SETUP_TEXT);
    if !read_only {
        let named_fonts = language != 0xb && language != 0xc;
        if named_fonts {
            text(SETUP_TEXT, 0x9cc33c, 0xad9c64usize); // "Traditional<%s - COMMENT - fonts>"
            settings_choice(menu, SETUP_TEXT, 1, 0x3d, usize::MAX);
            text(SETUP_TEXT, 0x9cc318, 0xad9c64usize); // "Futuristic<%s - COMMENT - fonts>"
            settings_choice(menu, SETUP_TEXT, 1, 0x3d, usize::MAX - 1);
        }
        let count = font_count().min(0x14);
        if count > 0 {
            if named_fonts {
                let (green, black) = (setup_colour(G_GUI_COLOUR_GREEN), setup_colour(G_GUI_COLOUR_BLACK));
                item(current_human_display(), 1, 0, 0, 0, 0, 0, 0, 0x1000010, green, yellow, 1, 1, black, 0, 0, 0, 0, menu);
            }
            for k in 0..count as u32 {
                font_name(k, name.as_mut_ptr() as usize, 0xc8);
                if font_allowed(language, name.as_ptr() as usize) != 0 {
                    settings_choice(menu, name.as_ptr() as usize, 1, 0x3d, k as usize);
                }
            }
        }
    }
    // Wages .. Matches Stored
    area = list_area(toggles.as_mut_ptr(), rows, heights.as_mut_ptr());
    let r = row + 2;
    settings_label(area, r, 0xa80db0); // "Wages"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x200000, 0xa80da8, false); // "Weekly"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x400000, 0xa80da0, false); // "Monthly"
    settings_toggle_text(area, 3, r, cfg(CONFIG_OPTIONS), 0x800000, 0xa80d98, false); // "Yearly"
    let r = row + 3;
    settings_label(area, r, 0xa80d88); // "Screen Mode"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 8, 0xa80d60, false); // "Full<%s - COMMENT - full screen mode>"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x10, 0xa80d54, false); // "Windowed"
    let r = row + 4;
    settings_label(area, r, 0xa80d40); // "Background Changes"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x20, 0xa80d18, false); // "On<..>"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x40, 0xa80cf0, false); // "Off<..>"
    let r = row + 5;
    settings_label(area, r, 0xa80ce8); // "Sound"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x80, 0xa80ccc, false); // "On<..>"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x100, 0xa80cb0, false); // "Off<..>"
    let r = row + 6;
    settings_label(area, r, 0xa80ca4); // "Database"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 1, 0xa80c78, fixed); // "Normal<..>"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 2, 0xa80c48, fixed); // "Minimum<..>"
    settings_toggle_text(area, 3, r, cfg(CONFIG_OPTIONS), 4, 0xa80c40, fixed); // "Maximum"
    let r = row + 7;
    settings_label(area, r, 0xa80c30); // "Matches Stored"
    for (column, bit, number) in [(1u32, 0x200u32, 0xa80c28usize), (2, 0x400, 0xa80c20), (3, 0x800, 0xa80c18)] {
        format(SETUP_TEXT, number); // "10000", "5000", "20000"
        settings_toggle(area, column, r, cfg(CONFIG_OPTIONS), bit, fixed);
    }
    // Auto Save
    area = list_area(buttons.as_mut_ptr().add(2), rows, heights.as_mut_ptr());
    let r = row + 8;
    settings_label(area, r, 0xa80c0c); // "Auto Save"
    let menu = settings_dropdown(area, r, read_only);
    text(SETUP_TEXT, 0xa80c04); // "Never"
    if !read_only {
        settings_choice(menu, SETUP_TEXT, 1, 0x2c, 0x1000);
    }
    if cfg(CONFIG_OPTIONS) & 0x1000 != 0 {
        copy_text(name.as_mut_ptr(), SETUP_TEXT as *const u8);
    }
    for (label, days) in [(0xa80bf8usize, 7i16), (0xa80be8, 0xe), (0xa80bdc, 0x1e), (0xa80bc8, 0x5a), (0xa80bb4, 0xb4), (0xa80ba8, 0x168)] {
        text(SETUP_TEXT, label); // "Every Week" .. "Every Year"
        if !read_only {
            settings_choice(menu, SETUP_TEXT, 1, 0x2d, days as usize);
        }
        if cfg(CONFIG_OPTIONS) & 0x2000 != 0 && read_i16(config + CONFIG_AUTO_SAVE_DAYS) == days {
            copy_text(name.as_mut_ptr(), SETUP_TEXT as *const u8);
        }
    }
    settings_current(area, r, &name);
    // Save Compressed
    area = list_area(toggles.as_mut_ptr(), rows, heights.as_mut_ptr());
    let r = row + 9;
    settings_label(area, r, 0xa80b94); // "Save Compressed"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x4000, 0x98e8d0, false); // "Yes"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x8000, 0x98e8cc, false); // "No"
    // Save Safely
    area = list_area(toggles.as_mut_ptr(), rows, heights.as_mut_ptr());
    let r = row + 10;
    settings_label(area, r, 0xa80b84); // "Save Safely"
    text(SETUP_TEXT, 0x98e8d0); // "Yes"
    let options = cfg(CONFIG_OPTIONS);
    // the Yes button only when No is set and Yes is not (the original's test); otherwise the selected text
    let yes_set = if options & 0x20000000 == 0 && options & 0x40000000 != 0 { 0 } else { 0x20000000 };
    settings_toggle(area, 1, r, yes_set, 0x20000000, false);
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x40000000, 0x98e8cc, false); // "No"
    // Flashing Text, Background Matches, Print To
    area = list_area(toggles.as_mut_ptr(), rows, heights.as_mut_ptr());
    let r = row + 0xb;
    settings_label(area, r, 0xa80b74); // "Flashing Text"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x10000, 0xa80b50, false); // "Off<..>"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x20000, 0xa80b2c, false); // "On<..>"
    let r = row + 0xc;
    settings_label(area, r, 0xa80b18); // "Background Matches"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x40000, 0xa80aec, false); // "Normal<..>"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x80000, 0xa80ac4, false); // "Off<..>"
    settings_toggle_text(area, 3, r, cfg(CONFIG_OPTIONS), 0x100000, 0xa80a9c, false); // "High<..>"
    let r = row + 0xd;
    settings_label(area, r, 0xa80a90); // "Print To"
    settings_toggle_text(area, 1, r, cfg(CONFIG_OPTIONS), 0x1000000, 0xa80a88, false); // "Printer"
    settings_toggle_text(area, 2, r, cfg(CONFIG_OPTIONS), 0x2000000, 0xa80a80, false); // "File"
    settings_toggle_text(area, 3, r, cfg(CONFIG_OPTIONS), 0x4000000, 0xa80a78, false);
}

/// The Game Settings box's answer to item `item` (param 0 the Config copy, param 1 the origin), by the clicked id:
/// 0x27 Reset (`Config::set_defaults` on the copy), −11; 0x28 Cancel: the live Config is applied again
/// (`Config::apply_and_save` on `G_CONFIG`) and the copy destroyed and freed, −9; 0x29 Ok: the copy is applied
/// (`Config::apply_and_save`) and destroyed and freed, −9; 0x2a a language (`Config::set_language` with the item's value;
/// from Setup, origin 0 or 1, with a database loaded, `select_default_league_by_language`), −11; 0x3d a font
/// (`Config::set_font`), 0x2b a currency (`Config::set_currency`), 0x2c an option bit (`Config::set_option`), all −11;
/// 0x2d an auto save interval (`Config::set_option`(0x2000) and `Config::set_auto_save_interval` with the item's action
/// data), −11; anything else 0.
/// Original: `game_settings_box_handle_input` (0x8259a0, Setup.cpp, cdecl).
pub(crate) unsafe extern "C" fn game_settings_box_handle_input(item: u32) -> i32 {
    covered!("tel12_1 game settings input");
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CONFIG_DESTROY);
    let defaults: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONFIG_SET_DEFAULTS);
    let apply: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONFIG_APPLY_AND_SAVE);
    let set_language: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONFIG_SET_LANGUAGE);
    let set_font: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONFIG_SET_FONT);
    let set_currency: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONFIG_SET_CURRENCY);
    let set_option: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONFIG_SET_OPTION);
    let set_interval: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONFIG_SET_AUTO_SAVE_INTERVAL);
    let default_league: unsafe extern "C" fn() -> u32 = core::mem::transmute(FN_SELECT_DEFAULT_LEAGUE_BY_LANGUAGE);
    let raw = item;
    let item = item as i16;
    let config = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    let origin = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 1);
    let free_copy = || {
        if config != 0 {
            destroy(config);
            exe::crt_operator_delete(config);
        }
    };
    match setup_clicked_id(item) {
        0x27 => {
            defaults(config);
            -11
        }
        0x28 => {
            apply(read_usize(G_CONFIG));
            free_copy();
            -9
        }
        0x29 => {
            apply(config);
            free_copy();
            -9
        }
        0x2a => {
            set_language(config, setup_clicked_value(item));
            if (origin == 0 || origin == 1) && read_usize(G_NATION_TABLE) != 0 {
                default_league();
            }
            -11
        }
        0x3d => {
            set_font(config, setup_clicked_value(item));
            -11
        }
        0x2b => {
            set_currency(config, setup_clicked_value(item));
            -11
        }
        0x2c => {
            set_option(config, setup_clicked_value(item));
            -11
        }
        0x2d => {
            set_option(config, 0x2000);
            set_interval(config, exe::screenmanager_get_widget_action_data(SCREEN_MANAGER, raw));
            -11
        }
        _ => 0,
    }
}

/// Comparer of `open_game_settings_box`: only the original runs (it pushes the box and allocates the copy).
pub(crate) unsafe extern "C" fn compare_open_game_settings_box(origin: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(origin)
}

/// Comparer of `open_game_settings_from_side_bar`: only the original runs (it opens the box).
pub(crate) unsafe extern "C" fn compare_open_game_settings_from_side_bar(item: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(%%I1%%)(item)
}

/// Comparer of `game_settings_box_build`: side by side over the display list, with the option colour made once.
pub(crate) unsafe extern "C" fn compare_game_settings_box_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I2%%);
    let extra = [(G_SETTINGS_OPTION_COLOUR, 2), (G_SETTINGS_STATICS_GUARD, 1)];
    compare_by_display_list_with("Game Settings box", &extra, || o(), || game_settings_box_build());
}

/// Comparer of `game_settings_box_handle_input`: the answers that only change the copy (Reset, a language, font,
/// currency, option or auto save interval) run side by side over the copy and the Setup regions (a language can pick
/// the default league); Cancel and Ok (they apply a Config and free the copy) and a box without a copy run only in the
/// original.
pub(crate) unsafe extern "C" fn compare_game_settings_box_handle_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I3%%);
    let config = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0);
    let id = setup_clicked_id(item as i16);
    if config == 0 || !matches!(id, 0x27 | 0x2a | 0x2b | 0x2c | 0x2d | 0x3d) {
        return o(item);
    }
    let mut regions = setup_answer_regions();
    regions.push((config, CONFIG_SIZE));
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || game_settings_box_handle_input(item));
    SCOREBOARD.record(equal && r == n, || format!("Game Settings answer to item {}: original={} rust={} differences at {:?}", item as i16, r, n, first_differences(&mo, &mr)));
    r
}
