# Date.cpp: os textos de data com o dia da semana (curto e longo) e o nome do mes (leitor sx09, rodada 5, 23/11/2026).
# Depende do sx09_1 (GameDate, DATE_CPP, date_month_index, days_before_month, write_date_name): integrar junto ou
# depois. Define os auxiliares dos textos de data (date_text_buffer_ok, date_text_parts, write_time_suffix,
# write_short_weekday, date_short_month_name, date_ordinal, compare_date_text), usados tambem pelo sx09_6 e sx09_7.
# Convencoes conferidas na desmontagem (as tres thiscall(data, saida, tamanho i16, hora byte), ret 0xc, devolvem 1, ou
# 0 depois da caixa de erro):
# - o mes e o primeiro laco (o ultimo mes com dias antes dele < dia do ano); sem nenhum (dia <= -1) o mes vem de um byte
#   da pilha nao iniciado: a copia chama a original, e o comparador so roda a original;
# - o dia da semana e calculado em linha (igual ao 0x54a3f0, crate::date::weekday) com tabelas de saltos
#   0x54b2ac/0x54b2c8 (curto, buffer 0xae2478), 0x54beb0/0x54becc e 0x54c370/0x54c38c (longo, os mesmos modelos e o
#   mesmo buffer 0xae2434 de date_get_weekday_name: a copia chama exe::date_get_weekday_name);
# - 0x54d210/0x54d3b0 (nome do mes) cdecl, 0x90c310 utils_format_ordinal cdecl(dia, 1) com buffer 0xde2734 (o ano, ja
#   empilhado antes da chamada do nome do mes, fica como argumento do langlib_sprintf);
# - o {} dos modelos longos recebe 0x64 (push 0x64);
# - a hora nos longos e langlib_sprintf(0xae2484, "Morning"...), igual a date_get_time_of_day_name(hora, 1).
# Chamadores na carreira: 0x54ae10 pelo CM3_MATCH_ENGINE::init_match (0x6b88aa, 0x6b8a01); 0x54ba20 pelo
# contract_manager (0x4d5307), pelo match_day (0x6b5002) e pelo laco do dia (0x5cddb5); 0x54bef0 pela midia (0x746e41,
# 0x7471ec, 0x749401) e pelo texto das noticias (0x77c164). Tambem pelas telas.
SAVE = False
ROTINAS = [('date_get_short_weekday_date_string', 0x54ae10, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('date_get_long_date_string', 0x54ba20, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('date_get_long_date_string_no_year', 0x54bef0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('as datas: os textos com o dia da semana e o nome do mes',
        'dates: the texts with the weekday and the month name',
        [('date_get_short_weekday_date_string', 'Date::get_short_weekday_date_string', '0x54ae10', 'Date.cpp', 'thiscall, ret 0xc',
          'escreve a data como dia da semana curto, dia ordinal, mes curto e ano ("Sat 10th Feb 1972"), com AM, PM ou EVE no fim quando pedida a hora (0 a 2); buffer nulo ou menor que 0x12: a caixa de erro e 0',
          'writes the date as short weekday, ordinal day, short month and year ("Sat 10th Feb 1972"), with AM, PM or EVE at the end when the time (0 to 2) is asked; a null buffer or one below 0x12: the error box and 0'),
         ('date_get_long_date_string', 'Date::get_long_date_string', '0x54ba20', 'Date.cpp', 'thiscall, ret 0xc',
          'escreve a data como dia da semana, dia ordinal, nome do mes e ano ("Saturday 10th February 1972"), com "(Morning)", "(Afternoon)" ou "(Evening)" quando pedida a hora; buffer nulo ou menor que 0x32: a caixa de erro e 0',
          'writes the date as weekday, ordinal day, month name and year ("Saturday 10th February 1972"), with "(Morning)", "(Afternoon)" or "(Evening)" when the time is asked; a null buffer or one below 0x32: the error box and 0'),
         ('date_get_long_date_string_no_year', 'Date::get_long_date_string_no_year', '0x54bef0', 'Date.cpp', 'thiscall, ret 0xc',
          'como a data longa, sem o ano ("Saturday 10th February")',
          'like the long date, without the year ("Saturday 10th February")')])
