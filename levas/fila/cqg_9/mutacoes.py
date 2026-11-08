# Mutacoes (cqg_9), todas na 0x79bf60 (os arbitros do jogo novo: 1 vez por carreira, na criacao do jogo da carreira
# automatica). Os arbitros mantidos, os sorteados e a reparticao das vagas mudam a tabela dos arbitros e os sorteios do
# comeco (0x79c5c0 sorteia os que faltam).
# - Ficam 5 por nacao (em vez de 6) nas nacoes sem clubes de base.
# - O pedido de cada vaga de liga soma 11 em vez de 10 (mais arbitros sorteados).
# - A parte de cada vaga uma a mais (os arbitros por competicao).
MUTACOES = [
    ('cqg9_ficam_5', 'let limit = if *((nac + 0x11c) as *const u8) == 0 { 5 } else { 0x3c }; // cqg9: os árbitros que ficam por nação',
     'let limit = if *((nac + 0x11c) as *const u8) == 0 { 4 } else { 0x3c }; // mutacao: os árbitros que ficam por nação', 'cqg_9 arbitro mantido'),
    ('cqg9_pedido_mais_11', 'let pedido = cqg9_w(cqg9_d(cqg9_d(0xadadfc) + 4 * k as usize) + 0x3e) as i32 / 2 + 0xa; // cqg9: o pedido da vaga',
     'let pedido = cqg9_w(cqg9_d(cqg9_d(0xadadfc) + 4 * k as usize) + 0x3e) as i32 / 2 + 0xb; // mutacao: o pedido da vaga', 'cqg_9 vaga de competicao'),
    ('cqg9_parte_mais_1', 'cqg9_pd(b + 0x16 + 16 * j as usize, (cqg9_i(b + 4) / c) as u32); // cqg9: a parte de cada vaga',
     'cqg9_pd(b + 0x16 + 16 * j as usize, (cqg9_i(b + 4) / c + 1) as u32); // mutacao: a parte de cada vaga', 'cqg_9 parte da vaga'),
]
