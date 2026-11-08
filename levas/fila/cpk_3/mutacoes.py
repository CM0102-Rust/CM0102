# Mutacoes (com o ramo de cada uma). A clausula acionada com 3 em vez de 2 (o comparando acusa pelo plano; a carreira,
# no fim da temporada 2001/02, pelos contratos dos rebaixados). A oferta tirada sem descontar a pendente (o comparando
# acusa pelo byte; a carreira pelas ofertas da pessoa depois). O suspenso sem a suspensao da competicao (o comparando
# acusa pelo valor no primeiro suspenso; a carreira pela escalacao automatica, que passa a pick os suspensos).
MUTACOES = [
    ('cpk_clausula_estado', '*((contrato + 0x1f) as *mut u8) = 2; // a cláusula acionada',
     '*((contrato + 0x1f) as *mut u8) = 3; // mutacao: 3',
     'cpk_3 clausula de rebaixamento'),
    ('cpk_oferta_pendente', '*n = (*n).wrapping_sub(1); // uma pendente a menos',
     '// mutacao: sem descontar',
     'cpk_3 tirar oferta de contrato'),
    ('cpk_suspenso_competicao', 'if b == 0 || julgar(b, jogador, 0, extra, partida) == 0 { // sem a suspensão da competição',
     'if b == 0 || b != 0 { // mutacao: sem a da competicao',
     'cpk_3 suspenso pela competicao'),
]
