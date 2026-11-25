# Mutacoes (rx11_7), pela carreira com as 8 ligas ate 2002-08-01 (os premios franceses de 20 de dezembro de 2001 e de
# 6 de junho de 2002; a reputacao ganha com um premio muda o mercado):
# - o jovem (sub-23) do ano sem limite de idade;
# - os premios de dezembro em 20 de novembro (o mesmo dia de outro mes: a lista da epoca muda);
# - os premios de equipe da divisao com reputacao no lugar de 0/0 (o ganho dos escolhidos muda).
MUTACOES = [
    ('rx11_7_sub23_sem_idade',
     'let under_23 = [0x17, 0, 0x11, -1, -1, -1, -1]; // rx11_7: the under 23 player from the First Division',
     'let under_23 = [-1, 0, 0x11, -1, -1, -1, -1]; // mutation: no age limit',
     'rx11_7 french june awards'),
    ('rx11_7_dezembro_em_novembro',
     'if awards_day_is(today, 0x14, 0xb) { // rx11_7: 20 December',
     'if awards_day_is(today, 0x14, 0xa) { // mutation: November',
     'rx11_7 french december awards'),
    ('rx11_7_equipe_com_reputacao',
     "specs.push(AwardSpec::Team([n, i + 1, k(keys[1]), 0, 0, comp, 1, 0, f, 1]));",
     "specs.push(AwardSpec::Team([n, i + 1, k(keys[1]), 6000, 3000, comp, 1, 0, f, 1])); // mutation",
     'rx11_7 french awards built'),
]
