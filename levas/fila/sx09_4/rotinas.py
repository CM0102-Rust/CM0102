# Date.cpp: o nome da hora do dia e a gramatica das palavras da data (leitor sx09, rodada 5, 23/11/2026). Depende do
# sx09_1 (write_date_name): integrar junto ou depois.
# Convencoes conferidas na desmontagem (todas cdecl):
# - date_get_time_of_day_name (0x54d040): (hora byte, maiuscula dword) -> 0xae2484; "Morning"/"Afternoon"/"Evening" ou
#   em minusculas pela langlib; fora de 0..2, strcpy da cadeia vazia.
# - date_get_time_of_day_gender (0x54d130): (hora byte) -> al; le a lingua (langlib_get_language 0x66f4d0) ate quatro
#   vezes. O prologo comeca com call rel32 (call 0x66f4d0): entra em RELATIVAS (rust/src/detours.rs) com (0x54d130, 0).
# - date_get_weekday_gender (0x54d550): (lingua dword, dia i16) -> al.
SAVE = False
ROTINAS = [('date_get_time_of_day_name', 0x54d040, [0x8b, 0x44, 0x24, 0x04, 0x25, 0xff, 0x00, 0x00, 0x00]),
           ('date_get_time_of_day_gender', 0x54d130, [0xe8, 0x9b, 0x23, 0x12, 0x00]),
           ('date_get_weekday_gender', 0x54d550, [0x8b, 0x44, 0x24, 0x04, 0x83, 0xf8, 0x09])]
DOCS = ('as datas: o nome da hora do dia e a gramatica das palavras da data',
        'dates: the time of day name and the grammar of the date words',
        [('date_get_time_of_day_name', 'date_get_time_of_day_name', '0x54d040', 'Date.cpp', 'cdecl',
          'escreve no buffer compartilhado "Morning", "Afternoon" ou "Evening" (0 manha, 1 tarde, 2 noite; com maiuscula ou em minusculas, traduzido) e o devolve; outro valor deixa a cadeia vazia',
          'writes "Morning", "Afternoon" or "Evening" (0 morning, 1 afternoon, 2 evening; capitalised or in lower case, translated) to the shared buffer and returns it; any other value leaves the empty string'),
         ('date_get_time_of_day_gender', 'date_get_time_of_day_gender', '0x54d130', 'Date.cpp', 'cdecl',
          'devolve o codigo gramatical da palavra da hora do dia na lingua atual (para escolher o artigo): linguas 9 e 3, 1; 5, 2; 4, 1 para manha e tarde e 3 para a noite; nas outras, 0',
          'returns the grammar code of the time of day word in the current language (to pick the article): languages 9 and 3, 1; 5, 2; 4, 1 for morning and afternoon and 3 for the evening; in the others, 0'),
         ('date_get_weekday_gender', 'date_get_weekday_gender', '0x54d550', 'Date.cpp', 'cdecl',
          'devolve a marca gramatical do nome do dia da semana numa lingua: na 9, 0 para sabado e domingo e 1 para os outros dias; nas outras, 1 so na lingua 3',
          'returns the grammar flag of a weekday name in a language: in 9, 0 for Saturday and Sunday and 1 for the other days; in the others, 1 only in language 3')])
