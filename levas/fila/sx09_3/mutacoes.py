# Mutacoes (sx09_3). Os nomes vao para os textos das noticias (news.cpp 0x7809a4, o texto da suspensao pela data sem
# dia da semana, as datas longas da midia): o texto nao volta para a simulacao, entao o comparador de cada rotina
# (buffer lado a lado) as acusa, como as de texto aceitas em 12/11/2026.
MUTACOES = [
    ('sx09_dia_da_semana',
     'write_date_name(WEEKDAY_NAME_BUFFER, date_name_template(&WEEKDAY_NAMES, weekday as i16 as i32)) // sx09_3: the weekday',
     'write_date_name(WEEKDAY_NAME_BUFFER, date_name_template(&WEEKDAY_NAMES, (weekday as i16 as i32 + 1) % 7)) // mutation: the next day',
     'sx09_3 weekday name'),
    ('sx09_mes_curto',
     'write_date_name(SHORT_MONTH_NAME_BUFFER, date_name_template(&SHORT_MONTH_NAMES, month as i8 as i32)) // sx09_3: the short month',
     'write_date_name(SHORT_MONTH_NAME_BUFFER, date_name_template(&MONTH_NAMES, month as i8 as i32)) // mutation: the full name',
     'sx09_3 short month name'),
    ('sx09_nome_do_mes',
     'write_date_name(MONTH_NAME_BUFFER, date_name_template(&MONTH_NAMES, month as i8 as i32)) // sx09_3: the month',
     'write_date_name(MONTH_NAME_BUFFER, date_name_template(&MONTH_NAMES, (month as i8 as i32 + 1) % 12)) // mutation: the next month',
     'sx09_3 month name'),
]
