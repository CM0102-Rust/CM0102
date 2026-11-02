# Mutacoes (cqh_9), todas na 0x419840 (a fuga do jogador). As reputacoes cortadas e as antipatias mudam as notas,
# as transferencias e o elenco.
# - O corte da reputacao: sorteio(41) x 0,02 em vez de 0,01 (ate 80%).
# - A volta: 22..171 dias em vez de 21..170 (a data da lista das fugas).
# - O companheiro nao ganha antipatia.
MUTACOES = [
    ('cqh9_corte_duplo', 'let k = f64_em(0x967890) - r as f64 * f64_em(0x967898); // cqh9: o corte da reputação',
     'let k = f64_em(0x967890) - r as f64 * f64_em(0x967898) * 2.0; // mutacao: o corte da reputação', 'cqh_9 fuga'),
    ('cqh9_volta_mais_1', 'let dias = if t == 4 { cqh9_sortear(0x1f) + 2 } else { cqh9_sortear(0x96) + 0x15 }; // cqh9: os dias até a volta',
     'let dias = if t == 4 { cqh9_sortear(0x1f) + 3 } else { cqh9_sortear(0x96) + 0x16 }; // mutacao: os dias até a volta', 'cqh_9 fuga'),
    ('cqh9_sem_antipatia', 'cqh9_antipatia(q, p); // cqh9: o companheiro não gosta dele',
     'let _ = q; // mutacao: o companheiro não gosta dele', 'cqh_9 fuga'),
]
