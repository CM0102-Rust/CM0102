# Mutacoes. A nota-alvo entra em todas as buscas de transferencia da IA (5994 chamadas ate 2002-06-01) e o comparador
# confere a nota e a base a cada chamada: a base / 15 em vez de / 16 (todas as notas sem forcada), a faixa de 3000 a
# 5000 com 100 a mais (a faixa da maioria dos clubes) e o clube rico (riqueza 2) multiplicando por 1,05 (pesa quando a
# nota passa de 5000, o comum nessa faixa). Os ramos ficam depois de todas as condicoes, colados no efeito.
MUTACOES = [
    ('cpu13_base_15', 'esi = soma / 16; // cpu13: a base',
     'esi = soma / 15; // mutacao: / 15',
     'cpu_13 base pela reputação'),
    ('cpu13_faixa_do_meio', 'esi.wrapping_mul(5).wrapping_sub(5000) // cpu13: a faixa do meio',
     'esi.wrapping_mul(5).wrapping_sub(4900) // mutacao: 100 a mais',
     'cpu_13 faixa de 3000 a 5000'),
    ('cpu13_rico', 'let (x, y) = (v * f64_em(0x968fa0), edi.wrapping_add(250) as f64); // cpu13: o rico',
     'let (x, y) = (v * (f64_em(0x968fa0) + 0.025), edi.wrapping_add(250) as f64); // mutacao: 1,05',
     'cpu_13 clube rico'),
]
