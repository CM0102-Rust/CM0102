# As honras e a virada da NSL australiana (aus_nsl.cpp). Leitor mwf (12/11/2026), rodada 2 (o mundo até 2002-08-01;
# docs/cobertura_mundo.txt: cada uma 1 vez, no fim da temporada australiana de 2001/02 e na virada).
# - 0x411d90 (AusNsl::record_season_honours): na tabela da final (fase 1) o status 1 é o campeão e o 2 o vice; o
#   terceiro do histórico é o líder da fase regular (a primeira linha da tabela), e o extra 0.
# - 0x411de0 (AusNsl::start_new_season): o molde das viradas sem destruir as fases nem gravar +0x30 [inferência: as
#   regras da NSL refazem as fases], com a tabela dos 13 clubes da Austrália (0x411b70).
# Convenções: virtuais (mov ecx; call [eax+n], sem push) -> thiscall, ret 0; dentro: 0x4afce0 (5 pushes; add esp, 0x14
# -> cdecl).
# Comparadores: as honras lado a lado sobre o histórico das competições (só a original quando o histórico precisa
# crescer); a virada só a original, com o ano +1.
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('aus_nsl_record_season_honours', 0x411d90, [0x8b, 0x41, 0x0c, 0x56, 0x57]),
           ('aus_nsl_start_new_season', 0x411de0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as honras e a virada da NSL australiana',
        'the honours and the season turnover of the Australian NSL',
        [('aus_nsl_record_season_honours', 'AusNsl::record_season_honours', '0x411d90', 'aus_nsl.cpp', 'thiscall, ret 0',
          'regra: o campeão da NSL é o vencedor da grande final, o vice o perdedor dela, e o terceiro do histórico é o líder da fase regular',
          'rule: the NSL champion is the grand final winner, the runner-up its loser, and the third in the history is the regular season leader'),
         ('aus_nsl_start_new_season', 'AusNsl::start_new_season', '0x411de0', 'aus_nsl.cpp', 'thiscall, ret 0',
          'regra: a virada da temporada da NSL arquiva a temporada, limpa a tabela e as datas, passa ao ano seguinte e refaz as regras, a tabela com os 13 clubes australianos, os jogos e a ordem; depois as posições previstas, as expectativas da diretoria e a renovação dos árbitros; não mexe nas fases',
          'rule: the NSL season turnover archives the season, clears the table and the dates, moves to the next year and rebuilds the rules, the table with the 13 Australian clubs, the fixtures and the order; then the predicted positions, the board expectations and the referee renewal; it leaves the stages alone')])
CAMPOS = [('liga +0x4c, +0xb1, +0xba, +0x40 (a virada); histórico das competições (as honras); valor/ponteiro; grava',
           'league +0x4c, +0xb1, +0xba, +0x40 (the turnover); competition history (the honours); value/pointer; writes')]
