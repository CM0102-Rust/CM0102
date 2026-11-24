# Date.cpp: o dia do mes, o mes e o ano bissexto (leitor sx09, rodada 5, 23/11/2026). Depende do sx09_1 (GameDate,
# days_before_month, date_month_index): integrar junto ou depois.
# Convencoes conferidas na desmontagem:
# - Date::get_day_of_month (0x54cf70): thiscall, sem argumentos; procura de dezembro para tras; o chamador le o ax (a
#   palavra alta e o resto da conta com o ponteiro da tabela, reproduzida); -1 quando nenhum mes casa.
# - Date::get_month (0x54cfb0): thiscall, sem argumentos; 0..11 ou eax = -1.
# - date_is_leap_year (0x54cfe0): cdecl(ano, so a palavra baixa).
SAVE = False
ROTINAS = [('date_get_day_of_month', 0x54cf70, [0x8b, 0x41, 0x04, 0x56, 0x85, 0xc0]),
           ('date_get_month', 0x54cfb0, [0x8b, 0x41, 0x04, 0xba, 0x60, 0x84, 0x9b, 0x00]),
           ('date_is_leap_year', 0x54cfe0, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xe1, 0xff, 0xff, 0x00, 0x00])]
DOCS = ('as datas: o dia do mes, o mes e o ano bissexto',
        'dates: the day of the month, the month and the leap year',
        [('date_get_day_of_month', 'Date::get_day_of_month', '0x54cf70', 'Date.cpp', 'thiscall',
          'devolve o dia do mes: de dezembro para tras, o primeiro mes cujos dias antes dele (na tabela do ano normal ou bissexto, pela marca da data) sao menos que o dia do ano; o dia do ano menos esses dias; nenhum (dia -1 ou menos): -1',
          "returns the day of the month: from December back, the first month whose days before it (in the ordinary or leap year table, by the date's flag) are fewer than the day of the year; the day of the year minus those days; none (a day of -1 or less): -1"),
         ('date_get_month', 'Date::get_month', '0x54cfb0', 'Date.cpp', 'thiscall',
          'devolve o mes da data, 0 para janeiro a 11 para dezembro (a mesma procura), ou -1',
          'returns the month of the date, 0 for January to 11 for December (the same search), or -1'),
         ('date_is_leap_year', 'date_is_leap_year', '0x54cfe0', 'Date.cpp', 'cdecl',
          'diz se o ano (16 bits) e bissexto pela regra gregoriana: impar ou nao divisivel por 4, nao; nao divisivel por 100, sim; senao, so se divisivel por 400',
          'tells whether the year (16 bits) is a leap year by the Gregorian rule: odd or not divisible by 4, no; not divisible by 100, yes; otherwise only when divisible by 400')])
