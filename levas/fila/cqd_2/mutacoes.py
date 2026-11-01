# Mutacoes (cqd_2), no lance aceito pelo clube vendedor (0x8c0080; 748 chamadas na carreira automatica ate 2002-06-01,
# docs/cobertura_simulacao.txt, 17/10, pela oferta que vence hoje 0x8c63a0 e pela resposta da IA 0x8c8090):
# - a oferta aceita reagendada um dia depois (em vez da fatia seguinte): o contrato ao jogador e a conclusao atrasam,
#   e as janelas, os elencos e os saldos mudam;
# - o emprestimo recusado pelo alvo com interesse abaixo de 2 (antes, 0,75): quase todo emprestimo aceito pelo clube cai
#   na recusa em se mudar (pega se houver emprestimo da IA ate 2002-06-01: contador "cqd_2 emprestimo");
# e no emprestimo possivel (0x8d0040; 309108 chamadas, pelas listas de observacao da IA 0x83fe1f e 0x84aa2b):
# - o clube que nao pode chegar ao jogador (0x8ce990: na fila, sob oferta, em troca, a regra basca...) segue como se
#   pudesse: a IA passa a mirar emprestimos de jogadores que antes saiam da lista, e as listas, as ofertas e os elencos
#   mudam.
# Os contadores ficam colados no efeito: o do lance aceito depois de todas as condicoes da troca; o do emprestimo logo
# antes da comparacao mutada; o do clube que chega depois do clube atual, da ficha e do cargo, antes do teste mutado.
MUTACOES = [
    ('cqd2_aceita_um_dia_depois',
     'agendar(this, o, 0, 1); // cqd2: a próxima fatia',
     'agendar(this, o, 1, 1); // mutacao: um dia depois',
     'cqd_2 lance aceito'),
    ('cqd2_emprestimo_recusado',
     'if !(r >= 0.75) { // cqd2: o alvo não quer se mudar',
     'if !(r >= 2.0) { // mutacao: o alvo quase nunca quer se mudar',
     'cqd_2 empréstimo'),
    ('cqd2_clube_chega_sempre',
     'if chegar(this, jogador, clube, &mut motivo, 0) == 0 { // cqd2: o clube não chega ao jogador',
     'if chegar(this, jogador, clube, &mut motivo, 0) == 0 && false { // mutacao: o clube chega sempre',
     'cqd_2 clube chega'),
]
