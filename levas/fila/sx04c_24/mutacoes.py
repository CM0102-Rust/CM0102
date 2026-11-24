# Mutacoes (sx04c_24), para o comparador numa sessao do Lucas (a carreira nao abre a tela dos premios):
# - a semana conta so com 3 ou mais jogos (outra equipe da semana);
# - as reservas que nao entraram tambem contam.
MUTACOES = [
    ('sx04c_24_tres_jogos',
     '        if n > 1 {',
     '        if n > 2 { // mutation',
     'sx04c_24 team of the week: a week'),
    ('sx04c_24_reservas',
     '        if slot > 10 && read_u8(row + match_record_row::MINUTE_ON) == 0xff {',
     '        if slot > 10 && read_u8(row + match_record_row::MINUTE_ON) == 0xfe { // mutation',
     'sx04c_24 team of the week built'),
]
