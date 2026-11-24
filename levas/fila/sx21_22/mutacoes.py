# Mutacoes (sx21_22). Com a Argentina carregada: a contagem e a regra mudam quem os clubes argentinos contratam na
# segunda janela, de 14 de dezembro a 4 de marco (carreira e comparador).
MUTACOES = [
    ('sx21_22_sem_zerar',
     'reset_window_signings(this, date, 1); // sx21_22: Argentina clears one counter',
     'reset_window_signings(this, date, 0); // mutation: the count is kept',
     'sx21_22 argentina daily'),
    ('sx21_22_conta_dois',
     'write_u8(entry, read_u8(entry).wrapping_add(1)); // sx21_22: one more signing',
     'write_u8(entry, read_u8(entry).wrapping_add(2)); // mutation: counts two',
     'sx21_22 argentina signing counted'),
    ('sx21_22_tres',
     'read_u8(season_loans_entry(Club(club))) as i8 >= 2 { // sx21_22: two signings in the second window',
     'read_u8(season_loans_entry(Club(club))) as i8 >= 3 { // mutation: three',
     'sx21_22 argentina third signing refused'),
]
