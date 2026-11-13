# A Copa da Liga galesa (wel_lge_cup.cpp): a tabela do primeiro grupo, as regras e a passagem de fase; os moldes das duas
# copas galesas em grupos (welsh_cup_group_table, welsh_cup_rules, welsh_cup_advance_stage, usados pelo mwb_20).
# Depende do motor só (eng_prm_apply_league_rules, compare_state_league_advance_stage). Leitor mwb (12/11/2026), lista do
# mundo (Escócia e País de Gales).
# - 0x926700 (WelLgeCup::create_table): o objeto da copa é ele mesmo um grupo de 3 (a coluna 0 do bolo de 18 inscritos,
#   +0xee/+0xf2); a notícia do sorteio do grupo (0x684230).
# - 0x926800 (WelLgeCup::set_up_rules): 2 turnos, o primeiro do grupo passa, fundo de 100 mil.
# - 0x926a70 (WelLgeCup::advance_stage): os cinco outros grupos são as fases 0 a 4; na 5 o mata-mata (0x926d00, ainda
#   na original).
# Convenções conferidas: thiscall, ret 0 (0x687430 thiscall, ret 0x10; 0x684230 mov ecx -> thiscall, ret 0; a virtual
# +0x3c push 0; push &+0x3a; push &+0xa9; push -1 -> ret 0x10; 0x926d00 mov ecx -> thiscall, ret 0).
# Comparadores: a tabela só a original (aloca e manda notícia), com o plano; as regras lado a lado nos bytes; a fase só
# a original (o +0x30 sobe um, ou fica na última).
# A copa é montada na criação da carreira (o construtor 0x925e90 ainda na original) e de novo na virada de 2002.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('wel_lge_cup_create_table', 0x926700, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('wel_lge_cup_set_up_rules', 0x926800, [0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x04]),
           ('wel_lge_cup_advance_stage', 0x926a70, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('a Copa da Liga galesa: o primeiro grupo, as regras e a passagem de fase',
        'the Welsh League Cup: the first group, the rules and the next stage',
        [('wel_lge_cup_create_table', 'WelLgeCup::create_table', '0x926700', 'wel_lge_cup.cpp', 'thiscall, ret 0',
          'regra: a própria copa é o primeiro grupo, de 3 clubes: o primeiro de cada linha do bolo de inscritos (a coluna do grupo 0), tirados do bolo; depois a notícia do sorteio do grupo',
          "rule: the cup itself is the first group, of 3 clubs: the first of each row of the entrant pool (group 0's column), taken out of the pool; then the group draw news"),
         ('wel_lge_cup_set_up_rules', 'WelLgeCup::set_up_rules', '0x926800', 'wel_lge_cup.cpp', 'thiscall, ret 0',
          'regra: os grupos jogam 2 turnos, 3 pontos por vitória e 1 por empate, categoria 4; o primeiro de cada grupo passa; desempate pelos códigos 4, 1 e 2 (confronto direto, saldo, gols pró [inferência]); fundo de prêmios de 100 mil',
          'rule: the groups play 2 rounds, 3 points for a win and 1 for a draw, category 4; the winner of each group goes through; tie-breaks codes 4, 1 and 2 (head-to-head, goal difference, goals for [inference]); a prize fund of 100,000'),
         ('wel_lge_cup_advance_stage', 'WelLgeCup::advance_stage', '0x926a70', 'wel_lge_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Liga galesa passa de grupo em grupo (as fases 0 a 4 são os outros cinco grupos) e, ao chegar à fase 5, monta o mata-mata dos oito melhores',
          'rule: the Welsh League Cup moves from group to group (stages 0 to 4 are the other five groups) and, on reaching stage 5, builds the knockout of the best eight')])
