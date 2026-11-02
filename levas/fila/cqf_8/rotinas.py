# O histórico da carreira do jogador (player_stats.cpp), pedido de 02/11 (agente C, família das pessoas):
# - 0x7c3e40 (PlayerStats::build_career_history, 2416 bytes): a tabela de seis entradas de 0x2b bytes por temporada e
#   clube, montada da temporada atual (0x7c3920) e do arquivo de histórico (FILE_LLIST::get_nodes 0x58f120, do fim para
#   o começo), com os anos vazios preenchidos, as entradas repetidas somadas e os grupos vazios tirados. 1087 chamadas
#   na carreira automática até 2002-06-01 (cobertura de 02/11): quase todas pelas permissões de trabalho da criação do
#   jogo (0x8c6e60, já em Rust: o clube e o ano do primeiro grupo de cada temporada decidem a permissão pela carreira
#   ou a segunda nacionalidade, pessoa +0x1e) e pelos gols na carreira e pelo clube (0x7c3d20, 0x7c3bb0). As mutações
#   estão nela.
# - 0x7c6e30 (player_stats_years_since_at_club, 400 bytes): passageira (a 0x5b03e0 que a chama não rodou na carreira).
# - 0x7c6de0 (player_stats_compare_history_by_staff, 78 bytes): a ordem do qsort do staff_history.dat na criação das
#   estatísticas (0x7bfd50): passageira.
# Iguais no cm0102-1080.exe (as três inteiras, conferido byte a byte).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
#   0x7c3e40: thiscall (mov ecx, [0xb74350]; push &conta; push pessoa em 0x7c6f4a; e as Rust 0x8c6e60, 0x7c3d20,
#   0x7c3bb0), 2 na pilha (ret 8), devolve a tabela (malloc). Dentro: 0x944e46 malloc cdecl (add esp, 4), 1; 0x669950
#   cdecl (add esp, 4), 1; 0x7c3920 thiscall (mov ecx, [esp+0x14] = this; push categoria; push ficha), 2; 0x58f120
#   thiscall (mov ecx, [this]; push +0x1c; push +0x18), 2; 0x68d1d0 thiscall (mov ecx, ebx = a lista), 0; 0x945667
#   realloc cdecl (add esp, 8), 2; 0x68d040 thiscall (mov ecx, ebx), 0; 0x944c94 cdecl (add esp, 4), 1; a caixa
#   (0x944cff, 0x90d130, 0x944c9f, 0x5e8290: aviso_em).
#   0x7c6e30: a 0x5b03e0 põe mov ecx, [0xb74350] e push clube; push pessoa (0x5b052d), mas ela não usa o ecx (lê o
#   [0xb74350] de novo) e termina em ret 8 -> stdcall, 2. Dentro: 0x7c3e40 thiscall (mov ecx, [0xb74350]), 2; 0x9452ca
#   free cdecl (add esp, 4), 1 (também com a tabela nula).
#   0x7c6de0: cdecl (ret), 2: ponteiro passado ao qsort em 0x7c05ad.
# Leitura de memória: a 0x7c3e40 só lê as entradas abaixo da conta (a de cada grupo seguinte, +0x102, só quando ele
#   existe); as entradas da tabela são sempre montadas inteiras (zeradas na pilha antes) ou copiadas inteiras do nó.
# Comparadores: 0x7c3e40 monta as duas tabelas (o arquivo só é lido; as duas pelo malloc do jogo) e compara a conta e as
#   entradas abaixo dela, liberando a da cópia; 0x7c6e30 e 0x7c6de0 pela devolução. Os casos de erro: só a original.
SAVE = False
ROTINAS = [('historico_da_carreira', 0x7c3e40, [0x81, 0xec, 0x48, 0x02, 0x00, 0x00]),
           ('anos_desde_o_clube', 0x7c6e30, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('ordem_do_historico_por_pessoa', 0x7c6de0, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('o histórico da carreira do jogador (a tabela das temporadas por clube montada do arquivo de histórico), os anos desde que jogou num clube e a ordem do histórico por pessoa',
        'the player career history (the table of seasons per club built from the history file), the years since he played for a club and the history order by person',
        [('historico_da_carreira', 'PlayerStats::build_career_history', '0x7c3e40', 'player_stats.cpp', 'thiscall',
          'a tabela da carreira do jogador, seis entradas (categorias) por temporada e clube: a temporada atual, as do arquivo de histórico do fim para o começo com os anos vazios preenchidos, as entradas repetidas da mesma temporada e clube somadas e os grupos vazios tirados',
          'the player career table, six entries (categories) per season and club: the current season, the ones from the history file from newest to oldest with the empty years filled, the repeated entries of the same season and club summed and the empty groups removed'),
         ('anos_desde_o_clube', 'player_stats_years_since_at_club', '0x7c6e30', 'player_stats.cpp', 'stdcall',
          'quantos anos desde que a pessoa jogou pelo clube: 0 se está nele, −1 se nunca',
          'how many years since the person played for the club: 0 when he is there, −1 when never'),
         ('ordem_do_historico_por_pessoa', 'player_stats_compare_history_by_staff', '0x7c6de0', 'player_stats.cpp', 'cdecl',
          'a ordem do qsort dos registros de histórico: pela pessoa, pelo ano e pelo primeiro campo',
          'the qsort order of the history records: by person, by year and by the first field')])
CAMPOS = [('tabela da carreira (0x2b bytes por entrada, 6 por temporada): +0 ano; +2 id do clube (−1 nenhum); +6..+0x26 os 32 bytes da categoria (+6 jogos, +8 gols [inferência pelos usos da 0x7c3d20/0x7c3bb0], +0x14 palavra); +0x26 valor da transferência (−1 nenhum); +0x2a categoria × 8 e o tipo (3 bits); valor; grava (historico_da_carreira)',
           'career table (0x2b bytes per entry, 6 per season): +0 year; +2 club id (−1 none); +6..+0x26 the 32 category bytes (+6 games, +8 goals [inference from 0x7c3d20/0x7c3bb0 uses], +0x14 word); +0x26 transfer fee (−1 none); +0x2a category × 8 and the kind (3 bits); value; writes (historico_da_carreira)'),
          ('PlayerStats ([0xb74350]) +0 o arquivo de histórico (FILE_LLIST), +0x30 registros de 0x27 bytes por ficha de jogador (+0x18/+0x1c as posições do primeiro e do último nó); ponteiro/valor; lê',
           'PlayerStats ([0xb74350]) +0 the history file (FILE_LLIST), +0x30 0x27-byte records per player record (+0x18/+0x1c the first and last node offsets); pointer/value; reads'),
          ('pessoa +0x39 clube, +0x61 ficha; ponteiro; lê; [0xae2c92] o ano de hoje; valor; lê (anos_desde_o_clube)',
           'person +0x39 club, +0x61 record; pointer; reads; [0xae2c92] the current year; value; reads (anos_desde_o_clube)')]
INTERESSA = ['CMRTE', 'SGE', 'CMScout']
