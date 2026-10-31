# Mutacoes (com o ramo de cada uma). Todas na contraproposta com clausula (os motivos da recusa so rodam na tela). Em
# cada um dos tres ramos, o valor da contraproposta fica em 0,9 do lance e nao em 0,8: a resposta do clube do computador
# compara o valor da opcao escolhida (0x8e7b50) com o lance e manda a contraproposta ao comprador, que decide por ela
# (outro valor muda transferencias e dinheiro). Os ramos dizem se a carreira passou por eles.
MUTACOES = [
    ('cps4_percentual_0_9', 'cps4_por(o + 0x1b, cps4_vezes(o, 0.8)); // cps4: o valor do percentual',
     'cps4_por(o + 0x1b, cps4_vezes(o, 0.9)); // mutacao: 0,9',
     'cps_4 percentual da venda'),
    ('cps4_selecao_0_9', 'cps4_por(o + 0x1b, cps4_vezes(o, 0.8)); // cps4: o valor do prêmio na seleção',
     'cps4_por(o + 0x1b, cps4_vezes(o, 0.9)); // mutacao: 0,9',
     'cps_4 premio na selecao'),
    ('cps4_time_0_9', 'cps4_por(o + 0x1b, cps4_vezes(o, 0.8)); // cps4: o valor do prêmio no time',
     'cps4_por(o + 0x1b, cps4_vezes(o, 0.9)); // mutacao: 0,9',
     'cps_4 premio no time'),
]
