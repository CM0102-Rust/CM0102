# Mutacoes (cqu_13), nas copas entre continentes de 2001/02 (a Intercontinental em dezembro, a Interamericana em
# janeiro): o vice da Liga dos Campeoes no lugar do campeao europeu; o vice da CONCACAF no lugar do campeao (a America
# do Norte tem a MLS carregada); o vice da Libertadores na Interamericana.
MUTACOES = [
    ('cqu13_vice_europeu',
     'cqu13_vaga(this, 0x9cfa10, 0x1388, 0x9cf6e8, 0, 0xad5848, 0xea); // cqu13: o campeão europeu',
     'cqu13_vaga(this, 0x9cfa10, 0x1388, 0x9cf6e8, 1, 0xad5848, 0xea); // mutacao: o vice europeu',
     'cqu_13 clube europeu'),
    ('cqu13_vice_da_concacaf',
     'cqu13_vaga(this, 0x9cfa14, 0x3e8, 0x9cf72c, 0, 0x9e9d94, 0xea); // cqu13: o campeão da CONCACAF',
     'cqu13_vaga(this, 0x9cfa14, 0x3e8, 0x9cf72c, 1, 0x9e9d94, 0xea); // mutacao: o vice da CONCACAF',
     'cqu_13 clube norte-americano'),
    ('cqu13_vice_da_libertadores_na_interamericana',
     'cqu13_vaga(this, 0x9cfa1c, 0x1388, 0x9cf63c, 0, 0x9e9d94, 0x118); // cqu13: o campeão da Libertadores na Interamericana',
     'cqu13_vaga(this, 0x9cfa1c, 0x1388, 0x9cf63c, 1, 0x9e9d94, 0x118); // mutacao: o vice',
     'cqu_13 clube sul-americano da interamericana'),
]
