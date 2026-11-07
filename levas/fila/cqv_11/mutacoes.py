# Mutacoes (cqv_11), nas quartas da Mercosul de 2001 (clubes brasileiros jogam): o quarto segundo no lugar do terceiro
# (outro clube nas quartas); os terceiros dos grupos disputando as vagas de melhor segundo; o primeiro do grupo A
# trocado com o primeiro segundo no vetor das quartas (outro sorteio de confrontos).
MUTACOES = [
    ('cqv11_quarto_segundo',
     'cqv11_pd(v + (k + s as usize) * 4, cqv11_d(cqv11_linha(segundos, s))); // cqv11: o segundo s das quartas',
     'cqv11_pd(v + (k + s as usize) * 4, cqv11_d(cqv11_linha(segundos, s + 1))); // mutacao: o seguinte',
     'cqv_11 melhores segundos'),
    ('cqv11_terceiros_na_disputa',
     '+ 0x3b; // cqv11: a linha do segundo do grupo',
     '+ 0x76; // mutacao: o terceiro',
     'cqv_11 melhores segundos'),
    ('cqv11_ordem_das_quartas',
     'let oito = v; // cqv11: os 8 das quartas na ordem',
     'let oito = { let a = cqv11_d(v); cqv11_pd(v, cqv11_d(v + k * 4)); cqv11_pd(v + k * 4, a); v }; // mutacao: trocados',
     'cqv_11 melhores segundos'),
]
