# Mutacoes (cqe_1), na resposta da IA ao lance (0x8c8090; 791 chamadas na carreira automatica ate 2002-06-01,
# docs/cobertura_simulacao.txt, 17/10, pela oferta que vence hoje 0x8c63a0 e pelo lance aceito 0x8c0080):
# - a opcao aceita so com valor x 1,2 abaixo do pedido (antes, 0,85): o vendedor da IA quase nao aceita mais pela
#   opcao, e os lances viram contrapropostas ou recusas (contador "cqe_1 aceita");
# - sem contraproposta: o lance que nao foi aceito e recusado sempre (contador "cqe_1 contraproposta");
# - o preco sempre o piso do contrato (0x4d1600), sem o valor k da oferta quando ele e maior: as opcoes (dinheiro, troca,
#   clausula) saem mais baratas e o aceite e a contraproposta mudam (contador "cqe_1 preço pelo k").
# Os contadores ficam colados no efeito, depois de todas as condicoes do trecho mutado.
MUTACOES = [
    ('cqe1_aceita_menos',
     'if v as f64 * f64_em(0x969520) < pedido_f && troca_recusada == 0 { // cqe1: a opção × 0,85 abaixo do pedido',
     'if v as f64 * 1.2 < pedido_f && troca_recusada == 0 { // mutacao: a opção × 1,2 abaixo do pedido',
     'cqe_1 aceita'),
    ('cqe1_sem_contraproposta',
     'if v as f64 * f64_em(0x969040) < pedido_f && tipo != 1 && tipo != 2 && ensaio == 0 { // cqe1: a contraproposta',
     'if v as f64 * f64_em(0x969040) < pedido_f && tipo != 1 && tipo != 2 && ensaio == 0 && false { // mutacao: sem contraproposta',
     'cqe_1 contraproposta'),
    ('cqe1_preco_sempre_o_piso',
     'let preco = if k > piso { // cqe1: o preço pelo valor k',
     'let preco = if k > piso && false { // mutacao: sempre o piso',
     'cqe_1 preço pelo k'),
]
