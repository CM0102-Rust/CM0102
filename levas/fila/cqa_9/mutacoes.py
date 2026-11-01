# Mutacoes (cqa_9), todas no 0x4d0c60 (5,6 milhoes de chamadas ate 2002-06-01: quente, sem contador de ramo). O valor
# do jogador decide ofertas, precos, listas de transferencia, a verba da diretoria e o valor dos regens contratados
# (+0x52): mudar o valor de um grupo grande de jogadores muda as transferencias e os sorteios delas em poucos dias.
# - O auge de quem joga na frente (+0x15 >= 15) cai de 28 para 27: o fator da idade e a tabela 0x9a7f30 mudam.
# - O contrato curto (menos de 350 dias, com a lei Bosman) passa a ser abaixo de 300: o valor dos contratos entre 300
#   e 349 dias muda de curva.
# - O indice da tabela pela idade (auge - idade + 10) anda uma casa: o valor de quase todos muda.
MUTACOES = [
    ('cqa9_auge_da_frente', '        (13_000_000.0, 0x1c) // cqa9: a base de quem joga na frente',
     '        (13_000_000.0, 0x1b) // mutacao: o auge de quem joga na frente'),
    ('cqa9_contrato_curto', '            if dias < 0x15e { // cqa9: o contrato curto',
     '            if dias < 0x12c { // mutacao: o contrato curto'),
    ('cqa9_indice_da_idade', '    let mut i = auge.wrapping_sub(idade_ef).wrapping_add(10); // cqa9: o índice da tabela pela idade',
     '    let mut i = auge.wrapping_sub(idade_ef).wrapping_add(9); // mutacao: o índice da tabela pela idade'),
]
