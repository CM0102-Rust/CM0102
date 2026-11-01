# As estatísticas do jogo e os recordes do clube (player_stats.cpp, club_records.cpp): PlayerStats::record_match
# (0x7c0d70, 19.414 chamadas na cobertura de 17/10: todo jogo da carreira, pela fechar_partida_do_dia 0x6b5f00 e pelos
# jogos simulados 0x84be20; os registros das categorias 2, 3 e 4 são as médias que o motor de partida lê pela get_stat
# 0x7c1e30: jogos +0 e nota +0xe), Club_Records::update_league_position_records (0x447430, 8.260 chamadas: a melhor e
# a pior posição da temporada e de sempre; passageira) e Club_Records::send_record_news (0x448e30, 1.548 chamadas:
# as notícias dos recordes da fila; passageira). A 0x7c0d70 chama só rotinas em Rust (0x7c3920, 0x7c5d80, 0x4b0700,
# 0x4cf5a0, 0x4cf6d0, 0x53b900, 0x5c6710, o sorteio); x87 só em somas de inteiros em double (exatas). A 0x447430
# chama a 0x449590 (original, pura), a fase 0x48d200 (cpw_12) e rotinas em Rust; a 0x448e30, a 0x453120 (original,
# preenche a notícia) e rotinas em Rust. Sem SEH com destrutor (a 0x448e30 tem o quadro da notícia local, sem
# destruir, como na original). Iguais no cm0102-1080.exe.
# Comparadores: a 0x7c0d70 lado a lado (somas em double, registros das categorias de cada linha, +0x20..+0x27 do
# registro do jogador, +0x3b do contrato; no jogo simulado com o estado do sorteio), só a original quando um registro
# seria criado ou com a caixa; a 0x447430 com a original antes e a cópia em ensaio (sem ordenar) nos recordes de cada
# clube e na fila, só a original quando um registro seria criado ou a fila pode crescer; a 0x448e30 com a cópia em
# ensaio antes (sem preencher, entregar nem soltar) e a original depois, no cabeçalho da fila.
SAVE = False
ROTINAS = [('registrar_partida_nas_estatisticas', 0x7c0d70, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('recordes_de_posicao_na_liga', 0x447430, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('noticias_de_recordes', 0x448e30, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as estatísticas do jogo e os recordes do clube: o jogo nas estatísticas dos jogadores, os recordes de posição na liga e as notícias dos recordes',
        "the match statistics and the club records: the match in the players' statistics, the league position records and the record news items",
        [('registrar_partida_nas_estatisticas', 'PlayerStats::record_match', '0x7c0d70', 'player_stats.cpp', 'thiscall, ret 8',
          'pela categoria de cada lado (amistoso, reservas, tipo da competição; sorteio no jogo simulado), soma a nota e o jogo nas somas em double e cada linha que jogou nos registros das categorias (jogos, gols, placar, nota, minutos...), nos jogos e gols do jogador e no +0x3b do contrato; anda a forma meio byte',
          "by each side's category (friendly, reserves, competition type; a draw in a simulated match), adds the rating and the match to the double sums and each player who played to the category records (games, goals, score, rating, minutes...), to the player's games and goals and to contract +0x3b; shifts the form half a byte"),
         ('recordes_de_posicao_na_liga', 'Club_Records::update_league_position_records', '0x447430', 'club_records.cpp', 'thiscall, ret 8',
          'ordena a tabela de cada fase da liga e grava a melhor e a pior posição de cada clube na temporada e, com o segundo argumento, de sempre (com a notícia na fila para clube humano ou seleção)',
          "sorts the table of each league stage and stores each club's best and worst position for the season and, with the second argument, all time (queueing the news item for a human club or national side)"),
         ('noticias_de_recordes', 'Club_Records::send_record_news', '0x448e30', 'club_records.cpp', 'thiscall, ret 0',
          'entrega a notícia de cada recorde da fila (clube humano ou seleção) e esvazia a fila (solta com capacidade de 100 ou mais)',
          'delivers the news item of each queued record (human club or national side) and empties the queue (freed at capacity 100 or more)')])
CAMPOS = [('PlayerStats +0x10..+0x28: as somas das notas e dos jogos (double; +0x20/+0x28 com a marca +0x4c do jogo); valor; grava (registrar_partida_nas_estatisticas)',
           'PlayerStats +0x10..+0x28: the rating and match sums (double; +0x20/+0x28 with match flag +0x4c); value; writes (registrar_partida_nas_estatisticas)'),
          ('registro de categoria (0x20 bytes) +0 jogos, +1 entradas, +7/+8 gols a favor/contra, +9/+0xa vitórias/derrotas, +0xd melhor em campo, +0xe nota, +0x1c minutos; valor; grava (registrar_partida_nas_estatisticas)',
           'category record (0x20 bytes) +0 games, +1 sub appearances, +7/+8 goals for/against, +9/+0xa wins/losses, +0xd man of the match, +0xe rating, +0x1c minutes; value; writes (registrar_partida_nas_estatisticas)'),
          ('registro do jogador (PlayerStats +0x30, 0x27 bytes) +0x20 jogos, +0x22 gols, +0x24..+0x26 a forma (meio byte por jogo); valor; grava (registrar_partida_nas_estatisticas)',
           "player record (PlayerStats +0x30, 0x27 bytes) +0x20 games, +0x22 goals, +0x24..+0x26 the form (half a byte per match); value; writes (registrar_partida_nas_estatisticas)"),
          ('contrato +0x3b: os jogos pelo clube; valor; grava (registrar_partida_nas_estatisticas)',
           'contract +0x3b: the games for the club; value; writes (registrar_partida_nas_estatisticas)'),
          ('linha do jogo (0x49 bytes a partir de +0x190 + 0x625 × lado) +0 pessoa, +4 jogador, +0x21..+0x2d contagens, +0x30/+0x32/+0x34 saída, entrada e minutos, +0x35 nota; valor; lê (registrar_partida_nas_estatisticas)',
           'match line (0x49 bytes from +0x190 + 0x625 x side) +0 staff, +4 player, +0x21..+0x2d counts, +0x30/+0x32/+0x34 off, on and minutes, +0x35 rating; value; reads (registrar_partida_nas_estatisticas)'),
          ('recorde (temporada 0x4528d0 / sempre 0x452710) +0/+1/+5 melhor posição, competição e ano; +7/+8/+0xc a pior; valor e id; lê e grava (recordes_de_posicao_na_liga)',
           'record (season 0x4528d0 / all time 0x452710) +0/+1/+5 best position, competition and year; +7/+8/+0xc the worst; value and id; reads and writes (recordes_de_posicao_na_liga)'),
          ('Club_Records +0x24/+0x28/+0x2c: a fila das notícias de recorde (6 bytes: id, tipo, período), capacidade e conta; ponteiro e valor; lê e grava (recordes_de_posicao_na_liga, noticias_de_recordes)',
           'Club_Records +0x24/+0x28/+0x2c: the record news queue (6 bytes: id, type, period), capacity and count; pointer and value; reads and writes (recordes_de_posicao_na_liga, noticias_de_recordes)'),
          ('competição ([0xae23d0], 0x6b bytes) +0x69: a reputação; valor; lê (recordes_de_posicao_na_liga)',
           'competition ([0xae23d0], 0x6b bytes) +0x69: the reputation; value; reads (recordes_de_posicao_na_liga)')]
INTERESSA = ['CMRTE', 'CMScout']
