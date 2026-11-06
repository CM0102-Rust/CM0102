# Mutacoes (cqr_13): o 4o e o 5o sorteados trocam de grupo (o ajudante dos sorteados, lido pela tabela do grupo A e
# pela fase do grupo B: a composicao dos dois grupos muda); o sorteio uma semana depois (as datas sao levadas ao dia da
# semana: 7 dias); os times de cada fase de tras para a frente (o mando e a ordem dos jogos de cada rodada mudam).
# Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqr13_quarto_e_quinto_trocados',
     'cqr13_d(cqr13_d(this + 0xee) + k * 4) // cqr13: o sorteado k',
     'cqr13_d(cqr13_d(this + 0xee) + (if k == 3 { 4 } else if k == 4 { 3 } else { k }) * 4) // mutacao: o 4o e o 5o trocados',
     'cqr_13 linha do primeiro grupo'),
    ('cqr13_sorteio_uma_semana_depois',
     'let dia = cqr13_w(this + 0xb6) as u16; // cqr13: o dia do sorteio',
     'let dia = (cqr13_w(this + 0xb6) as u16).wrapping_add(7); // mutacao: uma semana depois',
     'cqr_13 sorteio da copa america'),
    ('cqr13_times_de_tras_para_frente',
     'cqr13_pd(times + n * 4, cqr13_sorteado(this, (g + 1) as usize + n * 3)); // cqr13: o time na vaga do grupo',
     'cqr13_pd(times + (3 - n) * 4, cqr13_sorteado(this, (g + 1) as usize + n * 3)); // mutacao: de tras para a frente',
     'cqr_13 time do grupo'),
]
