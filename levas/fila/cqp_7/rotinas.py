# A virada da temporada e a fase seguinte dos estaduais: Baiano, Centro-Oeste e Gaúcho (bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp; agente C da frente das competições,
# 05/11/2026). Os nove estaduais de pontos corridos com mata-mata (Baiano, Centro-Oeste, Gaúcho, Goiano, Norte,
# Nordeste, Pernambucano, Catarinense e Paulista; cqp_7, cqp_8, cqp_9) têm a mesma virada e a mesma fase seguinte:
# conferidas instrução a instrução (só mudam o fonte, as linhas das caixas, a rotina da tabela e o mata-mata chamado).
# Na carreira de 8 ligas até 2002-06-01 (docs/cobertura_competicoes.txt), a virada roda uma vez (o estadual de 2002) e
# a fase seguinte duas (o mata-mata de 2001 e o de 2002).
# - 0x42fe10 BraRegBahia::start_new_season e 0x42ffc0 BraRegBahia::advance_stage (o Baiano; a tabela 0x42fd10, o mata-mata 0x430050).
# - 0x431090 BraRegCentral::start_new_season e 0x431240 BraRegCentral::advance_stage (o Centro-Oeste; a tabela 0x430ef0, o mata-mata 0x4312d0).
# - 0x432370 BraRegGaucho::start_new_season e 0x432520 BraRegGaucho::advance_stage (o Gaúcho; a tabela 0x4321d0, o mata-mata 0x4325b0).
# Iguais no cm0102-1080.exe (inteiras). Convenções conferidas nos pontos de chamada da original:
# - 0x687970 (League::free_table): push 0; mov ecx, esi -> thiscall, ret 4 (mapa);
# - 0x9452ca (free): push lista; call; add esp, 4 -> cdecl;
# - 0x4a1c50 (Comp_Stats::archive_season_and_reset): push 1; mov ecx, [esi+8] -> thiscall, ret 4;
# - o destrutor das fases (virtual 0): push 1; mov ecx, fase; call [edx] -> thiscall, ret 4;
# - as virtuais +0x8c (set_up_rules) e +0x5c (set_predicted_positions): mov ecx, esi, sem push -> thiscall, ret 0;
# - a tabela do estadual, 0x6835c0 (League::create_fixtures): mov ecx, esi; call, sem push -> thiscall, ret 0;
# - 0x6827d0 (League::sort_table): push 0; mov ecx, esi -> thiscall, ret 4;
# - 0x79cee0 (OfficialsManager::renew_officials): push [esi+4]; mov ecx, [0xb74340] -> thiscall, ret 4 (o eax dela é o
#   que a virada devolve);
# - o mata-mata (cqn_7..9): call direto com o ecx da liga -> thiscall, ret 0;
# - a caixa de erro (0x944cff, 0x90d130, 0x944c9f, 0x5e8290): aviso_em, como nas levas anteriores.
# Comparadores: só a original, com o plano (a virada libera e aloca, sorteia jogos e árbitros: confere o ano + 1; a
# fase seguinte monta o mata-mata: confere +0x30 + 1).
# Determinismo: nada de pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqp7_virada_do_baiano', 0x42fe10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp7_fase_seguinte_do_baiano', 0x42ffc0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqp7_virada_do_centro_oeste', 0x431090, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp7_fase_seguinte_do_centro_oeste', 0x431240, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('cqp7_virada_do_gaucho', 0x432370, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqp7_fase_seguinte_do_gaucho', 0x432520, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('a virada da temporada e a fase seguinte dos estaduais: Baiano, Centro-Oeste e Gaúcho',
        'the season turnover and the next stage of the state leagues: Bahia, Centre-West and Rio Grande do Sul',
        [('cqp7_virada_do_baiano', 'BraRegBahia::start_new_season', '0x42fe10', 'bra_reg_bahia.cpp', 'thiscall, ret 0', 'A virada do Baiano: guarda a temporada nas estatísticas, apaga a tabela e as fases da anterior, passa ao ano seguinte e refaz as regras, a tabela com os clubes do estadual, os jogos (pontos corridos) e a ordem da tabela; depois as posições previstas e a renovação dos árbitros do estadual', 'The Bahia turnover: archives the season in the statistics, clears the previous table and stages, moves to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then the predicted positions and the renewal of the state referees'),
         ('cqp7_fase_seguinte_do_baiano', 'BraRegBahia::advance_stage', '0x42ffc0', 'bra_reg_bahia.cpp', 'thiscall, ret 0', 'A fase seguinte do Baiano: os pontos corridos são a fase −1; quando acabam, a fase passa a 0 e os 4 primeiros vão às semifinais (1º × 4º, 2º × 3º); depois da última fase é erro (a caixa)', 'The Bahia next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)'),
         ('cqp7_virada_do_centro_oeste', 'BraRegCentral::start_new_season', '0x431090', 'bra_reg_central.cpp', 'thiscall, ret 0', 'A virada do Centro-Oeste: guarda a temporada nas estatísticas, apaga a tabela e as fases da anterior, passa ao ano seguinte e refaz as regras, a tabela com os clubes do estadual, os jogos (pontos corridos) e a ordem da tabela; depois as posições previstas e a renovação dos árbitros do estadual', 'The Centre-West turnover: archives the season in the statistics, clears the previous table and stages, moves to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then the predicted positions and the renewal of the state referees'),
         ('cqp7_fase_seguinte_do_centro_oeste', 'BraRegCentral::advance_stage', '0x431240', 'bra_reg_central.cpp', 'thiscall, ret 0', 'A fase seguinte do Centro-Oeste: os pontos corridos são a fase −1; quando acabam, a fase passa a 0 e os 8 primeiros vão às quartas de final (1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º); depois da última fase é erro (a caixa)', 'The Centre-West next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 8 go to the quarter-finals (1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th); past the last stage it is an error (the box)'),
         ('cqp7_virada_do_gaucho', 'BraRegGaucho::start_new_season', '0x432370', 'bra_reg_gaucho.cpp', 'thiscall, ret 0', 'A virada do Gaúcho: guarda a temporada nas estatísticas, apaga a tabela e as fases da anterior, passa ao ano seguinte e refaz as regras, a tabela com os clubes do estadual, os jogos (pontos corridos) e a ordem da tabela; depois as posições previstas e a renovação dos árbitros do estadual', 'The Rio Grande do Sul turnover: archives the season in the statistics, clears the previous table and stages, moves to the next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then the predicted positions and the renewal of the state referees'),
         ('cqp7_fase_seguinte_do_gaucho', 'BraRegGaucho::advance_stage', '0x432520', 'bra_reg_gaucho.cpp', 'thiscall, ret 0', 'A fase seguinte do Gaúcho: os pontos corridos são a fase −1; quando acabam, a fase passa a 0 e os 4 primeiros vão às semifinais (1º × 4º, 2º × 3º); depois da última fase é erro (a caixa)', 'The Rio Grande do Sul next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)')])
CAMPOS = [('liga +0x4c: [inferência] marca da temporada; valor; grava 0 (virada)', 'league +0x4c: [inference] season flag; value; writes 0 (turnover)'),
          ('liga +0xba: lista auxiliar; ponteiro; libera e zera (virada)', 'league +0xba: auxiliary list; pointer; frees and clears (turnover)'),
          ('liga +8: o bloco de estatísticas (Comp_Stats); ponteiro; lê (virada)', 'league +8: the statistics block (Comp_Stats); pointer; reads (turnover)'),
          ('liga +0xc: as fases (vetor de ponteiros); ponteiro; lê e destrói (virada)', 'league +0xc: the stages (pointer array); pointer; reads and destroys (turnover)'),
          ('liga +0x2c: o número de fases; valor; lê (fase seguinte)', 'league +0x2c: the number of stages; value; reads (next stage)'),
          ('liga +0x30: a fase atual (−1 = pontos corridos); valor; lê e grava', 'league +0x30: the current stage (−1 = round robin); value; reads and writes'),
          ('liga +0x40: o ano da temporada; valor; lê e grava + 1 (virada)', 'league +0x40: the season year; value; reads and writes + 1 (turnover)'),
          ('liga +4: o registro da competição no banco; ponteiro; lê (árbitros)', 'league +4: the competition record in the database; pointer; reads (referees)'),
          ('[0xb74340] o gerente dos árbitros; ponteiro; lê', '[0xb74340] the officials manager; pointer; reads')]
INTERESSA = []
