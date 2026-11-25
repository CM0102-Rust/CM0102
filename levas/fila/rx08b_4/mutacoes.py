# Mutacoes para a sessao do Lucas (TELAS: a cadeia nao roda). O comparador da decisao acusa a primeira; a segunda (o
# boato) so se ve jogando em modo Rust (o tratador roda so a original na comparacao).
MUTACOES = [
    ('rx08b_4_cedo_demais_invertido',
     'if ultimatum_reputation(manager) >= 0x1482 && game_days >= 0xb4 { ANSWER_REFUSED } else { ANSWER_REFUSED_MILDLY } // rx08b_4: too soon after the last one',
     'if ultimatum_reputation(manager) >= 0x1482 && game_days >= 0xb4 { ANSWER_REFUSED_MILDLY } else { ANSWER_REFUSED } // rx08b_4 mutation',
     'rx08b_4 ultimatum answered'),
    ('rx08b_4_sem_boato',
     'let public = reputation > 0x1a5e || reputation > random(0x1a5e) + 0x7d0; // rx08b_4: the rumour',
     'let public = reputation > 0x1a5e; // rx08b_4 mutation',
     'rx08b_4 ultimatum handled'),
]
