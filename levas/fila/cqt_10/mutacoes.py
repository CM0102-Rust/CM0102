# Mutacoes (cqt_10), nos grupos do Campeonato de Clubes da Oceania de janeiro de 2002: o 4o e o 5o escolhidos trocam de
# grupo (o ajudante lido pelos dois grupos); as linhas do grupo A de tras para a frente; os times do grupo B de tras
# para a frente (o mando e a ordem dos jogos de cada rodada mudam). Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqt10_quarto_e_quinto_trocados',
     'cqt10_d(cqt10_d(this + 0xee) + k * 4) // cqt10: o escolhido k',
     'cqt10_d(cqt10_d(this + 0xee) + (if k == 3 { 4 } else if k == 4 { 3 } else { k }) * 4) // mutacao: o 4o e o 5o trocados',
     'cqt_10 linha do grupo a'),
    ('cqt10_grupo_a_de_tras_para_frente',
     'linha(this, i as u8 as u32, cqt10_escolhido(this, i as usize), 0, 0); // cqt10: a linha i do grupo A',
     'linha(this, i as u8 as u32, cqt10_escolhido(this, 3 - i as usize), 0, 0); // mutacao: de tras para a frente',
     'cqt_10 linha do grupo a'),
    ('cqt10_grupo_b_de_tras_para_frente',
     'cqt10_pd(times + n * 4, cqt10_escolhido(this, 4 + n)); // cqt10: o time na vaga do grupo B',
     'cqt10_pd(times + (3 - n) * 4, cqt10_escolhido(this, 4 + n)); // mutacao: de tras para a frente',
     'cqt_10 time do grupo b'),
]
