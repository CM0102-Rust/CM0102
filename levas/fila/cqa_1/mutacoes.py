# Mutacoes (cqa_1). As tres no premio da rodada (0x522240), que roda a cada jogo decidido das copas inglesas ate
# 2002-06-01: a FA Cup paga 10 mil ao vencedor de cada rodada (de setembro a maio) e a Copa da Liga paga do fundo
# (+0x60 = -1, de agosto a fevereiro). O dinheiro vai ao saldo do clube (0x5999a0), que a carreira le (verba,
# propostas, salarios, diretoria): o vencedor pela metade e o premio do vencedor ao perdedor mexem em todos os
# pagamentos; o fundo sem a divisao pelos confrontos muda os premios da Copa da Liga. Os contadores ficam depois das
# condicoes (premio nao zero; o ramo do fundo depois das contas de divisao), colados no efeito.
MUTACOES = [
    ('cqa1_vencedor_pela_metade',
     'cqa1_somar(cqa1_financas(cqa1_d(vencedor)), ao_vencedor); // cqa1: o prêmio do vencedor',
     'cqa1_somar(cqa1_financas(cqa1_d(vencedor)), ao_vencedor / 2); // mutacao: o vencedor recebe a metade',
     'cqa_1 premio do vencedor'),
    ('cqa1_premio_ao_perdedor',
     'cqa1_somar(cqa1_financas(cqa1_d(vencedor)), ao_vencedor); // cqa1: o prêmio do vencedor',
     'cqa1_somar(cqa1_financas(cqa1_d(perdedor)), ao_vencedor); // mutacao: o premio do vencedor vai ao perdedor',
     'cqa_1 premio do vencedor'),
    ('cqa1_fundo_sem_os_confrontos',
     'let q = q0 / confrontos; // cqa1: o fundo pelas rodadas que faltam e pelos confrontos',
     'let q = q0 + 0 * confrontos; // mutacao: o fundo sem a divisao pelos confrontos',
     'cqa_1 premio pelo fundo'),
]
