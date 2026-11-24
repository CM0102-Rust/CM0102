# O mata-mata da Eurocopa, a passagem de fase e as honras da temporada (euro_champ.cpp; leitor cx05, rodada 5,
# 24/11/2026). Simulacao com as 8 ligas e ATE 2004-07-15 (as quartas em 22 de junho de 2004, a final em 30 de junho;
# 0 chamadas na cobertura do mundo ate 2002-08-01).
# - 0x57bb40 EuroChamp::create_knockout_stage thiscall() ret 0 (do advance_stage na fase 3): RulingBody::
#   reset_bookings_for_club 0x7e9180 (push 0; mov ecx, corpo -> thiscall, ret 4) no corpo de [[0xae28f0]] pela regra
#   disciplinar +0x50 (com sinal), malloc, League::get_table_row 0x6826d0 (push k; mov ecx -> thiscall, ret 4),
#   nations_knockout_stage do cx05_3 (Cup_Stage::Cup_Stage 0x522e00 com a fase 3 e o argumento 12 = 1), Cup::
#   set_up_round 0x51c800 (push 0 -> thiscall, ret 4). Prologo mov eax, fs:[0] (6 bytes).
# - 0x57b880 EuroChamp::advance_stage thiscall() ret 0 (virtual +0x28): con_merc_cup_advance_stage_cqs11_next_stage
#   com o mata-mata em 3 (0x57bb40 pelo endereco). Prologo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30] (6 bytes).
# - 0x57b730 EuroChamp::record_season_honours thiscall() ret 0 (virtual +0x30): a virtual +0x30 da fase [+0xc][3]
#   (thiscall, ret 0) e NationalTeams::clear_competition_for_comp_clubs 0x775420 (push competicao; mov ecx, [0xb74318]
#   -> thiscall, ret 4). Prologo push esi; mov esi, ecx; mov eax, [esi+0xc] (6 bytes).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Depende do cx05_3 (nations_knockout_stage): integrar depois dele.
# Comparadores: so a original nas tres (alocam e sorteiam, mexem no historico e nas competicoes das selecoes), com o
# plano: o mata-mata em [+0xc][3] e +0x30 = 3; +0x30 sobe um.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-07-15'
ROTINAS = [('euro_champ_create_knockout_stage', 0x57bb40, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('euro_champ_advance_stage', 0x57b880, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('euro_champ_record_season_honours', 0x57b730, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x0c])]
DOCS = ('o mata-mata e as honras da Eurocopa', 'the European Championship knockout and honours',
        [('euro_champ_create_knockout_stage', 'EuroChamp::create_knockout_stage', '0x57bb40', 'euro_champ.cpp', 'thiscall, ret 0',
          'regra: as quartas-de-final da Eurocopa têm os dois primeiros de cada grupo, na ordem 1º do grupo 2, 2º do 1, 1º do 3, 2º do 4, 1º do 1, 2º do 2, 1º do 4 e 2º do 3 (os confrontos em pares nessa ordem [inferência]); antes, os cartões da competição são zerados',
          'rule: the European Championship quarter-finals have the two best of each group, in the order group 2 winner, group 1 runner-up, group 3 winner, group 4 runner-up, group 1 winner, group 2 runner-up, group 4 winner and group 3 runner-up (the ties in pairs in that order [inference]); before that the competition\'s bookings are cleared'),
         ('euro_champ_advance_stage', 'EuroChamp::advance_stage', '0x57b880', 'euro_champ.cpp', 'thiscall, ret 0',
          'regra: a Eurocopa passa de fase quando a atual acaba; ao chegar à fase 3 (depois dos quatro grupos) monta as quartas-de-final; passar da última fase é erro',
          'rule: the European Championship moves on one stage when the current one ends; on reaching stage 3 (after the four groups) it builds the quarter-finals; going past the last stage is an error'),
         ('euro_champ_record_season_honours', 'EuroChamp::record_season_honours', '0x57b730', 'euro_champ.cpp', 'thiscall, ret 0',
          'regra: as honras da temporada da Eurocopa (campeão e vice) são as do mata-mata, a quarta fase; depois as seleções da competição deixam de tê-la como competição atual',
          'rule: the European Championship season honours (winner and runner-up) are those of the knockout, the fourth stage; then the competition\'s national teams no longer have it as their current competition')])
CAMPOS = []
INTERESSA = []
