# Mutacoes das ordens por coordenada. A da latitude ordena pela longitude: a copa regional inglesa (eng_auto_cup
# 0x56b730, divisao norte/sul) roda na carreira da Inglaterra ate 2002-06-01, que deve acusar. A da longitude ordena
# pela latitude: so roda em ligas e copas de outras nacoes (bel_first, hol_cup, ita_c_cup, spa_first, tur_first):
# ESCAPA da carreira da Inglaterra (lacuna); a carreira ampla (ESP, ITA) e o jogo real comparando com essas ligas
# podem acusar.
MUTACOES = [
    ('dbr_latitude', '    dbr_ordem_pela_coordenada(ca, cb, 0x23) // a latitude', '    dbr_ordem_pela_coordenada(ca, cb, 0x2b) // mutacao: a longitude no lugar da latitude'),
    ('dbr_longitude', '    dbr_ordem_pela_coordenada(ca, cb, 0x2b) // a longitude', '    dbr_ordem_pela_coordenada(ca, cb, 0x23) // mutacao: a latitude no lugar da longitude'),
]
