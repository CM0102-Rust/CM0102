# Mutacoes da leva134 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva134.py --carreira 2002-06-01 --confirmar 2001-10-01
MUTACOES = [
    ('i_zera29', "    *((d + match_day_fields::MARKED_GROUP_COUNT) as *mut u8) = 0;\n    *((d + match_day_fields::MARKED_COMPETITION_COUNT) as *mut u8) = 0;", "    *((d + match_day_fields::MARKED_GROUP_COUNT) as *mut u8) = 1;\n    *((d + match_day_fields::MARKED_COMPETITION_COUNT) as *mut u8) = 0;"),
    ('i_2b', "    pr(d + match_day_fields::HAS_INTEREST, 0);", "    pr(d + match_day_fields::HAS_INTEREST, 1);"),
    ('i_comp_zera', "            pr(comp + day_competition::INTERESTING, 0);\n            pr(comp + day_competition::HAS_MARKED, 0);", "            pr(comp + day_competition::INTERESTING, 1);\n            pr(comp + day_competition::HAS_MARKED, 0);"),
    ('i_16', "for k in 0..16 {\n                    let pe", "for k in 0..15 {\n                    let pe", "interesse humano na competicao"),
    ('i_k', "(ler(0xae2364) + k - 16)", "(ler(0xae2364) + k - 15)", "interesse humano na competicao"),
    ('i_outro', "|| (other != 0 && in_competition(object, other) != 0)", "|| false", "interesse humano na competicao"),
    ('i_marca_comp', "crate::comparison::branch(\"interesse humano na competicao\");\n                        pr(comp + day_competition::INTERESTING, 1);", "crate::comparison::branch(\"interesse humano na competicao\");\n                        pr(comp + day_competition::INTERESTING, 0);", "interesse humano na competicao"),
    ('i_6b61e0', "crate::comparison::branch(\"interesse pela 6b61e0\");\n                    pr(comp + day_competition::INTERESTING, 1);", "crate::comparison::branch(\"interesse pela 6b61e0\");\n                    pr(comp + day_competition::INTERESTING, 0);", "interesse pela 6b61e0"),
    ('i_byte44', "human(g(m + day_match::HOME_CLUB), 0, *((m + day_match::HOME_SIDE_MARK) as *const u8) as u32, 0, 0)", "human(g(m + day_match::HOME_CLUB), 0, *((m + day_match::AWAY_SIDE_MARK) as *const u8) as u32, 0, 0)"),
    ('i_5fe420', "|| f5fe420(g(0xb63c98), m + 4) != 0;", "|| f5fe420(g(0xb63c98), m + 8) != 0;"),
    ('i_marcada', "pr(m + day_match::MARKED, 1);", "pr(m + day_match::MARKED, 2);", "interesse partida marcada"),
]
