# Mutacoes (sx09_7), item de telas: a cadeia nao as roda; servem para provar o comparador numa sessao do Lucas
# (JOGAR_COMPARANDO.bat). Caminho no jogo: a data da barra de menu (sempre visivel, 0x54a950) e a lista de jogos do
# clube (Clube > Jogos, 0x54b550).
MUTACOES = [
    ('sx09_data_da_barra',
     'story_lang!(out, 0x9b86f8, DATE_TEXT_GRAMMAR, weekday, day, month, year); // sx09_7: the weekday and the date',
     'story_lang!(out, 0x9b86f8, DATE_TEXT_GRAMMAR, weekday, month, day, year); // mutation: the month first',
     'sx09_7 weekday numeric date'),
    ('sx09_dia_curto_sem_ano',
     'story_lang!(out, 0x9b89b0, SHORT_WEEKDAY_BUFFER, date_ordinal(day), name); // sx09_7: weekday, day and month',
     'story_lang!(out, 0x9b89b0, SHORT_WEEKDAY_BUFFER, date_ordinal(day + 1), name); // mutation: the next day',
     'sx09_7 short weekday date without the year'),
]
