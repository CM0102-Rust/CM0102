# Mutacoes (cqn_7). O mata-mata de cada estadual e montado quando a fase passa de -1 a 0, logo depois da ultima
# rodada dos pontos corridos (duas vezes na carreira de 8 ligas ate 2002-06-01). Cada uma troca quem joga contra quem
# (outras chaves, outro mandante no primeiro jogo ou outro clube no mata-mata) e mexe so nas linhas: as sementes ficam
# as da original, entao o par clube-semente muda mesmo se o sorteio da rodada seguir as sementes; outros jogos, que a carreira le
# (o modelo da cpz_1, acusada inteira na 262a). O contador fica no corpo comum, logo depois da lista gravada (a
# unica condicao antes e a memoria da lista), com o nome de cada estadual.
# Prova do agente (05/11, carreira Rust de 8 ligas ate 2002-06-01): as tres ACUSADAS em 2001-147.
MUTACOES = [
    ('cqn7_baiano_chaves_trocadas',
     'cqn7_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98acac, 0x21b, 0x22f, "cqn_7 baiano"); // cqn7: 1º × 4º, 2º × 3º Baiano',
     'cqn7_mata_mata(this, &[0, 2, 1, 3], &[0, 3, 1, 2], 0x98acac, 0x21b, 0x22f, "cqn_7 baiano"); // mutacao: 1º × 3º e 2º × 4º',
     'cqn_7 baiano'),
    ('cqn7_norte_chaves_trocadas',
     'cqn7_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98af14, 0x21b, 0x22f, "cqn_7 norte"); // cqn7: 1º × 4º, 2º × 3º Norte',
     'cqn7_mata_mata(this, &[3, 0, 2, 1], &[0, 3, 1, 2], 0x98af14, 0x21b, 0x22f, "cqn_7 norte"); // mutacao: o pior colocado manda o primeiro jogo',
     'cqn_7 norte'),
    ('cqn7_nordeste_chaves_trocadas',
     'cqn7_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98af54, 0x20d, 0x221, "cqn_7 nordeste"); // cqn7: 1º × 4º, 2º × 3º Nordeste',
     'cqn7_mata_mata(this, &[0, 4, 1, 2], &[0, 3, 1, 2], 0x98af54, 0x20d, 0x221, "cqn_7 nordeste"); // mutacao: o 5º no lugar do 4º',
     'cqn_7 nordeste'),
]
