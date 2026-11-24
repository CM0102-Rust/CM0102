# Mutacoes (sx21_23). Com a Argentina carregada: a oferta muda quem os clubes argentinos contratam na segunda janela
# (carreira e comparador).
MUTACOES = [
    ('sx21_23_tres',
     'read_u8(season_loans_entry(Club(club))) as i8 as i32 + gets as i32 > 2 { // sx21_23: two signings in the second window',
     'read_u8(season_loans_entry(Club(club))) as i8 as i32 + gets as i32 > 3 { // mutation: three',
     'sx21_23 argentina offer over two signings'),
]
