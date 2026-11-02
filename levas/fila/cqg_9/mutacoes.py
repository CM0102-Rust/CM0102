# Mutacoes (cqg_9): os arbitros do jogo novo (1 vez por carreira, na criacao do jogo da carreira automatica). Os
# arbitros mantidos, as vagas e os sorteados mudam a tabela dos arbitros (banco do jogo, na impressao diaria) e a
# quantidade de sorteios do comeco.
# - Na 0x79bf60: ficam 5 por nacao (em vez de 6) nas nacoes sem clubes de base.
# - Na 0x79bf60: o pedido de cada vaga de liga soma 11 em vez de 10 (mais arbitros sorteados).
# - Na 0x79c5c0: a nota +0x22 do arbitro novo de 2 a 81 em vez de 1 a 80.
MUTACOES = [
    ('cqg9_ficam_5', 'let limite = if *((nac + 0x11c) as *const u8) == 0 { 5 } else { 0x3c }; // cqg9: os árbitros que ficam por nação',
     'let limite = if *((nac + 0x11c) as *const u8) == 0 { 4 } else { 0x3c }; // mutacao: os árbitros que ficam por nação', 'cqg_9 arbitro mantido'),
    ('cqg9_pedido_mais_11', 'let pedido = cqg9_w(cqg9_d(cqg9_d(0xadadfc) + 4 * k as usize) + 0x3e) as i32 / 2 + 0xa; // cqg9: o pedido da vaga',
     'let pedido = cqg9_w(cqg9_d(cqg9_d(0xadadfc) + 4 * k as usize) + 0x3e) as i32 / 2 + 0xb; // mutacao: o pedido da vaga', 'cqg_9 vaga de competicao'),
    ('cqg9_nota_nova_mais_2', 'cqg9_pw(r + 0x22, x.wrapping_add(1) as i16); // cqg9: a nota +0x22 do árbitro novo',
     'cqg9_pw(r + 0x22, x.wrapping_add(2) as i16); // mutacao: a nota +0x22 do árbitro novo', 'cqg_9 arbitro novo'),
]
