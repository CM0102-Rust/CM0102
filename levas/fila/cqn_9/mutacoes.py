# Mutacoes (cqn_9). O mata-mata de cada estadual e montado quando a fase passa de -1 a 0, logo depois da ultima
# rodada dos pontos corridos (duas vezes na carreira de 8 ligas ate 2002-06-01). Cada uma troca quem joga contra quem
# (outras chaves, outro mandante no primeiro jogo ou outro clube no mata-mata) e mexe so nas linhas: as sementes ficam
# as da original, entao o par clube-semente muda mesmo se o sorteio da rodada seguir as sementes; outros jogos, que a carreira le
# (o modelo da cpz_1, acusada inteira na 262a). O contador fica no corpo comum, logo depois da lista gravada (a
# unica match_condition antes e a memoria da lista), com o nome de cada estadual.
# Prova do agente (05/11, carreira Rust de 8 ligas ate 2002-06-01): as tres ACUSADAS (Gaucho em 2001-147, Centro-Oeste e Goiano em 2001-133).
MUTACOES = [
    ('cqn9_gaucho_chaves_trocadas',
     'create_knockout_from_table(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98ae50, 0x217, 0x22b, "cqn_9 gaucho"); // cqn9: 1º × 4º, 2º × 3º Gaúcho',
     'create_knockout_from_table(this, &[0, 2, 1, 3], &[0, 3, 1, 2], 0x98ae50, 0x217, 0x22b, "cqn_9 gaucho"); // mutacao: 1º × 3º e 2º × 4º',
     'cqn_9 gaucho'),
    ('cqn9_centro_oeste_chaves_trocadas',
     'create_knockout_from_table(this, &[0, 7, 2, 5, 6, 1, 4, 3], &[0, 7, 2, 5, 6, 1, 4, 3], 0x98ae10, 0x21c, 0x234, "cqn_9 centro_oeste"); // cqn9: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º Centro-Oeste',
     'create_knockout_from_table(this, &[0, 6, 2, 5, 7, 1, 4, 3], &[0, 7, 2, 5, 6, 1, 4, 3], 0x98ae10, 0x21c, 0x234, "cqn_9 centro_oeste"); // mutacao: o 7º e o 8º trocados',
     'cqn_9 centro_oeste'),
    ('cqn9_goiano_chaves_trocadas',
     'create_knockout_from_table(this, &[0, 7, 2, 5, 6, 1, 4, 3], &[0, 7, 2, 5, 6, 1, 4, 3], 0x98ae90, 0x214, 0x22c, "cqn_9 goiano"); // cqn9: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º Goiano',
     'create_knockout_from_table(this, &[0, 7, 2, 5, 6, 1, 8, 3], &[0, 7, 2, 5, 6, 1, 4, 3], 0x98ae90, 0x214, 0x22c, "cqn_9 goiano"); // mutacao: o 9º no lugar do 5º',
     'cqn_9 goiano'),
]
