# Mutacoes (cx05_4), carreira de 8 ligas ate 2004-11-01 (os jogos da Copa da Asia de 13 a 29 de outubro de 2004).
# O texto nao volta a simulacao: a mutacao do texto e acusada pela comparacao (compare_text_writer); a do progresso pelo
# comparador lado a lado e pela classificacao final da estatistica.
# - o mata-mata do texto na fase 3: as quartas, semis e final ficam sem texto;
# - os registros do mata-mata na fase 3: os campeoes, vices e eliminados do mata-mata nao ganham registro no historico;
# - o 4o do grupo vira 7 e nao 11 nas estatisticas.
MUTACOES = [
    ('cx05_4_texto_mata_mata_3',
     '    let knockout = 2i8; // cx05_4: the Asian Cup knockout stage',
     '    let knockout = 3i8; // mutation'),
    ('cx05_4_registros_mata_mata_3',
     '    let knockout = 2i8; // cx05_4: the stage of the Asian Cup knockout records',
     '    let knockout = 3i8; // mutation'),
    ('cx05_4_quarto_do_grupo_7',
     '[[1, 3, 5, 0xb], [1, 1, 1, 9], [5, 5, 9, 0xb]]); // cx05_4: the Asian Cup codes',
     '[[1, 3, 5, 7], [1, 1, 1, 9], [5, 5, 9, 0xb]]); // mutation'),
]
