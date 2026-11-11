use super::*;

/// Picks a random pundit: type 1 on Sunday becomes 7; local types (2, 4): the club's pundits; the others: of the type
/// and nation (−1 any); up to 20, random(n) and the id, or −1.
///
/// Ret 0xc; 0x1094-byte frame through 0x945610: `PunditList::pick_random_pundit(club, type, nation)`, from the press
/// stories (`exe::media_create_player_media_story`, 0x733a60) and from the news texts. Type 1 on Sunday (0x54a3f0 = 6) becomes 7. Types 2 and 4
/// (local): those of list `[this+4]` whose +0xcb is the id of `[[club+0x69]+0x38]` (none: −1); the others: those of
/// list `[this]` of the type (+0xca) and the nation (+0xcf; −1 accepts any). Up to 20; none: −1; otherwise the id (word
/// +0, signed) of one of them by random(n).
/// Original: `PunditList::pick_random_pundit` (0x7dab90, printouts.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn pick_random_pundit(this: usize, club: usize, kind: u32, nation: i32) -> i32 {
    let club = Club(club);
    let mut t = kind as i8;
    if t == 1 {
        if exe::date_get_weekday(G_CURRENT_DATE) as i16 == 6 {
            t = 7;
        }
    }
    let mut ids = [0i16; 20];
    let mut n = 0usize;
    if t == 2 || t == 4 {
        let est = Stadium(club.stadium().addr());
        if est.is_null() || est.city() == 0 {
            return -1;
        }
        let l = read_usize(this + 4);
        list_rewind(l);
        if read_i16_as_i32(l) <= 0 {
            return -1;
        }
        let mut i = 0;
        loop {
            let p = contract_manager_list_next(l);
            if p == 0 {
                break;
            }
            if read_u32(p + pundit::CITY_ID) == read_u32(club.stadium().city()) { // cpz10: o comentarista local
                covered!("cpz_10 local commentator");
                ids[n] = read_i16_as_i32(p) as i16;
                n += 1;
                if n >= 20 {
                    break;
                }
            }
            i += 1;
            if i >= read_i16_as_i32(read_usize(this + 4)) {
                break;
            }
        }
    } else {
        let l = read_usize(this);
        list_rewind(l);
        if read_i16_as_i32(l) <= 0 {
            return -1;
        }
        let mut i = 0;
        loop {
            let p = contract_manager_list_next(l);
            if p == 0 {
                break;
            }
            if read_i8(p + pundit::MEDIA_TYPE) == t {
                let accepted = if nation == -1 {
                    covered!("cpz_10 commentator of any nation");
                    true // cpz10: any nation
                } else {
                    read_u32(p + pundit::NATION_ID) as i32 == nation
                };
                if accepted {
                    ids[n] = read_i16_as_i32(p) as i16;
                    n += 1;
                    if n >= 20 {
                        break;
                    }
                }
            }
            i += 1;
            if i >= read_i16_as_i32(read_usize(this)) {
                break;
            }
        }
    }
    if n == 0 {
        return -1;
    }
    covered!("cpz_10 commentator draw");
    let r = exe::utils_random(n as i32); // cpz10: the commentator random draw
    ids[r as usize] as i32
}

/// Default media outlet name by type: 2 local paper, 3 radio, 4 local radio, 5 magazine, else national paper.
pub(crate) fn default_media_name(kind: i8) -> usize {
    match kind {
        2 => 0xa24284,
        3 => 0xa794cc,
        4 => 0xa241f4,
        5 => 0xa794b8,
        _ => 0xa242cc,
    }
}

/// Returns the name (+2) and flag (+0xd3) of the pundit with the id, or the default outlet type name and flag 0.
///
/// Ret 0x10: `PunditList::get_pundit_name(type, id, name, flag)`, from the news texts. Without `name`: flag 0. Id −1:
/// the type's default name and flag 0. Otherwise the pundit with the id (word +0) in list `[this+4]` (types 2 and 4) or
/// `[this]`: the name (+2, sprintf "%s" 0x944c9f) and the flag (+0xd3); without him, the default name and flag 0.
/// Original: `PunditList::get_pundit_name` (0x7dad20, printouts.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn get_pundit_name(this: usize, kind: u32, id: u32, name: usize, mark: *mut u8) {
    if name == 0 {
        *mark = 0;
        return;
    }
    let sprintf: unsafe extern "C" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let t = kind as i8;
    let id = id as i16;
    if id != -1 {
        let l = read_usize(this + if t == 2 || t == 4 { 4 } else { 0 });
        list_rewind(l);
        if read_i16_as_i32(l) > 0 {
            let mut i = 0;
            loop {
                let p = contract_manager_list_next(l);
                if p == 0 {
                    break;
                }
                if read_i16_as_i32(p) as i16 == id {
                    sprintf(name, 0x991944, p + 2);
                    *mark = read_u8(p + pundit::GENDER);
                    return;
                }
                i += 1;
                if i >= read_i16_as_i32(l) {
                    break;
                }
            }
        }
    }
    sprintf(name, 0x991944, default_media_name(t));
    *mark = 0;
}

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy carries the branch. Without news both do not
// fazem nada.

