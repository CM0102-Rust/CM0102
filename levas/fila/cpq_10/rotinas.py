# As estatísticas dos jogadores e a aposentadoria marcada (player_stats.cpp, player_regen.cpp): o registro de
# estatísticas da categoria do jogador (PlayerStats::get_category_record 0x7c3920, 11 chamadores: a
# PlayerStats::record_match 0x7c10e6 a cada jogador de cada partida, a get_stat 0x7c1e30 das médias do motor de
# partida e os recordes do clube; o mapa dá 76 bytes, são 262 com a tabela de saltos), criar o registro
# (PlayerStats::create_stats 0x7c5d80, pela record_match 0x7c10f9 quando falta: reaproveita um registro solto ou
# acrescenta no fim, com realloc 0x945667) e se a pessoa está na lista de aposentadorias
# (PLAYER_REGEN::is_retirement_pending 0x7ae350, pela MANAGER_MANAGER::is_staff_suitable_for_job 0x69fa4b: quem não é
# jogador e vai se aposentar não serve para técnico). Sem SEH; os casos de erro (caixa "Error") ficam na original.
# Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira até 2002-06-01. Comparadores: o registro e a
# aposentadoria pelo valor; criar só a original (realoca), conferindo o plano feito antes (índice, conta, soltos e
# o registro zerado).
SAVE = False
ROTINAS = [('registro_da_categoria', 0x7c3920, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('criar_estatisticas', 0x7c5d80, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('aposentadoria_marcada', 0x7ae350, [0x8b, 0x44, 0x24, 0x04, 0x56])]
DOCS = ('as estatísticas dos jogadores e a aposentadoria marcada: o registro da categoria, criar o registro e se a pessoa está na lista de aposentadorias',
        'player statistics and the pending retirement: the category record, creating the record and whether the person is on the retirement list',
        [('registro_da_categoria', 'PlayerStats::get_category_record', '0x7c3920', 'player_stats.cpp', 'thiscall, ret 8',
          'o registro de 0x20 bytes da categoria (1..6) do jogador, pelo índice no registro de 39 bytes do id dele; 0 sem índice',
          'the 0x20-byte record of the player\'s category (1..6), through the index in the 39-byte entry of his id; 0 without an index'),
         ('criar_estatisticas', 'PlayerStats::create_stats', '0x7c5d80', 'player_stats.cpp', 'thiscall, ret 8',
          'o registro da categoria do jogador; sem índice, o primeiro registro solto (+0xe = 0xffff) ou um novo no fim (cresce de 100 em 100), zerado',
          'the player\'s category record; without an index, the first free record (+0xe = 0xffff) or a new one at the end (grows by 100), zeroed'),
         ('aposentadoria_marcada', 'PLAYER_REGEN::is_retirement_pending', '0x7ae350', 'player_regen.cpp', 'thiscall, ret 4',
          '1 quando o id da pessoa está na lista de aposentadorias, senão 0',
          '1 when the person\'s id is on the retirement list, else 0')])
CAMPOS = [('PlayerStats +0x30: o índice por jogador (39 bytes por id; +0..+0x14 os índices das 6 categorias, −1 sem registro); ponteiro e valor; lê (registro_da_categoria) e grava (criar_estatisticas)',
           'PlayerStats +0x30: the per-player index (39 bytes per id; +0..+0x14 the indexes of the 6 categories, −1 without a record); pointer and value; reads (registro_da_categoria) and writes (criar_estatisticas)'),
          ('PlayerStats +4, +8, +0xc, +0x38: os registros de 0x20 bytes, a capacidade, a conta e os soltos; ponteiro e valor; lê e grava (criar_estatisticas)',
           'PlayerStats +4, +8, +0xc, +0x38: the 0x20-byte records, capacity, count and free ones; pointer and value; reads and writes (criar_estatisticas)'),
          ('registro de estatísticas +0xe: 0xffff no registro solto; valor; lê (criar_estatisticas)',
           'statistics record +0xe: 0xffff in a free record; value; reads (criar_estatisticas)'),
          ('PLAYER_REGEN +4, +8: a lista de aposentadorias (0x1a bytes, +0 o id) e a conta; ponteiro e valor; lê (aposentadoria_marcada)',
           'PLAYER_REGEN +4, +8: the retirement list (0x1a bytes, +0 the id) and its count; pointer and value; reads (aposentadoria_marcada)')]
INTERESSA = ['CMRTE', 'CMScout', 'SGE']
