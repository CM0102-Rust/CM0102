# Mutacoes (sx18_23), pela carreira com LIGAS 'ENG KOR ARG ITA FRA ESP AUS' ate 2002-08-01: as suspensoes tiram jogadores das escalacoes seguintes;
# as de construtor (escala) sao acusadas pelo comparador do construtor (os campos contra a classe).
MUTACOES = [
    ('sx18_23_quarto_coreano',
     'if count >= 3 && (count - 3) % 2 == 0 { // sx18_23',
     'if count >= 4 && (count - 4) % 2 == 0 { // mutation',
     'sx18_23 korean booking ban'),
    ('sx18_23_terceiro_asiatico',
     'plain_card_verdict(row, entry, false, 2, [0xff, 0xff, ASIA',
     'plain_card_verdict(row, entry, false, 3, [0xff, 0xff, ASIA',
     'sx18_23 asian cards'),
]
