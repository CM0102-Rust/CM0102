# O dia das selecoes e as buscas da sessao do tecnico humano (national_teams.cpp, scout_manager.cpp):
# - 0x76b100 (NationalTeams::daily_update): pelo dia do jogo (0x5ccb07: mov ecx, [0xb74318]; push 0xae2c90); 516
#   chamadas na carreira automatica ate 2002-06-01 (docs/cobertura_simulacao.txt). As convocacoes (0x76bab0), as
#   reservas das selecoes (0x76abf0) e a remarcacao dos jogos dos clubes com convocados (0x7777e0) mudam as partidas.
# - 0x7f8420 (SCOUT_MANAGER::current_search_has_no_club) e 0x7f8500 (SCOUT_MANAGER::is_club_in_session_searches):
#   passageiras (so pelas telas das buscas; nao rodam ate 2002-06-01).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   As proprias: 0x76b100 thiscall, 1 arg (push 0xae2c90; mov ecx, [0xb74318]; ret 4); 0x7f8420 thiscall sem args
#   (mov ecx, [0xb74448] em 0x482a95; ret); 0x7f8500 thiscall, 1 arg (push edi; ecx = [0xb74448]; ret 4).
#   Na 0x76b100: 0x54c770 thiscall (push 2; push saida; mov ecx, esi = hoje; ret 8), 0x9452ca free cdecl (add esp,
#   4), 0x549f70 thiscall (4 pushes: semana -1, ano, mes, dia; lea ecx, saida; ret 0x10; le o ano e a semana em
#   palavra), 0x76abf0/0x7774b0/0x7779e0 thiscall sem args (mov ecx, edi), 0x5abf10 thiscall (4 pushes: grupo,
#   fatia, dia, ano em dwords; mov ecx, [0xae2a58]; ret 0x10), 0x7775c0 thiscall (push &data; mov ecx, edi; ret 4),
#   0x54c8e0/0x54c5b0/0x417c10 thiscall (ret 8), 0x68d1b0 thiscall (mov ecx, lista), 0x76bab0 thiscall (push p; mov
#   ecx, edi; ret 4), 0x5409b0 cdecl (push; add esp, 4), 0x7771c0 thiscall (push p; push b; push time; mov ecx, edi;
#   ret 0xc), 0x53b4f0 cdecl (push nacao; add esp, 4; os dois pushes antes sao da 0x7771c0), 0x7777e0 thiscall (push
#   &data; mov ecx, edi; ret 4), 0x68d1f0 thiscall (push item; mov ecx, [edi]; ret 4), 0x58d460 thiscall (push hoje;
#   mov ecx, [edi + 0x1c]; ret 4). Nas 0x7f8420/0x7f8500, so a caixa (pela original).
# Comparadores: 0x76b100 so a original (convoca, remarca, aloca e libera: nao pode rodar duas vezes), conferindo o
# plano do que ela mesma grava: nenhum item do dia de hoje sobra na lista em +0. 0x7f8420 e 0x7f8500 pelo valor (so
# leem); com a caixa, so a original. SEH da 0x76b100 so das datas locais (sem destrutor). Iguais no cm0102-1080.exe
# (as tres inteiras).
SAVE = False
ROTINAS = [('dia_das_selecoes', 0x76b100, [0x6a, 0xff, 0x68, 0x08, 0xdd, 0x95, 0x00]),
           ('busca_atual_sem_clube', 0x7f8420, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
           ('clube_nas_buscas_da_sessao', 0x7f8500, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00])]
DOCS = ('o dia das seleções e as buscas da sessão do técnico humano',
        'the national teams day and the human manager session searches',
        [('dia_das_selecoes', 'NationalTeams::daily_update', '0x76b100', 'national_teams.cpp', 'thiscall',
          'o dia das seleções: a tabela dos clubes com jogo sai; em 1º de janeiro, 15 de abril e 1º de agosto as reservas se refazem; para os jogos de daqui a duas semanas, os clubes com jogo em volta da data são marcados, as seleções convocam e os convocados por clube são registrados (fora das fases finais); com a tabela, os jogos dos clubes com convocados se remarcam; saem os itens do dia, o ranking da FIFA anda, a cada 70 dias os registros são podados e vão as notícias dos classificados',
          'the national teams day: the table of clubs with a match is freed; on 1 January, 15 April and 1 August the pools are rebuilt; for the matches two weeks ahead, the clubs playing around the date are marked, the national teams call up and the call-ups per club are recorded (outside the finals); with the table, the matches of clubs with called-up players are rescheduled; the day\'s items leave, the FIFA ranking moves, every 70 days the records are pruned and the qualified nations news goes out'),
         ('busca_atual_sem_clube', 'SCOUT_MANAGER::current_search_has_no_club', '0x7f8420', 'scout_manager.cpp', 'thiscall',
          'a busca atual da sessão do técnico humano da vez não tem clube (+0x191 = −2)',
          'the current search of the current human manager session has no club (+0x191 = −2)'),
         ('clube_nas_buscas_da_sessao', 'SCOUT_MANAGER::is_club_in_session_searches', '0x7f8500', 'scout_manager.cpp', 'thiscall',
          'alguma das 10 buscas da sessão do técnico humano da vez é do clube',
          'one of the 10 searches of the current human manager session is for the club')])
CAMPOS = [('NationalTeams +0 lista de itens com dia em +0x10; ponteiro; lê e tira os do dia (dia_das_selecoes)',
           'NationalTeams +0 list of items with a day at +0x10; pointer; reads and removes the day\'s items (dia_das_selecoes)'),
          ('NationalTeams +0x1c ranking da FIFA; ponteiro; lê (dia_das_selecoes)', 'NationalTeams +0x1c FIFA ranking; pointer; reads (dia_das_selecoes)'),
          ('NationalTeams +0x24 tabela dos clubes com jogo; ponteiro; libera e zera (dia_das_selecoes)',
           'NationalTeams +0x24 table of clubs with a match; pointer; frees and clears (dia_das_selecoes)'),
          ('partida +0x14 competição (+0 chave), +0x1c/+0x20 seleções, +0x40 marca, +0x4d bits 0x100/0x200; ponteiro/valor; lê (dia_das_selecoes)',
           'fixture +0x14 competition (+0 key), +0x1c/+0x20 national sides, +0x40 flag, +0x4d bits 0x100/0x200; pointer/value; reads (dia_das_selecoes)'),
          ('SCOUT_MANAGER +0xc sessões dos 16 humanos; ponteiro; lê (busca_atual_sem_clube, clube_nas_buscas_da_sessao)',
           'SCOUT_MANAGER +0xc sessions of the 16 humans; pointer; reads (busca_atual_sem_clube, clube_nas_buscas_da_sessao)'),
          ('sessão de busca +0 busca atual (byte), buscas de 0x2b0 bytes com +0x191 o clube (−2 nenhum); valor/id; lê (busca_atual_sem_clube, clube_nas_buscas_da_sessao)',
           'search session +0 current search (byte), 0x2b0-byte searches with +0x191 the club (−2 none); value/id; reads (busca_atual_sem_clube, clube_nas_buscas_da_sessao)')]
INTERESSA = ['CMRTE']
