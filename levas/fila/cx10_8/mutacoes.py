# Mutacoes (cx10_8): textos, acusados pela comparacao com CM_TEXTOS=1 (o texto nao volta para a simulacao). Nenhuma abre
# caixa. Os dividendos saem no fim da temporada do clube (season_end_club_update) e as obras pelo fim da temporada e
# pelos eventos da semana: podem escapar se nenhuma dessas noticias sair na carreira ENG ate 2002-06-01.
MUTACOES = [
    ('cx10_8_dividendo_formato_6',
     '        exe::cash_format_money(mp, 0x3e8, exe::cash_get_display_amount(&amount, 4), 4);',
     '        exe::cash_format_money(mp, 0x3e8, exe::cash_get_display_amount(&amount, 6), 6); // mutation'),
    ('cx10_8_obras_cadeiras_como_geral',
     '            3 => fmt(buf, 0x9c97ac, number), // install <n> seats',
     '            3 => fmt(buf, 0x9c9800, number), // mutation'),
]
