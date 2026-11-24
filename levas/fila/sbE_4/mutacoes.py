# Mutacoes (sbE_4), com a Holanda ate 2002-08-01:
# - a primeira rodada da Eerste Divisie 7 dias depois (acusada pelo comparador lado a lado; o calendario de 2002/03 e
#   feito na virada);
# - o vencedor de um periodo anterior fica nas fases seguintes (pode ganhar dois periodos: outros classificados para a
#   repescagem de maio de 2002, a carreira);
# - o vencedor do periodo nao vai para a repescagem (outros classificados: a carreira).
MUTACOES = [
    ('sbE_4_primeira_rodada_depois',
     '(14, 7, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 0: 14 Aug',
     '(21, 7, 0, 5, 2, &[(0, 2, 1), (-1, -1, -1)]), // 0: 21 Aug (mutation)',
     'sbE_4 eerste divisie schedule'),
    ('sbE_4_vencedor_anterior_fica',
     '0xfe); // sbE_4: an earlier winner is out',
     '0xff); // mutation: an earlier winner stays in',
     'sbE_4 eerste divisie period stage'),
    ('sbE_4_vencedor_fora_da_repescagem',
     'row.set_status(1); // sbE_4: the period winner goes to the play-off',
     'row.set_status(0xff); // mutation: the period winner stays undecided',
     'sbE_4 eerste divisie period winner'),
]
