# Mutacoes (sx05b_18), pela carreira com Gales ate 2002-06-01 (os premios de 11 de maio e o tecnico do mes).
# - o jovem do ano sem o limite de idade;
# - o tecnico do mes de setembro a abril (agosto fora);
# - os premios em 11 de abril.
MUTACOES = [
    ('sx05b_18_jovem_sem_idade',
     '(2, 0x15, 0x11)] {',
     '(2, -1, 0x11)] { // mutation',
     'sx05b_18 welsh awards day'),
    ('sx05b_18_setembro',
     '2000, 1000, premier, 7, year, 3, year + 1, f, 0]),',
     '2000, 1000, premier, 8, year, 3, year + 1, f, 0]), // mutation',
     'sx05b_18 welsh awards built'),
    ('sx05b_18_abril',
     '    if awards_day_is(today, 0xb, 4) {',
     '    if awards_day_is(today, 0xb, 3) { // mutation'),
]
