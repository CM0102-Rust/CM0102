# Mutacoes (cx05_10), carreira de 8 ligas ate 2004-07-15 (os jogos da Eurocopa de 2004 de 9 a 30 de junho). O texto
# nao volta a simulacao: a mutacao do texto e acusada pela comparacao (compare_text_writer); a do progresso pelo
# comparador lado a lado.
# - o mata-mata do texto na fase 2: as quartas, a semi e a final ficam sem texto;
# - o codigo do campeao vira 2 (o chamador le o byte: a noticia do campeao muda);
# - o lider do grupo vira 1 e nao 5 no codigo atual.
MUTACOES = [
    ('cx05_10_texto_mata_mata_2',
     '    let knockout = 3i8; // cx05_10: the European Championship knockout stage',
     '    let knockout = 2i8; // mutation'),
    ('cx05_10_codigo_do_campeao_2',
     "            write_u8(a4, 3); // cx05_10: the winner's fate code",
     "            write_u8(a4, 2); // mutation"),
    ('cx05_10_lider_do_grupo_1',
     '[[5, 9, 9, 0xd], [1, 1, 9, 0xd], [5, 9, 9, 0xd]]); // cx05_10: the European Championship codes',
     '[[1, 9, 9, 0xd], [1, 1, 9, 0xd], [5, 9, 9, 0xd]]); // mutation'),
]
