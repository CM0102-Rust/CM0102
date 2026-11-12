# Mutacoes (cqz_10), na Eurocopa de 2004 montada na criacao: a fase final ja com as 16 inscritas (o objeto muda no banco
# da memoria); Italia e Finlandia trocadas na lista padrao (idem); a semeadura no dia da semeadura de qualquer ano (em
# 10/1/2002 a rotina semeia os potes dois anos antes: muda o objeto).
MUTACOES = [
    ('cqz10_euro_com_16',
     'if (league.year() as i32) - (year as u16 as i16 as i32) < 1 { // cqz10: the finals of this year already have their 16',
     'if (league.year() as i32) - (year as u16 as i16 as i32) < 4 { // mutation: always 16',
     'cqz_10 euro finals'),
    ('cqz10_lista_trocada',
     'const EURO_CHAMP_DEFAULT_FIELD: [usize; 14] = [KEY_NATION_ITALY, KEY_NATION_FINLAND,',
     'const EURO_CHAMP_DEFAULT_FIELD: [usize; 14] = [KEY_NATION_FINLAND, KEY_NATION_ITALY,',
     'cqz_10 euro field'),
    ('cqz10_semeadura_todo_ano',
     'if seeding_day && seeding_year { // cqz10: the seeding day',
     'if seeding_day || seeding_year { // mutation: the seeding day of any year',
     'cqz_10 euro day'),
]
