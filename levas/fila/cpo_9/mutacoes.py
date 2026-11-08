# Mutacoes (com o ramo de cada uma). Acrescentar sem o vetor pela pessoa (o 0x842180 nao acha os clubes que observam o
# jogador: menos interessados, outras transferencias; o comparando acusa pela conta). Prever "recusa" no lugar de
# "negocia" (os lances da IA abaixo do preco deixam de ser negociados: as transferencias mudam; o comparando acusa pelo
# valor). O periodo do emprestimo recuando dois meses de cada vez (o fim do emprestimo muda: a volta do jogador e as
# escalacoes; o comparando acusa pelas saidas). Os ramos dizem se a carreira passou por eles.
MUTACOES = [
    ('cpo_lista_pela_pessoa', 'insert_sorted_pair(this, 0, 0x10, pessoa, clube, 0x757ea0); // o vetor pela pessoa',
     '// mutacao: sem o vetor pela pessoa',
     'cpo_9 nova entrada'),
    ('cpo_prever_negocia', 'crate::comparacao::ramo("cpo_9 negocia");\n    1\n}',
     'crate::comparacao::ramo("cpo_9 negocia");\n    2 // mutacao: recusa\n}',
     'cpo_9 negocia'),
    ('cpo_periodo_recua', 'let m = (*mes).wrapping_sub(1);',
     'let m = (*mes).wrapping_sub(2); // mutacao: dois meses',
     'cpo_9 recua o mes'),
]
