# Mutacoes da leva135 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva135.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('r_quinze', "[t - *w(engine::LAST_STANCE_CALL_TICK) as i32 > 0xa5, t - *w(engine::LAST_STANCE_CALL_TICK + 2) as i32 > 0xa5]", "[t - *w(engine::LAST_STANCE_CALL_TICK) as i32 > 0x50, t - *w(engine::LAST_STANCE_CALL_TICK + 2) as i32 > 0xa5]"),
    ('r_sem_folego', "if minute > 0x3c && random(6) == 0 {", "if minute > 0x3c && random(7) == 0 {", "restart out of breath"),
    ('r_folego', "read_f32(j + match_player::STAMINA) < 6750.0", "read_f32(j + match_player::STAMINA) < 6000.0", "restart out of breath"),
    ('r_postura', "if new != velha || *w(engine::CLOCK) <= 1 {", "if new != velha || *w(engine::CLOCK) <= 2 {"),
    ('r_acrescimo', "let r = random(10) as i8 as i32;", "let r = random(11) as i8 as i32;", "restart stoppage time"),
    ('r_cansar', "exe::match_player_scale_anger(j, (r as f64 * read_f64(rdata::F64_0_1111111111) + 1.0) as f32);", "exe::match_player_scale_anger(j, (r as f64 * read_f64(rdata::F64_0_1111111111) + 1.1) as f32);", "restart stoppage time"),
    ('r_sub', "(minute as i32) > random(0x32) + 10", "(minute as i32) > random(0x33) + 10", "restart AI substitution"),
    ('r_sub55', "*w(engine::CLOCK) as i32 - *w(last) as i32 > 0x37", "*w(engine::CLOCK) as i32 - *w(last) as i32 > 0x30", "restart AI substitution"),
    ('r_acresc_narr', "let v = if q <= 2 { 2 } else { q as i8 };", "let v = if q <= 2 { 3 } else { q as i8 };", "restart stoppage time narrated"),
    ('r_postura1', "let f = if (manager != 0 && read_f32(m.addr() + (0x476a + 4 * side)) < 2.5 && tb(non_player::ATTACKING) <= 6) || !attack { 0x20000 } else { 0x10000 };", "let f = if (manager != 0 && read_f32(m.addr() + (0x476a + 4 * side)) < 2.5 && tb(non_player::ATTACKING) <= 7) || !attack { 0x20000 } else { 0x10000 };", "restart stance 1"),
    ('r_postura2', "exe::formation_set_team_instruction(tobj, if (tb(non_player::DIRECTNESS) as i32) < lim(4) { 0x1000 } else { 0x800 });", "exe::formation_set_team_instruction(tobj, if (tb(non_player::DIRECTNESS) as i32) < lim(5) { 0x1000 } else { 0x800 });", "restart stance 2"),
    ('r_postura5', "exe::formation_set_team_instruction(tobj, if e > 0x10 && attack { 0x10000 } else if e > 0xc { 0x20000 } else { 0x8000 });", "exe::formation_set_team_instruction(tobj, if e > 0x10 && attack { 0x10000 } else if e > 0xd { 0x20000 } else { 0x8000 });", "restart stance 5"),
    ('r_canto', "if away || random(v1 - v2 + 5) == 0 {", "if away || random(v1 - v2 + 6) == 0 {", "restart goal_crowd_reaction chant"),
    ('r_bola_parada', "                5 => 3,\n                6 | 7 => 4,", "                5 => 3,\n                6 | 7 => 5,"),
]
