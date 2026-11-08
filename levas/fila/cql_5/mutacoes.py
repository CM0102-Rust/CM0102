# Mutacoes (cql_5), no fim da liga da Serie C1 (maio de 2002, na carreira de 8 ligas): o play-off e o play-out sao
# criados uma vez em cada grupo e jogados antes de 2002-06-01.
# - o play-off de acesso com o 6o no lugar do 5o (outro clube joga);
# - o play-out com o 13o no lugar do 14o;
# - as posicoes do play-off (que o construtor da fase usa para o mando) trocadas.
MUTACOES = [
    ('cql5_playoff_com_o_sexto',
     'const PROMOTION_PLAYOFF_ROWS: [usize; 4] = [4, 1, 3, 2]; // cql5: as linhas do play-off de acesso',
     'const PROMOTION_PLAYOFF_ROWS: [usize; 4] = [5, 1, 3, 2]; // mutacao: o 6o no lugar do 5o',
     'cql_5 league end stages'),
    ('cql5_playout_com_o_decimo_terceiro',
     'const PLAYOUT_ROWS: [usize; 4] = [16, 13, 15, 14]; // cql5: as linhas do play-out',
     'const PLAYOUT_ROWS: [usize; 4] = [16, 12, 15, 14]; // mutacao: o 13o no lugar do 14o',
     'cql_5 league end stages'),
    ('cql5_posicoes_trocadas',
     'let posicoes: [u8; 4] = [4, 1, 3, 2]; // cql5: as posições do play-off',
     'let posicoes: [u8; 4] = [1, 4, 2, 3]; // mutacao: as posicoes trocadas',
     'cql_5 league end stages'),
]
