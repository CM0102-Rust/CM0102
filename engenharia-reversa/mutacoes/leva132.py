# Mutacoes da leva132 (ver docs/ROTINAS.md). Sem comparacao lado a lado: so pela carreira.
# As acoes finais (0x5896d0, 0x589bc0, 0x547d90) nunca rodam nas carreiras automaticas: as mutacoes
# mexem nos filtros, que mudam quantos chegam aos sorteios.
# Uso: python mutacoes.py mutacoes\leva132.py --carreira 2002-06-01
MUTACOES = [
    ('c_100', "w(club + club::REPUTATION) < 0x128e && sortear(100) != 0 {", "w(club + club::REPUTATION) < 0x128e && sortear(2) != 0 {"),
    ('c_80', "w(club + club::REPUTATION) < 0x128e && sortear(100) != 0 {", "w(club + club::REPUTATION) < 0x1000 && sortear(100) != 0 {"),
    ('c_cf', "    if g(club + club::MANAGER) == 0 {\n        return;\n    }\n    if teste_600a50", "    if g(club + club::ASSISTANT_MANAGER) == 0 {\n        return;\n    }\n    if teste_600a50"),
    ('c_jovem', "|| byte(p, 0x18) > 0x14 {", "|| byte(p, 0x18) > 0x13 {", "clube segue"),
    ('c_59', "|| byte(p, 0x59) <= 0xf {", "|| byte(p, 0x59) <= 0xe {", "clube segue"),
    ('c_57', "if byte(p, 0x57) <= (r as u8).wrapping_add(10) as i8", "if byte(p, 0x57) <= (r as u8).wrapping_add(0) as i8", "clube jovem"),
    ('c_ficha', "|| w(g(p + staff::PLAYER) + 0xb) >= 0x109a {", "|| w(g(p + staff::PLAYER) + 0xb) >= 0x2000 {", "clube jovem"),
    ('c_lado', "let side = (read(club) != read(t)) as usize;", "let side = (read(club) == read(t)) as usize;", "clube segue"),
    ('c_n', "                && n > 2\n", "                && n > 0\n", "clube elenco"),
    ('c_elenco57', "&& byte(s, 0x57) > 0xf", "&& byte(s, 0x57) > 0x5", "clube elenco"),
    ('c_acao', "f547d90(p, club, 1);", "f547d90(p, club, 0);", "clube 547d90"),
]
