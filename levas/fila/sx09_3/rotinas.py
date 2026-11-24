# Date.cpp: os nomes do dia da semana e do mes (leitor sx09, rodada 5, 23/11/2026). Depende do sx09_1
# (write_date_name): integrar junto ou depois.
# Convencoes conferidas na desmontagem (todas cdecl, devolvem o buffer compartilhado):
# - date_get_weekday_name (0x54d170): o dia como i16 (movsx word), tabela de saltos 0x54d1f4 (0 segunda .. 6 domingo),
#   langlib_sprintf(0xae2434, nome); fora de 0..6, strcpy da cadeia vazia 0xad9c64.
# - date_get_short_month_name (0x54d210): o mes como byte com sinal, tabela 0x54d380, buffer 0xae24a0; "Mar" e "Dec"
#   levam o comentario do tradutor e o argumento 0xad9c64.
# - date_get_month_name (0x54d3b0): tabela 0x54d51c, buffer 0xae2450; "May" leva o comentario.
# A copia passa a cadeia vazia como argumento extra a todos os modelos (so os que tem o comentario a leem): o texto e o
# mesmo.
SAVE = False
ROTINAS = [('date_get_weekday_name', 0x54d170, [0x0f, 0xbf, 0x44, 0x24, 0x04]),
           ('date_get_short_month_name', 0x54d210, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
           ('date_get_month_name', 0x54d3b0, [0x0f, 0xbe, 0x44, 0x24, 0x04])]
DOCS = ('as datas: os nomes do dia da semana e do mes',
        'dates: the weekday and month names',
        [('date_get_weekday_name', 'date_get_weekday_name', '0x54d170', 'Date.cpp', 'cdecl',
          'escreve no buffer compartilhado o nome traduzido do dia da semana (0 segunda a 6 domingo) e o devolve; outro valor deixa a cadeia vazia',
          'writes the translated weekday name (0 Monday to 6 Sunday) to the shared buffer and returns it; any other value leaves the empty string'),
         ('date_get_short_month_name', 'date_get_short_month_name', '0x54d210', 'Date.cpp', 'cdecl',
          'escreve no buffer compartilhado o nome do mes em tres letras (0 janeiro a 11 dezembro, traduzido) e o devolve; outro valor deixa a cadeia vazia',
          'writes the three-letter month name (0 January to 11 December, translated) to the shared buffer and returns it; any other value leaves the empty string'),
         ('date_get_month_name', 'date_get_month_name', '0x54d3b0', 'Date.cpp', 'cdecl',
          'escreve no buffer compartilhado o nome completo do mes (traduzido) e o devolve; outro valor deixa a cadeia vazia',
          'writes the full month name (translated) to the shared buffer and returns it; any other value leaves the empty string')])
