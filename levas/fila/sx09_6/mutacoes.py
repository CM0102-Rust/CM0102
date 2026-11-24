# Mutacao (sx09_6). A data da suspensao no texto da noticia (Discipline::write_ban_news_text, 38 vezes ate
# 2002-06-01) e o texto das noticias: o texto nao volta para a simulacao, entao o comparador da rotina (o texto, os
# buffers compartilhados e o retorno lado a lado) a acusa, como as de texto aceitas em 12/11/2026.
MUTACOES = [
    ('sx09_data_sem_dia_da_semana',
     'story_lang!(out, 0x9b8c4c, date_ordinal(day), name, year); // sx09_6: day, month and year',
     'story_lang!(out, 0x9b8c4c, date_ordinal(day), name, year - 1); // mutation: the year before',
     'sx09_6 date without the weekday'),
]
