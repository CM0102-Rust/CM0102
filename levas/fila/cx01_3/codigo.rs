// ---------- cx01_3: the promotion and relegation override news and a club's progress in the statistics (comp.cpp) ----------

/// Posts the news (type 0x23) of a club relegated because of its parent club, or refused promotion, to the managers of
/// this competition's division and of the other competition's.
///
/// `Comp::send_promotion_relegation_override_news(other, club, parent, kind)` (thiscall, ret 0x10; SEH frame only for
/// the local news item). Without a club, the comp.cpp error box (line 0x897) and nothing else. Otherwise a local news
/// item 0x23 (`exe::new_newsitem`) with slot 0 this competition's id, 1 the club's id, 2 the parent club's id and 3 the
/// other competition record's id (−1 for none), 4 the kind (sign-extended byte: 1 relegated because of the parent
/// club, 2 not promoted); delivered to this competition's division managers (`exe::news_send_to_division_managers`)
/// and, with another competition, to its division's too.
/// Original: `Comp::send_promotion_relegation_override_news` (0x48ece0, comp.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn comp_send_promotion_relegation_override_news(this: usize, other: usize, club: usize, parent: usize, kind: u32) {
    if club == 0 {
        show_error_at(0x99b80c, 0x897); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\comp.cpp"
        return;
    }
    covered!("cx01_3 override news");
    let comp = League(this).competition();
    let id_or_none = |p: usize| if p != 0 { read_u32(p) } else { u32::MAX };
    let mut n = [0u8; 0xe8];
    let p = n.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x23, 0);
    exe::newsitem_set_slot(p, 0, read_u32(comp.addr()));
    exe::newsitem_set_slot(p, 1, read_u32(club));
    exe::newsitem_set_slot(p, 2, id_or_none(parent));
    exe::newsitem_set_slot(p, 3, id_or_none(other));
    exe::newsitem_set_slot(p, 4, kind as u8 as i8 as i32 as u32);
    exe::news_send_to_division_managers(read_usize(G_NEWS), p, comp);
    if other != 0 {
        exe::news_send_to_division_managers(read_usize(G_NEWS), p, other);
    }
}

/// The competition record with id `id` (`G_CLUB_COMP_TABLE`, 0x6b bytes each).
unsafe fn competition_record(id: u32) -> Competition {
    Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(0x6b)))
}

/// Writes the promotion and relegation override news (type 0x23) into Text `out`.
///
/// `Comp::write_promotion_relegation_override_news_text(news, body, out)` (thiscall, ret 0xc). The news slots: 1 the
/// club index, 2 the parent club index, 3 the other competition's id, 4 the kind (byte). Kind 1, a reserve club
/// relegated because of its parent club: the body has the club, this competition's long name, the parent club and the
/// other competition's short name (format 0x99bf24), with links to the club and the parent club (0x7d5) and to both
/// competitions (0x7d0, the other one's record as the data); the headline has the club and this competition's short
/// name (0x99be0c), linked. Kind 2, a promotion refused: the body has the club and this competition's long name
/// (0x99be70), linked; the headline only the club (0x99bddc), linked. Any other kind writes nothing. The lines go to
/// the global text buffer 0xde1f64 (`FN_LANGLIB_SPRINTF`) with the genders first, then `exe::text_set_string`.
/// Defect of the original: in the kind 1 body this competition's long name goes with its short name's gender (+0x52).
/// Original: `Comp::write_promotion_relegation_override_news_text` (0x48ee40, comp.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_write_promotion_relegation_override_news_text(this: usize, news: usize, body: u32, out: usize) {
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let kind = exe::newsitem_get_slot(news, 4) as u8;
    if kind != 1 && kind != 2 {
        return;
    }
    let comp = League(this).competition();
    let c = club_at(exe::newsitem_get_slot(news, 1) as i32 as isize);
    if body != 0 && kind == 1 {
        covered!("cx01_3 override text: reserve club relegated");
        let parent = club_at(exe::newsitem_get_slot(news, 2) as i32 as isize);
        let other = competition_record(exe::newsitem_get_slot(news, 3));
        let (team, rival) = (c.addr() + club::SHORT_NAME, parent.addr() + club::SHORT_NAME);
        let (cg, pg) = (c.short_gender_name() as i8 as i32, parent.short_gender_name() as i8 as i32);
        let kg = comp.short_gender_name() as i8 as i32; // the defect: the long name below goes with this gender
        let og = other.short_gender_name() as i8 as i32;
        lang_sprintf(0xde1f64, 0x99bf24, cg, cg, kg, kg, pg, pg, og, og, team, comp.name_addr(), rival, other.short_name_addr());
        exe::text_set_string(out, 0xde1f64 as *const u8);
        exe::text_add_link(out, team as *const u8, 0x7d5, c);
        exe::text_add_link(out, rival as *const u8, 0x7d5, parent);
        exe::text_add_link(out, comp.name_addr() as *const u8, 0x7d0, comp);
        exe::text_add_link(out, other.short_name_addr() as *const u8, 0x7d0, other);
        return;
    }
    let team = c.addr() + club::SHORT_NAME;
    let cg = c.short_gender_name() as i8 as i32;
    let name = match (body != 0, kind) {
        (true, _) => {
            covered!("cx01_3 override text: promotion refused");
            lang_sprintf(0xde1f64, 0x99be70, cg, cg, comp.gender_name() as i8 as i32, comp.gender_name() as i8 as i32, team, comp.name_addr());
            Some(comp.name_addr())
        }
        (false, 1) => {
            covered!("cx01_3 override text: relegated headline");
            let sg = comp.short_gender_name() as i8 as i32;
            lang_sprintf(0xde1f64, 0x99be0c, cg, cg, sg, sg, team, comp.short_name_addr());
            Some(comp.short_name_addr())
        }
        _ => {
            covered!("cx01_3 override text: not promoted headline");
            lang_sprintf(0xde1f64, 0x99bddc, cg, cg, team);
            None
        }
    };
    exe::text_set_string(out, 0xde1f64 as *const u8);
    exe::text_add_link(out, team as *const u8, 0x7d5, c);
    if let Some(name) = name {
        exe::text_add_link(out, name as *const u8, 0x7d0, comp);
    }
}

/// Stores a club's three stage codes in the competition statistics: a stage (`STAGE_INDEX` not 0xff) passes the call on
/// to its parent competition's `SET_CLUB_PROGRESS` (the object of its competition id in `G_COMP_OBJECT_TABLE`, not
/// checked for null); otherwise the club's entry in the statistics (`exe::comp_stats_find_club_entry`) gets the codes as
/// its current, best and worst positions. The second argument is not read.
/// Original: `Comp::set_club_progress` (0x48e380, comp.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn comp_set_club_progress(this: usize, club: usize, arg: u32, current: u32, best: u32, worst: u32) {
    let comp = League(this);
    if comp.stage_index() != 0xff {
        covered!("cx01_3 progress to the parent");
        let parent = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp.competition().addr()) as usize * 4);
        let set: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(parent) + comp_vtable::SET_CLUB_PROGRESS));
        set(parent, club, arg, current, best, worst);
        return;
    }
    covered!("cx01_3 progress in the statistics");
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(comp.comp_stats(), club));
    entry.set_current_position(current as u8);
    entry.set_best_position(best as u8);
    entry.set_worst_position(worst as u8);
}

/// Comparer for `comp_send_promotion_relegation_override_news`: it delivers the news (cannot run twice); only the
/// original runs.
pub(crate) unsafe extern "thiscall" fn compare_comp_send_promotion_relegation_override_news(this: usize, other: usize, club: usize, parent: usize, kind: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(%%I0%%)(this, other, club, parent, kind)
}

/// Comparer for `comp_write_promotion_relegation_override_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_comp_write_promotion_relegation_override_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, body, out), |t| comp_write_promotion_relegation_override_news_text(this, news, body, t),
        || format!("comp_write_promotion_relegation_override_news_text({:#x}, news {:#x}, body {})", this, news, body));
}

/// Comparer for `comp_set_club_progress`: the three position bytes of the club's entry in the statistics of the
/// competition that keeps them (this one, or the parent for a stage) side by side; with no statistics or no entry,
/// only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_comp_set_club_progress(this: usize, club: usize, arg: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    let comp = League(this);
    let owner = if comp.stage_index() != 0xff {
        League(read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp.competition().addr()) as usize * 4))
    } else {
        comp
    };
    let stats = if owner.is_null() { 0 } else { owner.comp_stats() };
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, arg, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, arg, current, best, worst),
        || comp_set_club_progress(this, club, arg, current, best, worst));
    SCOREBOARD.record(same, || format!("comp_set_club_progress({:#x}, club {:#x}, {}, {}, {}): original {:x?} rust {:x?}", this, club, current as u8, best as u8, worst as u8, mo, mr));
}
