use super::*;

/// Computes a 1..20 value from the person's word +4 and `nivel`: x = (w + 2 x nivel) x 0.05 + 10, then (x + 0.1 x^2) /
/// 3 + 0.5, clamped to 1..20 and truncated [inference: how well the person's attribute is known, fog of war].
///
/// [pt] { 0x544410 }: x = (palavra +4 da pessoa + 2 x `nivel`) x 0,05 + 10; (x + 0,1 x^2) x (1/3) + 0,5,
/// limitado a 1..20 (NaN vira 1) e truncado.
pub(crate) unsafe extern "C" fn chance_to_know(person: usize, level: i8) -> i32 {
    let x = (read_i16(person + 4) as f64 + (level as f64 + level as f64)) * 0.05 + 10.0;
    let y = (x * x * 0.1 + x) * read_f64(rdata::F64_0_3333333333) + 0.5;
    ftol(if !(y > 1.0) { 1.0 } else if !(y < 20.0) { 20.0 } else { y })
}

/// Fog of war: records knowledge of `pessoa` (9-byte records in [this], by id): stamps today's date at +1 and raises
/// the knowledge level (+0). Without a scout/observer (`fonte` +0x69) the level becomes 3 (fully known); with one,
/// below level 3 a roll on the observer's judging rating decides whether the level goes up by one, jumps to 2 or to 3.
///
/// [pt] { 0x5b09d0 } (ret 8, fog_of_war.cpp): o que se sabe da pessoa (registros de 9 bytes em [this],
/// pelo id): grava a data de hoje (0xae2c90) em +1; sem observador (`fonte` +0x69) o nivel (+0)
/// vai a 3. Com ele e nivel abaixo de 3: c = 0x544410(observador, byte +0x17), sorteio(c) (byte);
/// c < 15 e sorteio 0: nivel + 1; senao nivel 0 e sorteio < 6: 2; senao 3.
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
///
/// [pt] { 0x5afe00 } (stdcall, fog_of_war.cpp): depois da partida, para cada lado (clube em +0x20 e
/// +0x1c) controlado por humano (0x600a50) com tecnico e quadro de conhecimento (0xae2a60, pelo
/// id do tecnico): as 20 vagas da escalacao (`dados` + lado x 0x625 + 0x190, 0x49 bytes cada) com
/// pessoa e +4 validos (reservas, da 11a em diante, so com +0x32 != 0xff) ficam conhecidas (0x5b09d0).
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

