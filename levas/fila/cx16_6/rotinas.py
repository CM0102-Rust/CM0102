# As listas de inscritas das eliminatórias da CONCACAF e da Oceania no jogo salvo (wc_concacaf_cup.cpp,
# wc_oceania_league.cpp; leitor cx16, rodada 5, 24/11/2026). Item do SAVE (CARGA): o arquivo temporário de cada
# competição (Cup::save_cup_to_disk / Cup::load_from_disk 0x51fc00 e League::save_to_temp_file /
# League::load_from_temp_file 0x687b10) chama as virtuais +0x24 (gravar) e +0x20 (ler) com o FILE aberto; as seções
# existem em qualquer carreira (a CONCACAF dormente com a lista de 2000, a Oceania com a sua).
# - 0x917aa0 WcConcacafCup::load_extra_data thiscall(arquivo) ret 4 -> 0/1: fread 0x945bfe (cdecl) de +0xba, +0xbb e
#   +0xbc (1 byte cada; a soma != 3: caixa 0x75a e 0), de +0xb6 (4; caixa 0x762), +0xb2 = calloc(0x23, 4) (0x945501;
#   null: caixa 0x76a), fread(lista, 4, contagem) (caixa 0x772); os índices viram clubes (0xae23b4 + 0x245 x i; -1 ->
#   0); 1. A lista tem 35 vagas qualquer que seja a contagem lida (sempre 35 num save).
# - 0x917cd0 WcConcacafCup::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 (cdecl) de +0xba, +0xbb, +0xbc, +0xb6,
#   a lista como índices (o id do clube, +0; nulo -> -1) e de volta a ponteiros; devolve a contagem.
# - 0x91fd20 WcOceaniaLeague::load_extra_data thiscall(arquivo) ret 4 -> 0/1: fread de +0xee (1; caixa 0x4b8), de +0xf3
#   (4; caixa 0x4c0), +0xef = malloc(4 x contagem) (caixa 0x4c8), os índices (caixa 0x4d0) viram clubes; 1.
# Prólogos: sub esp, 0x200 (6 bytes) e push ebx; push esi; push edi; mov edi, [esp+0x10] (7 bytes). Iguais no
# cm0102-1080.exe. Comparadores: as leituras lado a lado com o arquivo de volta à posição (ftell/fseek), a cópia numa
# lista própria (liberada; o estado da original devolvido): o retorno, os campos, as entradas da lista e a posição final;
# a gravação só a original (a prova é a do save).
SAVE = True
CARGA = True
ROTINAS = [('wc_concacaf_cup_load_extra_data', 0x917aa0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wc_concacaf_cup_save_extra_data', 0x917cd0, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('wc_oceania_league_load_extra_data', 0x91fd20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as listas de inscritas das eliminatórias da CONCACAF e da Oceania no jogo salvo',
        'the CONCACAF and Oceania qualifying entrant lists in the saved game',
        [('wc_concacaf_cup_load_extra_data', 'WcConcacafCup::load_extra_data', '0x917aa0', 'wc_concacaf_cup.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário das eliminatórias da CONCACAF as três contagens de isentas, a contagem de inscritas e a lista delas (os índices viram clubes); leitura curta: a caixa de erro e 0',
          'reads from the CONCACAF qualifying temporary file the three exempt counts, the entrant count and their list (the indices become clubs); a short read: the error box and 0'),
         ('wc_concacaf_cup_save_extra_data', 'WcConcacafCup::save_extra_data', '0x917cd0', 'wc_concacaf_cup.cpp', 'thiscall, ret 4',
          'grava no arquivo temporário das eliminatórias da CONCACAF as três contagens de isentas, a contagem de inscritas e a lista delas como índices de clube, na ordem da leitura',
          'writes into the CONCACAF qualifying temporary file the three exempt counts, the entrant count and their list as club indices, in the order the load reads them'),
         ('wc_oceania_league_load_extra_data', 'WcOceaniaLeague::load_extra_data', '0x91fd20', 'wc_oceania_league.cpp', 'thiscall, ret 4',
          'lê do arquivo temporário das eliminatórias da Oceania a contagem de isentas, a contagem de inscritas e a lista delas (os índices viram clubes); leitura curta: a caixa de erro e 0',
          'reads from the Oceania qualifying temporary file the exempt count, the entrant count and their list (the indices become clubs); a short read: the error box and 0')])
CAMPOS = []
INTERESSA = []
