# A passagem de fase, os registros dos tecnicos e o progresso das selecoes na Copa das Confederacoes (fifa_confed.cpp;
# leitor cx06, rodada 5, 24/11/2026). Simulacao com as 8 ligas e ATE 2003-08-15 (a copa de 2003, jogos de 29/07 a
# 09/08/2003); 0 chamadas na cobertura do mundo ate 2002-08-01: sem a data longa o item nao prova nada (o mesmo ATE do
# cx04_6).
# - 0x58c070 FifaConfed::advance_stage thiscall() ret 0 (virtual 0x28): +0x30 >= +0x2c - 1: caixa 0x291; senao +0x30 + 1
#   e, na fase 1, create_knockout_stage 0x58c2d0 (cx06_7, pelo endereco). Prologo mov edx, [ecx+0x2c]; mov eax,
#   [ecx+0x30] (6 bytes).
# - 0x58be70 FifaConfed::update_staff_records_for_status thiscall(clube, desfecho, fase, &saida, partida, a6) ret 0x18
#   (virtual 0x44) -> 0: nos grupos (fase < 1) 6 -> Staff_Records::add_qualified_for_round_record 0x86bdd0 (0, 0x96,
#   0x19), 0xfe -> add_failed_to_qualify_record 0x86c1d0 (0x41c, 0x19); na fase 1: 0 -> qualificado com +0x32 da partida,
#   o codigo da rodada (+0x6f da rodada +0x34 na tabela +0xa3 da fase 1) e o peso comp_util_scale_by_round 0x4bf850
#   (0x32, 0x64, rodada, 2); 0xff -> add_knocked_out_record 0x86c000 (+0x32, +0x30, peso (0x32, 0x64, rodada, 5)); 1, 2 e 3
#   os registros de campeao (0x86a800, com *saida = 4 sem conferir o ponteiro), vice (0x86b0b0) e terceiro (0x86b710); 4
#   nada. Todos sobre [0xdd7f00] (thiscall). O molde do national_tournament_staff_records com outros numeros.
# - 0x58cba0 FifaConfed::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual 0x68): o molde do
#   da Oceania (group_championship_progress do cx04_5) com fase < 1 e as tabelas de salto 0x58cca0 (atual 1, 3, 5, 7),
#   0x58cc80 (melhor 1, 3, 5, 7) e 0x58cc90 (pior 2, 3, 5, 7).
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: a passagem so a original (a semifinal aloca), com o plano (+0x30 sobe um); os registros so a original
# (o retorno 0 e a saida 4 do campeao); o progresso lado a lado nos 3 bytes da entrada (compare_group_championship_progress
# do cx04_5).
# Depende do cx04_5 (group_championship_progress, compare_group_championship_progress): integrar depois dele.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('fifa_confed_advance_stage', 0x58c070, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('fifa_confed_update_staff_records_for_status', 0x58be70, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('fifa_confed_set_club_progress', 0x58cba0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('a passagem de fase, os registros dos técnicos e o progresso das seleções na Copa das Confederações',
        "the Confederations Cup's stage advance, staff records and club progress",
        [('fifa_confed_advance_stage', 'FifaConfed::advance_stage', '0x58c070', 'fifa_confed.cpp', 'thiscall, ret 0',
          'passa a Copa das Confederações à fase seguinte; ao entrar na fase 1 cria as semifinais; depois da última fase, a caixa de erro',
          'moves the Confederations Cup to its next stage; entering stage 1 it creates the semi-finals; past the last stage, the error box'),
         ('fifa_confed_update_staff_records_for_status', 'FifaConfed::update_staff_records_for_status', '0x58be70', 'fifa_confed.cpp', 'thiscall, ret 0x18',
          'registra no histórico dos técnicos o destino de uma seleção na Copa das Confederações: nos grupos, a classificação (peso 25) ou a eliminação (peso 25, que encerra as convocações); no mata-mata, a classificação por rodada (peso 50 a 100), a eliminação, e o campeão, o vice e o terceiro',
          "records a nation's fate in the Confederations Cup in the managers' histories: in the groups, qualification (weight 25) or elimination (weight 25, which ends the call-ups); in the knockout, qualification by round (weight 50 to 100), elimination, and the winners, runners-up and third place"),
         ('fifa_confed_set_club_progress', 'FifaConfed::set_club_progress', '0x58cba0', 'fifa_confed.cpp', 'thiscall, ret 0x14',
          'guarda o progresso de uma seleção na Copa das Confederações: nos grupos os códigos 1 a 4 viram 1, 3, 5 e 7 (o pior, 2, 3, 5 e 7)',
          "stores a nation's progress in the Confederations Cup: in the groups the codes 1 to 4 become 1, 3, 5 and 7 (the worst, 2, 3, 5 and 7)")])
CAMPOS = []
INTERESSA = []
