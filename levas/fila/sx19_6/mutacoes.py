# Mutacoes (sx19_6), pela carreira ENG SWE FIN NOR POR ate 2002-08-01:
# - Portugal suspendendo a cada quarto amarelo (e nao a cada quinto);
# - a Suecia suspendendo tambem no quarto amarelo (e nao so no terceiro e nos impares do quinto em diante).
MUTACOES = [
    ('sx19_6_portugal_quarto',
     'stepped_card_verdict(this, row, fixture, entry, &PORTUGAL_RULING_BODY, 5) // sx19_6: every fifth Portuguese booking',
     'stepped_card_verdict(this, row, fixture, entry, &PORTUGAL_RULING_BODY, 4) // mutation: every fourth',
     'sx19_6 portuguese cards'),
    ('sx19_6_suecia_quarto',
     'if count == 3 || (count > 4 && count % 2 == 1) { // sx19_6: the Swedish booking bans',
     'if count == 3 || count == 4 || (count > 4 && count % 2 == 1) { // mutation: the fourth bans too',
     'sx19_6 swedish booking ban'),
]
