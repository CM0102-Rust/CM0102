# Mutacoes (com o ramo de cada uma). O jogador que chega a um clube que exige permissao fica sem ela (o vetor de
# permissoes, que vai para o save, e as concessoes seguintes 0x8c7950 mudam). O clube nao paga as luvas do contrato
# assinado (o saldo do clube fica maior: verbas e transferencias seguintes mudam). O contrato oferecido ao jogador de
# uma transferencia com fator de salario 1,1 (os salarios e as respostas do jogador mudam). Os ramos dizem se a
# carreira passou por eles.
MUTACOES = [
    ('cpq7_sem_permissao', 'dar(this, pessoa, cpq7_d(clube + 0x53), ano, 0, 1); // a permissão nova',
     'let _ = (dar, ano); // mutacao: sem a permissao nova',
     'cpq_7 new permit'),
    ('cpq7_sem_luvas', 'luvas(fin, valor); // as luvas',
     'let _ = (luvas, fin, valor); // mutacao: sem luvas',
     'cpq_7 signing-on fee'),
    ('cpq7_fator', 'cpq7_d(oferta + 0x20), 1.0f64, 2) != 0 {',
     'cpq7_d(oferta + 0x20), 1.1f64, 2) != 0 { // mutacao: fator 1,1',
     'cpq_7 offer contract'),
]
