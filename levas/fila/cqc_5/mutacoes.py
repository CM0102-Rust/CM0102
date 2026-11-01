# Mutacoes (cqc_5), nos jogos do confronto da copa (0x51cb90), que a copia Rust da montagem da rodada (0x51c800,
# "cps_13 jogos da rodada" na base) chama para cada confronto marcado; 1148 chamadas na carreira ate 2002-06-01:
# - a ida com o mando invertido: o visitante joga em casa (o estadio, a renda e o fator casa mudam, e com eles o
#   resultado, a fase seguinte e as financas dos dois clubes);
# - a ida uma semana depois: a partida cai noutra data (o calendario dos clubes, o cansaco, os conflitos com a liga e
#   o dia da fase seguinte mudam);
# - a volta sem inverter o mando: o mesmo clube joga as duas em casa (pega se alguma rodada de duas maos, como as
#   semifinais da copa da liga inglesa em janeiro, rodar antes de 2002-06-01: contador "cqc_5 jogo de volta").
# Os contadores ficam colados no efeito: a partida da ida vai ao calendario depois de todas as condicoes; o da volta
# depois da copa ativa e da rodada de duas maos.
MUTACOES = [
    ('cqc5_ida_com_mando_invertido',
     'let (lc, lf) = (cqc5_w(t), cqc5_w(t + 2)); // cqc5: os lados da ida',
     'let (lc, lf) = (cqc5_w(t + 2), cqc5_w(t)); // mutacao: a ida com o mando invertido',
     'cqc_5 jogo do confronto'),
    ('cqc5_ida_uma_semana_depois',
     'cqc5_pw(p + 0x2a, cqc5_w(e + 3) as u16); // cqc5: o dia da ida',
     'cqc5_pw(p + 0x2a, cqc5_w(e + 3).wrapping_add(7) as u16); // mutacao: a ida uma semana depois',
     'cqc_5 jogo do confronto'),
    ('cqc5_volta_sem_inverter',
     'let (c1, c2, i1, i2) = (cqc5_d(p + 0x1c), cqc5_d(p + 0x20), cqc5_d(p + 0xc), cqc5_d(p + 0x10)); // cqc5: os lados da volta',
     'let (c2, c1, i2, i1) = (cqc5_d(p + 0x1c), cqc5_d(p + 0x20), cqc5_d(p + 0xc), cqc5_d(p + 0x10)); // mutacao: a volta sem inverter',
     'cqc_5 jogo de volta'),
]
