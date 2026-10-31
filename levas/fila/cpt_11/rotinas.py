# A pré-temporada dos amistosos (friendly.cpp): Friendly::update_for_date (0x5c2460, 2064 chamadas na cobertura de
# 17/10: o dia dos amistosos, pela tabela virtual), Friendly::plan_ai_preseason_friendlies (0x5c27a0, 2: os candidatos
# da IA, as ordens e o plano da pré-temporada) e Friendly::set_preseason_windows (0x5c2d40, 2: a janela de cada nação).
# O SEH da 0x5c2460 e da 0x5c2d40 só cobre datas (destrutor 0x5a53c0, um ret) e a lista nova durante o construtor.
# A 0x5c2d40 calcula a data de início da liga (0x549ef0, 0x54c5b0) e não usa: a cópia chama igual (só locais) e pula
# as duas datas locais construídas e destruídas sem uso (0x9463af/0x9462bb). A 0x5c2460 devolve `mov al, 1` (sobra em
# cima): a cópia devolve 1. Os pedidos vencidos são olhados mesmo depois de tirados (como a original). Iguais no
# cm0102-1080.exe. Simulação: as mutações pela carreira e a confirmação no jogo real comparando. Comparadores: o dia e o
# plano só a original (cancelam, sorteiam, alocam); as janelas pelo plano feito antes (sem gravar nem alocar).
SAVE = False
ROTINAS = [('atualizar_amistosos_do_dia', 0x5c2460, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('planejar_pre_temporada_da_ia', 0x5c27a0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('janelas_da_pre_temporada', 0x5c2d40, [0x6a, 0xff, 0x68, 0x4e, 0x9e, 0x95, 0x00])]
DOCS = ('a pré-temporada dos amistosos: o dia dos amistosos, o plano da pré-temporada da IA e as janelas da pré-temporada',
        'the pre-season friendlies: the friendlies day, the AI pre-season plan and the pre-season windows',
        [('atualizar_amistosos_do_dia', 'Friendly::update_for_date', '0x5c2460', 'friendly.cpp', 'thiscall, ret 8',
          'nos dias da pré-temporada planeja os amistosos da IA e cancela torneios velhos; todo dia cria os amistosos planejados, responde os pedidos e vence torneios, pedidos e convites',
          'on the pre-season days plans the AI friendlies and cancels old tournaments; every day creates the planned friendlies, answers requests and expires tournaments, requests and invitations'),
         ('planejar_pre_temporada_da_ia', 'Friendly::plan_ai_preseason_friendlies', '0x5c27a0', 'friendly.cpp', 'thiscall, ret 0',
          'monta um candidato por clube com nação (faixa = reputação / 500), ordena pela faixa e pela nação, põe as janelas e planeja torneios, excursões e amistosos',
          'builds one candidate per club with a nation (band = reputation / 500), sorts by band and by nation, sets the windows and plans tournaments, tours and friendlies'),
         ('janelas_da_pre_temporada', 'Friendly::set_preseason_windows', '0x5c2d40', 'friendly.cpp', 'thiscall, ret 8',
          'para cada nação em uso, grava nos candidatos dela a janela até o início da temporada − 1 semana (18 dias antes, no mínimo 3) e dá lista de datas ao primeiro e ao último da liga',
          "for each nation in use, writes to its candidates the window up to the season start − 1 week (18 days before, at least 3) and gives a date list to the league's first and last club")])
CAMPOS = [('Friendly +0xa7: o início da temporada (data); +0xb1 os 127 torneios (0x69 bytes: +0 em uso, +4 a data, +0x65 a lista); +0x34c8 os pedidos (+0x10/+0x14 os clubes, +0 a data); +0x34cc os convites (10 de 0x19 bytes, a data em +9, a marca em +0x15); +0x34d0/+0x34d4 as respostas; +0x34e8/+0x34ea os dias da pré-temporada; valor e ponteiro; lê e grava (atualizar_amistosos_do_dia)',
           'Friendly +0xa7: the season start (date); +0xb1 the 127 tournaments (0x69 bytes: +0 in use, +4 the date, +0x65 the list); +0x34c8 the requests (+0x10/+0x14 the clubs, +0 the date); +0x34cc the invitations (10 of 0x19 bytes, date at +9, flag at +0x15); +0x34d0/+0x34d4 the replies; +0x34e8/+0x34ea the pre-season days; value and pointer; reads and writes (atualizar_amistosos_do_dia)'),
          ('candidato do amistoso (0x16 bytes) +0: o id do clube; +5 a faixa; +6 o id da nação; +0xa/+0x12 as listas; +0xe/+0x10 a janela; valor e ponteiro; grava (planejar_pre_temporada_da_ia, janelas_da_pre_temporada)',
           'friendly candidate (0x16 bytes) +0: club id; +5 the band; +6 nation id; +0xa/+0x12 the lists; +0xe/+0x10 the window; value and pointer; writes (planejar_pre_temporada_da_ia, janelas_da_pre_temporada)'),
          ('clube +0x53: a nação; +0x80 a reputação (palavra); ponteiro e valor; lê (planejar_pre_temporada_da_ia, janelas_da_pre_temporada)',
           'club +0x53: the nation; +0x80 the reputation (word); pointer and value; reads (planejar_pre_temporada_da_ia, janelas_da_pre_temporada)'),
          ('[0xb63d60] as 35 temporadas das nações (0x48 bytes: +0 a nação, +0x26 a liga, +0x2a o início); [0xdd6ec8] o início do jogo; valor e ponteiro; lê',
           '[0xb63d60] the 35 nation seasons (0x48 bytes: +0 the nation, +0x26 the league, +0x2a the start); [0xdd6ec8] the game start; value and pointer; reads')]
INTERESSA = ['CMRTE']
