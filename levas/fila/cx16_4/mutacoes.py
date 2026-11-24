# Mutacoes (cx16_4), ENG ate 2002-08-01: a campanha europeia desde 2008 na criacao (o ano da secao muda no banco da
# memoria, como a cqz4_america_do_sul_2008); o comparador roda so a original.
MUTACOES = [
    ('cx16_4_europa_desde_2008',
     'qualifying_campaign_year(year).max(0x7d4) // cx16_4: the European campaign, from 2004',
     'qualifying_campaign_year(year).max(0x7d8) // mutation: from 2008',
     'cx16_4 european qualifying'),
]
