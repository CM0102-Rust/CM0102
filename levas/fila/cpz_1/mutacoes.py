# Mutacoes (cpz_1). Os mata-matas da First, da Second e da Third sao montados quando a fase de cada uma passa de -1 a
# 0, logo depois da ultima rodada da temporada regular (fim de abril / comeco de maio de 2002, antes de 2002-06-01).
# As tres trocam quem joga contra quem: outras chaves, outro mandante no primeiro jogo ou outro clube no mata-mata;
# outros jogos (sorteios da partida, receitas, cansaco e moral dos elencos), que a carreira le. Cada uma numa das
# tres rotinas (a ancora e o arranjo de linhas de cada uma, com o comentario do item); os contadores ficam no corpo
# comum, logo depois da lista gravada (a unica condicao antes e a memoria da lista), com o nome de cada divisao.
MUTACOES = [
    ('cpz1_first_chaves_trocadas',
     'cpz1_mata_mata(this, [5, 2, 4, 3], 0x9c4374, 0x2f0, 0x306, Some(0xa0), "cpz_1 mata-mata da first"); // cpz1: 6º × 3º e 5º × 4º da First',
     'cpz1_mata_mata(this, [5, 4, 2, 3], 0x9c4374, 0x2f0, 0x306, Some(0xa0), "cpz_1 mata-mata da first"); // mutacao: 6º × 5º e 3º × 4º',
     'cpz_1 mata-mata da first'),
    ('cpz1_second_setimo_no_lugar_do_sexto',
     'cpz1_mata_mata(this, [5, 2, 4, 3], 0x9c43e8, 0x2b6, 0x2cc, None, "cpz_1 mata-mata da second"); // cpz1: 6º × 3º e 5º × 4º da Second',
     'cpz1_mata_mata(this, [6, 2, 4, 3], 0x9c43e8, 0x2b6, 0x2cc, None, "cpz_1 mata-mata da second"); // mutacao: o 7º no lugar do 6º',
     'cpz_1 mata-mata da second'),
    ('cpz1_third_mandos_trocados',
     'cpz1_mata_mata(this, [6, 3, 5, 4], 0x9c4424, 0x2be, 0x2d5, None, "cpz_1 mata-mata da third"); // cpz1: 7º × 4º e 6º × 5º da Third',
     'cpz1_mata_mata(this, [3, 6, 4, 5], 0x9c4424, 0x2be, 0x2d5, None, "cpz_1 mata-mata da third"); // mutacao: o mais bem colocado manda o primeiro jogo',
     'cpz_1 mata-mata da third'),
]
