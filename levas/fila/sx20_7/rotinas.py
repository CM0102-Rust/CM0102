# As estatisticas simuladas e os registros de pessoas no save (simulated_stats.cpp, staff_records.cpp; leitor sx20,
# rodada 5, 24/11/2026): SimulatedStats::load (0x84c510), Staff_Records::load (0x86fce0) e Staff_Records::save
# (0x86ffd0). Rotinas inteiras (0x84c510..0x84c670 ret 4; 0x86fce0..0x86ffc1 ret 4; 0x86ffd0..0x87021b ret 4).
# DEPENDE DO sx20_2 e do sx20_6 (SIMULATED_STATS_SOURCE, STAFF_RECORDS_SOURCE, MANAGER_RECORD_SIZE), que dependem do
# sx20_1 (SIMULATED_GROUP_SIZE).
# Quem chama: SimulatedStats::load, o construtor com save (0x84ba5f, na carga); Staff_Records::load, o construtor com
# save (0x86867c); Staff_Records::save, o save_game_data (0x82ebe3, passo 8 do SaveStep, o retorno nao e lido).
# Convencoes conferidas na desmontagem: as tres thiscall(this, save) ret 4; 0x9326e0 zopen cdecl(save, nome, 0 ler / 1
# gravar, 0x16, 4); 0x932c30 zread / 0x932e10 zwrite cdecl(dados, tamanho, quantos, item); 0x932b00 zclose cdecl(&item);
# a caixa de erro sem fechar o item; SimulatedStats::load sem save devolve 0 sem caixa; Staff_Records::load nao
# confere o save; Staff_Records::save nao devolve valor (eax de sobra). A versao dos dados em [0xdd7110]
# (G_GAME_DATA_VERSION, |v| < 0x3e031: registro de 0xce bytes).
# Defeitos do original: o malloc dos grupos nao e conferido; depois de erro o item fica aberto.
SAVE = True
CARGA = True  # as duas cargas so rodam carregando: a prova do save carrega o save de ferias com e sem a leva
ROTINAS = [('simulated_stats_load', 0x84c510, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('staff_records_load', 0x86fce0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('staff_records_save', 0x86ffd0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('as estatisticas simuladas e os registros de pessoas no save',
        'the simulated statistics and staff records in the saved game',
        [('simulated_stats_load', 'SimulatedStats::load', '0x84c510', 'simulated_stats.cpp', 'thiscall, ret 4',
          'le os grupos do save: sem save 0; o item simulated_stats.dat (falta: a caixa, linha 0x1dd, e 0), o numero de grupos em +0 (curto: 0x1e4 e 0) e uma tabela nova de grupos x 0x44 bytes em +4 lida de uma vez (curta: 0x1ef e 0); fecha e devolve 1',
          'reads the groups from the save: no save 0; the simulated_stats.dat item (missing: the box, line 0x1dd, and 0), the group count at +0 (short: 0x1e4 and 0) and a new table of groups x 0x44 bytes at +4 read at once (short: 0x1ef and 0); closes and returns 1'),
         ('staff_records_load', 'Staff_Records::load', '0x86fce0', 'staff_records.cpp', 'thiscall, ret 4',
          'le os registros de pessoas do save: o item staff records.dat (falta: 0x1103 e 0); cada registro de tecnico (nao jogadores + 16), com 0xce bytes e os pontos do mes na liga zerados em save de versao abaixo de 0x3e031 (em valor absoluto), senao 0xd2; as cabecas das listas de historico de pessoas e de clubes (8 bytes cada); o mes e o ano guardados; fecha e devolve 1; erro: a caixa e 0 com o item aberto (o construtor refaz do zero)',
          'reads the staff records from the save: the staff records.dat item (missing: 0x1103 and 0); each manager record (non-players + 16), 0xce bytes with the month league points cleared in a save of version below 0x3e031 (absolute value), else 0xd2; the person and club history list heads (8 bytes each); the stored month and year; closes and returns 1; on error the box and 0 with the item open (the constructor rebuilds from scratch)'),
         ('staff_records_save', 'Staff_Records::save', '0x86ffd0', 'staff_records.cpp', 'thiscall, ret 4',
          'grava os registros de pessoas no save: o item staff records.dat (falta: 0x1155), os registros de tecnico (nao jogadores + 16 de 0xd2 bytes), as cabecas das listas de pessoas e de clubes, o mes e o ano; fecha; erro: a caixa com o item aberto',
          'writes the staff records into the save: the staff records.dat item (missing: 0x1155), the manager records (non-players + 16 of 0xd2 bytes), the person and club list heads, the month and year; closes; on error the box with the item open')])
