# Mutacoes. A 0x76db00 roda 977 vezes na carreira automatica ate 2002-06-01 (cobertura de 17/10), sempre pela
# convocacao 0x76c120 (tres chamadas, todas sem entrar e sem texto: conferir quem do elenco atual fica e quem sai). A
# devolucao decide se o jogador sai ou fica na selecao (+0x24/+0x28 da pessoa, no hash diario do banco): o tipo sem
# janela devolvendo 0, a data do tipo ja passada devolvendo 0 e o aposentado da selecao passando adiante. Os ramos
# ficam colados no efeito (dentro do ramo, antes da devolucao).
MUTACOES = [
    ('cpv12_sem_janela_0', 'return 1; // cpv12: sem janela',
     'return 0; // mutacao: sem janela devolve 0',
     'cpv_12 without window'),
    ('cpv12_data_passada_0', 'return 1; // cpv12: data passada',
     'return 0; // mutacao: data passada devolve 0',
     'cpv_12 past date'),
    ('cpv12_aposentado_segue', 'if aposentado(p, principal(cpv12_d(p + 0x1a))) != 0 { // cpv12: aposentado',
     'if aposentado(p, principal(cpv12_d(p + 0x1a))) != 0 && false { // mutacao: aposentado segue',
     'cpv_12 retired'),
]
