# A troca da Segunda B espanhola com a divisão inferior e os seus quatro grupos; os movimentos da Segunda Divisão sueca
# (spa_first.cpp, swe_prm.cpp; leitor rx13, rodada 6, 25/11/2026). Só existem com a base NORMAL (a Segunda B e a
# Segunda Divisão sueca são divisões inferiores). Simulação com a Espanha e a Suécia até 2002-09-01, como os rx09: as
# viradas espanhola (SpaFirst::start_new_season, 0x84fef6/0x84fefd) e sueca (SwePrm::start_new_season, já Rust, que a
# chama por endereço) no verão de 2002; a temporada seguinte começa com os clubes movidos.
# - 0x850300 spa_first_exchange_second_division_b_clubs thiscall ret 0: até 200 clubes (de 0 a [0xae235c] - 2 x
#   [0xae2350]) com divisão [0x9cf764] e nação [0x9cf488]; a Segunda B [[0xadadfc] + 4 x [0x9cf7b8]] e as fases 0..2
#   (+0xc): um grupo de 19 clubes dá a vaga extra; cada linha +0x37 = 3: League::relegate_club_to_division 0x6831a0
#   thiscall(this, clube, [0xae23d0] + [0x9cf764] x 0x6b, 1) ret 0xc e o dword +0x60 do clube = 0. Vagas = rebaixados
#   (+1 com a extra); candidatos < vagas: caixa 0x3b7 (os rebaixamentos ficam). Senão
#   comp_util_sort_and_shuffle_top_clubs 0x4afbd0 cdecl(lista, n, vagas) e, enquanto houver vaga, club_get_linked_b_team
#   0x540a50 cdecl(clube, &é_a, 1) (o local não é iniciado; só é lido com parceiro): parceiro com é_a 0 e a divisão do
#   parceiro a Segunda B: pula; senão League::move_club_to_division 0x6830b0 thiscall(this, clube, registro, 1) ret 0xc.
#   Prólogo sub esp, 0x534 (6).
# - 0x850530 spa_first_assign_second_division_b_groups thiscall ret 0: os clubes com divisão [0x9cf7b8] (até 80); não
#   80: caixa 0x3f7. qsort 0x945333(lista, 80, 4, 0x53fd40 latitude) e qsort(lista + 20, 60, 4, 0x53fe50 longitude); o
#   dword +0x60: k < 20 o grupo [0x9cf638], < 40 [0x9cf62c], < 60 [0x9cf630], senão [0x9cf634] (registros). Prólogo
#   sub esp, 0x34c (6).
# - 0x890b40 swe_prm_apply_second_division_moves thiscall ret 0: a Segunda [[0xadadfc] + 4 x [0x9cf7c8]]; a fase 6 (a
#   repescagem, copa): cada entrada de +0xa7 (6 bytes) com +5 = 1 e o clube fora do registro da Segunda ([+4]):
#   move_club_to_division(this, clube, registro, 1); a Segunda e as fases 0..3: linhas +0x37 = 3 para [0x9cf6ac]
#   (relegate); os 12 ids em +0xee: move_club_to_division para [0x9cf7c8]. Prólogo push ecx; mov eax, [0xadadfc] (6).
# Iguais no cm0102-1080.exe. Comparadores: as trocas só a original (sorteiam e movem clubes), conferindo depois os
# rebaixados e os doze; os grupos lado a lado no dword +0x60 dos 80 clubes.
SAVE = False
LIGAS = 'ENG ESP SWE'
ATE = '2002-09-01'
BASE = 'NORMAL'
ROTINAS = [('spa_first_exchange_second_division_b_clubs', 0x850300, [0x81, 0xec, 0x34, 0x05, 0x00, 0x00]),
           ('spa_first_assign_second_division_b_groups', 0x850530, [0x81, 0xec, 0x4c, 0x03, 0x00, 0x00]),
           ('swe_prm_apply_second_division_moves', 0x890b40, [0x51, 0xa1, 0xfc, 0xad, 0xad, 0x00])]
DOCS = ('a troca da Segunda B espanhola com a divisão inferior e os seus quatro grupos; os movimentos da Segunda Divisão sueca',
        "the Spanish Segunda B's exchange with the lower division and its four groups; the Swedish Second Division's moves",
        [('spa_first_exchange_second_division_b_clubs', 'spa_first_exchange_second_division_b_clubs', '0x850300', 'spa_first.cpp', 'thiscall, ret 0',
          'regra: na virada espanhola com a Segunda B jogada, os rebaixados dos seus quatro grupos descem para a divisão inferior, e o mesmo número de clubes espanhóis da divisão inferior (um a mais quando um grupo teve 19 clubes), os mais fortes primeiro, sobem para a Segunda B; um time B cujo time principal já está na Segunda B é pulado',
          'rule: at the Spanish turnover with a played Segunda B, the clubs relegated from its four groups go down to the generic lower division, and as many Spanish clubs of the lower division (one more when a group had 19 clubs), the strongest first, go up into the Segunda B; a reserve side whose first team is already in the Segunda B is skipped'),
         ('spa_first_assign_second_division_b_groups', 'spa_first_assign_second_division_b_groups', '0x850530', 'spa_first.cpp', 'thiscall, ret 0',
          'regra: os 80 clubes da Segunda B são divididos em quatro grupos de 20 pela geografia: do sul para o norte pela cidade do estádio, os 20 mais ao sul vão para o grupo 4; os outros 60, de oeste para leste, 20 para cada um dos grupos 1, 2 e 3; com outro número que não 80 nada é dividido (uma caixa de erro)',
          'rule: the 80 Segunda B clubs are dealt into its four groups of 20 by geography: from south to north by their stadium city, the 20 southernmost go to group 4; the other 60, from west to east, 20 each to groups 1, 2 and 3; with a number other than 80 nothing is dealt (an error box)'),
         ('swe_prm_apply_second_division_moves', 'swe_prm_apply_second_division_moves', '0x890b40', 'swe_prm.cpp', 'thiscall, ret 0',
          'regra: na virada sueca com a Segunda Divisão jogável, os vencedores da repescagem de acesso sobem para ela, os rebaixados das suas quatro seções descem para a divisão inferior, e os doze clubes que ela guardou para o acesso sobem',
          'rule: at the Swedish turnover with a playable Second Division, the winners of its promotion play-off go up into it, the clubs relegated from its four sections go down to the lower division, and the twelve clubs it kept for promotion go up')])
CAMPOS = [('clube +0x60 (club::RESERVE_DIVISION): na Espanha, o registro do grupo da Segunda B (zerado no rebaixamento)',
           'club +0x60 (club::RESERVE_DIVISION): in Spain, the Segunda B group record (cleared on relegation)')]
INTERESSA = []
