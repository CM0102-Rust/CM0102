# Mutacoes (cqu_6): cada construtor roda uma vez, na criacao da carreira de 8 ligas (contadores "<item> <copa> nova"); a
# copa montada um ano adiante leva as rodadas, os inscritos e os jogos para a temporada seguinte.
# A dos inscritos (o dia da Copa do Rei os junta uma vez, antes da primeira rodada): a Primeira nas vagas da Segunda
# (os clubes da Primeira entram mais cedo na copa: outros jogos).
MUTACOES = [
    ('cqu6_copa_do_rei_um_ano_adiante',
     'construct_spa_cup_body(Cup(this), year, record) // cqu6: the Copa del Rey',
     'construct_spa_cup_body(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqu_6 copa del rey new'),
    ('cqu6_supercopa_espanhola_um_ano_adiante',
     'construct_spa_super_body(Cup(this), year, record) // cqu6: the Spanish Super Cup',
     'construct_spa_super_body(Cup(this), year + 1, record) // mutation: one year ahead',
     'cqu_6 spanish super cup new'),
    ('cqu6_primeira_antes_da_segunda',
     'spa_cup_put_block(this, missing_w, &second, ns); spa_cup_put_block(this, missing_w + ns.max(0), &first, nf); // cqu6: the Segunda and then the Primera',
     'spa_cup_put_block(this, missing_w, &first, nf); spa_cup_put_block(this, missing_w + nf.max(0), &second, ns); // mutation: the Primera before the Segunda',
     'cqu_6 second division'),
]
