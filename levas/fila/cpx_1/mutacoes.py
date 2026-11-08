# Mutacoes (cpx_1), na recusa (0x8bf680, 35 chamadas ate 2002-06-01 na cobertura de 17/10) e na retirada (0x8bfcb0,
# 28 chamadas) das ofertas de transferencia. Sem o desassossego na recusa (0x8c1290 sorteia sempre: sorteio(5750)
# contra a injury_word_0c do comprador), a fila do sorteio anda uma casa a menos a cada recusa e o resto da carreira muda. O
# desassossego em toda retirada (e nao so no motivo 2) acrescenta sorteios nas retiradas de outro motivo. Sem desmarcar
# os lances do comprador pelo jogador (0x84aeb0, na recusa e na retirada), a lista de observacao do comprador fica com
# o lance marcado e a avaliacao das listas (0x84a020) escolhe outros alvos. As tres mudam sorteios e transferencias.
MUTACOES = [
    ('cpx1_recusa_sem_desassossego', '    call_with_offer(0x8c1290, this, o); // cpx1: o desassossego da recusa',
     '    // mutacao: sem o desassossego na recusa',
     'cpx_1 recusa desassossega'),
    ('cpx1_retirada_desassossega_sempre', '    if motivo as u16 == 2 { // cpx1: só o motivo 2 desassossega',
     '    if motivo as u16 == 2 || true { // mutacao: toda retirada desassossega',
     'cpx_1 retirada sem desassossego'),
    ('cpx1_sem_desmarcar_comprador', '    clear_bid_markers(this, cpx1_clube(cpx1_d(o + 8)), p); // cpx1: desmarcar os lances do comprador',
     '    let _ = p; // mutacao: os lances do comprador ficam marcados',
     'cpx_1 recusa desmarca'),
]
