# Mutacoes (cqa_3). As tres no sorteio da rodada da copa (0x51c410, 1.106 chamadas na carreira ate 2002-06-01, pela
# atualizacao diaria da copa 0x51f890 e pela montagem da rodada 0x51c800). A Copa da Liga sorteia pela semente (regra
# 0x400 em +0xd, 0x403 em 0x56c661, de agosto a janeiro) e a FA Cup pelo sorteio livre (+0xd = 0, 0x51e220, de
# setembro a maio): trocar o sorteio muda os confrontos e o consumo do sorteio, e com eles os jogos, a renda e os
# premios das rodadas, que a carreira le. A do adiamento muda a data do sorteio quando um confronto anterior ainda nao
# tem vencedor (o replay da FA Cup): PODE ESCAPAR se nenhuma rodada da carreira chegar a data do sorteio com um time em
# aberto (o contador "cqa_3 sorteio adiado" mostra). Os contadores ficam colados em cada chamada, depois das condicoes.
MUTACOES = [
    ('cqa3_semente_vira_livre',
     'sorteio(0x51d2d0); // cqa3: pela semente',
     'sorteio(0x51e220); // mutacao: o sorteio livre no lugar do da semente',
     'cqa_3 sorteio pela semente'),
    ('cqa3_livre_vira_semente',
     'sorteio(0x51e220); // cqa3: o sorteio livre',
     'sorteio(0x51d2d0); // mutacao: o sorteio pela semente no lugar do livre',
     'cqa_3 sorteio livre'),
    ('cqa3_sorteio_adiado_dois_dias',
     'let r = mais(&mut a, &mut b, 1); // cqa3: o sorteio adiado',
     'let r = mais(&mut a, &mut b, 2); // mutacao: o sorteio adiado 2 dias',
     'cqa_3 sorteio adiado'),
]
