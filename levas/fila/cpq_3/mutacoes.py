# Mutacoes dos registros de carreira (cpq_3). O acesso com +400 de injury_word_0c em vez de +500 e o rebaixamento sem o
# moral do elenco (sem os sorteios da 0x544710): a injury_word_0c do clube e o moral estao no banco e os quatro campeonatos
# ingleses acabam antes de 2002-06-01 (a 251a contou 4 "liga honras da temporada"); se o fim da temporada cair
# depois, PODEM ESCAPAR (ver os ramos). A classificacao com os pontos da liga na copa: os pontos do tecnico ficam fora
# do banco (a carreira so os le no tecnico do mes e no do ano); o comparando acusa (lado a lado pelo registro).
MUTACOES = [
    ('cpq3_acesso_400', '    cpq3_pw(clube + 0x80, cpq3_w(clube + 0x80).wrapping_add(500));',
     '    cpq3_pw(clube + 0x80, cpq3_w(clube + 0x80).wrapping_add(400)); // mutacao: +400',
     'cpq_3 acesso'),
    ('cpq3_rebaixamento_sem_moral', '        adjust_morale(p, -0xf);',
     '        // mutacao: sem o moral',
     'cpq_3 rebaixamento'),
    ('cpq3_classificado_na_copa', '        crate::comparison::branch("cpq_3 classificado clube");\n        staff_records_add_i32(r + 0xba, pontos);',
     '        crate::comparison::branch("cpq_3 classificado clube");\n        staff_records_add_i32(r + 0xbe, pontos); // mutacao: na copa',
     'cpq_3 classificado clube'),
]
