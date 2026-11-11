# Mutacoes (cqv_12), na virada da Libertadores (a de 2002 joga de fevereiro a maio, com clubes brasileiros): sem o ano
# seguinte; sem a semeadura das vagas; o sorteio marcado como feito (a Libertadores nova nao se sorteia).
MUTACOES = [
    ('cqv12_sem_o_ano_seguinte',
     '    conmebol_next_year(league); // cqv12: the Libertadores of the next year',
     '    write_u32(this + league::CURRENT_STAGE, u32::MAX); // mutacao: sem o ano seguinte',
     'cqv_12 libertadores turnover'),
    ('cqv12_sem_a_semeadura',
     '    seed(read_usize(G_CONMEBOL_SEEDING), 0); // cqv12: the places of the new Libertadores',
     '    let _ = seed; // mutacao: sem a semeadura',
     'cqv_12 libertadores turnover'),
    ('cqv12_sorteio_ja_feito',
     '    write_u32(this + league::ACTIVE, 0); // cqv12: the new Libertadores not drawn yet',
     '    write_u32(this + league::ACTIVE, 1); // mutacao: sorteio ja feito',
     'cqv_12 libertadores turnover'),
]
