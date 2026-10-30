# Mutacoes dos registros de carreira (cpq_3). O acesso com +400 de reputacao em vez de +500 e o rebaixamento sem o
# moral do elenco (sem os sorteios da 0x544710): a reputacao do clube e o moral estao no banco e os quatro campeonatos
# ingleses acabam antes de 2002-06-01 (a 251a contou 4 "liga honras da temporada"); se o fim da temporada cair
# depois, PODEM ESCAPAR (ver os ramos). A classificacao com os pontos da liga na copa: os pontos do tecnico ficam fora
# do banco (a carreira so os le no tecnico do mes e no do ano); o comparando acusa (lado a lado pelo registro).
MUTACOES = [
    ('cpq3_acesso_400', '    cpq3_pw(clube + 0x80, cpq3_w(clube + 0x80).wrapping_add(500));',
     '    cpq3_pw(clube + 0x80, cpq3_w(clube + 0x80).wrapping_add(400)); // mutacao: +400',
     'cpq_3 acesso'),
    ('cpq3_rebaixamento_sem_moral', '        cpq3_moral(p, -0xf);',
     '        // mutacao: sem o moral',
     'cpq_3 rebaixamento'),
    ('cpq3_classificado_na_copa', '        crate::comparacao::ramo("cpq_3 classificado clube");\n        cpq3_somar(r + 0xba, pontos);',
     '        crate::comparacao::ramo("cpq_3 classificado clube");\n        cpq3_somar(r + 0xbe, pontos); // mutacao: na copa',
     'cpq_3 classificado clube'),
]
