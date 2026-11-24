# Mutacoes (sx09_2). O mes de uma data decide as janelas de transferencia (england_rules, scotland_rules), os
# contratos e a agenda; o ano bissexto, o calendario das competicoes (fix_man.cpp 0x5a9e62, 0x5aa4b7); o dia do mes, os
# contratos e os textos. Os comparadores (retorno lado a lado) acusam as tres na primeira chamada.
MUTACOES = [
    ('sx09_dia_do_mes',
     '(high | d.day as u16 as u32).wrapping_sub(0x009b_0000 | before) // sx09_2: the day of the month',
     '(high | d.day.wrapping_add(1) as u16 as u32).wrapping_sub(0x009b_0000 | before) // mutation: a day later',
     'sx09_2 day of month'),
    ('sx09_mes',
     'Some(month) => month as u32, // sx09_2: the month',
     'Some(month) => (month as u32 + 1) % 12, // mutation: the next month',
     'sx09_2 month'),
    ('sx09_bissexto',
     'crate::date::is_leap_year(year as u16) as u32 // sx09_2: the Gregorian rule',
     '(crate::date::is_leap_year(year as u16) ^ 1) as u32 // mutation: leap years swapped',
     'sx09_2 leap year'),
]
