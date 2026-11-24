# Mutacoes (sx09_5). Os textos de data vao para as noticias, a midia e o relatorio da partida: nao voltam para a
# simulacao, entao o comparador de cada rotina (o texto, os buffers compartilhados e o retorno lado a lado) as acusa,
# como as de texto aceitas em 12/11/2026. As tres estao no ramo sem a hora, o que os chamadores da carreira pedem.
MUTACOES = [
    ('sx09_data_curta',
     'story_lang!(out, 0x9b881c, SHORT_WEEKDAY_BUFFER, date_ordinal(day), name, year); // sx09_5: day, month, year',
     'story_lang!(out, 0x9b881c, SHORT_WEEKDAY_BUFFER, date_ordinal(day + 1), name, year); // mutation: the next day',
     'sx09_5 short weekday date'),
    ('sx09_data_longa',
     'story_lang!(out, 0x9b8aa0, DATE_TEXT_GRAMMAR, weekday, date_ordinal(day), name, year); // sx09_5: the long date',
     'story_lang!(out, 0x9b8aa0, DATE_TEXT_GRAMMAR, weekday, date_ordinal(day), name, year + 1); // mutation: the next year',
     'sx09_5 long date'),
    ('sx09_data_sem_ano',
     'story_lang!(out, 0x9b8b84, DATE_TEXT_GRAMMAR, weekday, date_ordinal(day), name); // sx09_5: without the year',
     'story_lang!(out, 0x9b8b84, DATE_TEXT_GRAMMAR, name, date_ordinal(day), weekday); // mutation: the weekday and the month swapped',
     'sx09_5 long date without the year'),
]
