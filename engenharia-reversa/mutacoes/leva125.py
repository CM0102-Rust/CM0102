# Mutacoes da leva125 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva125.py --jogo 2001-10-01
MUTACOES = [
    ('f_vazio', "        return 0;\n    }\n    if *b(engine::RESTART_TYPE) != 0 {", "        return 1;\n    }\n    if *b(engine::RESTART_TYPE) != 0 {"),
    ('f_marcada', "(aqui < marcada && side == 1)", "(aqui <= marcada && side == 1)"),
    ('f_oito', "        *b(engine::RESTART_TYPE) = 7;\n        *b(engine::POSSESSION_SIDE) = zero(j) as u8;", "        *b(engine::RESTART_TYPE) = 6;\n        *b(engine::POSSESSION_SIDE) = zero(j) as u8;"),
    ('f_codigo', "0x2d => 0x20fd,", "0x2d => 0x20fc,"),
    ('f_cartao', "6 if !sofreu => 0x1ff5,", "6 if !sofreu => 0x1ff6,"),
    ('f_area', "byte(u(engine::PENDING_PLAY_OFFENDER), 0x27) == 1 { (10..=11).contains(&y) }", "byte(u(engine::PENDING_PLAY_OFFENDER), 0x27) == 1 { (9..=11).contains(&y) }"),
    ('f_penalti', "*d = (*d).wrapping_sub(500);\n            som = 0x1f7a;", "*d = (*d).wrapping_sub(499);\n            som = 0x1f7a;"),
    ('f_parada', "matches!(k, 0 | 5..=13 | 0x3b | 0x3c)", "matches!(k, 0 | 5..=12 | 0x3b | 0x3c)"),
    ('f_nove', "if sortear(9) == 0 {\n        narrate(call, 0x20df", "if sortear(8) == 0 {\n        narrate(call, 0x20df"),
    ('f_lado', "let side_of = |p: usize| if sofreu { zero(p) } else { ub(p, match_player::SIDE) };", "let side_of = |p: usize| if sofreu { ub(p, match_player::SIDE) } else { zero(p) };"),
    ('p_achou', "                *((this + 0x1a6) as *mut u8) = 0;\n                *((this + 0x1a5) as *mut u8) = 0;\n                return 1;", "                *((this + 0x1a6) as *mut u8) = 0;\n                *((this + 0x1a5) as *mut u8) = 1;\n                return 1;"),
    ('f_cansar', "call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, u(engine::PENDING_PLAY_OFFENDER), [0; 5]);", "call(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, u(engine::PENDING_PLAY_OFFENDER), [1, 0, 0, 0, 0]);"),
]
