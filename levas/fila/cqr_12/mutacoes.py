# Mutacoes (cqr_12): o 4o e o 5o sorteados trocam de grupo (o ajudante dos sorteados, lido pela tabela do grupo 1 e
# pela fase do grupo 2: a composicao dos dois grupos muda); o sorteio uma semana depois (as datas sao levadas ao dia
# da semana: 7 dias); os times de cada fase de trás para a frente (o mando e a ordem dos jogos de cada rodada mudam).
# Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqr12_quarto_e_quinto_trocados',
     'cqr12_d(cqr12_d(this + 0xee) + k * 4) // cqr12: o sorteado k',
     'cqr12_d(cqr12_d(this + 0xee) + (if k == 3 { 4 } else if k == 4 { 3 } else { k }) * 4) // mutacao: o 4o e o 5o trocados',
     'cqr_12 first group row'),
    ('cqr12_sorteio_uma_semana_depois',
     'let dia = cqr12_w(this + 0xb6) as u16; // cqr12: o dia do sorteio',
     'let dia = (cqr12_w(this + 0xb6) as u16).wrapping_add(7); // mutacao: uma semana depois',
     'cqr_12 merconorte draw'),
    ('cqr12_times_de_tras_para_frente',
     'cqr12_pd(times + n * 4, con_merc_cup_drawn_club(this, k as usize)); // cqr12: o time na vaga do grupo',
     'cqr12_pd(times + (3 - n) * 4, con_merc_cup_drawn_club(this, k as usize)); // mutacao: de tras para a frente',
     'cqr_12 group team'),
]
