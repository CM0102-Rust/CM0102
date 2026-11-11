# Mutacoes (cqu_13), nas copas entre continentes de 2001/02 (a Intercontinental em dezembro, a Interamericana em
# janeiro): o vice da Liga dos Campeoes no lugar do campeao europeu; o vice da CONCACAF no lugar do campeao (a America
# do Norte tem a MLS carregada); o vice da Libertadores na Interamericana.
MUTACOES = [
    ('cqu13_vice_europeu',
     'intercontinental_cup_slot(this, KEY_CONTINENT_EUROPE, 0x1388, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, 0, 0xad5848, 0xea); // cqu13: the European champion',
     'intercontinental_cup_slot(this, KEY_CONTINENT_EUROPE, 0x1388, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, 1, 0xad5848, 0xea); // mutation: the European runner-up',
     'cqu_13 european club'),
    ('cqu13_vice_da_concacaf',
     'intercontinental_cup_slot(this, KEY_CONTINENT_NORTH_AMERICA, 0x3e8, KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP, 0, 0x9e9d94, 0xea); // cqu13: the CONCACAF champion',
     'intercontinental_cup_slot(this, KEY_CONTINENT_NORTH_AMERICA, 0x3e8, KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP, 1, 0x9e9d94, 0xea); // mutation: the CONCACAF runner-up',
     'cqu_13 north american club'),
    ('cqu13_vice_da_libertadores_na_interamericana',
     'intercontinental_cup_slot(this, KEY_CONTINENT_SOUTH_AMERICA, 0x1388, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES, 0, 0x9e9d94, 0x118); // cqu13: the Libertadores champion in the Interamerican Cup',
     'intercontinental_cup_slot(this, KEY_CONTINENT_SOUTH_AMERICA, 0x1388, KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES, 1, 0x9e9d94, 0x118); // mutation: the runner-up',
     'cqu_13 interamerican south american club'),
]
