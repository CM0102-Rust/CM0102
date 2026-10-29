# Mutacoes (com o ramo de cada uma). O estatuto pela entrada da lista lido dos bits 2..5 (o comparando acusa pelo
# valor; a carreira pelos contratos e transferencias que leem o estatuto: 0x4cda70, 0x4ce4e0, 0x8c0080, 0x8c1290). O
# estatuto pelo contrato lido do nibble baixo (idem). Os alvos que nao sao do clube sem o evento 3 da lista (a lista do
# clube fica com eles: as transferencias da carreira mudam; o comparando nao acusa, a comparacao e so a original).
MUTACOES = [
    ('cpn_estatuto_lista', 'return (cpn6_b(e + 0x24) >> 1) & 0xf; // os bits 1..4 da entrada',
     'return (cpn6_b(e + 0x24) >> 2) & 0xf; // mutacao: os bits 2..5',
     'cpn_6 estatuto pela lista'),
    ('cpn_estatuto_contrato', 'cpn6_b(k + 0x4f) >> 4 // o nibble alto do contrato',
     'cpn6_b(k + 0x4f) & 0xf // mutacao: o nibble baixo',
     'cpn_6 estatuto pelo contrato'),
    ('cpn_alvos_evento', 'atualizar(this, p, clube, 3, tipo);',
     'let _ = (atualizar, tipo); // mutacao: sem o evento 3',
     'cpn_6 alvo fora do clube'),
]
