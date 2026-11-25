// ---------- rx08_8: the player statistics texts: the category names, the column headings, the recent ratings and one stat (player_stats.cpp) ----------

/// The player statistics' source file (the error boxes).
const PLAYER_STATS_CPP_PATH: usize = 0xa78310;

/// The empty string the translated templates with a COMMENT take as their extra argument.
const EMPTY_COMMENT: usize = 0xad9c64;

/// Writes a translated template into `buf` (`FN_LANGLIB_SPRINTF`), with the empty comment argument when `comment`.
unsafe fn player_stats_label(buf: usize, template: usize, comment: bool) {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if comment {
        sprintf(buf, template, EMPTY_COMMENT);
    } else {
        sprintf(buf, template);
    }
}

/// Writes the label of entry `which` (from 1) of `labels` into `buf`: a missing buffer or one shorter than 51 bytes
/// shows the player_stats.cpp box at `short_row` and answers 0; otherwise the buffer is emptied, an unknown entry shows
/// the box at `bad_row` and answers 0, a known one is written (`player_stats_label`) and answers 1.
unsafe fn player_stats_write_label(labels: &[(usize, bool)], which: u32, buf: usize, size: i32, short_row: u32, bad_row: u32) -> u32 {
    if buf == 0 || size < 0x33 {
        show_error_at(PLAYER_STATS_CPP_PATH, short_row);
        return 0;
    }
    write_u8(buf, 0);
    let k = (which as u8 as i8 as i32).wrapping_sub(1);
    match labels.get(k as usize).filter(|_| k >= 0) {
        Some(&(template, comment)) => {
            player_stats_label(buf, template, comment);
            1
        }
        None => {
            show_error_at(PLAYER_STATS_CPP_PATH, bad_row);
            0
        }
    }
}

/// The player statistics categories 1 to 6: non competitive (friendlies and reserves), league, cup, continental,
/// international and senior club; the templates with a COMMENT take the empty comment.
const PLAYER_STATS_CATEGORY_NAMES: [(usize, bool); 6] = [(0xa783dc, true), (0x990f4c, false), (0x990f48, false), (0xa783a4, true), (0xa78368, true), (0xa7835c, false)];

/// rule: the names of the player statistics categories: 1 "Non Competitive" (friendlies and reserves), 2 "League", 3
/// "Cup", 4 "Continental", 5 "International", 6 "Senior Club"
///
/// `player_stats_write_label` with `PLAYER_STATS_CATEGORY_NAMES`, the category byte (signed), the boxes of lines 0x3ce (no or short
/// buffer) and 0x404 (unknown category). `this` is not read.
/// Original: `player_stats_get_category_name` (0x7c14b0, player_stats.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn player_stats_get_category_name(_this: usize, category: u32, buf: usize, size: i32) -> u32 {
    covered!("rx08_8 category name");
    player_stats_write_label(&PLAYER_STATS_CATEGORY_NAMES, category, buf, size, 0x3ce, 0x404)
}

/// The column headings of the stat fields 1 to 17 (see `player_stats_get_field_label`).
const PLAYER_STATS_FIELD_LABELS: [(usize, bool); 17] = [(0xa78494, false), (0xa78490, false), (0xa78464, true), (0x990e40, false), (0xa7845c, false), (0xa78454, false),
    (0xa7844c, false), (0x990b1c, true), (0xa78444, false), (0x98e978, false), (0xa78440, false), (0xa7843c, false), (0xa7842c, false),
    (0xa161c8, false), (0xa78428, false), (0xa78420, false), (0xa78434, false)];

/// rule: the short column headings of the player stat fields: 1 "Apps", 2 "Gls", 3 "Con" (conceded), 4 "Pens", 5
/// "Asts", 6 "Tgls", 7 "Tcon", 8 "Won", 9 "Lost", 10 "Yel", 11 "Red", 12 "MoM", 13 "Pass %", 14 "Tck", 15 "Drb", 16 "Sh
/// Tar", 17 "Av R"
///
/// `player_stats_write_label` with `PLAYER_STATS_FIELD_LABELS`, the field byte (signed), the boxes of lines 0x5bc (no or short
/// buffer) and 0x63f (unknown field). `this` is not read.
/// Original: `player_stats_get_field_label` (0x7c1b20, player_stats.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn player_stats_get_field_label(_this: usize, field: u32, buf: usize, size: i32) -> u32 {
    covered!("rx08_8 field label");
    player_stats_write_label(&PLAYER_STATS_FIELD_LABELS, field, buf, size, 0x5bc, 0x63f) // rx08_8: the field headings
}

/// rule: a player's last five match ratings, oldest first, as "7-6-8-": each rating present (non zero) followed by a
/// dash, and the last dash dropped; none at all, "-"
///
/// `this.format_recent_ratings(player, buf, size)`: the buffer is emptied first (unchecked); no player: the
/// player_stats.cpp box of line 0x41d (and it goes on); no buffer or one shorter than 51 bytes: the box of line 0x424
/// (and it goes on). A player id (+0) not below the player data count (`G_PLAYER_DATA_COUNT`): "-" (`FN_CRT_SPRINTF`),
/// returning its count. Otherwise the player's entry (`player_stats_fields::PLAYER_ENTRIES`, 0x27 bytes) gives the five
/// 4-bit ratings, in order the low and high halves of `player_stats_entry::RECENT_RATINGS` and of the next byte and the
/// low half of `player_stats_entry::LAST_RATING`; each non zero one is appended as "%d-". A result of two or more
/// characters loses its last one (returns 0); an empty one becomes "-" (`news_strcpy`, returns 2). The pieces are
/// printed straight after each other in the buffer, as the original's sprintf into a local and strcat.
/// Original: `PlayerStats::format_recent_ratings` (0x7c1660, player_stats.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn format_recent_ratings(this: usize, player: usize, buf: usize, size: i32) -> u32 {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    write_u8(buf, 0);
    if player == 0 {
        show_error_at(PLAYER_STATS_CPP_PATH, 0x41d);
    }
    if buf == 0 || size < 0x33 {
        show_error_at(PLAYER_STATS_CPP_PATH, 0x424);
    }
    let id = read_i32(player);
    if id >= read_i32(G_PLAYER_DATA_COUNT) {
        return sprintf(buf, 0x98ba0c) as u32; // "-"
    }
    covered!("rx08_8 recent ratings");
    let entry = read_usize(this + player_stats_fields::PLAYER_ENTRIES) + id as usize * 0x27;
    let (a, b, c) = (read_u8(entry + player_stats_entry::RECENT_RATINGS), read_u8(entry + player_stats_entry::RECENT_RATINGS + 1), read_u8(entry + player_stats_entry::LAST_RATING));
    let mut len = 0usize;
    for rating in [a & 0xf, a >> 4, b & 0xf, b >> 4, c & 0xf] { // rx08_8: oldest first
        if rating != 0 {
            len += sprintf(buf + len, 0xa7841c, rating as u32) as usize; // "%d-" appended
        }
    }
    if len >= 2 {
        write_u8(buf + len - 1, 0);
        return 0;
    }
    news_strcpy(buf, 0x98ba0c); // "-"
    2
}

/// rule: one stat of a player in one category, as text: the stat field of the player's record of that category (see
/// `PlayerStats::format_record_field`); without a record, "-" ("----" for the average rating, field 17)
///
/// `this.format_player_stat(player, category, field, buf, size, subs, apps, zero)`: no buffer or one shorter than 51
/// bytes: the player_stats.cpp box of line 0x7c5 and 0. The buffer gets the placeholder ("----" for field 0x11, "-"
/// otherwise). No player: the box of line 0x7d7 and 0. A player id (+0) not below the player data count: 1. The
/// player's record of the category (`get_category_record`, the Rust port, thiscall(player, category)); none: 0.
/// Otherwise PlayerStats::format_record_field(record, field, buf, size, subs, apps, zero)
/// (`exe::playerstats_format_record_field`), whose result it passes on.
/// Original: `PlayerStats::format_player_stat` (0x7c25b0, player_stats.cpp, thiscall, ret 0x20).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn format_player_stat(this: usize, player: usize, category: u32, field: u32, buf: usize, size: i32, subs: u32, apps: u32, zero: u32) -> u32 {
    if buf == 0 || size < 0x33 {
        show_error_at(PLAYER_STATS_CPP_PATH, 0x7c5);
        return 0;
    }
    news_strcpy(buf, if field as u8 == 0x11 { 0x99cc4c } else { 0x98ba0c }); // rx08_8: the placeholder, "----" or "-"
    if player == 0 {
        show_error_at(PLAYER_STATS_CPP_PATH, 0x7d7);
        return 0;
    }
    if read_i32(player) >= read_i32(G_PLAYER_DATA_COUNT) {
        return 1;
    }
    let record = exe::playerstats_get_category_record(this, player, category);
    if record == 0 {
        return 0;
    }
    covered!("rx08_8 player stat");
    exe::playerstats_format_record_field(this, record, field, buf, size as u32, subs, apps, zero) as u32
}

/// Side by side on the first 51 bytes of a label buffer (the routines only write when the buffer is at least that
/// long); the result too. A missing or short buffer runs only the original (the box).
unsafe fn compare_player_stats_text(name: &str, buf: usize, size: i32, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    if buf == 0 || size < 0x33 {
        return original();
    }
    let (a, b, same, mo, mr) = crate::comparison::side_by_side(&[(buf, 0x33)], original, copy);
    SCOREBOARD.record(same && a == b, || format!("{}: original {} {:?}, rust {} {:?}", name, a, first_differences(&mo, &mr), b, first_differences(&mo, &mr)));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_player_stats_get_category_name(this: usize, category: u32, buf: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, i32) -> u32>(%%I0%%);
    compare_player_stats_text(&format!("player_stats_get_category_name({})", category as u8 as i8), buf, size, || o(this, category, buf, size), || player_stats_get_category_name(this, category, buf, size))
}

pub(crate) unsafe extern "thiscall" fn compare_player_stats_get_field_label(this: usize, field: u32, buf: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, i32) -> u32>(%%I1%%);
    compare_player_stats_text(&format!("player_stats_get_field_label({})", field as u8 as i8), buf, size, || o(this, field, buf, size), || player_stats_get_field_label(this, field, buf, size))
}

pub(crate) unsafe extern "thiscall" fn compare_format_recent_ratings(this: usize, player: usize, buf: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32) -> u32>(%%I2%%);
    if player == 0 {
        return o(this, player, buf, size);
    }
    compare_player_stats_text(&format!("format_recent_ratings(player {})", read_i32(player)), buf, size, || o(this, player, buf, size), || format_recent_ratings(this, player, buf, size))
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_format_player_stat(this: usize, player: usize, category: u32, field: u32, buf: usize, size: i32, subs: u32, apps: u32, zero: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, i32, u32, u32, u32) -> u32>(%%I3%%);
    if player == 0 {
        return o(this, player, category, field, buf, size, subs, apps, zero);
    }
    compare_player_stats_text(&format!("format_player_stat(player {}, category {}, field {})", read_i32(player), category as u8, field as u8), buf, size,
        || o(this, player, category, field, buf, size, subs, apps, zero), || format_player_stat(this, player, category, field, buf, size, subs, apps, zero))
}
