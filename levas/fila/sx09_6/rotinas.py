# Date.cpp: o texto da data sem o dia da semana (leitor sx09, rodada 5, 23/11/2026). Depende do sx09_1 e do sx09_5
# (date_text_buffer_ok, date_text_parts, date_text_has_time, date_ordinal, compare_date_text): integrar junto ou
# depois deles.
# Convencao conferida na desmontagem: thiscall(data, saida, tamanho i16, hora byte), ret 0xc; erros nas linhas 0x2ed e
# 0x2f4 (tamanho minimo 0x32); a hora e langlib_sprintf(0xae2484, "Morning"...), igual a
# date_get_time_of_day_name(hora, 1); o nome do mes 0x54d3b0 cdecl (o ano empilhado antes fica como argumento do
# langlib_sprintf); o ordinal 0x90c310 cdecl(dia, 1).
# Chamadores na carreira: Discipline::write_ban_news_text (0x557b57, a data da suspensao, 38 vezes ate 2002-06-01),
# o texto das noticias (0x77daa6) e o TRANSFER_MANAGER (0x8daf6b).
SAVE = False
ROTINAS = [('date_get_date_string_no_weekday', 0x54c3b0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('as datas: o texto sem o dia da semana',
        'dates: the text without the weekday',
        [('date_get_date_string_no_weekday', 'Date::get_date_string_no_weekday', '0x54c3b0', 'Date.cpp', 'thiscall, ret 0xc',
          'escreve a data como dia ordinal, nome do mes e ano ("10th February 1972"), com "(Morning)", "(Afternoon)" ou "(Evening)" quando pedida a hora (0 a 2); buffer nulo ou menor que 0x32: a caixa de erro e 0',
          'writes the date as ordinal day, month name and year ("10th February 1972"), with "(Morning)", "(Afternoon)" or "(Evening)" when the time (0 to 2) is asked; a null buffer or one below 0x32: the error box and 0')])
