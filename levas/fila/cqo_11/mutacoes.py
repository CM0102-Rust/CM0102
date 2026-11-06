# Mutacoes (cqo_11), nos inscritos das copas italianas (a Italia jogavel na carreira de 8 ligas): na Copa da Italia, o
# clube de cada pote sorteado entre 7 em vez de 8 (os grupos e a contagem dos sorteios mudam); na Copa da Serie C, as
# dezenas embaralhadas como noves (os grupos mudam); na Supercopa, o vice da Serie A no lugar do campeao. As tres rodam
# no construtor (comeco do jogo); os grupos e a Supercopa sao jogados em agosto de 2001, dentro da janela. Os contadores
# ficam colados no efeito.
MUTACOES = [
    ('cqo11_potes_sorteados_entre_7',
     'let mut r = cqo11_sortear(8) as i8 as i32; // cqo11: o clube sorteado de cada pote',
     'let mut r = cqo11_sortear(7) as i8 as i32; // mutacao: entre 7',
     'cqo_11 potes da copa da italia'),
    ('cqo11_dezenas_como_noves',
     'cqo11_embaralhar(l + k * 0x28, 0xa); // cqo11: as dezenas da Copa da Série C',
     'cqo11_embaralhar(l + k * 0x28, 9); // mutacao: noves',
     'cqo_11 dezenas da copa da serie c'),
    ('cqo11_vice_da_serie_a_na_supercopa',
     'let a = cqm12_colocado(0x9cf570, 0); // cqo11: o campeão da Série A',
     'let a = cqm12_colocado(0x9cf570, 1); // mutacao: o vice',
     'cqo_11 clubes da supercopa da italia'),
]
