# O calendario da Eerste Divisie (Primeira Divisao holandesa), as fases dos seus quatro periodos e os vencedores dos
# periodos (hol_first.cpp; leitor sbE, rodada 5, 24/11/2026). Simulacao com a Holanda (as ligas e a data dos itens mwc):
# o calendario na criacao do jogo (fase -1, pelo set_up_rules), na virada de 2002 e nos grupos da repescagem (fases 4 e
# 5, maio de 2002); as fases dos periodos nascem com os resultados (HolFirst::add_result 0x5f65b0, mwc_24); a atualizacao
# a cada atualizacao da tabela (virtual 0x94).
# - 0x5f5790 HolFirst::create_schedule thiscall(fase, &rodadas, &extra, &e_liga) ret 0x10 (virtual 0x3c): -1, 34 rodadas
#   de 14/ago a 14/mai (extra 0, caixa 0xad); 4 e 5, 6 rodadas de 17/mai a 3/jun (extra 0x418/0x419, caixa 0x171); datas
#   0x68a160 e faixas 0x68a1c0 (cdecl, intercaladas), malloc 0x944e46. Outra fase: 0 sem escrever. Prologo mov al,
#   [esp+4]; sub esp, 0x200 (10 bytes). Tabelas tiradas executando a original no unicorn (emula.py do cx14).
#   Defeito do original: a rodada 13 em 30/nov (mes 10 por 9), antes de 6/nov.
# - 0x5f6ad0 HolFirst::create_period_stage thiscall(periodo) ret 4: malloc (caixa 0x300) com os clubes da tabela;
#   League_Stage 0x68b100 (1 turno, sem tabela de rodadas, lugares 0,1,0,0, rotulo 0x439..0x43c, importancia 0, bits
#   0x22, 10 rodadas no terceiro periodo e 8 nos outros, modo 2) na vaga do periodo (sem teste); os vencedores de periodos
#   anteriores (status 1 na tabela) marcados 0xfe na fase nova (League::get_table_row 0x6826d0, duas vezes); +0x30 =
#   periodo; free. Prologo push -1; push imm32 (7 bytes).
# - 0x5f6d20 HolFirst::update_positions_and_status thiscall() ret 0 (virtual 0x94): League::update_positions_and_status
#   0x684cd0; o primeiro periodo p sem marca (+0xee..+0xf1); com +0x30 >= p - 1, o primeiro status 1 da fase p - 1: a
#   primeira linha do clube na tabela com status fora de 0 e 5 vira 1, HolFirst::add_period_winner_news 0x5f7330
#   (thiscall(clube, p), ret 8) e virtual 0x68 (clube, +0x44, 2, 2, 7); 0xfe nas fases p.. +0x30; a marca do periodo.
#   Prologo sub esp, 8; push ebx; push ebp (5 bytes).
# Depende do cx08_2 (compare_staged_schedule, ScheduleTable) e do cx08_16 (belgian_progress_virtual): integrar depois
# deles. O cx10_15 porta a 0x5f7330 (aqui chamada pelo endereco).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: o calendario lado a lado; a fase e a atualizacao so a original.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('hol_first_create_schedule', 0x5f5790, [0x8a, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('hol_first_create_period_stage', 0x5f6ad0, [0x6a, 0xff, 0x68, 0x9e, 0xad, 0x95, 0x00]),
           ('hol_first_update_positions_and_status', 0x5f6d20, [0x83, 0xec, 0x08, 0x53, 0x55])]
DOCS = ('o calendário da Eerste Divisie, as fases dos períodos e os vencedores dos períodos',
        'the Eerste Divisie schedule, its period stages and the period winners',
        [('hol_first_create_schedule', 'HolFirst::create_schedule', '0x5f5790', 'hol_first.cpp', 'thiscall, ret 0x10',
          'o calendário da Eerste Divisie: a temporada em 34 rodadas de 14 de agosto a 14 de maio do ano seguinte, quase todas com uma faixa de TV; cada grupo da repescagem (fases 4 e 5) em 6 rodadas de 17 de maio a 3 de junho',
          'the Eerste Divisie schedule: the season in 34 rounds from 14 August to 14 May of the next year, nearly all with one TV slot; each play-off group (stages 4 and 5) in 6 rounds from 17 May to 3 June'),
         ('hol_first_create_period_stage', 'HolFirst::create_period_stage', '0x5f6ad0', 'hol_first.cpp', 'thiscall, ret 4',
          'cada um dos quatro períodos da Eerste Divisie é uma fase com os 18 clubes (8 rodadas, 10 no terceiro); criada, vira a fase atual, e os vencedores de períodos anteriores ficam de fora dela',
          'each of the four Eerste Divisie periods is a stage with all 18 clubs (8 rounds, 10 in the third); once created it is the current stage, and earlier period winners are out of it'),
         ('hol_first_update_positions_and_status', 'HolFirst::update_positions_and_status', '0x5f6d20', 'hol_first.cpp', 'thiscall, ret 0',
          'depois da atualização comum, o vencedor do próximo período decidido vai para a repescagem (com a notícia e o progresso) e fica de fora dos períodos seguintes',
          'after the usual update, the winner of the next decided period goes to the play-off (with the news and the progress) and is out of the following periods')])
CAMPOS = []
INTERESSA = []
