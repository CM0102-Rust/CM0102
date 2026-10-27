# Mutacoes das permissoes de trabalho. A da oferta diz que o jogador da oferta nunca precisa: deve acusar (ofertas
# por jogadores de fora da UE passam). A dos anos pede 6 para a segunda nacionalidade e a da marca so apaga o bit 0 em
# vez de zerar: o comparando acusa na primeira atualizacao com permissao velha ou marca sem o bit 0x20; a carreira deve
# acusar pela nacionalidade (escalacao de estrangeiros).
MUTACOES = [
    ('cph_permissao_oferta', 'return 1; // o jogador da oferta',
     'return 0; // mutacao: nunca precisa'),
    ('cph_permissao_anos', 'if anos > 4 {',
     'if anos > 5 { // mutacao: 6 anos'),
    ('cph_permissao_marca', '*marca(pessoa) = 0; // a marca sem permissão',
     '*marca(pessoa) &= 0xfe; // mutacao: so o bit 0'),
]
