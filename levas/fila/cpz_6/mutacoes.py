# Mutacoes (cpz_6), na 0x7757d0, que a escalacao do computador (0x6dbc50 select_automatically, 0x6dbb80 starters)
# chama para cada jogador antes de cada partida: o jogador a servico da selecao fica fora do time. Nas datas FIFA da
# carreira automatica ate 2002-06-01 (setembro, outubro e novembro de 2001, fevereiro, marco e maio de 2002) os
# convocados das selecoes do computador tem jogo da selecao na janela ou convocacao ja comecada. Devolver 0 num dos
# dois casos (o convocado joga pelo clube) ou alargar a janela (ate 3 dias depois) muda quem joga pelos clubes nas
# rodadas perto das datas FIFA: outros resultados, outros sorteios. Os contadores ficam depois das condicoes
# (convocado, jogo na janela e o adversario se dado; ou a convocacao da selecao dele com data antes da pedida), colados
# na devolucao. A 0x7757d0 e quente (2,1 milhoes de chamadas): os contadores so nos ramos raros.
MUTACOES = [
    ('cpz6_convocado_joga_pelo_clube',
     'return sai(cpz6_d(jogador + 0x24)); // cpz6: a serviço da seleção',
     'return sai(0); // mutacao: o convocado joga pelo clube',
     'cpz_6 a servico da selecao'),
    ('cpz6_ja_no_time_joga_pelo_clube',
     'return sai(cpz6_d(jogador + 0x24)); // cpz6: já no time da seleção',
     'return sai(0); // mutacao: o ja convocado joga pelo clube',
     'cpz_6 no time da selecao'),
    ('cpz6_janela_ate_3_dias',
     'mais(data, phi, 1); // cpz6: o dia seguinte',
     'mais(data, phi, 3); // mutacao: a janela vai ate 3 dias depois',
     'cpz_6 a servico da selecao'),
]
