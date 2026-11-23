use super::*;

/// Computes a 1..20 value from the person's word +4 and `nivel`: x = (w + 2 x nivel) x 0.05 + 10, then (x + 0.1 x^2) /
/// 3 + 0.5, clamped to 1..20 and truncated [inference: how well the person's attribute is known, fog of war].
pub(crate) unsafe extern "C" fn chance_to_know(person: usize, level: i8) -> i32 {
    let x = (read_i16(person + 4) as f64 + (level as f64 + level as f64)) * 0.05 + 10.0;
    let y = (x * x * 0.1 + x) * read_f64(rdata::F64_0_3333333333) + 0.5;
    ftol(if !(y > 1.0) { 1.0 } else if !(y < 20.0) { 20.0 } else { y })
}

/// Fog of war: records knowledge of `pessoa` (9-byte records in [this], by id): stamps today's date at +1 and raises
/// the knowledge level (+0). Without a scout/observer (`fonte` +0x69) the level becomes 3 (fully known); with one,
/// below level 3 a roll on the observer's judging rating decides whether the level goes up by one, jumps to 2 or to 3.
pub(crate) unsafe extern "thiscall" fn know_person(this: usize, person: usize, font: usize) {
    let font = Staff(font);
    if person == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let reg = read_usize(this).wrapping_add((read_i32(person) as usize).wrapping_mul(9));
    let mut data = [0u8; 8];
    exe::date_operator_3d(reg + 1, data.as_mut_ptr() as usize, G_CURRENT_DATE);
    let obs = NonPlayer(if font.is_null() { 0 } else { font.non_player().addr() });
    let level = reg as *mut u8;
    if obs.is_null() {
        *level = 3;
        return;
    }
    if *level == 3 {
        return;
    }
    let c = exe::non_player_attribute_rating(obs, obs.judgement()) as i8;
    let r = random(c as i32) as i8;
    *level = if c < 15 && r == 0 {
        (*level).wrapping_add(1)
    } else if *level == 0 && r < 6 {
        2
    } else {
        3
    };
}

/// After a match (attribute masking / fog of war): for each side whose club is controlled by a human manager that has a
/// knowledge map, every valid player among the 20 lineup slots of the match data (substitutes only if used, +0x32 !=
/// 0xff) becomes known to that manager (`exe::fog_of_war_map_raise_knowledge`).
pub(crate) unsafe extern "stdcall" fn fog_after_match(fixture: usize, data: usize) {
    let n = || read_i32(G_STAFF_COUNT);
    for lado in 0..2usize {
        let club = Club(read_usize(fixture + if lado == 0 { 0x20 } else { 0x1c }));
        if exe::human_manager_is_human_club(club, 0, (1) as u32, (0) as u32, 0) == 0 {
            continue;
        }
        let q = club.manager().addr();
        if q == 0 {
            continue;
        }
        let k = n().wrapping_sub(read_i32(q)) as usize;
        let obj = read_usize(read_usize(G_FOG_OF_WAR).wrapping_add(k.wrapping_mul(4)).wrapping_sub(4));
        if obj == 0 {
            continue;
        }
        for vaga in 0..20usize {
            let e = MatchRecordRow(data + lado * match_record::SIDE_SIZE + match_record::ROW_STAFF_ID + vaga * match_record_row::SIZE);
            let i = read_i32(e.addr());
            if i == -1 || read_i32(e.addr() + 4) == -1 || (vaga > 10 && (e.minute_on() as u8) == 0xff) {
                continue;
            }
            let p = read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
            if p == 0 || read_i32(p) < n() {
                exe::fog_of_war_map_raise_knowledge(obj, p, 0);
            }
        }
    }
}

/// The daily fog of war update: each of the sixteen human manager maps that exists runs its daily knowledge update.
///
/// `this` holds 16 map pointers; each non-null one gets FOG_OF_WAR_MAP::daily_update (`FN_FOG_OF_WAR_MAP_DAILY_UPDATE`,
/// thiscall, ret 0), in slot order.
/// Original: `FOG_OF_WAR::daily_update` (0x5afce0, fog_of_war.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fog_of_war_daily_update(this: usize) {
    let update: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FOG_OF_WAR_MAP_DAILY_UPDATE);
    for k in 0..16 {
        let map = read_usize(this + k * 4);
        if map != 0 {
            covered!("mwg_17 fog of war map");
            update(map);
        }
    }
}

/// Comparer for `fog_of_war_daily_update`.
pub(crate) unsafe extern "thiscall" fn compare_fog_of_war_daily_update(this: usize) {
    let before: Vec<usize> = (0..16).map(|k| read_usize(this + k * 4)).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FOG_OF_WAR_DAILY_UPDATE)(this);
    let same = (0..16).all(|k| read_usize(this + k * 4) == before[k]);
    SCOREBOARD.record(same, || format!("fog_of_war_daily_update({:#x}): the maps changed", this));
}

/// FOG_OF_WAR_MAP::daily_update (thiscall, ret 0): one human manager's map's daily knowledge update.
pub(crate) const FN_FOG_OF_WAR_MAP_DAILY_UPDATE: usize = 0x5b0260;
