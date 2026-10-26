# Mutacoes da mascara da posicao, da contagem do elenco e do companheiro favorito. A da mascara tira o bit 0x10 dos
# zagueiros (+0x12 = 20 da 0x8 no lugar de 0x18): muda a mascara de todo zagueiro natural, usada no motor de partidas e
# na escolha do companheiro (diaria); a do companheiro troca o desempate % 500 por % 50 (muda a nota de todo candidato
# e o escolhido): a carreira ate 2002-06-01 deve acusar as duas. A da contagem conta os diferentes no lugar dos iguais:
# a 0x548150 so roda na midia (0x758010/0x758200); a carreira PODE NAO acusar (o comparando acusa na primeira chamada).
MUTACOES = [
    ('dbq_mascara', '    for (k, bit) in [(0x10, 2), (0x11, 4), (0x12, 0x18), (0x13, 0x38), (0x14, 0x30)] {',
     '    for (k, bit) in [(0x10, 2), (0x11, 4), (0x12, 0x8), (0x13, 0x38), (0x14, 0x30)] { // mutacao: zagueiro sem o 0x10'),
    ('dbq_contar_mascara', '        if p != 0 && dbq3_u32(p + 0x61) != 0 && mascara_de(p) == mascara { // os iguais',
     '        if p != 0 && dbq3_u32(p + 0x61) != 0 && mascara_de(p) != mascara { // mutacao: os diferentes'),
    ('dbq_companheiro', '        nota += oid.wrapping_add(id) % 500; // o desempate pelos ids',
     '        nota += oid.wrapping_add(id) % 50; // mutacao: % 50 no lugar de % 500'),
]
