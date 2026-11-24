# Os registros de carreira, os códigos de progresso e a virada da Copa do Mundo (world_cup.cpp; leitor cx17, rodada 5,
# 24/11/2026). Simulação: a Copa de 2002 (31 de maio a 30 de junho de 2002) roda em qualquer carreira; as virtuais +0x44
# (registros) e +0x68 (progresso) rodam com os resultados dela e a +0x8 (virada) uma vez depois do fim (a cobertura do
# mundo até 2002-08-01 contou 1 chamada da 0x92d330; as outras duas não estavam na lista de então). LIGAS = 'ENG' e
# ATE = '2002-08-01', como os cx16_1..4.
# - 0x92ccf0 WorldCup::update_staff_records_for_status thiscall(clube, desfecho, fase, saida, jogo, final) ret 0x18: fase
#   < 7 (com sinal): 6 -> Staff_Records::add_qualified_for_round_record 0x86bdd0 (clube, [this+4], 0, 0x14, 0x1e); 0xfe ->
#   add_failed_to_qualify_record 0x86c1d0 (clube, [this+4], 0x41c, 0x1e) e NationalTeams::clear_current_competition
#   0x775000 (push [clube+0x53]); fase 7: 0 -> o peso comp_util_scale_by_round 0x4bf850 (cdecl: 0x32, 0x64, palavra
#   +0x34 do jogo, 5) e o código da rodada seguinte da fase 7 ([[this+0xc]+0x1c]+0xa3, + 0x68 x rodada + 0x6f) em
#   0x86bdd0 (clube, [this+4], palavra +0x32, código, peso); 0xff -> add_knocked_out_record 0x86c000 (clube, [this+4],
#   palavra +0x32, palavra +0x30, peso) e o fim da campanha; 4 -> só o fim; 1 -> add_cup_winners_record 0x86a800
#   (clube, jogo, final), *saida = 4 e o fim; 2 -> 0x86b0b0, 3 -> 0x86b710, com o fim; fase > 7, nada. Devolve 0.
# - 0x92f9e0 WorldCup::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: a entrada
#   (Comp_Stats::find_club_entry 0x4a4850, sem conferir nulo); fase < 7: 1,2 -> 1 no melhor e 9 no atual e no pior, 3 ->
#   0x11, 4 -> 0x19 (tabelas de saltos 0x92faa8/b8/c8); senão como vieram. Grava +0x74, +0x73, +0x75.
# - 0x92d330 WorldCup::start_new_season thiscall() ret 0: +0x4c = 0; a virtual +8 das seis seções ([0xadadfc] + 4 x
#   [0x9cf770], [0x9cf76c], [0x9cf774], [0x9cf778], [0x9cf77c], [0x9cf780]); League::free_table 0x687970 (push 0), +0x3e
#   = 0 (palavra), +0x45 = 0, free(+0xba) zerado, Comp_Stats::archive_season_and_reset 0x4a1c50 (push 1) se houver, as
#   fases 0..=+0x30 com a virtual 0 e 1 (o vetor fica); ano + 4, +0x44 = 0xff, +0x30 = -1, +0x4b = 1;
#   WorldCup::set_up_team_list 0x92da40; a virtual +0x8c (regras): 0 -> caixa 0x332; senão +0xb1 = 0 e o retorno dela.
# Prólogos: mov al, [esp+0xc]; push esi (5 bytes), mov eax, [esp+4]; mov ecx, [ecx+8] (7) e sub esp, 0x200 (6). Iguais
# no cm0102-1080.exe (inteiras). Comparadores: os registros e a virada só a original (não rodam duas vezes), com o plano;
# o progresso lado a lado nos 3 bytes da entrada (compare_progress_codes do cx16_7).
# Usa write_progress_codes e compare_progress_codes do cx16_7: integrar depois dele (INTEGRAR_ANTES.txt).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('world_cup_update_staff_records_for_status', 0x92ccf0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('world_cup_set_club_progress', 0x92f9e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('world_cup_start_new_season', 0x92d330, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os registros de carreira, os códigos de progresso e a virada da Copa do Mundo',
        'the World Cup staff records, progress codes and turnover',
        [('world_cup_update_staff_records_for_status', 'WorldCup::update_staff_records_for_status', '0x92ccf0', 'world_cup.cpp', 'thiscall, ret 0x18',
          'regra: os registros de carreira da Copa do Mundo: nos grupos quem passa às oitavas ganha a classificação (peso 30) e quem cai o registro de não classificado (peso 30) e encerra a campanha; no mata-mata quem vence um confronto ganha a classificação para a rodada seguinte e quem perde a eliminação, as duas com peso de 50 a 100 pela rodada; campeão, vice e terceiro ganham os seus registros e encerram a campanha, como os outros eliminados; o campeão fica com a situação 4',
          'rule: the World Cup career records: in the groups a nation through to the last 16 gets the qualification (weight 30) and one going out the failed-to-qualify record (weight 30) and ends its campaign; in the knockout a tie winner gets the qualification for the next round and a loser the knock-out, both weighted 50 to 100 by round; the champion, runner-up and third get their records and end the campaign like the other eliminated nations; the champion status becomes 4'),
         ('world_cup_set_club_progress', 'WorldCup::set_club_progress', '0x92f9e0', 'world_cup.cpp', 'thiscall, ret 0x14',
          'regra: o progresso de uma seleção na Copa do Mundo: num grupo os dois primeiros são 1 no melhor e 9 no atual e no pior (as oitavas), o terceiro 17 e o quarto 25; no mata-mata os códigos ficam como vieram',
          'rule: a nation progress in the World Cup: in a group the top two are 1 at best and 9 now and at worst (the last 16), the third 17 and the fourth 25; in the knockout the codes stay as they came'),
         ('world_cup_start_new_season', 'WorldCup::start_new_season', '0x92d330', 'world_cup.cpp', 'thiscall, ret 0',
          'regra: na virada da Copa do Mundo as seis seções das eliminatórias começam a nova campanha, a Copa encerrada é limpa (tabela, datas, fases) com as estatísticas arquivadas, a edição avança quatro anos e a lista de seleções e as regras são montadas de novo',
          'rule: at the World Cup turnover the six qualifying sections start their new campaign, the finished World Cup is cleared (table, dates, stages) with its statistics archived, the edition moves on four years and the team list and the rules are built again')])
CAMPOS = []
INTERESSA = []
