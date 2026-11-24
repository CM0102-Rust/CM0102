# Os dados proprios das eliminatorias asiaticas da Copa do Mundo no jogo salvo (wc_asia_league.cpp; leitor cx15, rodada
# 5, 24/11/2026). Item do SAVE (grava e carrega): as eliminatorias existem em todo jogo, entao as duas rodam em toda
# gravacao e em toda carga (o FeriasTesteSave serve).
# - 0x914200 WcAsiaLeague::load_extra_data thiscall(arquivo) ret 4: fread 0x945bfe (cdecl) de +0xee (1 byte; curto,
#   caixa da linha 0x646 e 0) e +0xf3 (4; 0x64e); com +0xf3 != 0: +0xef = malloc(4 x +0xf3) (a lista antiga nao e
#   liberada; sem memoria, 0x659), fread dos ids (curto, 0x661) e cada id != -1 vira o registro do clube ([0xae23b4] +
#   0x245 x id), -1 vira 0; com 0 a lista fica. Devolve 1. Prologo sub esp, 0x200 (6 bytes).
# - 0x914410 WcAsiaLeague::save_extra_data thiscall(arquivo) ret 4: o mesmo corpo do WcAfricanCup::save_extra_data
#   (cx15_12, conferido instrucao a instrucao) com +0xee, +0xf3 e +0xef: save_qualifier_list. Prologo push ebx; mov
#   ebx, [esp+8]; push esi (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (leem e gravam o jogo); a carga com o plano (1 e a
# lista de registros de clube), a gravacao com a lista igual antes e depois.
SAVE = True
CARGA = True
ROTINAS = [('wc_asia_league_load_extra_data', 0x914200, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('wc_asia_league_save_extra_data', 0x914410, [0x53, 0x8b, 0x5c, 0x24, 0x08, 0x56])]
DOCS = ('os dados próprios das eliminatórias asiáticas no jogo salvo',
        'the Asian qualifiers\' own data in the saved game',
        [('wc_asia_league_load_extra_data', 'WcAsiaLeague::load_extra_data', '0x914200', 'wc_asia_league.cpp', 'thiscall, ret 4',
          'lê do jogo salvo o número de seleções isentas, o de inscritas e, havendo inscritas, a lista delas (pelos ids, numa lista nova)',
          'reads from the saved game the number of exempt national teams, the number of entrants and, when there are any, their list (by id, into a new list)'),
         ('wc_asia_league_save_extra_data', 'WcAsiaLeague::save_extra_data', '0x914410', 'wc_asia_league.cpp', 'thiscall, ret 4',
          'grava no jogo salvo o número de seleções isentas, o de inscritas e a lista delas pelos ids',
          'writes to the saved game the number of exempt national teams, the number of entrants and their list by id')])
CAMPOS = []
INTERESSA = []
