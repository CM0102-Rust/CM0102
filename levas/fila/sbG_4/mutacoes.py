# Mutacoes (sbG_4), carreira ENG ate 2002-06-01 (os torneios de pre-temporada de julho de 2001):
# - a resposta invertida (quem aceitava recusa): acusada pelo comparador (o plano das respostas) e pela carreira (os
#   torneios nao nascem e os amistosos de julho mudam);
# - a minicopa sem a primeira rodada montada (Cup::set_up_round): os jogos dela nao saem e os amistosos de julho mudam
#   (so a carreira acusa: o comparador roda a original).
MUTACOES = [
    ('sbG_4_resposta_invertida',
     '    !refuses\n}',
     '    refuses // mutation: the answer inverted\n}',
     'sbG_4 tournament reply'),
    ('sbG_4_copa_sem_rodada',
     '        exe::cup_set_up_round(comp, 0);',
     '        let _ = comp; // mutation: the cup round not set up',
     'sbG_4 tournament created'),
]
