# Mutacoes (cx05_19), carreira de 8 ligas ate 2003-08-15 (a virada do Mundial de Clubes de 2003).
# - as candidatas precisam de padrao 15 e nao 13: as sedes sorteadas mudam;
# - a sede sorteada e sempre a primeira candidata: as sedes mudam.
MUTACOES = [
    ('cx05_19_padrao_15',
     '        if read_i8(nation.addr() + nation::LEAGUE_STANDARD) >= 0xd // cx05_19: the strong leagues',
     '        if read_i8(nation.addr() + nation::LEAGUE_STANDARD) >= 0xf // mutation'),
    ('cx05_19_sempre_a_primeira',
     '            write_u32(e + host_entry::HOST, read_u32(candidates[r])); // cx05_19: a random strong nation',
     '            write_u32(e + host_entry::HOST, read_u32(candidates[0])); let _ = r; // mutation'),
]
