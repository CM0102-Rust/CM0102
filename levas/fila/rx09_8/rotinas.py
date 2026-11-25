# A tabela da secao Norte, a virada e a fase seguinte da Segunda Divisao sueca (swe_second.cpp; leitor rx09, rodada 6,
# 25/11/2026). So existe com a base NORMAL. Simulacao com a Espanha e a Suecia ate 2002-09-01 (a virada sueca no
# comeco de 2002; o acesso e a repescagem em novembro de 2001).
# - 0x8921b0 SweSecond::create_table thiscall() ret 0 -> 0/1: +0x3e = 12, malloc 0x2c4 (caixa 0x198 e 0); os clubes com
#   divisao e reserva, a reserva a secao Norte (0x9cf5f8) e a divisao o registro da liga (ponteiro); a linha (0x687430;
#   o k empurrado como dword com restos acima do byte) e club_get_linked_b_team 0x540a50(clube, &local, 1): resposta nao
#   zero com o local 0 -> +0x37 da linha = 0xfc; o 13o: caixa 0x1a9, a divisao zerada e fim. Devolve (k == +0x3e).
# - 0x892410 SweSecond::start_new_season thiscall() ret 0: o molde do rx09_2 com as caixas 0x20c, 0x213, 0x21a e 0x220
#   e create_section_stage(k) para k = 0..4.
# - 0x8925d0 SweSecond::advance_stage thiscall() ret 0: na ultima fase a caixa 0x23e; senao +0x30 + 1; em 5 o acesso
#   (0x892a10) e a repescagem (0x892bb0).
# Prologos: sub esp, 0x208 / 0x200 (6 bytes); iguais no cm0102-1080.exe.
# Comparadores: a tabela so a original e o plano (os clubes das linhas); a virada so a original (o ano + 1); a fase
# seguinte lado a lado quando nao constroi.
# Depende do rx09_2 (domestic_clubs, grouped_league_new_season) e do rx09_7 (SWEDISH_SECOND_SOURCE).
SAVE = False
LIGAS = 'ENG ESP SWE'
ATE = '2002-09-01'
BASE = 'NORMAL'
ROTINAS = [('swe_second_create_table', 0x8921b0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('swe_second_start_new_season', 0x892410, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('swe_second_advance_stage', 0x8925d0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a tabela da seção Norte, a virada e a fase seguinte da Segunda Divisão sueca',
        'the North section table, the turnover and the next stage of the Swedish Second Division',
        [('swe_second_create_table', 'SweSecond::create_table', '0x8921b0', 'swe_second.cpp', 'thiscall, ret 0',
          'regra: a seção Norte tem 12 clubes, os da Segunda cuja seção é a Norte, na ordem do banco; um time B (ligado a um clube principal) é marcado e não pode subir; o 13º clube perde a divisão (defeito do original)',
          'rule: the North section has 12 clubs, the Second Division ones whose section is the North, in database order; a B team (linked to a senior club) is marked and cannot go up; a 13th club loses its division (defect of the original)'),
         ('swe_second_start_new_season', 'SweSecond::start_new_season', '0x892410', 'swe_second.cpp', 'thiscall, ret 0',
          'regra: na virada a Segunda sueca arquiva a temporada, apaga as fases, passa o ano e refaz as regras, a tabela da seção Norte, a ordem, os jogos, as outras cinco seções, as posições previstas e os árbitros',
          'rule: at the turnover the Swedish Second Division archives the season, deletes the stages, moves the year on and rebuilds the rules, the North table, the order, the fixtures, the other five sections, the predicted positions and the officials'),
         ('swe_second_advance_stage', 'SweSecond::advance_stage', '0x8925d0', 'swe_second.cpp', 'thiscall, ret 0',
          'regra: depois das seis seções vêm o acesso e a repescagem do rebaixamento, ao mesmo tempo',
          'rule: after the six sections come the promotion play-off and the relegation play-off, together')])
CAMPOS = []
INTERESSA = []
