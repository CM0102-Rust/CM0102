# O estado da lesão do jogador (Injury::get_player_injury_status 0x62c400: 2 lesionado, 1 recuperando a forma, 0
# pronto; pela escalação automática 0x6dbc50, pelos titulares 0x6dbb80, pelas buscas dos clubes 0x549100.. e por
# dezenas de telas), a data da decisão sobre as ofertas (CONTRACT_MANAGER::schedule_offer_decision 0x4cd230, pela
# 0x4cc930 já em Rust e pela 0x4cc490) e os candidatos a técnico do ano (Staff_Records::get_manager_of_year_candidates
# 0x86fab0, pelos prêmios da nação 0x768940). A 0x4cd230 e a 0x86fab0 têm SEH só da data local (destrutor vazio
# 0x5a53c0). Chamadas originais: 0x62c680 (a forma do dia), 0x54cc30 e 0x54cab0 (datas, com estáticas); o resto já em
# Rust. Iguais no cm0102-1080.exe. Trocas: das candidatas da família, as de notícia e de texto (0x630c50, 0x7fc150,
# 0x4d6a00...) ficam de fora; a 0x62c080 só é chamada pela fechar_partida_do_dia (SO_NA_COMPARACAO). Simulação: as
# mutações pela carreira até 2002-06-01. Comparadores: o estado pela regra relida depois da original (a forma do dia
# não roda duas vezes); a decisão lado a lado quando a entrada já existe (senão a 0x4cd6d0 acrescenta: só a
# original); os candidatos lado a lado pelo vetor dos clubes.
SAVE = False
ROTINAS = [('estado_da_lesao', 0x62c400, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('marcar_decisao_da_oferta', 0x4cd230, [0x6a, 0xff, 0x68, 0x50, 0x6d, 0x95, 0x00]),
           ('candidatos_a_tecnico_do_ano', 0x86fab0, [0x6a, 0xff, 0x68, 0x08, 0x2e, 0x96, 0x00])]
DOCS = ('o estado da lesão do jogador, a data da decisão sobre as ofertas e os candidatos a técnico do ano',
        'the player injury status, the offer decision date and the manager of the year candidates',
        [('estado_da_lesao', 'Injury::get_player_injury_status', '0x62c400', 'injury.cpp', 'thiscall, ret 4',
          'atualiza a forma do dia (0x62c680) e devolve 2 enquanto a lesão dura, 1 enquanto recupera a forma e 0 pronto, pela diferença da data do jogo para a da lesão',
          'updates the daily fitness (0x62c680) and returns 2 while the injury lasts, 1 while recovering match fitness and 0 when fit, by the gap from the injury date to the game date'),
         ('marcar_decisao_da_oferta', 'CONTRACT_MANAGER::schedule_offer_decision', '0x4cd230', 'contract_manager.cpp', 'thiscall, ret 8',
          'acha ou cria a entrada de decisão de tipo 3 da pessoa e marca o dia e a fatia em que ela responde: pelo prazo da oferta, ou sem oferta pela data de hoje e pelo fim do contrato; no máximo daqui a 14 dias',
          'finds or creates the person\'s type 3 decision entry and sets the day and time slice of the answer: by the offer deadline, or without an offer by today and the contract end; at most 14 days ahead'),
         ('candidatos_a_tecnico_do_ano', 'Staff_Records::get_manager_of_year_candidates', '0x86fab0', 'staff_records.cpp', 'thiscall, ret 8',
          'ordena os clubes e guarda até três cujo técnico está no cargo há 28 semanas e tem jogos no registro',
          'sorts the clubs and keeps up to three whose manager has been in the job 28 weeks and has games in the record')])
CAMPOS = [('Injury +0: índice da lesão atual por pessoa (+0xe do registro de 31 bytes, −1 sem); +4: lesões de 48 bytes (+0x20 data, +0x28 e +0x2a dias de recuperação e de lesão); id e valor; lê (estado_da_lesao)',
           'Injury +0: current injury index per person (+0xe of the 31-byte record, −1 if none); +4: 48-byte injuries (+0x20 date, +0x28 and +0x2a recovery and injury days); id and value; reads (estado_da_lesao)'),
          ('CONTRACT_MANAGER +0xc decisões de 14 bytes (+0 id, +4 data, +0xc fatia, +0xd tipo), +0x14 conta, +0x18 começo, +0x1c contador; valor; lê e grava (marcar_decisao_da_oferta)',
           'CONTRACT_MANAGER +0xc 14-byte decisions (+0 id, +4 date, +0xc slice, +0xd type), +0x14 count, +0x18 start, +0x1c counter; value; reads and writes (marcar_decisao_da_oferta)'),
          ('contrato (80 bytes em CONTRACT_MANAGER +0) +0x2d: data do fim; valor; lê (marcar_decisao_da_oferta)',
           'contract (80 bytes at CONTRACT_MANAGER +0) +0x2d: end date; value; reads (marcar_decisao_da_oferta)'),
          ('oferta +0x2f: marca de prazo curto; valor; lê (marcar_decisao_da_oferta)',
           'offer +0x2f: short deadline flag; value; reads (marcar_decisao_da_oferta)'),
          ('[0xae2c88] fatia do dia; valor; lê (marcar_decisao_da_oferta)',
           '[0xae2c88] time slice of the day; value; reads (marcar_decisao_da_oferta)'),
          ('clube +0xcf: técnico; ponteiro; lê; técnico +0x3e: data de chegada; valor; lê; registro do técnico +0x8e: jogos; valor; lê (candidatos_a_tecnico_do_ano)',
           'club +0xcf: manager; pointer; reads; manager +0x3e: arrival date; value; reads; manager record +0x8e: games; value; reads (candidatos_a_tecnico_do_ano)')]
INTERESSA = ['CMRTE', 'CMScout']
