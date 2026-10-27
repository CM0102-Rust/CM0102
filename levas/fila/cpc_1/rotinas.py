# Competições (bloco novo): a próxima partida pelo filtro (fix_man.cpp 0x5ac920: sorteios de copa, Cup.cpp 0x519d90,
# 0x51a150, 0x522c70; 259 chamadas na carreira), o estádio em uso no dia (fix_man.cpp 0x5ae870: marcação dos
# amistosos 0x5c6f00/0x5c75a0 e 0x7e6380/0x7e6720; 2.203) e a data da entrada de data internacional (friendly.cpp
# 0x5cbdd0: 0x5cbb00; 220). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real
# comparando. Sem SEH, sem caixa no caminho (as da 0x5abf10 e da 0x54a130 são de falta de memória e de data
# inválida), sem sorteio. A 0x5ac920 passa à 0x5abf10 ano, dia e fatia com a metade alta de locais nunca escritas,
# mas a 0x5abf10 e o que ela chama (0x5aa2d0, 0x5a9cf0, 0x54cfe0) só usam a palavra/o byte: a cópia passa zero. A
# 0x5cbdd0 monta os argumentos da 0x54a130 sobre o eax de entrada (= o ano no único chamador): a cópia reproduz a
# metade alta pelo ano e o comparador chama a original com eax = ano.
SAVE = False
ROTINAS = [('proxima_partida', 0x5ac920, [0x83, 0xec, 0x14, 0x8b, 0x44, 0x24, 0x18]),
           ('estadio_em_uso', 0x5ae870, [0x83, 0xec, 0x14, 0x8b, 0x44, 0x24, 0x18]),
           ('data_do_amistoso_internacional', 0x5cbdd0, [0x66, 0x0f, 0xbe, 0x01, 0x8b, 0x54, 0x24, 0x04])]
DOCS = ('competições: a próxima partida pelo filtro, o estádio em uso no dia e a data do amistoso internacional',
        'competitions: the next fixture by filter, the stadium in use on a day and the international friendly date',
        [('proxima_partida', 'FIXTURE_MANAGER::find_next_fixture', '0x5ac920', 'fix_man.cpp', 'thiscall, ret 0x2c',
          'a partida seguinte (ou anterior) ao cursor ano/dia/fatia, ele incluído, pelo filtro de competição (+0), fase (+0x34) e dois clubes (+0xc/+0x10), cada um -1 = qualquer; até 5 anos; grava o cursor e devolve a partida, ou 0',
          'the next (or previous) fixture from the year/day/slice cursor, inclusive, by the competition (+0), stage (+0x34) and two club (+0xc/+0x10) filters, each -1 = any; up to 5 years; writes the cursor back and returns the fixture, or 0'),
         ('estadio_em_uso', 'FIXTURE_MANAGER::is_stadium_in_use', '0x5ae870', 'fix_man.cpp', 'thiscall, ret 0x10',
          '1 se alguma partida (+0x18) das listas do dia (grupos 0 a 7 e o do estádio, 0x48df40) usa o estádio, na fatia dada ou nas 0 a 2 (fatia -1)',
          '1 when a fixture (+0x18) in the day lists (groups 0 to 7 and the stadium one, 0x48df40) uses the stadium, in the given slice or in slices 0 to 2 (slice -1)'),
         ('data_do_amistoso_internacional', 'friendly_international_date_resolve', '0x5cbdd0', 'friendly.cpp', 'thiscall, ret 4',
          'a data (+3) pela 0x54a130 com o dia (+1), o mês (+2), o ano dado e o dia da semana (+0); a metade alta dos argumentos vem do ano (o eax do chamador)',
          'the date (+3) through 0x54a130 from the day (+1), month (+2), given year and weekday (+0); the high half of the arguments comes from the year (the caller eax)')])
CAMPOS = [('partida: +0 competição, +0xc/+0x10 clubes, +0x18 estádio (ponteiros), +0x34 fase (curto); lê (proxima_partida, estadio_em_uso)',
           'fixture: +0 competition, +0xc/+0x10 clubs, +0x18 stadium (pointers), +0x34 stage (short); reads (proxima_partida, estadio_em_uso)'),
          ('lista de partidas: +2 começo, +0xa cursor; grava o cursor (proxima_partida, estadio_em_uso)',
           'fixture list: +2 head, +0xa cursor; writes the cursor (proxima_partida, estadio_em_uso)'),
          ('data internacional: +0 dia da semana, +1 dia, +2 mês (bytes), +3 data; grava a data (data_do_amistoso_internacional)',
           'international date: +0 weekday, +1 day, +2 month (bytes), +3 date; writes the date (data_do_amistoso_internacional)')]
INTERESSA = ['SGE', 'CMRTE']
