# Mutacoes (cqv_12), na virada da Libertadores (a de 2002 joga de fevereiro a maio, com clubes brasileiros): sem o ano
# seguinte; sem a semeadura das vagas; o sorteio marcado como feito (a Libertadores nova nao se sorteia).
MUTACOES = [
    ('cqv12_sem_o_ano_seguinte',
     '    cqv12_ano_seguinte(this); // cqv12: a Libertadores do ano seguinte',
     '    cqv12_pd(this + 0x30, usize::MAX); // mutacao: sem o ano seguinte',
     'cqv_12 virada da libertadores'),
    ('cqv12_sem_a_semeadura',
     '    semear(cqv12_d(0xae1000), 0); // cqv12: as vagas da Libertadores nova',
     '    let _ = semear; // mutacao: sem a semeadura',
     'cqv_12 virada da libertadores'),
    ('cqv12_sorteio_ja_feito',
     '    cqv12_pd(this + 0x45, 0); // cqv12: a Libertadores nova ainda sem sorteio',
     '    cqv12_pd(this + 0x45, 1); // mutacao: sorteio ja feito',
     'cqv_12 virada da libertadores'),
]
