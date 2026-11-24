# Mutacoes (cx09_12) pela carreira com a Belgica ate 2002-08-01 (a tabela e as regras sao montadas na criacao do jogo e
# de novo na virada de julho de 2002; as posicoes previstas tambem):
# - 2 pontos por vitoria (e nao 3): a classificacao dos grupos e dos periodos muda (o comparador das regras acusa);
# - a tabela do grupo A em ordem inversa: o calendario e os jogos mudam;
# - a posicao prevista step x k + 2: o comparador das posicoes acusa (a mesma conta serve ao cx09_14, na Finlandia).
MUTACOES = [
    ('cx09_12_dois_pontos',
     'unsafe fn bel_third_rules() -> Vec<(usize, u8, u32)> {\n    vec![(0x3c, 2, 2), (0xc2, 1, 3),',
     'unsafe fn bel_third_rules() -> Vec<(usize, u8, u32)> {\n    vec![(0x3c, 2, 2), (0xc2, 1, 2), // mutation\n'),
    ('cx09_12_tabela_invertida',
     '            exe::league_init_table_row(this, row as u32, club.addr(), 0, 0);',
     '            exe::league_init_table_row(this, 15u8.wrapping_sub(row) as u32, club.addr(), 0, 0); // mutation'),
    ('cx09_12_prevista_mais_um',
     '            emit(read_usize(list + 4 * k as usize), (k as u8).wrapping_mul(step).wrapping_add(1));',
     '            emit(read_usize(list + 4 * k as usize), (k as u8).wrapping_mul(step).wrapping_add(2)); // mutation'),
]
