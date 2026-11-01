# Os temporários do save (uefa_seeding.cpp, conmebol_seeding.cpp, host_country.cpp): UefaSeeding::save_to_temp_file
# 0x905510, ConmebolSeeding::save_to_temp_file 0x4c6b70 e HostCountry::save_temp_file 0x5fb320, os passos 6, 7 e 0xe
# do save_game_data (0x82eb44, 0x82eb91, 0x82edca). Gravam os .tmp em CM3_TEMP, que o zadd_matching_files leva ao
# .qst. No .qst da prova (jogo novo): uefa_seeding.tmp 1.522 bytes (51 países, nenhum u32 extra), conmebol_seeding.tmp
# 117 (11 países), HostCountry.tmp 1.940 (57 anfitriões). Database.cpp e index.cpp esgotados (ver cpx_5). Iguais no
# cm0102-1080.exe; sem SEH; sem x87; só o fopen/fwrite/fclose do jogo, o caminho 0x90bde0 e a caixa de erro.
# Comparadores: só a original (gravam arquivo); a prova é a do save.
SAVE = True
ROTINAS = [('gravar_sementes_da_uefa', 0x905510, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('gravar_sementes_da_conmebol', 0x4c6b70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('gravar_paises_anfitrioes', 0x5fb320, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os temporários do save: as sementes da UEFA e da CONMEBOL e os países anfitriões',
        'the saved game temporary files: the UEFA and CONMEBOL seedings and the host countries',
        [('gravar_sementes_da_uefa', 'UefaSeeding::save_to_temp_file', '0x905510', 'uefa_seeding.cpp', 'thiscall',
          'grava CM3_TEMP\\uefa_seeding.tmp: quantos países (+0x438), 0x12 bytes por país de +0, +0x439 (u16), quantos (+0x43f) u32 de [+0x43b] e 50 registros de 0xc de +0x440; erro: a caixa "Error" sem fechar o arquivo',
          'writes CM3_TEMP\\uefa_seeding.tmp: the country count (+0x438), 0x12 bytes per country from +0, +0x439 (u16), as many u32 as +0x43f from [+0x43b] and 50 records of 0xc from +0x440; on error the "Error" box without closing the file'),
         ('gravar_sementes_da_conmebol', 'ConmebolSeeding::save_to_temp_file', '0x4c6b70', 'conmebol_seeding.cpp', 'thiscall',
          'grava CM3_TEMP\\conmebol_seeding.tmp: quantos países (+0x6e), 0xa bytes por país de +0, +0x6f (u16) e +0x71 (u32); erro: a caixa "Error" sem fechar o arquivo',
          'writes CM3_TEMP\\conmebol_seeding.tmp: the country count (+0x6e), 0xa bytes per country from +0, +0x6f (u16) and +0x71 (u32); on error the "Error" box without closing the file'),
         ('gravar_paises_anfitrioes', 'HostCountry::save_temp_file', '0x5fb320', 'host_country.cpp', 'thiscall',
          'grava CM3_TEMP\\HostCountry.tmp: quantos (+4, u16) e 0x22 bytes por anfitrião de [+0], sem conferir a gravação; erro ao abrir: a caixa "Error"',
          'writes CM3_TEMP\\HostCountry.tmp: the count (+4, u16) and 0x22 bytes per host from [+0], without checking the writes; on open error the "Error" box')])
CAMPOS = [('UefaSeeding +0: países (0x12 bytes cada); +0x438 quantos (i8); +0x439 (u16); +0x43b ponteiro dos u32 e +0x43f quantos (i8); +0x440 50 registros de 0xc; bytes; lê e grava no uefa_seeding.tmp (gravar_sementes_da_uefa)',
           'UefaSeeding +0: countries (0x12 bytes each); +0x438 count (i8); +0x439 (u16); +0x43b pointer to the u32s and +0x43f count (i8); +0x440 50 records of 0xc; bytes; reads and writes into uefa_seeding.tmp (gravar_sementes_da_uefa)'),
          ('ConmebolSeeding +0: países (0xa bytes cada); +0x6e quantos (i8); +0x6f (u16); +0x71 (u32); bytes; lê e grava no conmebol_seeding.tmp (gravar_sementes_da_conmebol)',
           'ConmebolSeeding +0: countries (0xa bytes each); +0x6e count (i8); +0x6f (u16); +0x71 (u32); bytes; reads and writes into conmebol_seeding.tmp (gravar_sementes_da_conmebol)'),
          ('HostCountry +0: anfitriões (0x22 bytes cada); ponteiro; +4 quantos (i16); lê e grava no HostCountry.tmp (gravar_paises_anfitrioes)',
           'HostCountry +0: hosts (0x22 bytes each); pointer; +4 count (i16); reads and writes into HostCountry.tmp (gravar_paises_anfitrioes)')]
INTERESSA = ['SGE']
