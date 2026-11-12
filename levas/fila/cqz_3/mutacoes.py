# Mutacoes (cqz_3): a Copa da Asia de 2002 em vez da de 2004 (o sorteio de 5/5/2002 entra na carreira); a Olimpiada de
# 2005 (o ano da copa muda na memoria do jogo, o banco); a Copa das Confederacoes de 2001 em vez da de 2003 (idem).
MUTACOES = [
    ('cqz3_asia_de_2002',
     'four_year_edition(year as i16, 0x7cc); // cqz3: the Asian Cup of the four-year cycle from 1996',
     'four_year_edition(year as i16, 0x7ce); // mutation: the cycle from 1998',
     'cqz_3 asian cup'),
    ('cqz3_olimpiada_de_2005',
     'four_year_edition(year as i16, 0x7cc); // cqz3: the Olympic year',
     'four_year_edition(year as i16, 0x7cd); // mutation: the cycle from 1997',
     'cqz_3 olympics'),
    ('cqz3_confederacoes_de_2001',
     'confed_cup_edition(year as i16); // cqz3: the Confederations Cup in an odd year after 2001',
     'confed_cup_edition(year as i16).wrapping_sub(2); // mutation: two years earlier',
     'cqz_3 confederations cup'),
]
