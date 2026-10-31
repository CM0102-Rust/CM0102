# Mutacoes. Sem pedidos de observacao, o jogo passa a ter jogador observado (a partida_sem_humano 0x6b66a0 devolve 0:
# a partida deixa de ser simulada sem o humano e o motor sorteia outra coisa). A competicao negativa tambem passa a ter
# observado (so pega se a carreira tiver partida com competicao negativa; senao o ramo nao roda e fica lacuna). Na
# lista do olheiro, todo jogador passa a estar na lista do humano de ferias: os clubes interessados (0x842180) ganham
# o clube dele e as transferencias mudam. O registro da primeira convocacao so grava o historico em arquivo (fora da
# impressao): sem mutacao.
MUTACOES = [
    ('cps8_sem_pedidos_observa', 'return 0; // cps8: sem pedidos de observação',
     'return 1; // mutacao: sem pedidos, com observado',
     'cps_8 sem pedidos'),
    ('cps8_competicao_negativa_observa', 'return 0; // cps8: competição negativa',
     'return 1; // mutacao: competicao negativa, com observado',
     'cps_8 competicao negativa'),
    ('cps8_sempre_na_lista', 'return r; // cps8: a resposta da lista do olheiro',
     'let _ = r; return 1; // mutacao: sempre na lista',
     'cps_8 lista do olheiro'),
]
