# Mutacoes (sbD_6), pela carreira das 8 ligas e pelos comparadores das inscricoes:
# - a cota do Nordeste na Copa do Brasil 7 -> 5 (entram outros clubes);
# - a Serie A entra a partir da quarta linha, nao da terceira;
# - a Copa dos Campeoes: o pior entre Goias e Santa Catarina no lugar do melhor.
MUTACOES = [
    ('sbD_6_cota_do_nordeste', '    (KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 7),',
     '    (KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 5), // mutation', 'sbD_6 brazilian cup entrants'),
    ('sbD_6_serie_a_da_quarta', '    let mut row: i8 = 2;\n    if serie_a.club_count() > 2 {',
     '    let mut row: i8 = 3; // mutation\n    if serie_a.club_count() > 2 {', 'sbD_6 brazilian cup entrants'),
    ('sbD_6_goias_santa_catarina', '    let second = if goias.1 > santa_catarina.1 {',
     '    let second = if goias.1 < santa_catarina.1 { /* mutation */', 'sbD_6 champions cup entrants'),
]
