# Mutacoes (cx05_18), carreira ENG ate 2002-08-01 (as sedes das finais europeias em 1 de julho de 2002, pelo dia da
# tabela de sedes; a ordem da tabela em 1 de janeiro de 2002 e na criacao do jogo).
# - o estadio nacional precisa de mais de 50000 lugares: as candidatas mudam (as sedes das finais mudam);
# - as candidatas na ordem inversa: as sedes trocam de final;
# - a ordem da tabela pelo dia antes do ano: a busca das sedes por data acha outras entradas (o comparador lado a lado
#   acusa).
MUTACOES = [
    ('cx05_18_estadio_60000',
     'read_i32(stadium + stadium::CAPACITY) > 40000 // cx05_18: a big enough stadium',
     'read_i32(stadium + stadium::CAPACITY) > 50000 // mutation'),
    ('cx05_18_sede_da_proxima',
     "            write_u32(e + host_entry::HOST, read_u32(candidates[next])); // cx05_18: the nation's id as the host",
     "            write_u32(e + host_entry::HOST, read_u32(candidates[2 - next])); // mutation"),
    ('cx05_18_dia_antes_do_ano',
     '        return ya as i32 - yb as i32; // cx05_18: the year first',
     '        return yb as i32 - ya as i32; // mutation'),
]
