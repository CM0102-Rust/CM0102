# Mutacoes (cqb_1). A renda roda a cada jogo de clubes da carreira ate 2002-06-01, e o saldo dos clubes e lido pela
# carreira (verba, propostas, salarios, diretoria, eventos financeiros da semana): a TV pelo publico com 1,25 no lugar
# de 1,75 (todo jogo com publico), o liquido do mandante pela metade (todo jogo na liga do mandante) e o preco do
# ingresso por 400 no lugar de 500 (todo jogo fora de campo neutro: a renda vai ao +0x38 do resultado e dai ao liquido
# do mandante). Contadores colados no efeito, depois das condicoes do trecho.
MUTACOES = [
    ('cqb1_tv_menor',
     'tv = match_pl_ftol((q as f64 + read_f64_at(0x968f98)) * publico as f64); // cqb1: a TV pelo público (1,75)',
     'tv = match_pl_ftol((q as f64 + read_f64_at(0x968928)) * publico as f64); // mutacao: 1,25 no lugar de 1,75',
     'cqb_1 publico pagante'),
    ('cqb1_liquido_pela_metade',
     'somar(mandante, 0x140, 0xa0, liquido); // cqb1: o líquido ao mandante',
     'somar(mandante, 0x140, 0xa0, liquido / 2); // mutacao: o liquido pela metade',
     'cqb_1 liquido ao mandante'),
    ('cqb1_preco_por_400',
     'let preco = div_trunc(cqb1_w(casa + 0x80) as i32, 500); // cqb1: o preço pela reputação',
     'let preco = div_trunc(cqb1_w(casa + 0x80) as i32, 400); // mutacao: por 400',
     'cqb_1 preco pela injury_word_0c'),
]
