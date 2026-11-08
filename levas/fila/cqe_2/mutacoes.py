# Mutacoes (cqe_2), na opcao de troca da resposta da IA ao lance (0x8c8860; 782 chamadas na carreira automatica ate
# 2002-06-01, docs/cobertura_simulacao.txt, 17/10, pela 0x8c8090). Na carreira automatica (so a Inglaterra) a troca
# fica vazia: o candidato da lista do vendedor precisa jogar no proprio comprador, e os contadores "cqe_2 interest",
# "cqe_2 swap rating" e "cqe_2 player in the swap" nao rodam ate 2002-06-01. As mutacoes miram o que roda a cada
# chamada:
# - o preco da opcao pela metade: a resposta da IA aceita pela opcao de troca lances que o dinheiro nao aceitava
#   (contador "cqe_2 option built");
# - o teste da lista de observacao do vendedor invertido: a opcao (e o sorteio da 0x8e78f0) passa a sair so para os
#   vendedores sem lista ativa (contador "cqe_2 option built");
# - so o alvo como candidato (as 25 vagas da lista do vendedor ficam de fora): as notas das vagas (0x6e2f20 com a marca
#   0x4000, que atualiza a lesao 0x62ed70) e as consultas deixam de rodar (contador "cqe_2 slot with target").
MUTACOES = [
    ('cqe2_preco_pela_metade',
     'cqe2_pd(nova + 0x1b, valor as u32 as usize); // cqe2: o preço da opção',
     'cqe2_pd(nova + 0x1b, (valor / 2) as u32 as usize); // mutacao: a metade',
     'cqe_2 option built'),
    ('cqe2_lista_do_vendedor_invertida',
     'if cqe2_b(reg + 4) == 0 { // cqe2: o vendedor sem lista de observação ativa',
     'if cqe2_b(reg + 4) != 0 { // mutacao: invertido',
     'cqe_2 option built'),
    ('cqe2_so_o_alvo',
     'if id <= -1 { // cqe2: a vaga sem alvo',
     'if true { // mutacao: so o alvo',
     'cqe_2 slot with target'),
]
