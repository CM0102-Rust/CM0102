# Mutacoes (cx08_5), com a Argentina ate 2002-08-01:
# - o River Plate parte com 87 pontos na temporada mais velha (o comparador do init acusa na criacao; os promedios mudam);
# - o promedio ignora os jogos da temporada mais velha (o comparador do update acusa no fim do Clausura; o rebaixamento
#   pode mudar);
# - o progresso limita os codigos em 17 (o comparador do progresso acusa).
MUTACOES = [
    ('cx08_5_river_87_pontos',
     '    (0x9d00ac, 86, 38, 78, 38), // River Plate',
     '    (0x9d00ac, 87, 38, 78, 38), // River Plate (mutation)',
     'cx08_5 argentine averages seeded'),
    ('cx08_5_promedio_sem_temporada_velha',
     'let games = read_i8(s + 8) as i32 + read_i8(s + 5) as i32 + read_i8(s + 2) as i32;',
     'let games = read_i8(s + 8) as i32 + read_i8(s + 5) as i32; // mutation',
     'cx08_5 argentine averages updated'),
    ('cx08_5_progresso_17',
     '(c, b, w) = (c.min(0x12), b.min(0x12), w.min(0x12));',
     '(c, b, w) = (c.min(0x11), b.min(0x11), w.min(0x11)); // mutation',
     'cx08_5 argentine progress capped'),
]
