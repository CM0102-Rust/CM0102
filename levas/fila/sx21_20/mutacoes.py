# Mutacoes (sx21_20). Com a Franca carregada: as contagens e a regra mudam quem os clubes franceses contratam de
# setembro a dezembro (carreira e comparador).
MUTACOES = [
    ('sx21_20_zero_so_compras',
     'reset_window_signings(this, date, 2); // sx21_20: France clears both counters',
     'reset_window_signings(this, date, 1); // mutation: the free signings count is kept',
     'sx21_20 france daily'),
    ('sx21_20_livres_nas_compras',
     'let entry = season_loans_entry(club) + if offer.pre_contract() != 0 { 1 } else { 0 }; // sx21_20: free transfers in byte 1',
     'let entry = season_loans_entry(club); // mutation: all in byte 0',
     'sx21_20 france signing counted'),
    ('sx21_20_duas_de_cada',
     'if read_u8(entry + byte) as i8 >= 1 { // sx21_20: one of each in the restricted window',
     'if read_u8(entry + byte) as i8 >= 2 { // mutation: two of each',
     'sx21_20 france second signing refused'),
]
