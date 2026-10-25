# Conversores da versão 1 (Db_files.cpp): os leitores de clubes, não jogadores e jogadores (0x54db50, 0x54e880,
# 0x54f030, já em Rust) os chamam pelo endereço quando a entrada do index traz a versão 1. A base de fábrica e o
# save são versão 2: nem a carreira nem a prova do save passam por eles (SAVE = True só para agrupar com a leva
# do banco). A prova é o autoteste_dbj_1 (CM_AUTOTESTE=1): arquivos temporários de versão 1 com sementes fixas,
# de 1 a 6 registros, inteiros ou cortados no meio, original x Rust pelo comparador (fread, fseek de volta).
# Sem SEH e sem variável local lida sem valor (o byte da reputação é lido num argumento da pilha, já usado).
SAVE = True
ROTINAS = [('ler_clubes_v1', 0x54dc90, [0x51, 0x8b, 0x44, 0x24, 0x14]),
           ('ler_nao_jogadores_v1', 0x54e9a0, [0x8b, 0x44, 0x24, 0x10, 0x53]),
           ('ler_jogadores_v1', 0x54f150, [0x8b, 0x44, 0x24, 0x10, 0x53])]
DOCS = ('os conversores da versão 1: clubes, não jogadores, jogadores',
        'the version 1 converters: clubs, non-players, players',
        [('ler_clubes_v1', 'db_files_read_clubs_v1', '0x54dc90', 'Db_files.cpp', 'cdecl',
          'lê o club.dat versão 1 campo a campo (577 bytes no arquivo para registros de 581, buracos em +0x37, +0x52 e +0x244), a reputação num byte (0 vira 1) × 500; 0 na primeira leitura que não vem, 1 se veio tudo',
          'reads version 1 club.dat field by field (577 bytes on file for 581-byte records, holes at +0x37, +0x52 and +0x244), the reputation in one byte (0 becomes 1) × 500; 0 at the first read that does not come, 1 if everything came'),
         ('ler_nao_jogadores_v1', 'db_files_read_players_v1', '0x54e9a0', 'Db_files.cpp', 'cdecl',
          'lê a tabela dos não jogadores (id 9; o mapa da comunidade a chama de players) versão 1 campo a campo (65 bytes no arquivo para registros de 68), as três reputações em bytes; 0 na primeira leitura que não vem, 1 se veio tudo',
          'reads the version 1 non-player table (id 9; the community map calls it players) field by field (65 bytes on file for 68-byte records), the three reputations in bytes; 0 at the first read that does not come, 1 if everything came'),
         ('ler_jogadores_v1', 'db_files_read_non_players_v1', '0x54f150', 'Db_files.cpp', 'cdecl',
          'lê a tabela dos jogadores (id 10; o mapa da comunidade a chama de non_players) versão 1 campo a campo (67 bytes no arquivo para registros de 70), as três reputações em bytes; 0 na primeira leitura que não vem, 1 se veio tudo',
          'reads the version 1 player table (id 10; the community map calls it non_players) field by field (67 bytes on file for 70-byte records), the three reputations in bytes; 0 at the first read that does not come, 1 if everything came')])
CAMPOS = [('TClub (581 bytes, [0xae23b4 / 0xae23b8]): +0x80 reputação (u16; versão 1: byte, 0 vira 1, × 500); registro inteiro menos +0x37, +0x52 e +0x244; grava (ler_clubes_v1)', 'TClub (581 bytes, [0xae23b4 / 0xae23b8]): +0x80 reputation (u16; version 1: byte, 0 becomes 1, × 500); whole record except +0x37, +0x52 and +0x244; writes (ler_clubes_v1)'),
          ('TNonPlayer (68 bytes, [0xae23c0]): +0x8/+0xa/+0xc reputações (u16; versão 1: bytes); registro inteiro; grava (ler_nao_jogadores_v1)', 'TNonPlayer (68 bytes, [0xae23c0]): +0x8/+0xa/+0xc reputations (u16; version 1: bytes); whole record; writes (ler_nao_jogadores_v1)'),
          ('TPlayer (70 bytes, [0xae23c4]): +0x9/+0xb/+0xd reputações (u16; versão 1: bytes); registro inteiro; grava (ler_jogadores_v1)', 'TPlayer (70 bytes, [0xae23c4]): +0x9/+0xb/+0xd reputations (u16; version 1: bytes); whole record; writes (ler_jogadores_v1)')]
INTERESSA = ['SGE', 'CMRTE']
