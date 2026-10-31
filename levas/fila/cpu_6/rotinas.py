# O clima da partida (weather.cpp): o vento, a chuva e a temperatura do jogo Weather::get_match_weather 0x923010 (na
# preparação da partida 0x6b7f20, em +0x4756 do motor, onde o vento 2/3/4 sorteia nos chutes de longe e em outros
# lances; e no adiamento 0x923630 da cpu_5), a temperatura pela faixa Weather::get_temperature 0x9232c0 (só pela
# 0x923010) e o estado do gramado Weather::get_pitch_condition 0x923970 (na preparação da partida, em +1 do motor: o
# gramado 2 e 0x20 sorteiam nos chutes). Fora de fechar_partida_do_dia e de eventos_do_pos_jogo. Iguais no
# cm0102-1080.exe. Sem SEH; x87 só no × 0,8 da temperatura da noite. A 0x923010 lê o vento e a faixa da temperatura da
# mesma leitura 2 da cidade (como a original; a chuva é a leitura 1). Comparadores: a 0x923010 lado a lado nos 4 bytes da
# saída, a 0x9232c0 e a 0x923970 pelo valor.
SAVE = False
ROTINAS = [('clima_da_partida', 0x923010, [0x51, 0x53, 0x55, 0x8b, 0xe9]),
           ('temperatura_do_jogo', 0x9232c0, [0x0f, 0xbe, 0x4c, 0x24, 0x04]),
           ('gramado_do_jogo', 0x923970, [0x8b, 0x4c, 0x24, 0x08, 0x53])]
DOCS = ('o clima da partida: o vento, a chuva e a temperatura do jogo, a temperatura pela faixa e o estado do gramado',
        'the match weather: the wind, rain and temperature of the match, the temperature by band and the pitch condition',
        [('clima_da_partida', 'Weather::get_match_weather', '0x923010', 'weather.cpp', 'thiscall, ret 8',
          'a cidade do estádio (senão a do mandante, senão a capital); o vento da cidade, a região e o período pelo dia; vento (0..4), chuva (0, 1, 2, 3, 5) e faixa da temperatura pelos limites do período; abaixo de zero, a chuva vira neve/gelo (6..9) pelo vento da cidade',
          'the stadium city (else the home club one, else the capital); the city wind, the region and the period by the day; wind (0..4), rain (0, 1, 2, 3, 5) and temperature band by the period limits; below zero, the rain becomes snow/ice (6..9) by the city wind'),
         ('temperatura_do_jogo', 'Weather::get_temperature', '0x9232c0', 'weather.cpp', 'thiscall, ret 8',
          '(dia × faixa) módulo 6/7/8 mais a base da faixa (−5 a 35; outra faixa −100); na fatia da noite, − 2 abaixo de 10, senão × 0,8',
          '(day × band) modulo 6/7/8 plus the band base (−5 to 35; other band −100); in the evening slot, − 2 below 10, otherwise × 0.8'),
         ('gramado_do_jogo', 'Weather::get_pitch_condition', '0x923970', 'weather.cpp', 'thiscall, ret 8',
          'pela chuva, a temperatura, a marca e os lugares do estádio e um resto do jogo módulo 20: 0, 1, 2, 4, 8, 0x10, 0x20 ou 0x40; estádio coberto, 0',
          'by the rain, the temperature, the stadium flag and places and a fixture remainder modulo 20: 0, 1, 2, 4, 8, 0x10, 0x20 or 0x40; covered stadium, 0')])
CAMPOS = [('Weather [0xde2f74] +0 regiões (0x6e bytes), +4 leituras das cidades (4 bytes: chuva, vento/temperatura, vento da cidade), +8 quantas regiões; lê (clima_da_partida)',
           'Weather [0xde2f74] +0 regions (0x6e bytes), +4 city readings (4 bytes: rain, wind/temperature, city wind), +8 region count; reads (clima_da_partida)'),
          ('região do clima +0x1e/+0x31/+0x44/+0x57 dias dos períodos, período (19 bytes) +0x20..+0x30 limites; valores; lê (clima_da_partida)',
           'weather region +0x1e/+0x31/+0x44/+0x57 period days, period (19 bytes) +0x20..+0x30 limits; values; reads (clima_da_partida)'),
          ('cidade +0 id, +0x1f nação, +0x34 região do clima; id/ponteiro/valor; lê (clima_da_partida)',
           'city +0 id, +0x1f nation, +0x34 weather region; id/pointer/value; reads (clima_da_partida)'),
          ('nação +0x7a capital; ponteiro; lê (clima_da_partida)', 'nation +0x7a capital; pointer; reads (clima_da_partida)'),
          ('jogo +0 id, +0xc/+0x10 valores, +0x18 estádio, +0x1c mandante, +0x2a dia, +0x3f fatia; lê (clima_da_partida, temperatura_do_jogo, gramado_do_jogo)',
           'fixture +0 id, +0xc/+0x10 values, +0x18 stadium, +0x1c home club, +0x2a day, +0x3f slot; reads (clima_da_partida, temperatura_do_jogo, gramado_do_jogo)'),
          ('estádio +0x38 cidade, +0x3c lugares, +0x4c coberto, +0x4d marca; lê (clima_da_partida, gramado_do_jogo)',
           'stadium +0x38 city, +0x3c places, +0x4c covered, +0x4d flag; reads (clima_da_partida, gramado_do_jogo)'),
          ('motor +0x4756 clima do jogo (vento, chuva, temperatura, vento da cidade), +1 gramado; grava pela preparação da partida (clima_da_partida, gramado_do_jogo)',
           'match engine +0x4756 match weather (wind, rain, temperature, city wind), +1 pitch; written through the match setup (clima_da_partida, gramado_do_jogo)')]
INTERESSA = ['CMRTE']
