# Mutacoes (cqf_7), todas na 0x8b2880 (a comissao do tecnico humano: 18 chamadas na carreira automatica ate
# 2002-06-01, nas contratacoes e saidas de treinadores do clube humano; o treinador novo entra com as cinco categorias,
# entao as cinco notas rodam). A nota vai para o fator do treinador (0x8b41d0), que pesa os sorteios do progresso do
# treino dos jogadores do clube humano (0x8b1750) e dai os atributos.
# - O divisor da quantidade sempre 1,0 (o treinador de cinco categorias teria a nota dobrada, ate 20).
# - A nota de goleiros soma 31 em vez de 21.
# - A nota de tatica divide por 2 em vez de 3.
MUTACOES = [
    ('cqf7_divisor_sempre_1', 'let q = match_pl_ftol(x as i8 as f64 / f32::from_bits(RATING_DIVISORS[n - 1]) as f64) as i8; // cqf7: o divisor da quantidade',
     'let q = match_pl_ftol(x as i8 as f64 / f32::from_bits(RATING_DIVISORS[0]) as f64) as i8; // mutacao: o divisor da quantidade', 'cqf_7 coach rating'),
    ('cqf7_goleiros_mais_31', '0 => (non_player_attribute_rating(f, 0x10) - b(0x12) + 0x15) / 2, // cqf7: a nota de goleiros',
     '0 => (non_player_attribute_rating(f, 0x10) - b(0x12) + 0x1f) / 2, // mutacao: a nota de goleiros', 'cqf_7 coach rating'),
    ('cqf7_tatica_por_2', '(a + non_player_attribute_rating(f, 0x10) + b(0x12)) / 3 // cqf7: a nota de tática',
     '(a + non_player_attribute_rating(f, 0x10) + b(0x12)) / 2 // mutacao: a nota de tática', 'cqf_7 coach rating'),
]
