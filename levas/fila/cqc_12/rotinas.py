# O dia das selecoes, a escolha do elenco e as buscas da sessao do tecnico humano (national_teams.cpp, scout_manager.cpp):
# - 0x76b100 (NationalTeams::daily_update): pelo dia do jogo (0x5ccb07: mov ecx, [0xb74318]; push 0xae2c90); 516
#   chamadas na carreira automatica ate 2002-06-01 (docs/cobertura_simulacao.txt). As convocacoes (0x76bab0), as
#   reservas das selecoes (0x76abf0) e a remarcacao dos jogos dos clubes com convocados (0x7777e0) mudam as partidas.
# - 0x7f8420 (SCOUT_MANAGER::current_search_has_no_club) e 0x7f8500 (SCOUT_MANAGER::is_club_in_session_searches):
#   passageiras (so pelas telas das buscas; nao rodam ate 2002-06-01).
# - 0x76c120 (NationalTeams::select_squad, ~4,9 KB; 4a rotina, 01/11/2026): pela convocacao (0x76bab0, Rust), pelas
#   tabelas (0x76a840) e pelas noticias (0x784100). CORRECAO DO ORIGINAL APROVADA PELO LUCAS (01/11/2026): a original so
#   preenche os codigos de posicao (local_86c) dos elencos de 22, 24 e 26; nos de 23 (fases finais: a Copa de 2002 em
#   29/05/2002) e 25 as vagas a partir da 22a ficam com a sobra da pilha, que vai a escala (0x6e2000) e ao filtro dos
#   escalados; com a 0x76b100 em Rust a sobra mudava a cada execucao e as carreiras Rust x2 saiam diferentes (FORA.md).
#   A porta da a essas vagas, em ordem, 0x202, 0x88 e 0x808 (as que a tabela de 24 tem a mais que a de 22): 23 = 22 +
#   0x202; 25 = 22 + 0x202, 0x88, 0x808 (o 25 cai na tabela de 22 na original). Ver cqc12_codigos.
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
#   0x76c120 thiscall, 5 args, ret 0x14 (0x76ab8e/0x76aba9: push 0; push 0; push 0xae2c90; push tipo; push nacao; mov
#   ecx, esi; 0x784630/0x78468a: push 1; push partida; push 0xae2c90; push tipo; push nacao; mov ecx, [0xb74318]).
#   Nela: 0x53b3d0/0x53b4f0/0x5409b0/0x53b620/0x5b7d20 cdecl 1 arg (add esp, 4); 0x547d30 cdecl 2; 0x540980 cdecl 2
#   (marca: so o byte); 0x600a50 cdecl 5; 0x6e2000 cdecl 15 (add esp, 0x3c; quantidade e tamanho so o byte); 0x945333
#   qsort cdecl 4; 0x944cf1 new cdecl 1; 0x5c6710 thiscall ([0xae2c74], ret 4); 0x549ef0/0x54c8e0/0x417c10 thiscall
#   ret 8; 0x54a110 e 0x68d020 thiscall sem args; 0x68d0d0 thiscall ret 8 (item de 0x12 bytes); 0x76d880 thiscall
#   (this, ret 4); 0x76d430 stdcall 4 (ret 0x10); 0x5fd6b0 thiscall ([0xb63c98], ret 8, al); 0x62c400 thiscall
#   ([0xb63d28], ret 4, al); 0x62ed70 thiscall ([0xb63d28], ret 4); 0x7a7eb0 stdcall 1 (ecx = [0xb7434c], al);
#   0x4cf5a0 thiscall ([0xae1050], ret 4); 0x55d270 thiscall ([0xae28f0], ret 0x10); 0x76db00 thiscall (this, 6 args,
#   ret 0x18; o tipo so o byte); 0x896c60 thiscall ([0xde1d08], ret 0x10); 0x8968f0/0x896e90 thiscall ([0xde1d08], ret
#   0xc); 0x76e0b0 7 args ret 0x1c e 0x76f1a0 3 args ret 0xc (ecx = this); a caixa (0x944cff, 0x90d130, 0x944c9f,
#   0x5e8290: aviso_em). A original passa a 0x76db00 (aceitacao, vaga da tatica) como jogador o dword do ultimo ponteiro
#   de percurso (fim das reservas ou a vaga do time anterior): reproduzido (memoria do jogo, deterministica). x87: as
#   reputacoes x 0.8 e x 0.7 (0x9689b0, 0x968e40) em f64.
# Comparadores: 0x76b100 so a original (convoca, remarca, aloca e libera: nao pode rodar duas vezes), conferindo o
# plano do que ela mesma grava: nenhum item do dia de hoje sobra na lista em +0. 0x7f8420 e 0x7f8500 pelo valor (so
# leem); com a caixa, so a original. 0x76c120 so a original (aloca, mexe nos elencos e nas vagas, manda noticias), com
# a fita aberta nas regioes que a escala le e a original muda depois (registro da nacao e +0x24..+0x28/+0x5e dos
# jogadores das reservas): a escala (0x6e2000) entra como unidade com a lista e os codigos e o estado de antes; a copia
# Rust monta lista e codigos nesse estado e confere (quantidade, modo, time, lista, codigos). Nos elencos de 23 e 25,
# as vagas a partir da 22a ficam fora (lixo na original; ramo da correcao). SEH da 0x76b100 e da 0x76c120 so das datas
# locais (sem destrutor). Iguais no cm0102-1080.exe (as quatro inteiras).
SAVE = False
ROTINAS = [('dia_das_selecoes', 0x76b100, [0x6a, 0xff, 0x68, 0x08, 0xdd, 0x95, 0x00]),
           ('busca_atual_sem_clube', 0x7f8420, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
           ('clube_nas_buscas_da_sessao', 0x7f8500, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
           ('escolher_elenco_da_selecao', 0x76c120, [0x6a, 0xff, 0x68, 0x7a, 0xdd, 0x95, 0x00])]
DOCS = ('o dia das seleções, a escolha do elenco da seleção (com a correção dos elencos de 23 e 25) e as buscas da sessão do técnico humano',
        'the national teams day, the national squad selection (with the fix for 23- and 25-man squads) and the human manager session searches',
        [('dia_das_selecoes', 'NationalTeams::daily_update', '0x76b100', 'national_teams.cpp', 'thiscall',
          'o dia das seleções: a tabela dos clubes com jogo sai; em 1º de janeiro, 15 de abril e 1º de agosto as reservas se refazem; para os jogos de daqui a duas semanas, os clubes com jogo em volta da data são marcados, as seleções convocam e os convocados por clube são registrados (fora das fases finais); com a tabela, os jogos dos clubes com convocados se remarcam; saem os itens do dia, o ranking da FIFA anda, a cada 70 dias os registros são podados e vão as notícias dos classificados',
          'the national teams day: the table of clubs with a match is freed; on 1 January, 15 April and 1 August the pools are rebuilt; for the matches two weeks ahead, the clubs playing around the date are marked, the national teams call up and the call-ups per club are recorded (outside the finals); with the table, the matches of clubs with called-up players are rescheduled; the day\'s items leave, the FIFA ranking moves, every 70 days the records are pruned and the qualified nations news goes out'),
         ('busca_atual_sem_clube', 'SCOUT_MANAGER::current_search_has_no_club', '0x7f8420', 'scout_manager.cpp', 'thiscall',
          'a busca atual da sessão do técnico humano da vez não tem clube (+0x191 = −2)',
          'the current search of the current human manager session has no club (+0x191 = −2)'),
         ('clube_nas_buscas_da_sessao', 'SCOUT_MANAGER::is_club_in_session_searches', '0x7f8500', 'scout_manager.cpp', 'thiscall',
          'alguma das 10 buscas da sessão do técnico humano da vez é do clube',
          'one of the 10 searches of the current human manager session is for the club'),
         ('escolher_elenco_da_selecao', 'NationalTeams::select_squad', '0x76c120', 'national_teams.cpp', 'thiscall',
          'a escolha do elenco da seleção (principal, B/olímpico ou sub-21): nas fases finais o item da lista; as datas do elenco, as idades e o tamanho; com técnico humano, o pedido a ele; senão saem os de outro time e os marcados, a lista das reservas vai à escala e cada escalado que serve na vaga entra no elenco (sai do time anterior e das vagas da tática); a ordem, a notícia e as vagas da tática. Correção do original aprovada (01/11/2026): os códigos de posição também nos elencos de 23 e 25 (a original deixava a sobra da pilha nas vagas a partir da 22ª): as 22 da original mais, em ordem, 0x202, 0x88 e 0x808, as vagas que a tabela de 24 tem a mais que a de 22',
          'the national squad selection (senior, B/Olympic or under-21): in the finals the list item; the squad dates, the ages and the size; with a human manager, the request to him; otherwise those from another side and the flagged ones leave, the pool list goes to the slot assignment and each picked player who fits the slot joins the squad (leaving the previous side and the tactic slots); the sorting, the news and the tactic slots. Approved fix of the original (01/11/2026): position codes also for 23- and 25-man squads (the original left stack leftovers in the slots from the 22nd on): the original 22 plus, in order, 0x202, 0x88 and 0x808, the slots the 24 table has beyond the 22 table')])
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
           'search session +0 current search (byte), 0x2b0-byte searches with +0x191 the club (−2 none); value/id; reads (busca_atual_sem_clube, clube_nas_buscas_da_sessao)'),
          ('NationalTeams +4 registros das nações (0x6e bytes: +8/+0xd.. as reservas por tipo, +0x32/+0x3a datas, +0x42 tamanho, +0x43 marca, +0x44 janela); ponteiro; lê e grava (escolher_elenco_da_selecao)',
           'NationalTeams +4 nation records (0x6e bytes: +8/+0xd.. pools per type, +0x32/+0x3a dates, +0x42 size, +0x43 flag, +0x44 window); pointer; reads and writes (escolher_elenco_da_selecao)'),
          ('NationalTeams +0xc/+0x10 vetores das reservas (jogadores); ponteiro; lê (escolher_elenco_da_selecao)', 'NationalTeams +0xc/+0x10 pool arrays (players); pointer; reads (escolher_elenco_da_selecao)'),
          ('time +0xd7 elenco (50 pessoas); ponteiro; lê e grava (escolher_elenco_da_selecao)', 'side +0xd7 squad (50 people); pointer; reads and writes (escolher_elenco_da_selecao)'),
          ('pessoa +0x24 time, +0x28 estado, +0x5e marcas de elenco; ponteiro/valor; lê e grava (escolher_elenco_da_selecao)', 'person +0x24 side, +0x28 status, +0x5e squad flags; pointer/value; reads and writes (escolher_elenco_da_selecao)')]
INTERESSA = ['CMRTE']
