# Mutacoes da leva123 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva123.py --jogo 2001-10-01
MUTACOES = [
    ('u_amarelo', "            exe::cm3_match_engine_referee_decide_booking(m, j, &mut t, cartao as i8, grav as i8, foul_strong as u8, &mut weight);\n            narrate(call, event::HAND_BALL, j.commentary_id() as u32 , (j.side() as u8) as u32 , u32::MAX);\n            let colour: i32 = match t {\n                2 => 0x1ff9,", "            exe::cm3_match_engine_referee_decide_booking(m, j, &mut t, cartao as i8, grav as i8, foul_strong as u8, &mut weight);\n            narrate(call, event::HAND_BALL, j.commentary_id() as u32 , (j.side() as u8) as u32 , u32::MAX);\n            let colour: i32 = match t {\n                2 => 0x1ff8,"),
    ('u_sem_cartao', "                if e.pending_incident_severity() as i32 > r && j.position() >= 0 { 0x210a } else { -1 }", "                if e.pending_incident_severity() as i32 >= r && j.position() >= 0 { 0x210a } else { -1 }"),
    ('u_tabela', "        let skips = v <= 0x3c && matches!(v, 0 | 5..=13 | 0x3b | 0x3c);", "        let skips = v <= 0x3c && matches!(v, 0 | 5..=12 | 0x3b | 0x3c);"),
    ('u_fora', "                    (weight as f64 * 0.16666666666666666 + 0.83) as f32", "                    (weight as f64 * 0.16666666666666666 + 0.84) as f32"),
    ('u_campo', "                    ((weight as f64 + 1.0) * 0.5) as f32", "                    ((weight as f64 + 1.0) * 0.6) as f32"),
    ('u_cansar', "                call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, j.addr(), [1, 0, 0, 0, 0]);", "                call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, j.addr(), [0; 5]);"),
    ('u_nove', "            if random(9) == 0 {", "            if random(8) == 0 {"),
    ('u_fim', "    m.set_punishment_state((2) as i8);", "    m.set_punishment_state((3) as i8);"),
]
