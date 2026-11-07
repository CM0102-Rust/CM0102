# Mutacoes (cqw_10), nos torneios de selecoes montados no comeco do jogo: a Copa Africana de 2004 em vez da de 2002
# (janeiro e fevereiro de 2002 sem ela); a Copa America de 2003 em vez da de 2001 (julho de 2001 sem ela); a Copa das
# Nacoes da Oceania de 2004 em vez da de 2002 (sem o sorteio de 2002).
MUTACOES = [
    ('cqw10_africana_de_2004',
     '(ano as i16).wrapping_add(1), 0); // cqw10: a Copa Africana do ano seguinte, par',
     '(ano as i16).wrapping_add(2), 0); // mutacao: a de 2004',
     'cqw_10 copa africana'),
    ('cqw10_copa_america_de_2003',
     'cqw10_edicao(ano as i16, 1); // cqw10: a Copa América no ano ímpar',
     'cqw10_edicao((ano as i16).wrapping_add(1), 1); // mutacao: a de 2003',
     'cqw_10 copa america'),
    ('cqw10_oceania_de_2004',
     '.max(0x7d2), 0); // cqw10: a Copa da Oceania de 2002 em diante',
     '.max(0x7d4), 0); // mutacao: de 2004 em diante',
     'cqw_10 copa da oceania'),
]
