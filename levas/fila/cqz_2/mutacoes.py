# Mutacoes (cqz_2): a semeadura da Eurocopa de 2004 em fevereiro (muda a data gravada na copa, o banco da memória; a semeadura de 2004 cai
# depois de 2002-06-01); a vitoria da Copa das Confederacoes valendo 2 (muda o objeto da copa, o banco da memória); o sorteio da Copa
# de 2002 em 20/12/2001 em vez de 27/12 (7 dias: a fase de grupos muda de sorteio e de jogos).
MUTACOES = [
    ('cqz2_euro_semeadura_fevereiro',
     'let seeding = 0; // cqz2: the seeding month of the European Championship (January)',
     'let seeding = 1; // mutation: the seeding in February',
     'cqz_2 euro rules'),
    ('cqz2_confed_vitoria_2',
     'let win = 3; // cqz2: the win in the Confederations Cup groups',
     'let win = 2; // mutation: 2 points a win',
     'cqz_2 confederations cup rules'),
    ('cqz2_copa_sorteio_7_dias',
     'let day = 0x1b; // cqz2: the day of the World Cup draw date',
     'let day = 0x14; // mutation: 7 days earlier',
     'cqz_2 world cup rules'),
]
