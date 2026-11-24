# Mutacoes (cx07_5), carreira de 8 ligas ate 2004-11-01 (o futebol olimpico de setembro de 2004).
# O texto nao volta a simulacao: a mutacao do texto e acusada pela comparacao (compare_text_writer); a do progresso pelo
# comparador lado a lado.
# - o texto do mata-mata na fase 2: as quartas, semis e final ficam sem texto (e o grupo D ganha o de mata-mata);
# - os registros do mata-mata na fase 2 (duvida: o historico dos tecnicos pode nao voltar a carreira ate a data);
# - o 4o do grupo vira 11 e nao 13 nas estatisticas.
MUTACOES = [
    ('cx07_5_texto_mata_mata_2',
     '    let knockout = OLYMPICS_KNOCKOUT; // cx07_5: the stage of the Olympic knockout texts',
     '    let knockout = 2i8; // mutation'),
    ('cx07_5_registros_mata_mata_2',
     '    let knockout = OLYMPICS_KNOCKOUT; // cx07_5: the stage of the Olympic knockout records',
     '    let knockout = 2i8; // mutation'),
    ('cx07_5_quarto_do_grupo_11',
     '[[5, 9, 9, 0xd], [1, 1, 9, 0xd], [5, 9, 9, 0xd]]); // cx07_5: the Olympic codes',
     '[[5, 9, 9, 0xb], [1, 1, 9, 0xd], [5, 9, 9, 0xd]]); // mutation'),
]
