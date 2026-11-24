# As regras, a virada e as posicoes previstas da Copa da Grecia (gre_cup.cpp; leitor cx02, rodada 5, 24/11/2026).
# Competicoes da Grecia; ATE 2002-08-01 (a virada da copa e no meio de 2002; as regras e as posicoes previstas rodam
# tambem na criacao, pelo construtor 0x5e88a0, ainda original, e na virada).
# - 0x5e9350 GreCup::set_up_rules thiscall() ret 0 -> 1 (a virtual 0x8c): o molde da NirLgeCup::set_up_rules
#   (set_cup_league_rules) com os desempates 4, 1, 2, 0, duas vagas por grupo e o premio 600.000.
# - 0x5e93f0 GreCup::start_new_season thiscall() ret 0 (a virtual 0x8): o molde da NirLgeCup::start_new_season
#   (start_irish_league_cup_season) com 0x5e9a80 (collect_entrants, mwd_30), 0x5e9260 (create_table) e 0x5e9650
#   (create_group_stage) chamados pelo endereco, 10 grupos, linhas 0x21c, 0x223, 0x229, 0x230.
# - 0x5ea060 GreCup::set_predicted_positions thiscall() ret 0 (a virtual 0x5c): o molde da ItaCup
#   (italian_cup_predicted_positions) com o mata-mata na fase 10 e faixas de 11, linhas 0x3f0 e 0x3f8.
# Prologos: 0x5e9350 push ebx; push esi; mov esi, ecx; mov al, 4 (6 bytes); 0x5e93f0 e 0x5ea060 sub esp, imm32.
# Iguais no cm0102-1080.exe.
# Comparadores: os das irmas (as regras conferidas depois da original; a virada so a original, o ano + 1; as posicoes
# lado a lado).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('gre_cup_set_up_rules', 0x5e9350, [0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x04]),
           ('gre_cup_start_new_season', 0x5e93f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('gre_cup_set_predicted_positions', 0x5ea060, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00])]
DOCS = ('as regras, a virada e as posições previstas da Copa da Grécia', 'the Greek Cup rules, turnover and predicted positions',
        [('gre_cup_set_up_rules', 'GreCup::set_up_rules', '0x5e9350', 'gre_cup.cpp', 'thiscall, ret 0',
          'regra: os grupos da Copa da Grécia dão 3 pontos por vitória e 1 por empate, desempate pelo confronto direto, saldo e gols [inferência]; passam dois por grupo; sem acesso nem rebaixamento; prêmio total de 600.000',
          'rule: the Greek Cup groups give 3 points for a win and 1 for a draw, ties split by head-to-head, goal difference and goals [inference]; two go through per group; no promotion or relegation; a 600,000 prize fund'),
         ('gre_cup_start_new_season', 'GreCup::start_new_season', '0x5e93f0', 'gre_cup.cpp', 'thiscall, ret 0',
          'regra: na virada, a Copa da Grécia arquiva a temporada, apaga as fases, passa o ano, escolhe os 44 inscritos e refaz as previsões, as regras, a tabela do primeiro grupo, os jogos e as fases dos dez grupos',
          'rule: at the turnover the Greek Cup archives the season, deletes the stages, moves the year on, picks the 44 entrants and rebuilds the predictions, the rules, the first group\'s table, the fixtures and the ten group stages'),
         ('gre_cup_set_predicted_positions', 'GreCup::set_predicted_positions', '0x5ea060', 'gre_cup.cpp', 'thiscall, ret 0',
          'regra: as posições previstas da Copa da Grécia: os 44 inscritos pela reputação; o mata-mata (fase 10) dá 1 ao campeão e 2 ao vice e uma posição a quem cai em cada rodada; os outros em faixas de 11',
          'rule: the Greek Cup predicted positions: the 44 entrants by reputation; the knockout (stage 10) gives 1 to the winner and 2 to the runner-up and a place to the losers of each round; the rest in bands of 11')])
CAMPOS = []
INTERESSA = []
