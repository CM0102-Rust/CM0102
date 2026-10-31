# Mutacoes. Todas nos jogos da selecao no periodo, que a permissao de trabalho 0x8c7950 limita a 16 e divide pelos
# jogos que o jogador jogou (com sorteios de 60 a 80 por cento): um jogo a mais, sem os jogos do mes de hoje e no maximo
# oito mudam permissoes aceitas e recusadas (e, com conta zero, os sorteios que a original nao faz). Na carreira nova o
# historico comeca vazio em julho de 2001 e cresce com as datas FIFA (agosto, setembro, outubro, novembro, fevereiro,
# marco, abril, maio). Chamada a cada permissao que chega aos jogos da selecao: sem contador. A atualizacao do dia 18
# so tira resultados de mais de um ano, que a carreira ate 2002-06-01 ainda nao tem: sem mutacao nela (as pontuacoes e
# posicoes so servem a tela e ao peso dos pontos, fora da impressao).
MUTACOES = [
    ('cpt1_um_jogo_a_mais', 'n // cpt1: os jogos no periodo',
     'n.wrapping_add(1) // mutacao: um jogo a mais'),
    ('cpt1_sem_o_mes_final', 'if v <= ate { // cpt1: ate o mes final',
     'if v < ate { // mutacao: sem o mes final'),
    ('cpt1_no_maximo_oito', 'n = n.wrapping_add(1); // cpt1: mais um jogo',
     'if n < 8 { n += 1; } // mutacao: no maximo oito'),
]
