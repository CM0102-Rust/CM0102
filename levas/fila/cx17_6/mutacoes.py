# Mutacoes (cx17_6), ENG ate 2002-08-01 (a Copa de 2002); acusadas pelos comparadores (lado a lado no retorno):
# - quem cai nos grupos com a situacao -3 no lugar de -4;
# - quem segue fora nos grupos (antes e agora 17 ou mais) com mudanca 1 no lugar de 0.
MUTACOES = [
    ('cx17_6_fora_nos_grupos_menos_3',
     '_ => -4, // cx17_6: out in the groups',
     '_ => -3, // mutation: out in the groups as -3',
     'cx17_6 world cup status'),
    ('cx17_6_segue_fora_muda',
     '[3, 3, 2, 0], // cx17_6: still out in the groups',
     '[3, 3, 2, 1], // mutation: still out changes',
     'cx17_6 world cup status change'),
]
