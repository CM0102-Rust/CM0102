# Mutacoes (cqg_7), todas na 0x7abac0 (o regen jovem: centenas por carreira ate 2002-06-01, pela fila 0x7a65d0). Os
# regens entram nos elencos e no mercado: a habilidade, a idade e a reputacao mudam a escolha dos clubes, os valores e
# as transferencias.
# - A habilidade sorteada pela PA soma 5 (todos os regens).
# - A idade de 16 a 18 (nacao grande, metade dos sorteios) vira 17 a 19: o nascimento muda, e com ele o +0x18.
# - A reputacao +9 do regen de nacao grande (249/250 dos sorteios) usa a habilidade x 30 em vez de x 20.
MUTACOES = [
    ('cqg7_habilidade_mais_5', 'cqg7_pw(f + 5, ftol(r as f64 + pa * f64_em(m2)) as i16); // cqg7: a habilidade sorteada pela PA',
     'cqg7_pw(f + 5, (ftol(r as f64 + pa * f64_em(m2)) as i16).wrapping_add(5)); // mutacao: a habilidade sorteada pela PA', 'cqg_7 habilidade'),
    ('cqg7_idade_17_a_19', 'idade = (cqg7_sortear(3) as i8).wrapping_add(0x10); // cqg7: a idade de 16 a 18',
     'idade = (cqg7_sortear(3) as i8).wrapping_add(0x11); // mutacao: a idade de 16 a 18', 'cqg_7 idade 16 a 18'),
    ('cqg7_reputacao_x30', 'cqg7_pw(f + 9, r.wrapping_add(cqg7_w(f + 5).wrapping_mul(0x14)).wrapping_add(0xfa)); // cqg7: a reputação +9 do regen',
     'cqg7_pw(f + 9, r.wrapping_add(cqg7_w(f + 5).wrapping_mul(0x1e)).wrapping_add(0xfa)); // mutacao: a reputação +9 do regen', 'cqg_7 reputacao do regen'),
]
