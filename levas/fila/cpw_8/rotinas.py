# O fisico do dia do jogador (Injury::update_player_fitness 0x62c680, pela condicao de jogo 0x62c510, pelo estado da
# lesao 0x62c400, pelo dia das lesoes 0x62b7b0 e pelas noticias da volta 0x62e010: toda pessoa que joga, uma vez por dia
# em que alguem pergunta: a forma +8 e a condicao de jogo +0xa do registro de 31 bytes, que a escalacao e o motor das
# partidas leem), a faixa de confianca da diretoria na liga (MANAGER_MANAGER::get_league_confidence_band 0x690bd0, pelo
# texto da confianca 0x690d80, tela) e as notas da diretoria (MANAGER_MANAGER::set_board_ratings 0x69f7c0, pela
# configuracao inicial dos jogadores 0x7a46a6). Chamadas originais pelo endereco: 0x54cc30 (diferenca de datas),
# 0x62e700 (o fim da lesao, cpv_10), 0x669bd0 (clube na temporada, cpu), 0x94627a (atexit) e _ftol (0x945640); o resto
# ja em Rust. Iguais no cm0102-1080.exe. Familias: injury.cpp e manager_manager.cpp (a de tecnicos so tem rotinas
# grandes, de noticia ou de tela; as duas pequenas vao de passageiras). Simulacao: as mutacoes pela carreira ate
# 2002-06-01, todas no fisico do dia. Comparadores: o fisico lado a lado (registro da pessoa, byte +0xa do registro de
# 79 bytes, estaticas 0xb63d08..0xb63d34 e sorteio), com a copia em ensaio sem o fim da lesao (quando chega la, so a
# original); a faixa pela devolucao; as notas lado a lado no registro de 0x49 bytes.
SAVE = False
ROTINAS = [('fisico_do_dia', 0x62c680, [0x6a, 0xff, 0x68, 0xf8, 0xb0, 0x95, 0x00]),
           ('faixa_da_confianca_na_liga', 0x690bd0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x09]),
           ('gravar_notas_da_diretoria', 0x69f7c0, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('o físico do dia do jogador: a forma e a condição de jogo até hoje; a faixa de confiança da diretoria na liga e as notas da diretoria',
        "the player's daily fitness: fitness and match condition up to today; the board's league confidence band and the board ratings",
        [('fisico_do_dia', 'Injury::update_player_fitness', '0x62c680', 'injury.cpp', 'thiscall, ret 4',
          'uma vez por dia: os dias desde a última atualização viram dias livres, em recuperação, parados (lesão, com o fim dela pela 0x62e700) ou de férias (clube fora da temporada, 0x669bd0); a forma (+8) recupera 1000 − 12 × (+0x1e − natural) por dia livre, perde 66 por dia parado e 55 por dia de férias, com teto pela forma natural e trechos encolhidos acima de 7000; a condição de jogo (+0xa) sobe pelos dias livres e em recuperação; grava a data de hoje',
          'once a day: the days since the last update become free days, recovery days, stopped days (injury, ending it through 0x62e700) or holiday days (club out of season, 0x669bd0); fitness (+8) recovers 1000 − 12 × (+0x1e − natural) per free day, loses 66 per stopped day and 55 per holiday day, capped by natural fitness and with shrunk bands above 7000; match condition (+0xa) rises with free and recovery days; stores today\'s date'),
         ('faixa_da_confianca_na_liga', 'MANAGER_MANAGER::get_league_confidence_band', '0x690bd0', 'manager_manager.cpp', 'thiscall, ret 4',
          'a confiança da diretoria na liga (+0x12 do registro de 0x49 bytes do clube) em faixas 0 a 6 (−2000, −1000, −500, 500, 1000, 2000)',
          "the board's league confidence (+0x12 of the club's 0x49-byte record) in bands 0 to 6 (−2000, −1000, −500, 500, 1000, 2000)"),
         ('gravar_notas_da_diretoria', 'MANAGER_MANAGER::set_board_ratings', '0x69f7c0', 'manager_manager.cpp', 'thiscall, ret 8',
          'as quatro notas da diretoria (+6, +8, +0xc, +0xa do registro de 0x49 bytes do clube) recebem o valor',
          "the four board ratings (+6, +8, +0xc, +0xa of the club's 0x49-byte record) get the value")])
CAMPOS = [('Injury +0: registro de 31 bytes por pessoa: +0 data da última atualização (8 bytes); +8 forma; +0xa condição de jogo; +0xc cansaço; +0xe índice da lesão atual (−1 nenhuma); +0x1e propensão; valor e índice; lê e grava (fisico_do_dia)',
           'Injury +0: 31-byte record per person: +0 last update date (8 bytes); +8 fitness; +0xa match condition; +0xc fatigue; +0xe current injury index (−1 none); +0x1e proneness; value and index; reads and writes (fisico_do_dia)'),
          ('Injury +4: tabela de lesões de 0x30 bytes (+0x20 começo, +0x28 fim, +0x2a dias parados); +8 conta; valor; lê (fisico_do_dia)',
           'Injury +4: 0x30-byte injury table (+0x20 start, +0x28 end, +0x2a stopped days); +8 count; value; reads (fisico_do_dia)'),
          ('[0xae2d0c] registro de 79 bytes da pessoa +0xa: dias de férias (até 50); valor; lê e grava (fisico_do_dia)',
           '[0xae2d0c] 79-byte person record +0xa: holiday days (at most 50); value; reads and writes (fisico_do_dia)'),
          ('pessoa +0x24 seleção, +0x39 clube, +0x5b, +0x5e bit 3; ficha +0x34 forma natural; ponteiro e valor; lê (fisico_do_dia)',
           'person +0x24 national team, +0x39 club, +0x5b, +0x5e bit 3; player data +0x34 natural fitness; pointer and value; reads (fisico_do_dia)'),
          ('[0xb63d08..0xb63d34] estáticas do físico: datas, guarda 0xb63d10, clube guardado 0xb63d2c e fora da temporada 0xb63d30; valor; lê e grava (fisico_do_dia)',
           '[0xb63d08..0xb63d34] fitness statics: dates, guard 0xb63d10, cached club 0xb63d2c and out-of-season flag 0xb63d30; value; reads and writes (fisico_do_dia)'),
          ('MANAGER_MANAGER +0: registro de 0x49 bytes por clube: +6, +8, +0xa, +0xc notas da diretoria; +0x12 confiança na liga; valor; lê (faixa_da_confianca_na_liga) e grava (gravar_notas_da_diretoria)',
           'MANAGER_MANAGER +0: 0x49-byte record per club: +6, +8, +0xa, +0xc board ratings; +0x12 league confidence; value; reads (faixa_da_confianca_na_liga) and writes (gravar_notas_da_diretoria)')]
INTERESSA = ['CMRTE']
