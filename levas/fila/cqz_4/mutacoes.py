# Mutacoes (cqz_4): a campanha asiatica desde 2000 (o ano da secao muda no banco da memoria); a importancia 0xf nos jogos
# das eliminatorias da Oceania (que jogam em 2001: muda a importancia das partidas); a campanha sul-americana desde 2008.
MUTACOES = [
    ('cqz4_asia_desde_2000',
     'qualifying_campaign_year(year as i16).max(0x7d4); // cqz4: the Asian campaign, from 2004',
     'qualifying_campaign_year(year as i16).max(0x7d0); // mutation: from 2000',
     'cqz_4 asian qualifying'),
    ('cqz4_oceania_importancia',
     'let importance = 0xa; // cqz4: the importance of the Oceania qualifying matches',
     'let importance = 0xf; // mutation: more important matches',
     'cqz_4 oceania qualifying'),
    ('cqz4_america_do_sul_2008',
     'qualifying_campaign_year(year as i16).max(0x7d4); // cqz4: the South American campaign, from 2004',
     'qualifying_campaign_year(year as i16).max(0x7d8); // mutation: from 2008',
     'cqz_4 south american qualifying'),
]
