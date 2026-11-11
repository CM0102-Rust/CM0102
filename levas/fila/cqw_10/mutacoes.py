# Mutacoes (cqw_10), nos torneios de selecoes montados no comeco do jogo: a Copa Africana de 2004 em vez da de 2002
# (janeiro e fevereiro de 2002 sem ela); a Copa America de 2003 em vez da de 2001 (julho de 2001 sem ela); a Copa das
# Nacoes da Oceania de 2004 em vez da de 2002 (sem o sorteio de 2002).
MUTACOES = [
    ('cqw10_africana_de_2004',
     '(year as i16).wrapping_add(1), 0); // cqw10: the African Nations Cup of the next year, even',
     '(year as i16).wrapping_add(2), 0); // mutation: the 2004 one',
     'cqw_10 african nations cup'),
    ('cqw10_copa_america_de_2003',
     'nations_cup_edition(year as i16, 1); // cqw10: the Copa América in an odd year',
     'nations_cup_edition((year as i16).wrapping_add(1), 1); // mutation: the 2003 one',
     'cqw_10 copa america'),
    ('cqw10_oceania_de_2004',
     '.max(0x7d2), 0); // cqw10: the Oceania Nations Cup from 2002 on',
     '.max(0x7d4), 0); // mutation: from 2004 on',
     'cqw_10 oceania nations cup'),
]
