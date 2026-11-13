# Mutations (mwe_6): a different prize share in each Premier Division table (the prize money paid at the end of
# the 2001 season changes the clubs' cash, and with it the transfer and wage decisions).
MUTACOES = [
    ('mwe6_premio_da_tippeligaen',
     'KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, 0xc350, || ()) // mwe6: the Tippeligaen prize share',
     'KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, 0x1c350, || ()) // mutation: another share',
     'mwe_6 tippeligaen table'),
    ('mwe6_premio_da_allsvenskan',
     'KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, 0xc350, || ()) // mwe6: the Allsvenskan prize share',
     'KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, 0x1c350, || ()) // mutation: another share',
     'mwe_6 allsvenskan table'),
    ('mwe6_premio_da_veikkausliiga',
     'KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, 0x88b8, || ()) // mwe6: the Veikkausliiga prize share',
     'KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, 0x188b8, || ()) // mutation: another share',
     'mwe_6 veikkausliiga table'),
]
