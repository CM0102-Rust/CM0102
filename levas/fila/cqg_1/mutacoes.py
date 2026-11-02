# Mutacoes (cqg_1), no contrato assinado (0x4cf720; 1970 chamadas na carreira automatica ate 2002-06-01,
# docs/cobertura_simulacao.txt, 17/10):
# - o emprestimo dura uma semana a mais (o fim pela duracao a partir de hoje + 7 dias): o jogador volta mais tarde
#   (contador "cqg_1 empréstimo");
# - sem reaproveitar a vaga livre do vetor dos contratos: o contrato vai sempre para o fim, e a ordem dos contratos (que
#   o dia dos contratos percorre) muda (contador "cqg_1 vaga livre");
# - o valor do contrato nao vai para +0x4e da pessoa (fica 0) (contador "cqg_1 valor na pessoa").
MUTACOES = [
    ('cqg1_emprestimo_mais_uma_semana',
     'let fim = mais(0xae2c90, tmp2.as_mut_ptr(), n); // cqg1: o fim pela duração',
     'let fim = mais(0xae2c90, tmp2.as_mut_ptr(), n + 7); // mutacao: uma semana a mais',
     'cqg_1 empréstimo'),
    ('cqg1_sem_vaga_livre',
     'if cqg1_d(r) as i32 == -1 { // cqg1: a vaga livre',
     'if cqg1_d(r) as i32 == -1 && false { // mutacao: sem a vaga livre',
     'cqg_1 vaga livre'),
    ('cqg1_valor_zero_na_pessoa',
     'cqg1_pd(pessoa + 0x4e, cqg1_d(c + 0xc)); // cqg1: o valor do contrato na pessoa',
     'cqg1_pd(pessoa + 0x4e, 0); // mutacao: valor zero',
     'cqg_1 valor na pessoa'),
]
