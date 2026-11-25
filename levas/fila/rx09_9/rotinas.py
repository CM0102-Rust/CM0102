# As secoes, o acesso e a repescagem do rebaixamento da Segunda Divisao sueca (swe_second.cpp; leitor rx09, rodada 6,
# 25/11/2026). So existe com a base NORMAL. Simulacao com a Espanha e a Suecia ate 2002-09-01: as secoes na criacao e
# na virada; o acesso e a repescagem em novembro de 2001.
# - 0x892670 SweSecond::create_section_stage thiscall(secao) ret 4: +0x3e lido antes; a virtual +0x3c com o dword do
#   argumento; a chave (0 -> 0x9cf5fc, 1 -> 0x9cf604, 2 -> 0x9cf600, 3 -> 0x9cf608, outro -> 0x9cf5f4); malloc 4 x +0x3e
#   (caixa 0x272); os clubes com divisao = o registro da liga e reserva da secao; o excedente: caixa 0x283 e a divisao
#   zerada; faltando: caixa 0x290; League_Stage com os lugares da liga (+0xbe), importancia +0x51 e bits +0xd9; sem a
#   fase, caixa 0x29b; senao free da lista e da tabela, a marca 0xfc dos times B (0x540a50) nas linhas da fase e +0x30.
# - 0x892a10 SweSecond::create_promotion_play_off_stage thiscall() ret 0: o primeiro de status 1 de cada secao (-1..4)
#   num local de 6; Cup_Stage(this, 6, lista, rodadas, registro, tabela, ano, 5, 1, extra, 0x14, 0, 0, 0, 0) em
#   +0xc[5]; caixa 0x2e0; free da tabela, Cup::draw_round(0) e +0x30 = 5.
# - 0x892bb0 SweSecond::create_relegation_play_off_stage thiscall() ret 0: os clubes da Lower Division sueca (nacao e
#   divisao pelos ids 0x9cf4a8 e 0x9cf6ac; a conta de clubes calculada uma vez; ate 400 num local); menos de 24: caixa
#   0x313; comp_util_sort_and_shuffle_top_clubs 0x4afbd0(lista, n, 0x18); os ids dos 12 primeiros em +0xee; rep movsd
#   dos 12 seguintes e os 6 vices (status 2) num local de 18; Cup_Stage(this, 0x12, ..., 6, 1, extra, 0x14, 0, 0, 0,
#   &sementes) com o local {2 x 12, 1 x 6}; caixa 0x33f; Cup::set_up_round(0), free da tabela e +0x30 = 6.
# Prologos push -1; push imm32 (7 bytes) e mov eax, fs:[0] (6 bytes); iguais no cm0102-1080.exe.
# Comparadores: so a original nas tres; os planos conferidos depois (os clubes das fases, os vencedores, os vices, os
# ids guardados e a fase atual).
# Depende do rx09_8 (swedish_second_section_clubs, mark_b_team_row), do rx09_7 (SWEDISH_SECOND_SOURCE), do rx09_3
# (league_cup_stage, cup_stage_clubs, compare_league_cup_stage), do rx09_2 (domestic_clubs) e do cx14_6.
SAVE = False
LIGAS = 'ENG ESP SWE'
ATE = '2002-09-01'
BASE = 'NORMAL'
ROTINAS = [('swe_second_create_section_stage', 0x892670, [0x6a, 0xff, 0x68, 0xfe, 0x31, 0x96, 0x00]),
           ('swe_second_create_promotion_play_off_stage', 0x892a10, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('swe_second_create_relegation_play_off_stage', 0x892bb0, [0x6a, 0xff, 0x68, 0x3e, 0x32, 0x96, 0x00])]
DOCS = ('as seções, o acesso e a repescagem do rebaixamento da Segunda Divisão sueca',
        'the sections, the promotion play-off and the relegation play-off of the Swedish Second Division',
        [('swe_second_create_section_stage', 'SweSecond::create_section_stage', '0x892670', 'swe_second.cpp', 'thiscall, ret 4',
          'regra: cada uma das outras cinco seções tem os clubes da Segunda daquela seção na ordem do banco, tantos quanto a seção Norte; os times B são marcados e não sobem; o excedente perde a divisão (defeito do original)',
          'rule: each of the other five sections has the Second Division clubs of that section in database order, as many as the North section; B teams are marked and do not go up; any extra club loses its division (defect of the original)'),
         ('swe_second_create_promotion_play_off_stage', 'SweSecond::create_promotion_play_off_stage', '0x892a10', 'swe_second.cpp', 'thiscall, ret 0',
          'regra: os seis campeões de seção jogam o acesso em três confrontos sorteados de ida e volta',
          'rule: the six section winners play the promotion play-off in three drawn two-leg ties'),
         ('swe_second_create_relegation_play_off_stage', 'SweSecond::create_relegation_play_off_stage', '0x892bb0', 'swe_second.cpp', 'thiscall, ret 0',
          'regra: dos clubes da divisão inferior sueca, os 24 tirados entre os 72 de maior reputação: os doze primeiros ficam guardados para subir e os doze seguintes jogam a repescagem, a que os seis vices de seção se juntam na segunda rodada',
          'rule: of the Swedish lower division clubs, 24 are drawn among the 72 of highest reputation: the first twelve are kept to go up and the next twelve play the relegation play-off, which the six section runners-up join in the second round')])
CAMPOS = [('Segunda sueca +0xee: os ids dos doze clubes da divisão inferior que sobem (12 dwords, gravados no save) [inferência]', 'Swedish Second Division +0xee: the ids of the twelve lower division clubs going up (12 dwords, saved) [inference]')]
INTERESSA = []
