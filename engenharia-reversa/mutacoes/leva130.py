# Mutacoes da leva130 (ver docs/ROTINAS.md). Sem comparacao lado a lado: so pela carreira.
# Uso: python mutacoes.py mutacoes\leva130.py --carreira 2002-06-01
MUTACOES = [
    ('e_clube', "if (clube != *((partida + 0x1c) as *const usize) && clube != *((partida + 0x20) as *const usize)) || away(clube) != 0 {", "if (clube != *((partida + 0x1c) as *const usize) && clube != *((partida + 0x1c) as *const usize)) || away(clube) != 0 {"),
    ('e_0b', "kind_0b(person(read(ev)), clube, partida);", "kind_0b(person(read(ev)), clube, *((partida + 0x1c) as *const usize));", "eventos 0x0b"),
    ('e_0a_marca', "por(c, 0x4000000, 0);", "por(c, 0x8000000, 0);", "eventos 0x0a"),
    ('e_0a_sorteio', "let b = sortear(5);\n                        if a < b || protegido", "let b = sortear(6);\n                        if a < b || protegido", "eventos 0x0a"),
    ('e_0a_limite', "let r = sortear(2000) + 0x1194;", "let r = sortear(2000) + 0x1000;", "eventos 0x0a"),
    ('e_1e_sorteio', "if a < sortear(10) {\n                            por(c, 0x40000, 0);", "if a < sortear(11) {\n                            por(c, 0x40000, 0);", "eventos 0x1e"),
    ('e_1e_tecnico', "adjust(clube, 0xa, 0x32);", "adjust(clube, 0xa, 0x33);", "eventos 0x1e tecnico"),
    ('e_1e_sem', "adjust(clube, 0xc, 0x4b);", "adjust(clube, 0xc, 0x4c);", "eventos 0x1e"),
    ('e_13_marca', "por_40(c, 0x40);", "por_40(c, 0x20);", "eventos 0x13 marca"),
    ('e_outros_idade', "} else if age < 0x20 {\n                            sortear(8)", "} else if age < 0x20 {\n                            sortear(9)", "eventos outros"),
    ('e_outros_tipo', "noticia(noticias(), p, clube, kind as u32, partida, 0);", "noticia(noticias(), p, clube, kind as u32 ^ 1, partida, 0);", "eventos outros"),
]
