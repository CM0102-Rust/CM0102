# Mutacoes (cx07_7), carreira de 8 ligas ate 2004-11-01 (o sorteio olimpico em 20 de marco de 2004, os grupos de 13 a
# 19 de setembro).
# - o grupo A com as sorteadas na ordem inversa das linhas (cada selecao cai noutra vaga da tabela de rodadas: outros
#   jogos em cada data);
# - os grupos B a D com as selecoes na ordem inversa do vetor (o mesmo efeito nas fases).
# Acusadas pela carreira (os sub-23 das selecoes jogam outras partidas, com cansaco e lesoes nas ligas).
# As posicoes previstas ficam sem mutacao: vao so para as estatisticas da competicao (a expectativa de cada selecao), que
# nada da carreira le ate a data; o comparador confere a original contra o plano.
MUTACOES = [
    ('cx07_7_grupo_a_invertido',
     '        let drawn = read_usize(read_usize(this + OLYMPICS_DRAWN_CLUBS) + i as usize * 4); // cx07_7: drawn entry i in row i',
     '        let drawn = read_usize(read_usize(this + OLYMPICS_DRAWN_CLUBS) + (3 - i) as usize * 4); // mutation'),
    ('cx07_7_grupos_b_a_d_invertidos',
     'write_usize(teams + k * 4, read_usize(read_usize(this + OLYMPICS_DRAWN_CLUBS) + (first + k) * 4));',
     'write_usize(teams + k * 4, read_usize(read_usize(this + OLYMPICS_DRAWN_CLUBS) + (first + 3 - k) * 4)); // mutation'),
]
