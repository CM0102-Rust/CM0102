# A diretoria e o técnico (manager_manager.cpp, staff_records.cpp): a diretoria do técnico novo
# (MANAGER_MANAGER::reset_board_for_new_manager 0x694c80, pela daily_update 0x6905ec e pela contratação 0x69f66f: a
# reputação do técnico, a confiança e os humores da diretoria com sorteios, as marcas zeradas, o registro do técnico),
# o motivo da saída do técnico (MANAGER_MANAGER::get_departure_reason 0x69b350, pela consider_sacking_manager 0x69b017
# e 0x69b30a: o motivo vai para a saída pendente) e o registro do técnico (Staff_Records::get_manager_record 0x86f5b0,
# 20 chamadores: as partidas 0x868864/0x86887d, a mudança de cargo, os prêmios). Sem SEH, sem leitura de lixo (as
# saídas da 0x690c30 e da 0x4ae300 são gravadas por elas). Iguais no cm0102-1080.exe. x87 em dupla precisão com os
# arredondamentos para float da original (0x694c80: a base e as somas com os sorteios guardadas em float; o double de
# 0x96d1b0 é um denormal e a base vai a 10000 quase sempre). Comparadores: a 0x694c80 só a original (as expectativas
# 0x6a3770 postam notícia e o registro 0x869aa0 grava o histórico em arquivo); a 0x69b350 lado a lado com o estado do
# sorteador (a cópia sem contar); a 0x86f5b0 lado a lado pelo registro (e pelo que a original zera quando a vaga é de
# outro). Dúvida: a 0x694c80 chama a 0x6a3770, que monta notícia na pilha (ver a lição das notícias): a pilha abaixo da
# cópia Rust é outra.
SAVE = False
ROTINAS = [('diretoria_do_tecnico_novo', 0x694c80, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('motivo_da_saida_do_tecnico', 0x69b350, [0x83, 0xec, 0x14, 0x53, 0x55]),
           ('registro_do_tecnico', 0x86f5b0, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00])]
DOCS = ('a diretoria e o técnico: a diretoria do técnico novo, o motivo da saída do técnico e o registro do técnico',
        'the board and the manager: the board for a new manager, the manager\'s departure reason and the manager record',
        [('diretoria_do_tecnico_novo', 'MANAGER_MANAGER::reset_board_for_new_manager', '0x694c80', 'manager_manager.cpp', 'thiscall, ret 4',
          'recalcula as expectativas, aproxima a reputação do técnico da do clube e sorteia a confiança e os humores da diretoria a partir da reputação; zera as mudanças e as marcas, registra o técnico e guarda a faixa de riqueza',
          'recalculates the expectations, moves the manager\'s reputation towards the club\'s and draws the board confidence and moods from the reputation; clears the changes and flags, records the manager and stores the wealth band'),
         ('motivo_da_saida_do_tecnico', 'MANAGER_MANAGER::get_departure_reason', '0x69b350', 'manager_manager.cpp', 'thiscall, ret 4',
          'o motivo (0 a 11) da saída do técnico: jogadores insatisfeitos, posição na liga, dívida, mudanças e humores da diretoria (com sorteios) e o tempo no cargo',
          'the reason (0 to 11) for the manager\'s departure: unsettled players, league position, debt, board changes and moods (with draws) and time in the job'),
         ('registro_do_tecnico', 'Staff_Records::get_manager_record', '0x86f5b0', 'staff_records.cpp', 'thiscall, ret 8',
          'acha (ou cria, ou toma de outro) o registro de 210 bytes do técnico e atualiza as nações, a passagem atual e as passagens mais longa e mais curta',
          'finds (or creates, or takes over) the manager\'s 210-byte record and updates the nations, the current spell and the longest and shortest spells')])
CAMPOS = [('MANAGER_MANAGER [this] linha da diretoria (0x49 bytes por clube) +1, +2, +3, +4, +5, +6, +8, +0xa, +0xc, +0x10..+0x16, +0x28, +0x30, +0x38, +0x40; valores; grava (diretoria_do_tecnico_novo), lê +2, +6..+0x16, +0x30, +0x38 (motivo_da_saida_do_tecnico)',
           'MANAGER_MANAGER [this] board row (0x49 bytes per club) +1, +2, +3, +4, +5, +6, +8, +0xa, +0xc, +0x10..+0x16, +0x28, +0x30, +0x38, +0x40; values; writes (diretoria_do_tecnico_novo), reads +2, +6..+0x16, +0x30, +0x38 (motivo_da_saida_do_tecnico)'),
          ('ficha de não jogador +8, +0xa, +0xc: as reputações doméstica, atual e de outra nação; valor; lê e grava (diretoria_do_tecnico_novo)',
           'non-player record +8, +0xa, +0xc: the home, current and world reputations; value; reads and writes (diretoria_do_tecnico_novo)'),
          ('Shortlist_Manager +0xdc (9 bytes por clube) +4; valor; grava 2 (diretoria_do_tecnico_novo)',
           'Shortlist_Manager +0xdc (9 bytes per club) +4; value; writes 2 (diretoria_do_tecnico_novo)'),
          ('contrato +0x45 bit 0x10000000: a queixa; valor; lê (motivo_da_saida_do_tecnico)',
           'contract +0x45 bit 0x10000000: the complaint; value; reads (motivo_da_saida_do_tecnico)'),
          ('Staff_Records +8: os registros de técnico (210 bytes: +0x4a..+0x7e as passagens, +0x86 dono, +0xba/+0xbe pontos, +0xc2/+0xc6 nações); ponteiro; lê e grava (registro_do_tecnico)',
           'Staff_Records +8: the manager records (210 bytes: +0x4a..+0x7e the spells, +0x86 owner, +0xba/+0xbe points, +0xc2/+0xc6 nations); pointer; reads and writes (registro_do_tecnico)'),
          ('[0xdd6ec8] a data a partir da qual as passagens contam; valor; lê (registro_do_tecnico)',
           '[0xdd6ec8] the date from which spells count; value; reads (registro_do_tecnico)')]
INTERESSA = ['CMRTE', 'GPF2']
