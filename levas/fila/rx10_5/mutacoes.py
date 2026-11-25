# Mutacoes (rx10_5), com a Alemanha e a base NORMAL ate 2002-08-01. Nenhuma abre caixa.
# - a 2a rodada da Norte uma semana antes (30/jul): os jogos da Norte mudam de data (a carreira; o comparador do
#   calendario acusa);
# - a 2a rodada da Sul uma semana antes: o mesmo na Sul;
# - a lista da fase Sul na ordem inversa: os confrontos da Sul mudam (a carreira);
# - a parte Norte da lista de clubes na ordem inversa: o comparador da lista acusa (lado a lado).
MUTACOES = [
    ('rx10_5_norte_semana_antes',
     '    (6, 7, 0, 6, 1, false), // 1: 6 Aug',
     '    (30, 6, 0, 6, 1, false), // mutation',
     'rx10_5 regionalliga north calendar'),
    ('rx10_5_sul_semana_antes',
     '    (6, 7, 0, 6, 1), // 1: 6 Aug',
     '    (30, 6, 0, 6, 1), // mutation',
     'rx10_5 regionalliga south calendar'),
    ('rx10_5_sul_invertida',
     '                write_usize(list + 4 * n as usize, club.addr()); // rx10_4: the next club of the section',
     '                write_usize(list + 4 * (p.count - 1 - n) as usize, club.addr()); // mutation',
     'rx10_5 regionalliga south stage'),
    ('rx10_5_lista_invertida',
     '        write_usize(a + 4 * i as usize, read_usize(l.table() + i as usize * league_table_row::SIZE));',
     '        write_usize(a + 4 * (l.club_count() - 1 - i) as usize, read_usize(l.table() + i as usize * league_table_row::SIZE)); // mutation',
     'rx10_5 regionalliga clubs'),
]
