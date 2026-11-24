# Mutacoes (cx16_1), ENG ate 2002-08-01:
# - a campanha da CONCACAF desde 2004 na criacao (o ano da secao muda no banco da memoria e a campanha deixa de ficar
#   dormente: os grupos da America Central nascem logo, como na cqz4_oceania_de_2004);
# - as faixas do meio trocadas (7 e 10): acusada pelo comparador da rank_teams (lado a lado, na criacao e na virada);
# - 23 vagas no mata-mata caribenho: acusada pelo comparador da build_team_table (lado a lado).
MUTACOES = [
    ('cx16_1_concacaf_desde_2004',
     'cup.set_year(qualifying_campaign_year(cup.year())); // cx16_1: the campaign of the four-year cycle',
     'cup.set_year(qualifying_campaign_year(cup.year()).max(0x7d4)); // mutation: the campaign from 2004',
     'cx16_1 concacaf qualifying'),
    ('cx16_1_faixas_trocadas',
     'v.extend((6..12).map(|s| if s < 9 { 7 } else { 10 })); // cx16_1: the middle bands',
     'v.extend((6..12).map(|s| if s < 9 { 10 } else { 7 })); // mutation: swapped bands',
     'cx16_1 concacaf predicted positions'),
    ('cx16_1_23_caribenhas',
     'cup.set_team_count((0x18i16).wrapping_sub(read_u8(this + 0xbb) as i8 as i16)); // cx16_1: 24 Caribbean places',
     'cup.set_team_count((0x17i16).wrapping_sub(read_u8(this + 0xbb) as i8 as i16)); // mutation: 23 places',
     'cx16_1 concacaf team table'),
]
