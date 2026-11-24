# As eliminatorias africanas da Copa do Mundo: os inscritos e as entradas da primeira fase (wc_african_cup.cpp; leitor
# cx15, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS): as eliminatorias existem em todo jogo; as duas rodam na criacao
# (WcAfricanCup::WcAfricanCup 0x90f2fb e 0x90f3f5) e na virada (wc_african_cup_start_new_season, ja em Rust, pelo
# endereco), uma vez cada ate 2002-08-01 na cobertura do mundo.
# - 0x9106a0 WcAfricanCup::collect_entrants thiscall() ret 0 -> 0 ou 1: o corpo do WcAsiaLeague::build_entrant_list
#   (0x913f60, ja em Rust) com a Africa (chave 0x9cfa08): +0xba = 0; sem [0xb5e134], caixa da linha 0x3b7 e sem sedes;
#   HostCountry::find_hosts 0x5fa730 (ecx = [0xb5e134]; 5 pushes -> thiscall, ret 0x14; o ano + 2 em dx com lixo nos
#   bytes altos) e cada sede africana soma 1 em +0xba; o ultimo campeao (comp_util_get_latest_placed_club 0x4afd80 cdecl
#   do registro da Copa, 0) africano e nao sede soma 1; +0xb6 = 50 - +0xba; +0xb2 = malloc(0xd0) (52 entradas; sem
#   memoria, caixa da linha 0x3e6 e 0); as nacoes africanas (contador de 16 bits) com a lista abaixo de 0x34: a selecao
#   (database_get_national_team 0x53b3d0 cdecl) entra se a nacao nao e sede e a selecao nao e o campeao; menos que +0xb6
#   devolve 0 sem ordenar; senao qsort 0x945333 (cdecl) com 0x4b07d0 e 1. Prologo sub esp, 0x20c (6 bytes).
# - 0x910120 WcAfricanCup::build_first_round_entries thiscall() ret 0 -> -1 ou 1: +0x3e = 50 (+0xba 0), 48 (abaixo de
#   3, com sinal) ou 46; +0xa7 = malloc(6 x +0x3e) (sem memoria, caixa da linha 0x2c5 e -1); a entrada k pega a lista
#   de tras para frente (indice +0xb6 - 1 - k, o mais fraco primeiro) e zera a vaga da lista; +4 = 1 na primeira metade
#   (k < +0x3e / 2, truncado) e 2 no resto, +5 = 0. Prologo sub esp, 0x200 (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores lado a lado a mao: a original primeiro (a lista e a tabela dela
# ficam); a copia com os mesmos campos de antes (a lista dos inscritos restaurada para a segunda), na lista ou tabela
# propria, liberada no fim; o retorno, os campos e as listas ou tabelas iguais. Sem [0xb5e134] a copia dos inscritos nao
# roda (a caixa sairia duas vezes).
SAVE = False
ROTINAS = [('wc_african_cup_collect_entrants', 0x9106a0, [0x81, 0xec, 0x0c, 0x02, 0x00, 0x00]),
           ('wc_african_cup_build_first_round_entries', 0x910120, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as eliminatórias africanas da Copa do Mundo: os inscritos e as entradas da primeira fase',
        'the African World Cup qualifiers: the entrants and the first round entries',
        [('wc_african_cup_collect_entrants', 'WcAfricanCup::collect_entrants', '0x9106a0', 'wc_african_cup.cpp', 'thiscall, ret 0',
          'as seleções africanas inscritas nas eliminatórias: 50 vagas menos as isentas (as sedes da Copa seguinte e o último campeão, quando africanos), preenchidas com as outras seleções africanas na ordem do banco e ordenadas pela força; faltando seleções, 0',
          'the African national teams entered in the qualifiers: 50 places minus the exempt ones (the hosts of the next World Cup and the last winner, when African), filled with the other African national teams in database order and sorted by strength; short of teams, 0'),
         ('wc_african_cup_build_first_round_entries', 'WcAfricanCup::build_first_round_entries', '0x910120', 'wc_african_cup.cpp', 'thiscall, ret 0',
          'as entradas da primeira fase africana: 50, 48 ou 46 conforme as isentas, tiradas da lista dos inscritos do mais fraco para o mais forte; a primeira metade com a marca 1 e o resto com 2',
          'the African first round entries: 50, 48 or 46 by the exempt ones, taken from the entrant list from the weakest to the strongest; the first half marked 1 and the rest 2')])
CAMPOS = []
INTERESSA = []
