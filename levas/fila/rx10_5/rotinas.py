# O calendario, a fase Sul e a lista de clubes da Regionalliga alema (ger_regional.cpp; leitor rx10, rodada 6,
# 25/11/2026). Simulacao com a Alemanha e a base NORMAL (ver rx10_4). O calendario e a fase Sul rodam na criacao do
# jogo (2001) e na virada de 2002; a lista de clubes quando o jogo a pede (League::get_club_array, virtual).
# - 0x5dcef0 GerRegional::create_schedule thiscall(fase, &rodadas, &extra, &liga) ret 0x10: fase >= 1 (com sinal): 0;
#   senao 38 rodadas na fase -1 em 2000 e 34 nas outras; liga = 1; malloc 0x41 x n (caixa 0xb1, sem gravar rodadas
#   nem extra); fase -1 extra 0x453 e as datas Norte (as quatro marcadas so em 2000), outras extra 0x456 e as datas
#   Sul; o numero de rodadas e um jogo vazio por rodada no fim. Tiradas por emulacao (unicorn, sbE/emula.py) nas
#   fases -1, 0 e 1 e nos anos 2000 e 2001. Prologo sub esp, 0x200 (6 bytes).
# - 0x5ddc50 GerRegional::create_south_stage thiscall(fase) ret 4: create_section_stage (o corpo do
#   PorSecondB::create_zone_stage 0x7d2870, conferido instrucao a instrucao) com 18 clubes, a divisao comparada com o
#   proprio registro (ponteiro), a secao Sul 0x9cf560, as vagas de um local [2, 0, 0, 4] e os times B marcados (0xfc);
#   caixas 0x1f2, 0x203, 0x210 e 0x21b. Prologo push -1; push 0x95a66e (7 bytes).
# - 0x5ddff0 GerRegional::get_club_array thiscall(&lista, &n) ret 8: n = 36 (37 em 2000), a Norte e depois a Sul
#   (caixa 0x24e). Prologo sub esp, 0x200 (6 bytes).
# Depende do rx10_4 (os auxiliares das ligas divididas). Iguais no cm0102-1080.exe. Comparadores: o calendario e a lista
# lado a lado; a fase so a original com o plano.
# Defeitos do original: na Sul as rodadas 21 e 22 caem no mesmo dia (25/fev); em 2000 a rodada 23 da Norte e 10/jun
# entre 4 e 11/fev [inferencia: 10/fev].
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
BASE = 'NORMAL'
ROTINAS = [('ger_regional_create_schedule', 0x5dcef0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ger_regional_create_south_stage', 0x5ddc50, [0x6a, 0xff, 0x68, 0x6e, 0xa6, 0x95, 0x00]),
           ('ger_regional_get_club_array', 0x5ddff0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o calendário, a fase Sul e a lista de clubes da Regionalliga alemã',
        'the German Regionalliga schedule, South stage and club array',
        [('ger_regional_create_schedule', 'GerRegional::create_schedule', '0x5dcef0', 'ger_regional.cpp', 'thiscall, ret 0x10',
          'regra: a Norte tem 34 rodadas de 30 de julho a 20 de maio (38 em 2000); a Sul tem 34 rodadas na mesma temporada, com as rodadas 21 e 22 no mesmo dia (25 de fevereiro: defeito do original)',
          'rule: the North has 34 rounds from 30 July to 20 May (38 in 2000); the South has 34 rounds over the same season, with rounds 21 and 22 on the same day (25 February: a defect of the original)'),
         ('ger_regional_create_south_stage', 'GerRegional::create_south_stage', '0x5ddc50', 'ger_regional.cpp', 'thiscall, ret 4',
          'regra: a Regionalliga Sul é uma fase própria com os 18 clubes da seção Sul, turno e returno, dois sobem e quatro caem; o time B cujo clube principal está acima não pode subir',
          'rule: the Regionalliga South is a stage of its own with the 18 clubs of the South section, home and away, two going up and four down; a B team whose senior club is above cannot go up'),
         ('ger_regional_get_club_array', 'GerRegional::get_club_array', '0x5ddff0', 'ger_regional.cpp', 'thiscall, ret 8',
          'regra: os clubes da Regionalliga são os da tabela Norte seguidos dos da fase Sul: 36 (37 em 2000)',
          'rule: the Regionalliga clubs are the North table\'s followed by the South stage\'s: 36 (37 in 2000)')])
CAMPOS = []
INTERESSA = []
