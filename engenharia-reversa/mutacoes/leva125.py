# Mutacoes da leva125 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva125.py --jogo 2001-10-01
MUTACOES = [
    ('f_vazio', "        return 0;\n    }\n    if (m.restart_type() as u8) != 0 {", "        return 1;\n    }\n    if (m.restart_type() as u8) != 0 {"),
    ('f_marcada', "(aqui < marcada && side == 1)", "(aqui <= marcada && side == 1)"),
    ('f_oito', "        m.set_restart_type((7) as i8);\n        m.set_possession_side((zero(MatchPlayer(j)) as u8) as i8);", "        m.set_restart_type((6) as i8);\n        m.set_possession_side((zero(MatchPlayer(j)) as u8) as i8);"),
    ('f_codigo', "0x2d => 0x20fd,", "0x2d => 0x20fc,"),
    ('f_cartao', "6 if !sofreu => 0x1ff5,", "6 if !sofreu => 0x1ff6,"),
    ('f_area', "read_i8(m.pending_play_offender() + 0x27) == 1 { (10..=11).contains(&y) }", "read_i8(m.pending_play_offender() + 0x27) == 1 { (9..=11).contains(&y) }"),
    ('f_penalti', "*d = (*d).wrapping_sub(500);\n            sound = 0x1f7a;", "*d = (*d).wrapping_sub(499);\n            sound = 0x1f7a;"),
    ('f_parada', "matches!(k, 0 | 5..=13 | 0x3b | 0x3c)", "matches!(k, 0 | 5..=12 | 0x3b | 0x3c)"),
    ('f_nove', "if random(9) == 0 {\n        narrate(call, 0x20df", "if random(8) == 0 {\n        narrate(call, 0x20df"),
    ('f_lado', "let side_of = |p: MatchPlayer| if sofreu { zero(p) } else { ((p.side() as u8) as u32) };", "let side_of = |p: usize| if sofreu { (read_u8(p + match_player::SIDE) as u32) } else { zero(p) };"),
    ('p_achou', "                *((this.addr() + 0x1a6) as *mut u8) = 0;\n                *((this.addr() + engine::SIDE_PLAYED_BALL) as *mut u8) = 0;\n                return 1;", "                *((this.addr() + 0x1a6) as *mut u8) = 0;\n                *((this.addr() + engine::SIDE_PLAYED_BALL) as *mut u8) = 1;\n                return 1;"),
    ('f_cansar', "call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, m.pending_play_offender(), [0; 5]);", "call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, m.pending_play_offender(), [1, 0, 0, 0, 0]);"),
]
