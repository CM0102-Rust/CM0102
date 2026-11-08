# Mutacoes (com o ramo de cada uma). Todas na fila da transferencia livre (a proxima janela tem a data regravada com
# hoje por quem a chama, e o emprestimo negociado so roda na tela). Com o sorteio do desafeto invertido, outros
# tecnicos entram nos desafetos dos liberados (com o espaco cheio, a 0x547af0 sorteia a vaga: a sequencia do sorteio
# muda). Sem os registros da liberacao, a historia do jogador e do tecnico perde a entrada (vai para o save). Com a
# compensacao em dobro, o clube recebe outro valor (o dinheiro muda as transferencias). Os ramos dizem se a carreira
# passou por eles.
MUTACOES = [
    ('cps3_desafeto_invertido', 'if r < sorteio(0x14) || gosta(p, clube) != 0 { // cps3: o sorteio do desafeto',
     'if r > sorteio(0x14) || gosta(p, clube) != 0 { // mutacao: invertido',
     'cps_3 released'),
    ('cps3_sem_registros', 'registros(cps3_d(0xdd7f00), p); // cps3: os registros da liberação',
     '// mutacao: sem os registros',
     'cps_3 released'),
    ('cps3_compensacao_em_dobro', 'let comp = cps3_d(e + 5); // cps3: a compensação ao clube',
     'let comp = cps3_d(e + 5).wrapping_mul(2); // mutacao: em dobro',
     'cps_3 compensation'),
]
