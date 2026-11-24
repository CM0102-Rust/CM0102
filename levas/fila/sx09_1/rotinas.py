# Date.cpp: acertar uma data, voltar semanas inteiras e a diferenca entre duas datas (leitor sx09, rodada 5,
# 23/11/2026). Item de base do grupo sx09: define o tipo GameDate (o crate::date::Data), as tabelas de dias antes de
# cada mes, o mes de uma data (date_month_index), a estatica local das rotinas de data e o escritor de nomes; os itens
# sx09_2..sx09_7 dependem dele (integrar junto ou depois).
# Convencoes conferidas na desmontagem:
# - Date::set (0x54a130): thiscall(data, dia i16, mes byte, ano u16, dia da semana u16) ret 0x10; erros nas linhas 0x72 e
#   0x79 (o construtor 0x549f70 e igual, com 0x34 e 0x3b); o ajuste da semana e 0x54a2d0 thiscall(data, alvo, destino =
#   a propria data) ret 8. O eax de volta e o argumento (0xffff) ou o do ajuste: nenhum chamador o le.
# - Date::sub_weeks (0x54cab0): thiscall(data, saida, semanas byte) ret 8; estatica 0xae24c8, guarda 0xae24aa (bit 0),
#   destrutor vazio 0x54cc20 no atexit (0x94627a).
# - Date::get_difference (0x54cc30): thiscall(data, saida, outra data por valor, 8 bytes) ret 0xc; estatica 0xae2428,
#   guarda 0xae249d, destrutor 0x54cda0; chama a si mesma (pelo endereco, ou seja, pelo desvio) com as datas trocadas.
SAVE = False
ROTINAS = [('date_set', 0x54a130, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('date_sub_weeks', 0x54cab0, [0x51, 0x56, 0x8b, 0xf1, 0x8a, 0x0d, 0xaa, 0x24, 0xae, 0x00]),
           ('date_get_difference', 0x54cc30, [0x6a, 0xff, 0x68, 0x58, 0x7a, 0x95, 0x00])]
DOCS = ('as datas: acertar uma data, voltar semanas inteiras e a diferenca entre duas datas',
        'dates: setting a date, going back whole weeks and the gap between two dates',
        [('date_set', 'Date::set', '0x54a130', 'Date.cpp', 'thiscall, ret 0x10',
          'acerta a data pelo dia do mes, o mes (0 a 11) e o ano: dia fora de 1..31 da a caixa de erro e vira 1; mes fora de 0..11 da a caixa e vira janeiro; grava o ano, a marca de ano bissexto (regra gregoriana) e o dia do ano (os dias antes do mes mais o dia); com um dia da semana diferente de 0xffff, leva a data ao mais proximo dele (tres dias para tras ou para a frente)',
          'sets the date from the day of the month, the month (0 to 11) and the year: a day outside 1..31 gives the error box and becomes 1; a month outside 0..11 gives the box and becomes January; stores the year, the leap-year flag (Gregorian rule) and the day of the year (the days before the month plus the day); with a weekday other than 0xffff, moves the date to the nearest one (three days back or forward)'),
         ('date_sub_weeks', 'Date::sub_weeks', '0x54cab0', 'Date.cpp', 'thiscall, ret 8',
          'devolve a data semanas inteiras antes (7 dias por semana, a contagem num byte): enquanto o dia do ano fica negativo, volta um ano e soma o tamanho dele (366 no bissexto, senao 365); o resultado fica na estatica da rotina e e copiado para a saida',
          "returns the date whole weeks earlier (7 days a week, the count in a byte): while the day of the year is negative, goes back a year and adds its length (366 in a leap year, else 365); the result stays in the routine's static and is copied to the output"),
         ('date_get_difference', 'Date::get_difference', '0x54cc30', 'Date.cpp', 'thiscall, ret 0xc',
          'escreve a distancia entre duas datas em dias e anos inteiros: se a outra e posterior, as duas trocam de lugar; com o dia do ano desta nao antes do da outra, os dias sao a diferenca dos dias e os anos a dos anos; senao os dias ganham o tamanho do ano desta (366 ou 365) e os anos perdem um; a marca de bissexto do resultado fica a da estatica (1, nunca acertada)',
          "writes the gap between two dates in days and whole years: if the other is later, the two swap places; with this date's day of the year not before the other's, the days are the difference of the days and the years that of the years; otherwise the days gain the length of this date's year (366 or 365) and the years lose one; the result's leap flag stays the static's (1, never set)")])
