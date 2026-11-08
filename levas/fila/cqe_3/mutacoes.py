# Mutacoes (cqe_3), nos lances da IA pelas vagas da lista de observacao (0x84a020; 210231 chamadas na carreira automatica
# ate 2002-06-01, docs/cobertura_simulacao.txt, 17/10, pelo laco do dia 0x5cccd0):
# - o sorteio do lance contra 2 x injury_word_0c + 2000 (antes, + 1000): menos lances, e os sorteios seguintes mudam (contador
#   "cqe_3 sorteio do lance");
# - o sorteio do pedido de emprestimo contra injury_word_0c / 5 (antes, / 10): menos pedidos no primeiro par (contador "cqe_3
#   sorteio do empréstimo");
# - o valor da vaga x 3 (antes, x 2) pela liga ativa com injury_word_0c ate 0x128e e conta abaixo de 25: o sorteio(valor) muda
#   (contador "cqe_3 dobro pela liga").
# Os contadores ficam colados no efeito: os dos sorteios logo antes do sorteio mutado, depois de todos os filtros da vaga;
# o do dobro dentro do ramo, junto da conta mutada.
MUTACOES = [
    ('cqe3_lance_mais_dificil',
     'let r1 = sorteio((cqe3_w(clube + 0x80) as i32).wrapping_mul(2).wrapping_add(1000)); // cqe3: o sorteio do lance',
     'let r1 = sorteio((cqe3_w(clube + 0x80) as i32).wrapping_mul(2).wrapping_add(2000)); // mutacao: + 2000',
     'cqe_3 sorteio do lance'),
    ('cqe3_emprestimo_mais_dificil',
     'let r1 = sorteio(cqe3_w(clube + 0x80) as i32 / 10); // cqe3: o sorteio do empréstimo',
     'let r1 = sorteio(cqe3_w(clube + 0x80) as i32 / 5); // mutacao: injury_word_0c / 5',
     'cqe_3 sorteio do empréstimo'),
    ('cqe3_triplo_pela_liga',
     'v = v.wrapping_mul(2); // cqe3: o dobro pela liga ativa',
     'v = v.wrapping_mul(3); // mutacao: o triplo',
     'cqe_3 dobro pela liga'),
]
