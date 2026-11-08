# Mutacoes da leva130 (ver docs/ROTINAS.md). Sem comparacao lado a lado: so pela carreira.
# Uso: python mutacoes.py mutacoes\leva130.py --carreira 2002-06-01
MUTACOES = [
    ('e_clube', "if (club != *((partida + fixture::HOME_CLUB) as *const usize) && club != *((partida + fixture::AWAY_CLUB) as *const usize)) || away(club) != 0 {", "if (club != *((partida + fixture::HOME_CLUB) as *const usize) && club != *((partida + fixture::HOME_CLUB) as *const usize)) || away(club) != 0 {"),
    ('e_0b', "kind_0b(person(read(ev)), club, partida);", "kind_0b(person(read(ev)), club, *((partida + fixture::HOME_CLUB) as *const usize));", "eventos 0x0b"),
    ('e_0a_marca', "por(c, 0x4000000, 0);", "por(c, 0x8000000, 0);", "eventos 0x0a"),
    ('e_0a_sorteio', "let b = random(5);\n                        if a < b || protegido", "let b = random(6);\n                        if a < b || protegido", "eventos 0x0a"),
    ('e_0a_limite', "let r = random(2000) + 0x1194;", "let r = random(2000) + 0x1000;", "eventos 0x0a"),
    ('e_1e_sorteio', "if a < random(10) {\n                            por(c, 0x40000, 0);", "if a < random(11) {\n                            por(c, 0x40000, 0);", "eventos 0x1e"),
    ('e_1e_tecnico', "adjust(club, 0xa, 0x32);", "adjust(club, 0xa, 0x33);", "eventos 0x1e tecnico"),
    ('e_1e_sem', "adjust(club, 0xc, 0x4b);", "adjust(club, 0xc, 0x4c);", "eventos 0x1e"),
    ('e_13_marca', "por_40(c, 0x40);", "por_40(c, 0x20);", "eventos 0x13 marca"),
    ('e_outros_idade', "} else if age < 0x20 {\n                            random(8)", "} else if age < 0x20 {\n                            random(9)", "eventos outros"),
    ('e_outros_tipo', "news(noticias(), p, club, kind as u32, partida, 0);", "news(noticias(), p, club, kind as u32 ^ 1, partida, 0);", "eventos outros"),
]
