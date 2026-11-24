# Date.cpp: os textos de data numericos e curtos das telas (leitor sx09, rodada 5, 23/11/2026). Item de telas: a prova
# e o Lucas jogando comparando. Depende do sx09_1 e do sx09_5 (os auxiliares dos textos de data e compare_date_text):
# integrar junto ou depois deles.
# Convencoes conferidas na desmontagem (as cinco thiscall(data, saida, tamanho i16, hora byte), ret 0xc, devolvem 1,
# ou 0 depois da caixa de erro):
# - 0x54a470: linhas 0xf8/0xff, minimo 9; langlib(saida, 0x9b8560, dia, mes+1, ano) ou 0x9b84a0 com o sufixo 0xae24ac.
# - 0x54a650: linhas 0x132/0x139, minimo 9; le a lingua (0x66f4d0) antes do sufixo; lingua 10: sprintf(saida,
#   "%02d-%02d-%02d"[ " %s"], ano%100, mes+1, dia[, sufixo]); senao langlib 0x9b8624/0x9b85ac (dia, mes+1, ano%100).
# - 0x54a950: linhas 0x181/0x188, minimo 0x12; o dia da semana em linha (tabelas 0x54add8/0x54adf4, buffer 0xae2434,
#   os modelos de date_get_weekday_name); langlib 0x9b86f8/0x9b867c (0x64 do {}, dia da semana, dia, mes+1, ano%100[,
#   sufixo]); o 0x64 e o ecx que sobrou do idiv por 100.
# - 0x54b2f0: linhas 0x1fb/0x202, minimo 0x12; o dia do mes pela procura de dezembro para tras (o mesmo valor);
#   langlib 0x9b8900/0x9b8898 (ordinal, mes curto, ano[, sufixo]).
# - 0x54b550: linhas 0x234/0x23b, minimo 0x12; dia da semana curto em linha (tabelas 0x54b9dc/0x54b9f8, buffer
#   0xae2478); langlib 0x9b89b0/0x9b894c (dia curto, ordinal, mes curto[, sufixo]).
# Chamadores (so telas): 0x54a470 notes.cpp (0x7958d1); 0x54a650 award_screens, club_screens e
# club_records_format_record_text (0x449810, o texto dos recordes na tela); 0x54a950 Menubar.cpp (0x75e155, a data da
# barra de menu) e 0x761cc4; 0x54b2f0 transfer_screens (0x8fa2fc); 0x54b550 club_screens, news_screens (0x788a16) e
# transfer_screens (0x8f5a98).
TELAS = True
SAVE = False
ROTINAS = [('date_get_numeric_date_string', 0x54a470, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('date_get_short_date_string', 0x54a650, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('date_get_weekday_numeric_date_string', 0x54a950, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('date_get_short_month_date_string', 0x54b2f0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('date_get_short_weekday_date_string_no_year', 0x54b550, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('as datas: os textos numericos e curtos das telas',
        'dates: the numeric and short texts of the screens',
        [('date_get_numeric_date_string', 'Date::get_numeric_date_string', '0x54a470', 'Date.cpp', 'thiscall, ret 0xc',
          'escreve a data como dia e mes com dois digitos e ano com quatro ("10.02.1972"), com AM, PM ou EVE quando pedida a hora (0 a 2); buffer nulo ou menor que 9: a caixa de erro e 0',
          'writes the date as a two-digit day and month and a four-digit year ("10.02.1972"), with AM, PM or EVE when the time (0 to 2) is asked; a null buffer or one below 9: the error box and 0'),
         ('date_get_short_date_string', 'Date::get_short_date_string', '0x54a650', 'Date.cpp', 'thiscall, ret 0xc',
          'escreve a data como dia, mes e ano com dois digitos ("10.2.72"), na lingua 10 ano primeiro ("72-02-10"), com AM, PM ou EVE quando pedida a hora; buffer nulo ou menor que 9: a caixa de erro e 0',
          'writes the date as day, month and two-digit year ("10.2.72"), in language 10 year first ("72-02-10"), with AM, PM or EVE when the time is asked; a null buffer or one below 9: the error box and 0'),
         ('date_get_weekday_numeric_date_string', 'Date::get_weekday_numeric_date_string', '0x54a950', 'Date.cpp', 'thiscall, ret 0xc',
          'escreve o nome do dia da semana, uma quebra de linha e a data numerica com o ano em dois digitos, com AM, PM ou EVE quando pedida a hora; buffer nulo ou menor que 0x12: a caixa de erro e 0',
          'writes the weekday name, a line break and the numeric date with a two-digit year, with AM, PM or EVE when the time is asked; a null buffer or one below 0x12: the error box and 0'),
         ('date_get_short_month_date_string', 'Date::get_short_month_date_string', '0x54b2f0', 'Date.cpp', 'thiscall, ret 0xc',
          'escreve a data como dia ordinal, mes curto e ano ("10th Feb 1972"), com AM, PM ou EVE quando pedida a hora; buffer nulo ou menor que 0x12: a caixa de erro e 0',
          'writes the date as ordinal day, short month and year ("10th Feb 1972"), with AM, PM or EVE when the time is asked; a null buffer or one below 0x12: the error box and 0'),
         ('date_get_short_weekday_date_string_no_year', 'Date::get_short_weekday_date_string_no_year', '0x54b550', 'Date.cpp', 'thiscall, ret 0xc',
          'escreve a data como dia da semana curto, dia ordinal e mes curto, sem o ano ("Sat 10th Feb"), com AM, PM ou EVE quando pedida a hora; buffer nulo ou menor que 0x12: a caixa de erro e 0',
          'writes the date as short weekday, ordinal day and short month without the year ("Sat 10th Feb"), with AM, PM or EVE when the time is asked; a null buffer or one below 0x12: the error box and 0')])
