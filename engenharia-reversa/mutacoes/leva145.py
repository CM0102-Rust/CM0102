# Mutacoes da leva145 (ver docs/ROTINAS.md). So pela carreira (o laco do motor nao tem comparador lado a lado):
# python mutacoes.py mutacoes\leva145.py --carreira 2002-06-01
MUTACOES = [
    ('l_limpeza', "*((f + match_player::POSITION) as *mut u8) = 0xfb;", "*((f + match_player::POSITION) as *mut u8) = 0x05;"),
    ('l_pontape', "narrate(if *w(engine::CLOCK) > 1 { 0x1f74 } else { 0x212f }", "narrate(if *w(engine::CLOCK) > 1 { 0x1f74 } else { 0x1f74 }"),
    ('l_tentativas', "let c = random(0x14) as i8;", "let c = random(0x13) as i8;"),
    ('l_sorteio5', "} else if random(5) == 0 && call_thiscall0(FN_CM3_MATCH_ENGINE_AWARD_PENDING_FOUL, m)", "} else if random(6) == 0 && call_thiscall0(FN_CM3_MATCH_ENGINE_AWARD_PENDING_FOUL, m)"),
    ('l_sorteio7', "            } else if random(7) == 0 {\n                let v = if byte(m, engine::PENDING_PLAY_KIND)", "            } else if random(8) == 0 {\n                let v = if byte(m, engine::PENDING_PLAY_KIND)"),
    ('l_direcao', "let d = if byte(m, engine::POSSESSION_SIDE) != 0 { 1 } else { 0xa };", "let d = if byte(m, engine::POSSESSION_SIDE) != 0 { 2 } else { 0xa };", "laco pick_restart_taker da bola parada"),
    ('l_acrescimos', "let v = random(match_pl_ftol(r as f64 * read_f64_at(0x9689b0)));", "let v = random(match_pl_ftol(r as f64 * read_f64_at(0x9689b0) * 1.5));"),
    ('l_cansou', "if random(match_pl_ftol(read_f32_at(0x96d2c0) as f64 - v * read_f32_at(0x968960) as f64)) == 0 {", "if random(match_pl_ftol(read_f32_at(0x96d2c0) as f64 - v * read_f32_at(0x968960) as f64)) == 1 {"),
    ('l_104', "if !cansou || byte(q, match_player::MOMENT_PACE) as i32 > random(10) {", "if !cansou || byte(q, match_player::MOMENT_PACE) as i32 > random(11) {"),
    ('l_lista', "if (action == 0x104 || action == 0x76) && i < 0x16 {", "if (action == 0x104 || action == 0x76) && i < 0x15 {", "laco entra na lista"),
    ('l_meio', "} else if byte(m, engine::POSSESSION_SIDE) == side && if side == 0 { by >= 3 } else { by <= 8 } {", "} else if byte(m, engine::POSSESSION_SIDE) == side && if side == 0 { by >= 4 } else { by <= 8 } {"),
    ('l_area', "let a1 = (2..=6).contains(&bx) && (10..=11).contains(&by);", "let a1 = (2..=6).contains(&bx) && (9..=11).contains(&by);"),
    ('l_guarda', "for (lado, campo) in [(0u32, 0x8eac), (1, 0x8ead)] {", "for (lado, campo) in [(0u32, 0x8ead), (1, 0x8eac)] {"),
    ('l_trinta', "*w(engine::CLOCK) = (*w(engine::CLOCK) as i32 / 30 * 30) as i16;", "*w(engine::CLOCK) = (*w(engine::CLOCK) as i32 / 15 * 15) as i16;", "laco fim para os penalties"),
    ('l_agregado', "*tb(fixture::AWAY_AGGREGATE) = (*tb(fixture::AWAY_AGGREGATE)).wrapping_add(*tb(fixture::AWAY_GOALS_EXTRA_TIME));", "*tb(fixture::AWAY_AGGREGATE) = (*tb(fixture::AWAY_AGGREGATE)).wrapping_add(*tb(fixture::HOME_GOALS_EXTRA_TIME));"),
    ('l_vagas', "            if k < 3 {\n                *((target + match_player::MARKERS + k as usize * 4) as *mut usize) = q;", "            if k < 2 {\n                *((target + match_player::MARKERS + k as usize * 4) as *mut usize) = q;"),
    ('l_pressao_area', "if *((m + 0xb049) as *const u32) & 0x1000 != 0 {", "if *((m + 0xb049) as *const u32) & 0x2000 != 0 {"),
    ('l_disputa', "if byte(m, engine::BALL_TRANSIT_TICKS) == 0 && byte(m, engine::BALL_HEIGHT) == 4 && u(engine::BALL_RECEIVER) == 0 {", "if byte(m, engine::BALL_TRANSIT_TICKS) == 0 && byte(m, engine::BALL_HEIGHT) == 3 && u(engine::BALL_RECEIVER) == 0 {", "laco contest marcada"),
]
