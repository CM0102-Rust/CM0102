# Mutacoes (cql_4), no fim da temporada da Serie A e da Serie B (maio de 2002, antes de 2002-06-01 na carreira de 8
# ligas; os contadores "cql_4 fim da serie a/b" rodam 1 vez cada na base). Sem empate na fronteira (o caso da
# carreira), a temporada fecha; as mutacoes fazem o desempate nascer ou deixam a marca do historico (+0x4c) como o
# construtor a deixou (1: o dia de liga nao grava o historico e os recordes da temporada).
# Escapou na prova do agente (05/11): "todo empate da Serie B vai ao desempate" (nenhum par vizinho empatado na Serie
# B dessa carreira, ou o jogo extra so muda algo depois de 2002-06-01); trocada pela marca do historico na B.
MUTACOES = [
    ('cql4_serie_a_todo_empate',
     'z[(i as usize).min(63)] != z[(i as usize + 1).min(63)] { // cql4: o par da Série A na fronteira',
     'true { // mutacao: todo empate da Serie A vai ao desempate',
     'cql_4 serie a end'),
    ('cql4_serie_b_sem_zerar_o_historico',
     '    let z = serie_b_zones();\n    cql4_pd(this + 0x4c, 0);',
     '    let z = serie_b_zones();',
     'cql_4 serie b end'),
    ('cql4_serie_a_sem_zerar_o_historico',
     '    let z = serie_a_zones(this);\n    let n = cql4_w(this + 0x3e) as i32 - 1;\n    cql4_pd(this + 0x4c, 0);',
     '    let z = serie_a_zones(this);\n    let n = cql4_w(this + 0x3e) as i32 - 1;',
     'cql_4 serie a end'),
]
