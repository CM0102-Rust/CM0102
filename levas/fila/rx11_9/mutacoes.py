# Mutacoes (rx11_9), pela carreira com 'ENG IRL NIR' ate 2002-06-01 (os premios irlandeses de 1o de maio de 2002 e os
# do mes de setembro a abril; a reputacao ganha com um premio muda o mercado):
# - o jovem do ano de cada divisao sem limite de idade;
# - os premios do mes so a partir de outubro;
# - os premios em 1o de junho (fora da janela).
MUTACOES = [
    ('rx11_9_jovem_sem_idade',
     '(2, [0x17, 0, 0x11, -1, -1, -1, -1]), (3, [-1, 0, 2, -1, -1, -1, -1])] {',
     '(2, [-1, 0, 0x11, -1, -1, -1, -1]), (3, [-1, 0, 2, -1, -1, -1, -1])] { // mutation',
     'rx11_9 irish awards day'),
    ('rx11_9_outubro',
     'AwardSpec::Month([n, i, k(key), rep.0, rep.1, comp, 8, y, 3, y + 1, f, players]);',
     'AwardSpec::Month([n, i, k(key), rep.0, rep.1, comp, 9, y, 3, y + 1, f, players]); // mutation',
     'rx11_9 irish awards built'),
    ('rx11_9_junho',
     'if awards_day_is(today, 1, 4) { // rx11_9: 1 May',
     'if awards_day_is(today, 1, 5) { // mutation: 1 June',
     'rx11_9 irish awards day'),
]
