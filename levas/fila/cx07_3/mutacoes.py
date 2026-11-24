# Mutacoes (cx07_3), carreira de 8 ligas ate 2002-12-31 (a Copa das Nacoes da Oceania de novembro de 2002).
# O texto nao volta a simulacao: a mutacao do texto e acusada pela comparacao (compare_text_writer); a do progresso pelo
# comparador lado a lado.
# - o texto do mata-mata na fase 2: as semifinais e a final ficam sem texto;
# - os registros do mata-mata na fase 2: campeao, vice, terceiro e eliminados do mata-mata sem registro no historico
#   (duvida: o historico dos tecnicos pode nao voltar a carreira ate a data; ver PRONTO.txt);
# - o 3o do grupo vira 7 e nao 5 nas estatisticas.
MUTACOES = [
    ('cx07_3_texto_mata_mata_2',
     '    let knockout = OCEANIA_NATIONS_KNOCKOUT; // cx07_3: the stage of the Oceania knockout texts',
     '    let knockout = 2i8; // mutation'),
    ('cx07_3_registros_mata_mata_2',
     '    let knockout = OCEANIA_NATIONS_KNOCKOUT; // cx07_3: the stage of the Oceania knockout records',
     '    let knockout = 2i8; // mutation'),
    ('cx07_3_terceiro_do_grupo_7',
     '[[1, 3, 5, 4], [1, 1, 5, 4], [3, 3, 5, 4]]); // cx07_3: the Oceania codes',
     '[[1, 3, 7, 4], [1, 1, 5, 4], [3, 3, 5, 4]]); // mutation'),
]
