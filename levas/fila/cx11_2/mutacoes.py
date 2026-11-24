# Mutacoes (cx11_2), com a Italia e a base NORMAL ate 2002-09-30 (os construtores rodam na criacao do jogo):
# - a C2/A recebe o calendario real da C2/B: nenhum nome da C2/B esta na tabela da C2/A, a lista fixa cai e a liga
#   sorteia o proprio calendario (outros jogos em cada rodada da temporada inteira);
# - o mesmo na C2/B com o calendario da C2/A.
# Se os nomes de 2001/02 nao estiverem no banco da carreira (como na C1 com a base minima, ver cqm_4), a lista cai nos
# dois casos e as duas escapam; entao a prova diz isso.
MUTACOES = [
    ('cx11_2_c2a_com_o_calendario_da_c2b',
     'fixed: 0x6545c0, // cx11_2: the real 2001/02 C2/A calendar',
     'fixed: 0x658750, // mutation: the C2/B calendar',
     'cx11_2 serie c2 built'),
    ('cx11_2_c2b_com_o_calendario_da_c2a',
     'fixed: 0x658750, // cx11_2: the real 2001/02 C2/B calendar',
     'fixed: 0x6545c0, // mutation: the C2/A calendar',
     'cx11_2 serie c2 built'),
]
