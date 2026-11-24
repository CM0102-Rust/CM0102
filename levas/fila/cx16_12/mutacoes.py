# Mutacoes (cx16_12), ENG ate 2005-12-31 (o fim das eliminatorias europeias, novembro de 2005): os vencedores da
# repescagem pela situacao 2 (os perdedores ficam classificados: quem vai a Copa muda); o comparador roda so a original,
# entao so a carreira Rust ate o fim de 2005 a acusa.
MUTACOES = [
    ('cx16_12_perdedores_classificados',
     'if read_u8(team + 5) == 1 { // cx16_12: a play-off winner',
     'if read_u8(team + 5) == 2 { // mutation: the losers',
     'cx16_12 european play-off qualifiers'),
]
