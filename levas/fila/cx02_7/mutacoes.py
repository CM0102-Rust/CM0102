# Mutacoes (cx02_7), carreira de 8 ligas ate 2002-08-01. Nenhuma abre caixa.
# - a vitoria na prorrogacao da J. League vale 3 e nao 2: a classificacao da J1/J2 muda (a carreira e o comparador);
# - o 3o ao 6o da Serie B no pote 3 e nao no 4: os grupos da Copa da Italia mudam (quem joga com quem);
# - o premio da Copa do Imperador 1.000.000: o comparador acusa na virada.
MUTACOES = [
    ('cx02_7_prorrogacao_3_pontos',
     '        if won(p.home_goals_extra_time(), p.away_goals_extra_time()) { 2 } else { 0 }',
     '        if won(p.home_goals_extra_time(), p.away_goals_extra_time()) { 3 } else { 0 } // mutation'),
    ('cx02_7_pote_4_vira_3',
     "    if (id == serie_b && (3..=6).contains(&position)) || (id == serie_a && read_u32(division.addr()) == serie_b) {\n        return 4;",
     "    if (id == serie_b && (3..=6).contains(&position)) || (id == serie_a && read_u32(division.addr()) == serie_b) {\n        return 3; // mutation"),
    ('cx02_7_premio_imperador',
     '    set_up_prize_rounds(Cup(this), 7_000_000)',
     '    set_up_prize_rounds(Cup(this), 1_000_000) // mutation'),
]
