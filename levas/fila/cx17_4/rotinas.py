# Os dados próprios das eliminatórias sul-americanas e da Copa do Mundo no jogo salvo (wc_south_american_league.cpp,
# world_cup.cpp; leitor cx17, rodada 5, 24/11/2026). Item do SAVE (CARGA): o arquivo temporário de cada competição
# (League::save_to_temp_file / League::load_from_temp_file 0x687b10) chama as virtuais +0x24 (gravar) e +0x20 (ler) com o
# FILE aberto; as duas seções existem em qualquer carreira. A gravação da sul-americana é a comum (+0x24 = 0x7ec1f0, fora
# deste item).
# - 0x921590 WcSouthAmericanLeague::load_extra_data thiscall(arquivo) ret 4 -> 0/1: fread 0x945bfe (cdecl) de +0xee
#   (1 byte; curta: caixa 0x2ff e 0).
# - 0x92f000 WorldCup::load_extra_data thiscall(arquivo) ret 4 -> 0/1: fread de +0xee (5 x 0x20: as 32 vagas do
#   sorteio, clube e pote; caixa 0x53e), de +0x18e (2 x 1; caixa 0x546) e de +0x190 (2 x 1; caixa 0x54d); 1.
# - 0x92f150 WorldCup::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 (cdecl) dos mesmos três, na mesma
#   ordem, sem conferir; devolve o eax da última gravação.
# Prólogos: mov eax, [esp+4]; sub esp, 0x200 (10 bytes), sub esp, 0x200 (6) e push esi; push edi; mov edi, [esp+0xc] (6).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: as leituras lado a lado com o arquivo de volta à posição
# (ftell/fseek) e os campos de volta ao que tinham: o retorno, os campos e a posição final; a gravação só a original (a
# prova é a do save).
SAVE = True
CARGA = True
ROTINAS = [('wc_south_american_league_load_extra_data', 0x921590, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('world_cup_load_extra_data', 0x92f000, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('world_cup_save_extra_data', 0x92f150, [0x56, 0x57, 0x8b, 0x7c, 0x24, 0x0c])]
DOCS = ('os dados próprios das eliminatórias sul-americanas e da Copa do Mundo no jogo salvo',
        'the South American qualifying and World Cup own data in the saved game',
        [('wc_south_american_league_load_extra_data', 'WcSouthAmericanLeague::load_extra_data', '0x921590', 'wc_south_american_league.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário das eliminatórias sul-americanas a contagem de isentas (um byte); leitura curta: a caixa de erro e 0',
          'reads from the South American qualifying temporary file the exempt count (one byte); a short read: the error box and 0'),
         ('world_cup_load_extra_data', 'WorldCup::load_extra_data', '0x92f000', 'world_cup.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário da Copa do Mundo as 32 vagas do sorteio (clube e pote) e a data da semeadura (dia e ano); leitura curta: a caixa de erro e 0',
          'reads from the World Cup temporary file the 32 draw slots (club and pot) and the seeding date (day and year); a short read: the error box and 0'),
         ('world_cup_save_extra_data', 'WorldCup::save_extra_data', '0x92f150', 'world_cup.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário da Copa do Mundo as 32 vagas do sorteio e a data da semeadura, na ordem da leitura',
          'writes into the World Cup temporary file the 32 draw slots and the seeding date, in the order the load reads them')])
CAMPOS = []
INTERESSA = []
