# Mutacoes (cqr_11): o 4o e o 5o sorteados trocam de grupo (o ajudante dos sorteados, lido pela tabela do grupo 1 e
# pela fase do grupo 2: a composicao dos dois grupos muda); o sorteio uma semana depois (as datas sao levadas ao dia
# da semana: 7 dias); os times de cada fase de trás para a frente (o mando e a ordem dos jogos de cada rodada mudam).
# Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqr11_quarto_e_quinto_trocados',
     'cqr11_d(cqr11_d(this + 0xee) + k * 4) // cqr11: o sorteado k',
     'cqr11_d(cqr11_d(this + 0xee) + (if k == 3 { 4 } else if k == 4 { 3 } else { k }) * 4) // mutacao: o 4o e o 5o trocados',
     'cqr_11 linha do primeiro grupo'),
    ('cqr11_sorteio_uma_semana_depois',
     'let dia = cqr11_w(this + 0xb6) as u16; // cqr11: o dia do sorteio',
     'let dia = (cqr11_w(this + 0xb6) as u16).wrapping_add(7); // mutacao: uma semana depois',
     'cqr_11 sorteio da mercosul'),
    ('cqr11_times_de_tras_para_frente',
     'cqr11_pd(times + n * 4, conmebol_merc_drawn_club(this, k as usize)); // cqr11: o time na vaga do grupo',
     'cqr11_pd(times + (3 - n) * 4, conmebol_merc_drawn_club(this, k as usize)); // mutacao: de tras para a frente',
     'cqr_11 time do grupo'),
]
