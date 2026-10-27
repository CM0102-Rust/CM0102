# Mutacoes da copa (cpb_2). A da copa terminada inverte o teste de +0xb1 (copa que nao acabou passa por terminada e
# vice-versa): muda o andamento de todas as copas, a carreira acusa. A do progresso troca o 1 do meio por 2 (o codigo
# do meio dos times que sairam numa rodada): grava os bytes de progresso na estatistica da copa (fora do banco que entra
# na impressao); so acusa se o progresso pesar em algo ate 2002-06-01 (classificacao das copas, reputacao no fim da
# temporada): PODE ESCAPAR da carreira; o comparando acusa na primeira eliminacao (a fita traz os argumentos).
MUTACOES = [
    ('cpb_progresso_meio', '        let v = if r + 1 == n - 1 { codigo(0x82).wrapping_add(1) } else { codigo(0xc3) }; // a rodada seguinte\n        (v, 1, v)',
     '        let v = if r + 1 == n - 1 { codigo(0x82).wrapping_add(1) } else { codigo(0xc3) }; // a rodada seguinte\n        (v, 2, v)'),
    ('cpb_terminou_invertido', '    if cpb1_u8(copa + 0xb1) == 0 {\n        return 0;\n    }\n    if etapas == 1',
     '    if cpb1_u8(copa + 0xb1) != 0 {\n        return 0;\n    }\n    if etapas == 1'),
]
