# Mutacoes (cx08_9), com a Argentina ate 2002-08-01:
# - o 2o e o 3o trocam de zona (o comparador das zonas acusa; os jogos das zonas mudam);
# - as zonas jogam em turno unico (os jogos mudam: a carreira).
MUTACOES = [
    ('cx08_9_troca_de_zona',
     'const ARG_SECOND_ZONE_ROWS: [[usize; 8]; 3] = [[1, 10, 11, 12, 13, 14, 15, 24], [2, 7, 8, 9, 16, 17, 18, 23],',
     'const ARG_SECOND_ZONE_ROWS: [[usize; 8]; 3] = [[2, 10, 11, 12, 13, 14, 15, 24], [1, 7, 8, 9, 16, 17, 18, 23], // mutation\n   ',
     'cx08_9 argentine zones'),
    ('cx08_9_zonas_turno_unico',
     'ctor(p, this.addr(), 8, list + 0x20 * g as usize, 2, this.competition().addr(), table,',
     'ctor(p, this.addr(), 8, list + 0x20 * g as usize, 1 /* mutation */, this.competition().addr(), table,',
     'cx08_9 argentine zones'),
]
