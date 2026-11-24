# As regras, as posicoes previstas e os registros de carreira da Copa da Holanda (hol_cup.cpp; leitor cx02, rodada 5,
# 24/11/2026). Competicoes da Holanda; ATE 2002-08-01 (a copa e montada no primeiro dia completo da temporada,
# HolCup::update_for_date 0x5f4020, ainda original; os registros na fase de grupos e no mata-mata).
# - 0x5f3e70 HolCup::set_up_rules thiscall() ret 0 -> 1: o molde da NirLgeCup::set_up_rules (set_cup_league_rules)
#   com os desempates 4, 1, 2, 0, duas vagas por grupo e o premio 800.000 (a mesma diferenca da GreCup, cx02_18).
# - 0x5f5120 HolCup::set_predicted_positions thiscall() ret 0: o molde da ItaCup (italian_cup_predicted_positions) com
#   o mata-mata na fase 19 e faixas de 20, linhas 0x4be e 0x4c6.
# - 0x5f4d40 HolCup::update_staff_records_for_status thiscall(...) ret 0x18 (169 chamadas no mundo): o molde da ItaCup
#   (italian_cup_staff_records) com o mata-mata na fase 19 e a rodada de classificacao 0x14.
# Prologos: push ebx; push esi; mov esi, ecx; mov al, 4 (6 bytes); sub esp, 0x220; mov al, [esp+0xc]; push esi.
# Iguais no cm0102-1080.exe.
# Comparadores: os das irmas (as regras conferidas depois da original; as posicoes lado a lado; os registros so a
# original, o retorno 0).
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('hol_cup_set_up_rules', 0x5f3e70, [0x53, 0x56, 0x8b, 0xf1, 0xb0, 0x04]),
           ('hol_cup_set_predicted_positions', 0x5f5120, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('hol_cup_update_staff_records_for_status', 0x5f4d40, [0x8a, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('as regras, as posições previstas e os registros de carreira da Copa da Holanda', 'the Dutch Cup rules, predicted positions and staff records',
        [('hol_cup_set_up_rules', 'HolCup::set_up_rules', '0x5f3e70', 'hol_cup.cpp', 'thiscall, ret 0',
          'regra: os grupos da Copa da Holanda dão 3 pontos por vitória e 1 por empate, desempate pelo confronto direto, saldo e gols [inferência]; passam dois por grupo; sem acesso nem rebaixamento; prêmio total de 800.000',
          'rule: the Dutch Cup groups give 3 points for a win and 1 for a draw, ties split by head-to-head, goal difference and goals [inference]; two go through per group; no promotion or relegation; an 800,000 prize fund'),
         ('hol_cup_set_predicted_positions', 'HolCup::set_predicted_positions', '0x5f5120', 'hol_cup.cpp', 'thiscall, ret 0',
          'regra: as posições previstas da Copa da Holanda: os inscritos pela reputação; o mata-mata (fase 19) dá 1 ao campeão e 2 ao vice e uma posição a quem cai em cada rodada; os outros em faixas de 20',
          'rule: the Dutch Cup predicted positions: the entrants by reputation; the knockout (stage 19) gives 1 to the winner and 2 to the runner-up and a place to the losers of each round; the rest in bands of 20'),
         ('hol_cup_update_staff_records_for_status', 'HolCup::update_staff_records_for_status', '0x5f4d40', 'hol_cup.cpp', 'thiscall, ret 0x18',
          'regra: na Copa da Holanda, quem passa de um grupo (fases 0 a 18) ganha o registro de classificação (rodada 0x14, peso 40) e quem cai o de não classificado; no mata-mata (fase 19) a eliminação e a passagem pesam pela rodada (50 a 100 em 5 rodadas), e o campeão e o vice ganham os registros da final',
          'rule: in the Dutch Cup a club going through a group (stages 0 to 18) gets the qualified record (round 0x14, weight 40) and one going out the failed record; in the knockout (stage 19) elimination and progress weigh by round (50 to 100 over 5 rounds), and the winner and runner-up get the final records')])
CAMPOS = []
INTERESSA = []
