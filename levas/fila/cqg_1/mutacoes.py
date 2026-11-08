# Mutacoes (cqg_1), no contrato assinado (0x4cf720; 1970 chamadas na carreira automatica ate 2002-06-01,
# docs/cobertura_simulacao.txt, 17/10). O ramo do emprestimo (+0x35 = 5) nao rodou na carreira da copia (contador
# "cqg_1 loan" ausente): sem mutacao nele.
# - a renovacao no mesmo clube tratada como contrato novo (sem manter os termos antigos: o tipo pelo cargo, a
#   felicidade zerada pela 0x866ee0, +0x3b zerado) (contador "cqg_1 same club", 1024 na carreira);
# - sem reaproveitar a vaga livre do vetor dos contratos: o contrato vai sempre para o fim, e a ordem dos contratos (que
#   o dia dos contratos percorre) muda (contador "cqg_1 free slot", 32);
# - o valor do contrato nao vai para +0x4e da pessoa (fica 0) (contador "cqg_1 value on the person", 1024).
MUTACOES = [
    ('cqg1_renovacao_como_nova',
     'if cqg1_d(r + 4) == cqg1_d(c + 4) { // cqg1: o mesmo clube',
     'if cqg1_d(r + 4) == cqg1_d(c + 4) && false { // mutacao: renovacao como contrato novo',
     'cqg_1 same club'),
    ('cqg1_sem_vaga_livre',
     'if cqg1_d(r) as i32 == -1 { // cqg1: a vaga livre',
     'if cqg1_d(r) as i32 == -1 && false { // mutacao: sem a vaga livre',
     'cqg_1 free slot'),
    ('cqg1_valor_zero_na_pessoa',
     'cqg1_pd(pessoa + 0x4e, cqg1_d(c + 0xc)); // cqg1: o valor do contrato na pessoa',
     'cqg1_pd(pessoa + 0x4e, 0); // mutacao: valor zero',
     'cqg_1 value on the person'),
]