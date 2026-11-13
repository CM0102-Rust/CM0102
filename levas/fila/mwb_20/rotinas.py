# A Copa Premier galesa (wel_prm_cup.cpp): a tabela do primeiro grupo, as regras e a passagem de fase. Depende do mwb_18
# (welsh_cup_group_table, welsh_cup_rules, welsh_cup_advance_stage): integrar junto ou depois dele. Leitor mwb
# (12/11/2026), lista do mundo (Escócia e País de Gales).
# - 0x928060 (WelPrmCup::create_group_table): o primeiro grupo, de 4 (inscritos 0, 2, 4 e 6 do bolo de 12).
# - 0x928160 (WelPrmCup::set_up_rules): 2 turnos, os 2 primeiros passam, fundo de 100 mil.
# - 0x9283c0 (WelPrmCup::advance_stage): o segundo grupo é a fase 0; na 1 o mata-mata (0x928650, ainda na original).
# Convenções conferidas: como as do mwb_18.
# Comparadores: como os do mwb_18.
LIGAS = 'ENG SCO WAL'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('wel_prm_cup_create_group_table', 0x928060, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('wel_prm_cup_set_up_rules', 0x928160, [0x56, 0x8b, 0xf1, 0xb0, 0x04]),
           ('wel_prm_cup_advance_stage', 0x9283c0, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30])]
DOCS = ('a Copa Premier galesa: o primeiro grupo, as regras e a passagem de fase',
        'the Welsh Premier Cup: the first group, the rules and the next stage',
        [('wel_prm_cup_create_group_table', 'WelPrmCup::create_group_table', '0x928060', 'wel_prm_cup.cpp', 'thiscall, ret 0',
          'regra: a própria copa é o primeiro grupo, de 4 clubes: os inscritos 0, 2, 4 e 6 do bolo, tirados do bolo; depois a notícia do sorteio do grupo',
          'rule: the cup itself is the first group, of 4 clubs: entrants 0, 2, 4 and 6 of the pool, taken out of the pool; then the group draw news'),
         ('wel_prm_cup_set_up_rules', 'WelPrmCup::set_up_rules', '0x928160', 'wel_prm_cup.cpp', 'thiscall, ret 0',
          'regra: os grupos jogam 2 turnos, 3 pontos por vitória e 1 por empate, categoria 4; os 2 primeiros de cada grupo passam; desempate pelos códigos 1, 2 e 4 (saldo, gols pró, confronto direto [inferência]); fundo de prêmios de 100 mil',
          'rule: the groups play 2 rounds, 3 points for a win and 1 for a draw, category 4; the top 2 of each group go through; tie-breaks codes 1, 2 and 4 (goal difference, goals for, head-to-head [inference]); a prize fund of 100,000'),
         ('wel_prm_cup_advance_stage', 'WelPrmCup::advance_stage', '0x9283c0', 'wel_prm_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa Premier galesa passa do primeiro grupo ao segundo (fase 0) e, ao chegar à fase 1, monta o mata-mata dos oito',
          'rule: the Welsh Premier Cup moves from the first group to the second (stage 0) and, on reaching stage 1, builds the knockout of the eight')])
