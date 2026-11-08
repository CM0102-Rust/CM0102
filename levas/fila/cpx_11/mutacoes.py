# Mutacoes. A 0x682e10 roda 25 vezes na carreira automatica ate 2002-06-01 (cobertura de 17/10): no fim da temporada
# de cada liga, o fundo de premios vai para o saldo dos clubes pela posicao (0x5999a0 no registro de financas). As
# tres mexem no dinheiro, que a carreira le depois (verbas, propostas, salarios, diretoria): sem premio; o premio
# invertido (o lanterna leva a maior parte); o lider sem premio. O ramo fica colado na soma, depois de todas as
# condicoes (fora do +0xe1 = 2/3 e do divisor zero).
MUTACOES = [
    ('cpx11_sem_premio',
     'somar(fin, valor); // cpx11: o prêmio',
     'let _ = (somar, fin); // mutacao: sem premio',
     'cpx_11 bonus paid'),
    ('cpx11_premio_invertido',
     'somar(fin, valor); // cpx11: o prêmio',
     'somar(fin, (i as i32 + 1).wrapping_mul(parte)); // mutacao: o premio invertido',
     'cpx_11 bonus paid'),
    ('cpx11_lider_sem_premio',
     'somar(fin, valor); // cpx11: o prêmio',
     'if i != 0 { somar(fin, valor); } // mutacao: o lider sem premio',
     'cpx_11 bonus paid'),
]
