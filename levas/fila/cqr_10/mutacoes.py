# Mutacoes (cqr_10): o 4o e o 5o sorteados trocam de grupo (o ajudante dos sorteados, lido pela tabela do grupo 1 e
# pela fase do grupo 2: a composicao dos dois grupos muda); o sorteio uma semana depois (as datas sao levadas ao dia
# da semana: 7 dias); os times de cada fase de trás para a frente (o mando e a ordem dos jogos de cada rodada mudam).
# Os contadores ficam colados no efeito.
MUTACOES = [
    ('cqr10_quarto_e_quinto_trocados',
     'cqr10_d(cqr10_d(this + 0xee) + k * 4) // cqr10: o sorteado k',
     'cqr10_d(cqr10_d(this + 0xee) + (if k == 3 { 4 } else if k == 4 { 3 } else { k }) * 4) // mutacao: o 4o e o 5o trocados',
     'cqr_10 linha do primeiro grupo'),
    ('cqr10_sorteio_uma_semana_depois',
     'let dia = cqr10_w(this + 0xb6) as u16; // cqr10: o dia do sorteio',
     'let dia = (cqr10_w(this + 0xb6) as u16).wrapping_add(7); // mutacao: uma semana depois',
     'cqr_10 sorteio da libertadores'),
    ('cqr10_times_de_tras_para_frente',
     'cqr10_pd(times + n * 4, cqr10_sorteado(this, k as usize)); // cqr10: o time na vaga do grupo',
     'cqr10_pd(times + (3 - n) * 4, cqr10_sorteado(this, k as usize)); // mutacao: de tras para a frente',
     'cqr_10 time do grupo'),
]
