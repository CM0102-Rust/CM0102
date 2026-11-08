# Mutacoes da leva132 (ver docs/ROTINAS.md). Sem comparacao lado a lado: so pela carreira.
# As acoes finais (0x5896d0, 0x589bc0, 0x547d90) nunca rodam nas carreiras automaticas: as mutacoes
# mexem nos filtros, que mudam quantos chegam aos sorteios.
# Uso: python mutacoes.py mutacoes\leva132.py --carreira 2002-06-01
MUTACOES = [
    ('c_100', "w(club + club::REPUTATION) < 0x128e && random(100) != 0 {", "w(club + club::REPUTATION) < 0x128e && random(2) != 0 {"),
    ('c_80', "w(club + club::REPUTATION) < 0x128e && random(100) != 0 {", "w(club + club::REPUTATION) < 0x1000 && random(100) != 0 {"),
    ('c_cf', "    if g(club + club::MANAGER) == 0 {\n        return;\n    }\n    if test_600a50", "    if g(club + club::ASSISTANT_MANAGER) == 0 {\n        return;\n    }\n    if test_600a50"),
    ('c_jovem', "|| byte(p, staff::YEAR_OF_BIRTH) > 0x14 {", "|| byte(p, staff::YEAR_OF_BIRTH) > 0x13 {", "club continues"),
    ('c_59', "|| byte(p, staff::LOYALTY) <= 0xf {", "|| byte(p, staff::LOYALTY) <= 0xe {", "club continues"),
    ('c_57', "if byte(p, staff::AMBITION) <= (r as u8).wrapping_add(10) as i8", "if byte(p, staff::AMBITION) <= (r as u8).wrapping_add(0) as i8", "club youth"),
    ('c_ficha', "|| w(g(p + staff::PLAYER) + 0xb) >= 0x109a {", "|| w(g(p + staff::PLAYER) + 0xb) >= 0x2000 {", "club youth"),
    ('c_lado', "let side = (read(club) != read(t)) as usize;", "let side = (read(club) == read(t)) as usize;", "club continues"),
    ('c_n', "                && n > 2\n", "                && n > 0\n", "club squad"),
    ('c_elenco57', "&& byte(s, staff::AMBITION) > 0xf", "&& byte(s, staff::AMBITION) > 0x5", "club squad"),
    ('c_acao', "f547d90(p, club, 1);", "f547d90(p, club, 0);", "club 547d90"),
]
