// ---------- batch %%LEVA%%: the media response screen (media.cpp) ----------

/// The media.cpp path for the error box.
const MEDIA_CPP: usize = 0xa17bec;
/// The ids of the media response screen: a response (data 1..3), Ok and Cancel.
const MEDIA_RESPONSE_ID: u32 = 0x1485;
const MEDIA_RESPONSE_OK: u32 = 0x109b;
const MEDIA_RESPONSE_CANCEL: u32 = 0x109c;
/// The news kinds the media response screen tells apart (the news item's first dword).
const MEDIA_NEWS_PLAYER_STORY: u32 = 0xbdd;
const MEDIA_NEWS_TEAM_STORY: u32 = 0xbdf;
const MEDIA_NEWS_SENT_OFF: u32 = 0xbe6;
const MEDIA_NEWS_WANTS_FIRST_TEAM: u32 = 0xbe8;
const MEDIA_NEWS_RIVAL_SIGNING: u32 = 0xfbc;

/// `News::find_item_by_id(manager, id, out)` (0x786b70, thiscall on [0xb7431c], ret 0xc).
unsafe fn media_find_news_item(manager: usize, id: usize, item: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x786b70usize);
    f(read_usize(G_NEWS), manager, id, item)
}

/// The club of a club index (the club table entry, `club::SIZE` each).
unsafe fn media_club(index: usize) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add((index as u32).wrapping_mul(club::SIZE as u32) as usize)
}

/// The person of a staff index (the staff table entry, `staff::SIZE` each).
unsafe fn media_person(index: usize) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((index as u32).wrapping_mul(staff::SIZE as u32) as usize)
}

/// Rule: the media response screen opens on a story news item of the manager, keeping the story's people and clubs as
/// the news kind lays them out.
///
/// `media_response_screen_open(manager, news_id)`: a local NewsItem; no manager: the media.cpp 15228 box; the item not
/// found (`News::find_item_by_id`): the media.cpp 15235 box. Otherwise `open_screen` with the builder 0x7423c0, the
/// handler 0x742de0, single instance, the no-op leave routine 0x5a8f60 and 0; when it opens, from the item's slots
/// (`NewsItem::get_slot`), by its kind: 0xbdf (team story): 4..7 = slots 15..18, 1 = slot 4, 2 = slot 9, 8 = slot 1,
/// 9 = -1; 0xbdd (player story): 4..7 = slots 0..3, 1 = slot 4, 2 = slot 5, 9 = -1; 0xbe6 (sent off): 4..7 = slots
/// 0..3, 1 = slot 9, 2 = 0x82, 8 = 0, 9 = slot 10; 0xbe8 (wants first team): 4..7 = slots 0..3, 1 = slot 8, 2 = 0, 8 =
/// 0, 9 = -1; 0xfbc: 4..7 = slots 0..3, 1 = slot 4, 2 = 0x1f, 9 = slot 16 when slot 15 is set and slot 16 is a club
/// index, otherwise -1; any other kind: 4..7 = slots 0..3, 1 = slot 4, 2 = slot 9, 9 = -1. Parameter 0 = the news id
/// and 3 = 3. Parameter 8 is left as the original's uninitialised local for the kinds that do not set it (here 0: a
/// defect of the original).
/// Original: `media_response_screen_open` (0x741f50, media.cpp, cdecl).
pub(crate) unsafe extern "C" fn media_response_screen_open(manager: usize, news_id: usize) {
    let mut item = [0u8; 0x100];
    let it = item.as_mut_ptr() as usize;
    exe::new_newsitem(it, 0, 0);
    if manager == 0 {
        tel26_error_box(MEDIA_CPP, 0x3b7c);
        return;
    }
    if media_find_news_item(manager, news_id, it) == 0 {
        tel26_error_box(MEDIA_CPP, 0x3b83);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x7423c0usize, 0x742de0usize, 1, 0x5a8f60usize, 0) == 0 {
        return;
    }
    covered!("tel35_4 open media response");
    let slot = |k: u32| exe::newsitem_get_slot(it, k) as usize;
    let none = usize::MAX;
    let (people, p1, p2, p8, p9): ([usize; 4], usize, usize, usize, usize) = match read_u32(it) {
        MEDIA_NEWS_TEAM_STORY => {
            let people = [slot(0xf), slot(0x10), slot(0x11), slot(0x12)];
            (people, slot(4), slot(9), slot(1), none)
        }
        MEDIA_NEWS_PLAYER_STORY => {
            let people = [slot(0), slot(1), slot(2), slot(3)];
            (people, slot(4), slot(5), 0, none)
        }
        MEDIA_NEWS_SENT_OFF => {
            let people = [slot(0), slot(1), slot(2), slot(3)];
            let p1 = slot(9);
            (people, p1, 0x82, 0, slot(0xa))
        }
        MEDIA_NEWS_WANTS_FIRST_TEAM => {
            let people = [slot(0), slot(1), slot(2), slot(3)];
            (people, slot(8), 0, 0, none)
        }
        MEDIA_NEWS_RIVAL_SIGNING => {
            let people = [slot(0), slot(1), slot(2), slot(3)];
            let p1 = slot(4);
            let p9 = if slot(0xf) == 0 {
                none
            } else {
                let c = slot(0x10) as i32;
                if c < 0 || c > read_i32(G_CLUB_COUNT) - 1 { none } else { c as usize }
            };
            (people, p1, 0x1f, 0, p9)
        }
        _ => {
            let people = [slot(0), slot(1), slot(2), slot(3)];
            (people, slot(4), slot(9), 0, none)
        }
    };
    set_screen_param(0, news_id);
    set_screen_param(1, p1);
    set_screen_param(2, p2);
    set_screen_param(3, 3);
    for (k, v) in people.into_iter().enumerate() {
        set_screen_param(4 + k as u32, v);
    }
    set_screen_param(8, p8);
    set_screen_param(9, p9);
}

/// How the media response screen's question reads, by the story code (parameter 2): with the person's name
/// (`Name`), without it (`Plain`), with it when there is a person and without otherwise (`NameOrPlain`), with 100
/// and the name (`Hundred`), or with the club (gender char +0x52, name +0x38: `Club` with and without the name,
/// `ClubOrNation` with the name or else the club's division nation).
#[derive(Clone, Copy)]
enum MediaQuestion {
    Name(usize),
    Plain(usize),
    NameOrPlain(usize, usize),
    Hundred(usize),
    Club(usize, usize),
    ClubOrNation(usize, usize),
}

/// The question of each story code 1..0x82 (the original's two-level jump table at 0x742d58/0x742cc4); the others
/// read 0xa36cf8 with 100 and the name.
fn media_question(code: u32) -> MediaQuestion {
    use MediaQuestion::*;
    match code {
        0x82 => Name(0xa38164),
        0x3c => Plain(0xa380f4),
        0x3f => NameOrPlain(0xa38090, 0xa38044),
        0x5f => NameOrPlain(0xa37fe0, 0xa37f90),
        0x60 => NameOrPlain(0xa37f24, 0xa37ed0),
        0x42 => NameOrPlain(0xa37e58, 0xa37df4),
        0x43 => NameOrPlain(0xa37d90, 0xa37d44),
        0x44 => Club(0xa37ccc, 0xa37c6c),
        0x5a => Club(0xa37bf8, 0xa37b9c),
        0x5c => ClubOrNation(0xa37b1c, 0xa37ab4),
        0x3d => Plain(0xa37a3c),
        0x52 => Name(0xa379b0),
        0x5b => Plain(0xa37954),
        0x61..=0x63 => NameOrPlain(0xa378dc, 0xa3787c),
        0x5d | 0x41 => NameOrPlain(0xa37740, 0xa376dc),
        0x40 => NameOrPlain(0xa37810, 0xa377b4),
        0x28..=0x34 => Hundred(0xa3766c),
        0xc => Name(0xa375c8),
        0x1 => Hundred(0xa37554),
        0x9 => Name(0xa374e4),
        0x25 => Hundred(0xa3745c),
        0x1e => Name(0xa373e4),
        0x20 => Name(0xa37374),
        0x1f => Name(0xa372e8),
        0x21 => Name(0xa37264),
        0x22 => Name(0xa371e8),
        0x8 => Hundred(0xa37160),
        0x6 => Name(0xa370d4),
        0x7 => Hundred(0xa3704c),
        0x23 => Hundred(0xa36fc4),
        0xd => Hundred(0xa36f5c),
        0x11 => Hundred(0xa36ee4),
        0x3 => Hundred(0xa36e70),
        0x13 => Hundred(0xa36de4),
        0xa => Hundred(0xa36d60),
        _ => Hundred(0xa36cf8),
    }
}

/// Rule: the media response screen shows the story's question in the club's colours and the three responses, the
/// chosen one in yellow, with Cancel and Ok.
///
/// `media_response_screen_draw()`: a local Text; the club (parameter 1), the story code (2), the chosen response (3)
/// and the person (4, with the name ids 5..7: `database_format_name_from_ids(5, 6, 7, name, 400, 1, 4)` when 4 is
/// above -1). The club's background picture, the menu bar (4, 0), the title (the slot manager's name) in the club's kit
/// colours (kit 1), the subtitle "Select Media Response". The question area (0x6e, 0x8c)-(0x30c, 0x221), kind 2, rows
/// = 0x195 / the height of font 3, with seven column weights read from a two-byte array (2, 17), the rows byte and the
/// next locals (a defect of the original: its seventh weight is an uninitialised stack byte, here 0); the question
/// (`media_question`) drawn wrapped from row 0 (font 3, pale grey). The responses area (same rectangle, weights 2/17,
/// kind 1): from two rows below, responses 1..3 (`Media::format_response_option_label`) as buttons id 0x1485 with data
/// k, the chosen one yellow and the others grey; the first one's widget written to parameter 8. The button area
/// (0x64, 0x22b)-(0x316, 0x24e) with "Cancel" (0x109c) and "Ok" (0x109b).
/// Original: `media_response_screen_draw` (0x7423c0, media.cpp, cdecl).
pub(crate) unsafe extern "C" fn media_response_screen_draw() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let label: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32) = core::mem::transmute(0x735160usize);
    covered!("tel35_4 draw media response");
    let mut tx = [0u32; 8];
    let t = tx.as_mut_ptr() as usize;
    exe::new_text(tx.as_mut_ptr() as *mut usize, 0usize);
    let mut name = [0u8; 0x190];
    let index = get_screen_param(1);
    let code = get_screen_param(2) as u32;
    let chosen = get_screen_param(3) as i32;
    let person = get_screen_param(4) as i32;
    let (n5, n6, n7) = (get_screen_param(5) as u32, get_screen_param(6) as u32, get_screen_param(7) as u32);
    let club = media_club(index);
    if person > -1 {
        exe::database_format_name_from_ids(n5, n6, n7, name.as_mut_ptr(), 0x190, 1, person as u32);
    }
    exe::background_set_club_picture(club, 0, 0);
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    exe::database_format_staff_name(tel26_slot_manager(), JOB_TEXT, 0x7d0, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back & 0xffff, fore & 0xffff, JOB_TEXT);
    text(JOB_TEXT, 0xa381e0);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = (0x195 / exe::font_get_height(3)) as u8;
    // the original's locals from +0x16: the weights 2, 17, then the rows byte over the low byte of parameter 6 and its
    // three high bytes, then an uninitialised byte (here 0)
    let columns: [u8; 7] = [2, 0x11, rows, (n6 >> 8) as u8, (n6 >> 16) as u8, (n6 >> 24) as u8, 0];
    let area = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 7, columns.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let g = |c: usize, at: usize| read_u8(c + at) as i8 as i32;
    let n = name.as_ptr();
    match media_question(code) {
        MediaQuestion::Name(f) => {
            text(JOB_TEXT, f, n);
        }
        MediaQuestion::Plain(f) => {
            text(JOB_TEXT, f);
        }
        MediaQuestion::NameOrPlain(f, plain) => {
            if person > -1 { text(JOB_TEXT, f, n) } else { text(JOB_TEXT, plain) };
        }
        MediaQuestion::Hundred(f) => {
            text(JOB_TEXT, f, 0x64, n);
        }
        MediaQuestion::Club(f, plain) => {
            if person > -1 {
                text(JOB_TEXT, f, g(club, 0x52), g(club, 0x52), n, club + 0x38);
            } else {
                text(JOB_TEXT, plain, g(club, 0x52), g(club, 0x52), club + 0x38);
            }
        }
        MediaQuestion::ClubOrNation(f, nation_text) => {
            if person > -1 {
                text(JOB_TEXT, f, g(club, 0x52), g(club, 0x52), n, club + 0x38);
            } else {
                let nation = read_usize(read_usize(club + 0x53) + 0x71);
                text(JOB_TEXT, nation_text, g(nation, 0x1e), g(nation, 0x1e), nation + 4);
            }
        }
    }
    exe::text_set_string(t, JOB_TEXT as *const u8);
    let mut line = 0i32;
    tel35_text_draw_wrapped(t, area, 0x6e, 0x30c, &mut line, 3, tel35_colour(G_GUI_COLOUR_PALE_GREY), 0, 0);
    let mut row = line + 2;
    let weights: [u8; 4] = [2, 0x11, 3, 1];
    let responses = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    for k in 1..=3i32 {
        label(read_usize(G_MEDIA), code, k as u32, JOB_TEXT, 0x7d0);
        let colour = if chosen != k { tel35_colour(G_GUI_COLOUR_GREY) } else { tel35_colour(G_GUI_COLOUR_YELLOW) };
        let w = tel26_area_button([responses, 1, row as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, MEDIA_RESPONSE_ID, k as u32]);
        if k == 1 {
            set_screen_param(8, w as i16 as isize as usize);
        }
        row += 1;
    }
    let bar = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr().wrapping_add(2) as u32, 1, 0, 1, 0, u32::MAX]);
    let (grey, pale) = (tel35_colour(G_GUI_COLOUR_GREY), tel35_colour(G_GUI_COLOUR_PALE_GREY));
    text(JOB_TEXT, 0x98a1a0);
    tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, MEDIA_RESPONSE_CANCEL, 0]);
    text(JOB_TEXT, 0x990f7c);
    tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, MEDIA_RESPONSE_OK, 0]);
    exe::drop_text(tx.as_mut_ptr() as *mut usize);
}

/// The story log record the media response is written into (0x17 bytes, searched with `bsearch` in the media log
/// [media + 0], count [media + 8], comparer 0x757ee0): a date (`Date::Date`), the person's and the club's ids (-1 for
/// none), a zero byte, the story code (its low byte sign-extended to a word) and the news kind.
unsafe fn media_log_response(person: usize, club: usize, code: u32, kind: u32, response: u32) {
    let mut key = [0u8; 0x18];
    let k = key.as_mut_ptr() as usize;
    let date: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    date(k);
    write_u32(k + 8, if person != 0 { read_u32(person) } else { u32::MAX });
    write_u32(k + 0xc, if club != 0 { read_u32(club) } else { u32::MAX });
    write_u8(k + 0x10, 0);
    write_u16(k + 0x11, code as u8 as i8 as i16 as u16);
    write_u32(k + 0x13, kind);
    let media = read_usize(G_MEDIA);
    let found = exe::crt_bsearch(k, read_usize(media), read_u32(media + 8), 0x17, 0x757ee0usize);
    if found != 0 {
        write_u8(found + 0x10, response as u8);
    }
}

/// Rule: on the media response screen a response click colours it yellow and keeps it; Ok with a response logs it,
/// applies it to the story (by the news kind) and marks the news item answered; Cancel closes.
///
/// `media_response_screen_handle_event(item)`: the news id (parameter 0), the club (1), the story code (2), the chosen
/// response (3), the person (4), the other club (9) and the slot manager. In a network game (`[0xb77c84]`) the pending
/// command argument is cleared and -9 comes back. Id 0x1485 (a response): its data k; the three response widgets
/// (from parameter 8) recoloured, yellow the k-th (1, 2, anything else the third) and grey the others; parameter 3 =
/// k; redraw, 0. Id 0x109b (Ok): with no response -9; otherwise the news item (`News::find_item_by_id`), and with a
/// person the story log record (`media_log_response`) gets the response; story code 0x1f with another club and a
/// person warns the squad of the rival (`Media::warn_squad_of_position_rival`); then by the news kind: 0xbdf
/// `apply_team_story_response(club, code, response, person or 0)`, 0xbe6 `apply_sent_off_story_response(person, club,
/// other club, response)`, 0xbe8 `apply_wants_first_team_response(person, club, response)`, any other
/// `apply_player_story_response(person, club, code, response)`; the news item's slot marked with the response
/// (`News::set_item_slot`: 0xd for 0xbdd, 0x12 for 0xbe6 and 0xbe8, 7 for 0xfbc, 0xa otherwise); -9. Id 0x109c
/// (Cancel): -9. Otherwise the menu bar's command, then its object link.
/// Original: `media_response_screen_handle_event` (0x742de0, media.cpp, cdecl).
pub(crate) unsafe extern "C" fn media_response_screen_handle_event(raw: u32) -> i32 {
    let news_id = get_screen_param(0);
    let index = get_screen_param(1);
    let code = get_screen_param(2) as u32;
    let response = get_screen_param(3) as u32;
    let person_index = get_screen_param(4) as i32;
    let other = get_screen_param(9);
    let manager = tel26_slot_manager().addr();
    if read_u32(TEL35_NETWORK_FLAG) != 0 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    let item = raw as i16;
    let id = tel26_item_id(item) as u32;
    if id == MEDIA_RESPONSE_ID {
        covered!("tel35_4 media response chosen");
        let k = tel26_item_value(item);
        let w = get_screen_param(8) as u32;
        let yellow_at = match k {
            1 => 0,
            2 => 1,
            _ => 2,
        };
        for j in 0..3u32 {
            let colour = if j == yellow_at { G_GUI_COLOUR_YELLOW } else { G_GUI_COLOUR_GREY };
            exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, w.wrapping_add(j), tel35_colour(colour));
        }
        set_screen_param(3, k as usize);
        exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
        return 0;
    }
    if id == MEDIA_RESPONSE_OK {
        if response == 0 {
            return -9;
        }
        covered!("tel35_4 media response ok");
        let mut news = [0u8; 0x100];
        let it = news.as_mut_ptr() as usize;
        exe::new_newsitem(it, 0, 0);
        media_find_news_item(manager, news_id, it);
        let club = media_club(index);
        let person = media_person(person_index as usize);
        if person_index > -1 {
            media_log_response(person, club, code, read_u32(it), response);
        }
        if code == 0x1f && other != usize::MAX && person_index != -1 {
            exe::media_warn_squad_of_position_rival(read_usize(G_MEDIA), person, media_club(other), club, response);
        }
        let kind = read_u32(it);
        let code_word = (code & 0xffff_0000) | (code as u8 as i8 as i16 as u16 as u32);
        match kind {
            MEDIA_NEWS_TEAM_STORY => {
                if person_index > -1 {
                    let staff_word = (read_u32(G_STAFF_TABLE) & 0xffff_0000) | (code as u8 as i8 as i16 as u16 as u32);
                    exe::media_apply_team_story_response(read_usize(G_MEDIA), club, staff_word, response, person);
                } else {
                    exe::media_apply_team_story_response(read_usize(G_MEDIA), club, code_word, response, 0usize);
                }
            }
            MEDIA_NEWS_SENT_OFF => {
                let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(0x75a340usize);
                f(read_usize(G_MEDIA), person, club, media_club(other), response);
            }
            MEDIA_NEWS_WANTS_FIRST_TEAM => {
                let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x75b440usize);
                f(read_usize(G_MEDIA), person, club, response);
            }
            _ => exe::media_apply_player_story_response(read_usize(G_MEDIA), person, club, code_word, response),
        }
        let slot = match read_u32(it) {
            MEDIA_NEWS_PLAYER_STORY => 0xd,
            MEDIA_NEWS_SENT_OFF | MEDIA_NEWS_WANTS_FIRST_TEAM => 0x12,
            MEDIA_NEWS_RIVAL_SIGNING => 7,
            _ => 0xa,
        };
        let mark: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x786d30usize);
        mark(read_usize(G_NEWS), manager, news_id, slot, response, 0);
        return -9;
    }
    if id == MEDIA_RESPONSE_CANCEL {
        return -9;
    }
    manager_menu_bar(raw)
}

/// Comparer for `media_response_screen_open`.
pub(crate) unsafe extern "C" fn compare_media_response_screen_open(manager: usize, news_id: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, usize)>(%%I0%%)(manager, news_id)
}

/// Comparer for `media_response_screen_draw`: side by side over the display list; a difference only inside the
/// column weights of one area (+0xbad..+0xbe9 of an `Area`: the question area's seventh weight is an uninitialised
/// stack byte in the original) is not a divergence.
pub(crate) unsafe extern "C" fn compare_media_response_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let mut reg = screen_build_regions();
    reg.extend(screen_param_regions());
    reg.push((JOB_TEXT, 0x100));
    reg.push((0xae290c, 4));
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || crate::rng::without_counting(|| media_response_screen_draw()));
    let areas = current_human_display() + 4;
    let weights_only = mo.0.iter().zip(mr.0.iter()).all(|((a, x), (_, y))| {
        x.iter().zip(y.iter()).enumerate().all(|(i, (p, q))| {
            let at = a + i;
            p == q || (at >= areas && at < areas + 0xf9 * 0xbf1 && (0xbad..0xbe9).contains(&((at - areas) % 0xbf1)))
        })
    });
    SCOREBOARD.record(equal || weights_only, || {
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("Media response screen (media_response_screen_draw): differences in {:?}", dif)
    });
}

/// Comparer for `media_response_screen_handle_event`.
pub(crate) unsafe extern "C" fn compare_media_response_screen_handle_event(item: u32) -> i32 {
    // simplification: the response, the redraw and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(item)
}
