# As notas do mês (month_ratings.cpp): MONTH_RATINGS::record_match_ratings 0x767ee0 (pelo 0x7683a0, a cada partida das
# ligas com prêmio do mês), MONTH_RATINGS::add_player_rating 0x767fa0 (pela 0x767ee0, uma vez por jogador da partida) e
# MONTH_RATINGS::find_best_player 0x768130 (pelo prêmio do mês 0x767150, na virada do mês: o jogador e o jovem do mês de
# cada divisão inglesa). O vencedor ganha reputação (0x412f00), lida pela carreira. Família fora das do brief (finance,
# club, stadium, game e human_manager só têm textos de notícia, carga do save, rotinas de 1,8 a 5,3 KB ou de fim de
# temporada): month_ratings.cpp, que nenhum outro agente usa. Iguais no cm0102-1080.exe; sem SEH; sem x87 (a média fica
# no 0x768200, original, passado ao qsort 0x945333). Comparadores: lado a lado nas três (cabeçalho e registros da
# tabela; a 0x768130 também pelo registro devolvido); só a original quando a tabela precisa crescer (malloc/realloc).
SAVE = False
ROTINAS = [('guardar_notas_da_partida', 0x767ee0, [0x8b, 0x44, 0x24, 0x08, 0x53]),
           ('nota_do_jogador_no_mes', 0x767fa0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('melhor_do_mes', 0x768130, [0x83, 0xec, 0x0c, 0x53, 0x55])]
DOCS = ('as notas do mês: as notas da partida, a nota do jogador no mês e o melhor do mês',
        'the month ratings: the match ratings, the player rating of the month and the best of the month',
        [('guardar_notas_da_partida', 'MONTH_RATINGS::record_match_ratings', '0x767ee0', 'month_ratings.cpp', 'thiscall, ret 8',
          'os dois lados da partida, 20 vagas cada: fora a vaga vazia (0xfb) e a reserva que não entrou, a nota de cada pessoa vai para as notas do mês',
          'both sides of the match, 20 slots each: except the empty slot (0xfb) and the substitute who did not come on, each person rating goes to the month ratings'),
         ('nota_do_jogador_no_mes', 'MONTH_RATINGS::add_player_rating', '0x767fa0', 'month_ratings.cpp', 'thiscall, ret 4',
          'acha ou acrescenta o registro de 12 bytes da pessoa (cresce 300 quando cheia; a liga do clube e a idade), soma a nota e conta um jogo',
          'finds or appends the 12-byte record of the person (grows by 300 when full; the club league and the age), adds the rating and counts one match'),
         ('melhor_do_mes', 'MONTH_RATINGS::find_best_player', '0x768130', 'month_ratings.cpp', 'thiscall, ret 8',
          'ordena pela média e devolve o primeiro da liga com 3 jogos (depois 2, 1, 0), com o filtro de idade e a liga atual do clube',
          'sorts by average and returns the first of the league with 3 matches (then 2, 1, 0), with the age filter and the current club league')])
CAMPOS = [('notas do mês (MONTH_RATINGS) +0 tabela, +4 conta, +8 capacidade, +0xc mês, +0xd ano; ponteiro/valores; lê e grava (nota_do_jogador_no_mes, melhor_do_mes ordena)',
           'month ratings (MONTH_RATINGS) +0 table, +4 count, +8 capacity, +0xc month, +0xd year; pointer/values; reads and writes (nota_do_jogador_no_mes, melhor_do_mes sorts)'),
          ('registro da nota (12 bytes) +0 pessoa, +4 liga, +8 soma das notas, +0xa jogos, +0xb idade; id/valores; grava (nota_do_jogador_no_mes), lê (melhor_do_mes)',
           'rating record (12 bytes) +0 person, +4 league, +8 rating sum, +0xa matches, +0xb age; id/values; writes (nota_do_jogador_no_mes), reads (melhor_do_mes)'),
          ('vaga da partida (0x49 bytes, 20 por lado a partir de +0x190, lados a 0x625) +0 pessoa, +0x1d marca 0xfb, +0x32 entrou (0xff não), +0x35 nota; id/valores; lê (guardar_notas_da_partida, nota_do_jogador_no_mes)',
           'match slot (0x49 bytes, 20 per side from +0x190, sides 0x625 apart) +0 person, +0x1d 0xfb flag, +0x32 came on (0xff no), +0x35 rating; id/values; reads (guardar_notas_da_partida, nota_do_jogador_no_mes)'),
          ('pessoa +0x18 idade, +0x39 clube; clube +0x57 liga; valores/ponteiros; lê (nota_do_jogador_no_mes, melhor_do_mes)',
           'staff +0x18 age, +0x39 club; club +0x57 league; values/pointers; reads (nota_do_jogador_no_mes, melhor_do_mes)'),
          ('[0xae2364] conta de pessoas; valor; lê (guardar_notas_da_partida)',
           '[0xae2364] staff count; value; reads (guardar_notas_da_partida)')]
INTERESSA = ['CMRTE']
