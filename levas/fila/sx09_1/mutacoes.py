# Mutacoes (sx09_1). As tres mudam datas que a carreira usa: Date::set acerta datas na carga do banco (Database.cpp),
# nos amistosos (friendly.cpp), na sede (host_country.cpp) e no Setup; Date::sub_weeks e chamada pelos amistosos
# (0x5c2521), pelo contrato (0x4cfa54), pelo banco (0x535d1f) e pela multa (Fine::is_fine_justified, as ultimas oito
# semanas); Date::get_difference pelas lesoes (injury.cpp 0x62c4c8, 0x62e2b9, 0x630bd6) e pelos recordes. O
# comparador de cada rotina acusa as tres na primeira chamada (memoria lado a lado).
# - o dia do ano uma semana adiante (7 dias, para nao cair no mesmo dia da semana);
# - seis dias por semana;
# - a diferenca entre anos sem tirar o ano incompleto.
MUTACOES = [
    ('sx09_dia_do_ano',
     '(*this).day = days_before_month(leap, month as usize).wrapping_add(day); // sx09_1: the day of the year',
     '(*this).day = days_before_month(leap, month as usize).wrapping_add(day + 7); // mutation: a week later',
     'sx09_1 date set'),
    ('sx09_seis_dias',
     'let mut day = (*this).day.wrapping_sub((weeks as u8 as u16 * 7) as i16); // sx09_1: seven days a week',
     'let mut day = (*this).day.wrapping_sub((weeks as u8 as u16 * 6) as i16); // mutation: six days a week',
     'sx09_1 weeks back'),
    ('sx09_ano_incompleto',
     '(date.day.wrapping_sub(other.day).wrapping_add(length), date.year.wrapping_sub(other.year).wrapping_sub(1)) // sx09_1: a year less',
     '(date.day.wrapping_sub(other.day).wrapping_add(length), date.year.wrapping_sub(other.year)) // mutation: the whole year counted',
     'sx09_1 difference across a year'),
]
