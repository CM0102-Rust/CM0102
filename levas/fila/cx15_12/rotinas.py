# Os dados proprios das eliminatorias africanas da Copa do Mundo no jogo salvo (wc_african_cup.cpp; leitor cx15, rodada
# 5, 24/11/2026). Item do SAVE (grava e carrega): as eliminatorias existem em todo jogo, entao as duas rodam em toda
# gravacao e em toda carga (o FeriasTesteSave serve).
# - 0x910f00 WcAfricanCup::load_extra_data thiscall(arquivo) ret 4: fread 0x945bfe (cdecl) de +0xba (1 byte; curto,
#   caixa da linha 0x4e3 e 0) e +0xb6 (4; 0x4eb); +0xb2 = malloc(0xd0) (a lista antiga nao e liberada; sem memoria,
#   0x4f3); fread de +0xb6 ids (curto, 0x4fb); cada id != -1 vira o registro do clube ([0xae23b4] + 0x245 x id), -1
#   vira 0. Devolve 1. Prologo sub esp, 0x200 (6 bytes).
# - 0x911110 WcAfricanCup::save_extra_data thiscall(arquivo) ret 4: fwrite 0x945ce6 (cdecl, sem conferir) de +0xba (1) e
#   +0xb6 (4); a lista vira ids no lugar (o id do registro, nulo -1), fwrite das +0xb6 entradas e os ids voltam a
#   registros; devolve +0xb6 (o eax). Prologo push ebx; mov ebx, [esp+8]; push esi (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (leem e gravam o jogo); a carga com o plano (1 e a
# lista de registros de clube), a gravacao com a lista igual antes e depois.
# Os ajudantes QualifierList, qualifier_list_from_ids, save_qualifier_list e qualifier_list_is_clubs sao usados pelo
# cx15_13 (as asiaticas).
SAVE = True
CARGA = True
ROTINAS = [('wc_african_cup_load_extra_data', 0x910f00, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wc_african_cup_save_extra_data', 0x911110, [0x53, 0x8b, 0x5c, 0x24, 0x08, 0x56])]
DOCS = ('os dados próprios das eliminatórias africanas no jogo salvo',
        'the African qualifiers\' own data in the saved game',
        [('wc_african_cup_load_extra_data', 'WcAfricanCup::load_extra_data', '0x910f00', 'wc_african_cup.cpp', 'thiscall, ret 4',
          'lê do jogo salvo o número de seleções isentas, o de vagas e a lista das seleções inscritas (pelos ids, numa lista nova de 52 vagas)',
          'reads from the saved game the number of exempt national teams, the places and the list of the entered national teams (by id, into a new 52-place list)'),
         ('wc_african_cup_save_extra_data', 'WcAfricanCup::save_extra_data', '0x911110', 'wc_african_cup.cpp', 'thiscall, ret 4',
          'grava no jogo salvo o número de seleções isentas, o de vagas e a lista das seleções inscritas pelos ids',
          'writes to the saved game the number of exempt national teams, the places and the list of the entered national teams by id')])
CAMPOS = []
INTERESSA = []
