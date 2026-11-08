# Mutacoes (cqb_2). As tres no impulso das financas (0x59e050), tentado toda semana para os clubes da faixa 0 ("cpv_6
# impulso na faixa 0" passou de 2048 na base). O divisor 0,1 no lugar de 0,0075 (faixa 0, injury_word_0c ate 4250, a
# maioria dos clubes ingleses de divisao baixa) faz o impulso acontecer muito mais: dinheiro no saldo, noticias,
# sorteios. Um sorteio a mais no caminho com a ficha do presidente e no caminho sem ela desloca todos os sorteios
# seguintes da carreira. Os contadores ficam depois das condicoes, colados no efeito.
MUTACOES = [
    ('cqb2_divisor_maior',
     'read_f64_at(0x96a738) // cqb2: o divisor da faixa 0 de reputação baixa',
     'read_f64_at(0x967880) // mutacao: 0,1 no lugar de 0,0075',
     'cqb_2 divisor da faixa 0 baixa'),
    ('cqb2_sorteio_a_mais_com_ficha',
     'let r1 = cpv6_sortear(match_pl_ftol(read_f64_at(0x968dd8) / div)); // cqb2: o sorteio com a ficha',
     'let r1 = { cpv6_sortear(5); cpv6_sortear(match_pl_ftol(read_f64_at(0x968dd8) / div)) }; // mutacao: um sorteio a mais',
     'cqb_2 com a ficha'),
    ('cqb2_sorteio_a_mais_sem_ficha',
     'let r = cpv6_sortear(match_pl_ftol(read_f64_at(0x96a728) / div)); // cqb2: o sorteio sem a ficha',
     'let r = { cpv6_sortear(5); cpv6_sortear(match_pl_ftol(read_f64_at(0x96a728) / div)) }; // mutacao: um sorteio a mais',
     'cqb_2 sem a ficha'),
]
