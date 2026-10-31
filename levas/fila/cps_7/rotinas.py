# Os estádios (stadium.cpp): o dia dos estádios Stadium::daily_update 0x85b820 (todo dia pelo laço do dia do jogo,
# 0x5cdae4, [0xdd7ef4]; e em 0x5ccb3d), reservar um estádio Stadium::add_booking 0x85de20 (pela escolha do estádio
# 0x85c260 e pela busca do estádio neutro 0x85c800) e se ele está livre Stadium::is_stadium_free_on_date 0x85ddb0
# (pela busca 0x85c800). O stadium.cpp fica fora da lista de cobertura (docs/cobertura_simulacao.txt), mas a 0x85b820
# roda todo dia e a busca neutra roda nos jogos da seleção inglesa marcados depois do primeiro dia (Wembley fechado).
# Fora de fechar_partida_do_dia e de eventos_do_pos_jogo. Iguais no cm0102-1080.exe. Sem SEH, sem x87 e sem local lida
# antes de gravada (a data local da 0x85ddb0 é inteira sobrescrita pela 0x417c10). Comparadores: a 0x85b820 lado a lado
# (no dia 0 do ano, que refaz a lista e aloca, só a original), a 0x85de20 lado a lado quando não aloca, a 0x85ddb0 pelo
# valor. A poda das reservas copia, como a original, as k entradas que começam em n − k − 1.
SAVE = False
ROTINAS = [('estadios_do_dia', 0x85b820, [0x83, 0xec, 0x14, 0x53, 0x8b, 0x5c, 0x24, 0x1c]),
           ('reservar_estadio', 0x85de20, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('estadio_livre_na_data', 0x85ddb0, [0x83, 0xec, 0x14, 0x56, 0x8b, 0xf1])]
DOCS = ('os estádios: o dia dos estádios (o estádio nacional fechado e as reservas vencidas), reservar um estádio numa data e se ele está livre',
        'the stadiums: the stadium day (the closed national stadium and the expired bookings), booking a stadium on a date and whether it is free',
        [('estadios_do_dia', 'Stadium::daily_update', '0x85b820', 'stadium.cpp', 'thiscall, ret 4',
          'no dia 0 refaz a lista dos estádios; antes de 2003 fecha o estádio nacional da Inglaterra (id em [0xa835a4], nação e seleção sem estádio) e o reabre em 1/6/2003 com 90000 lugares; a cada 30 dias tira as reservas vencidas e reordena',
          'on day 0 rebuilds the venue list; before 2003 closes the England national stadium (id in [0xa835a4], nation and national team without a stadium) and reopens it on 1/6/2003 with 90000 places; every 30 days removes the expired bookings and re-sorts'),
         ('reservar_estadio', 'Stadium::add_booking', '0x85de20', 'stadium.cpp', 'thiscall, ret 8',
          'acrescenta a reserva (id do estádio e data) à lista, que cresce de 30 em 30, e reordena por data e id',
          'appends the booking (stadium id and date) to the list, which grows 30 at a time, and re-sorts by date and id'),
         ('estadio_livre_na_data', 'Stadium::is_stadium_free_on_date', '0x85ddb0', 'stadium.cpp', 'thiscall, ret 8',
          '1 se não há reserva do estádio na data (busca binária na lista; sem lista, 1)',
          '1 when there is no booking of the stadium on the date (binary search in the list; no list, 1)')])
CAMPOS = [('Stadium [0xdd7ef4] +0 reservas (12 bytes: +0 id do estádio, +4 data), +4 conta, +8 capacidade; ponteiro/valores; lê e grava (estadios_do_dia, reservar_estadio), lê (estadio_livre_na_data)',
           'Stadium [0xdd7ef4] +0 bookings (12 bytes: +0 stadium id, +4 date), +4 count, +8 capacity; pointer/values; reads and writes (estadios_do_dia, reservar_estadio), reads (estadio_livre_na_data)'),
          ('nação +0x80 estádio nacional; ponteiro; lê e zera na Inglaterra [0x9cf2e4] (estadios_do_dia)',
           'nation +0x80 national stadium; pointer; reads and clears for England [0x9cf2e4] (estadios_do_dia)'),
          ('clube +0x69 estádio; ponteiro; zera na seleção principal da Inglaterra (estadios_do_dia)',
           'club +0x69 stadium; pointer; cleared for the England national team (estadios_do_dia)'),
          ('estádio (0x4e bytes em [0xae23b0]) +0 id, +0x3c/+0x40 lugares; valores; lê o id, grava os lugares na reabertura (estadios_do_dia)',
           'stadium (0x4e bytes at [0xae23b0]) +0 id, +0x3c/+0x40 places; values; reads the id, writes the places on reopening (estadios_do_dia)'),
          ('[0xa835a4] id do estádio nacional fechado (−1 = nenhum); id; grava (estadios_do_dia)',
           '[0xa835a4] id of the closed national stadium (−1 = none); id; writes (estadios_do_dia)')]
INTERESSA = ['CMRTE', 'SGE']
