# Mutacoes da diretoria e do tecnico (cpq_1). A demissao que nao entra na lista das saidas (o tecnico fica: a
# 251a contou 32 tecnicos tirados ate 2002-06-01). O ultimato que nao entra na lista (a 0x6ac470 nao acha o clube: os
# ultimatos cumpridos, 4 na 251a, nao demitem). O tecnico que sai e fica com o contrato (0x4d0b50; contratos e vagas
# mudam). Todas mudam o banco ou os sorteios da carreira.
MUTACOES = [
    ('cpq1_sem_demissao', '    crate::comparacao::ramo("cpq_1 demitido");\n    record_departure(this, clube, 1);',
     '    crate::comparacao::ramo("cpq_1 demitido"); // mutacao: a demissao nao entra na lista',
     'cpq_1 demitido'),
    ('cpq1_ultimato_fora_da_lista', '    list_add_tail(cpq1_d(this + 0x10), &id as *const u32 as usize, 4);',
     '    let _ = id; // mutacao: o ultimato nao entra na lista',
     'cpq_1 ultimato dado'),
    ('cpq1_tecnico_com_contrato', '        sem_contrato(cpq1_d(0xae1050), tec);',
     '        let _ = (sem_contrato, tec); // mutacao: o tecnico fica com o contrato',
     'cpq_1 tirar o tecnico'),
]
