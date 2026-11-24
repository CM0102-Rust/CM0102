# A palavra do gerente dos elencos lida do jogo salvo e as reservas dos estadios gravadas no Stadium.tmp
# (squad_manager.cpp, stadium.cpp; leitor cx15, rodada 5, 24/11/2026). Item do SAVE (grava e carrega): as duas rodam em
# toda carga e em toda gravacao (o FeriasTesteSave serve).
# - 0x859bc0 SQUAD_MANAGER::load thiscall(save) ret 4 (do construtor 0x859ba4, com o conteiner do save): zopen 0x9326e0
#   cdecl(save, "squad_manager.dat", 0, 0x16, 4) (0: caixa da linha 0x43 e 0); zread 0x932c30 cdecl(&v, 4, 1, item)
#   (!= 1: caixa da linha 0x4a e 0, o item fica aberto); +0 = a palavra baixa de v; zclose 0x932b00 cdecl(&item); 1.
#   Prologo sub esp, 0x208 (6 bytes).
# - 0x85bea0 Stadium::save_to_temp_file thiscall() ret 0 (0x82f0a8, na gravacao): o inverso do load_from_temp_file
#   (0x85ba40, ja em Rust). fopen 0x945b00 de utils_build_file_path 0x90bde0("CM3_TEMP", "Stadium.tmp") com "wb" (0:
#   caixa 0x179); fwrite 0x945ce6 (cdecl) de +8 (4; 0x181), +4 (4; 0x187), com reservas as +4 reservas de 12 bytes
#   (0x192), +0x14 (4; 0x19b), +0x10 (4; 0x1a1), +0x18 (1; 0x1a7); com sedes, o estadio (+0xc) de cada uma vira o id
#   (nulo -1), as +0x10 sedes de 16 bytes (0x1be: volta sem desfazer os ids) e os ids voltam a registros ([0xae23b0] +
#   0x4e x id; -1 nulo); [0xa835a4] (4; 0x1d1); fclose 0x945a09. Depois de uma caixa o arquivo fica aberto. Prologo sub
#   esp, 0x204 (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (leem e gravam o jogo); a carga com o retorno 1, a
#   gravacao com as sedes iguais antes e depois.
# Defeito do original (0x85bea0): se a gravacao das sedes falhar, a caixa sai e os estadios das sedes ficam como ids.
SAVE = True
CARGA = True
ROTINAS = [('squad_manager_load', 0x859bc0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('stadium_save_to_temp_file', 0x85bea0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('a palavra do gerente dos elencos lida do jogo salvo e as reservas dos estádios gravadas',
        'the squad manager\'s word read from the saved game and the stadium bookings saved',
        [('squad_manager_load', 'SQUAD_MANAGER::load', '0x859bc0', 'squad_manager.cpp', 'thiscall, ret 4',
          'lê do contêiner do jogo salvo o item squad_manager.dat e guarda o ano da inscrição europeia',
          'reads the squad_manager.dat item from the saved game container and keeps the European registration year'),
         ('stadium_save_to_temp_file', 'Stadium::save_to_temp_file', '0x85bea0', 'stadium.cpp', 'thiscall, ret 0',
          'grava no Stadium.tmp as reservas dos estádios, a lista das sedes neutras (os estádios pelos ids) e a sede guardada',
          'writes to Stadium.tmp the stadium bookings, the neutral venue list (the stadiums by id) and the stashed venue')])
CAMPOS = []
INTERESSA = []
