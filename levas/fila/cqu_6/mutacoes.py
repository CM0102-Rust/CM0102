# Mutacoes (cqu_6): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
# A dos inscritos (o dia da Copa do Rei os junta uma vez, antes da primeira rodada): a Primeira nas vagas da Segunda
# (os clubes da Primeira entram mais cedo na copa: outros jogos).
MUTACOES = [
    ('cqu6_copa_do_rei_um_ano_adiante',
     'cqu6_copa_do_rei_corpo(this, ano, dono) // cqu6: copa_do_rei',
     'cqu6_copa_do_rei_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_6 copa_do_rei nova'),
    ('cqu6_supercopa_espanhola_um_ano_adiante',
     'cqu6_supercopa_espanhola_corpo(this, ano, dono) // cqu6: supercopa_espanhola',
     'cqu6_supercopa_espanhola_corpo(this, ano + 1, dono) // mutacao: um ano adiante',
     'cqu_6 supercopa_espanhola nova'),
    ('cqu6_primeira_antes_da_segunda',
     'cqu6_bloco(this, faltam_w, &segunda, ns); cqu6_bloco(this, faltam_w + ns.max(0), &primeira, np); // cqu6: a Segunda e depois a Primeira',
     'cqu6_bloco(this, faltam_w, &primeira, np); cqu6_bloco(this, faltam_w + np.max(0), &segunda, ns); // mutacao: a Primeira antes da Segunda',
     'cqu_6 segunda divisao'),
]
