# Os registros pelo destino, as posicoes previstas e o historico da Primeira Divisao finlandesa (fin_first.cpp; leitor
# cx09, rodada 5, 24/11/2026). Simulacao com a Finlandia (as ligas do cx03_12: 'ENG SWE FIN NOR POR', ate 2002-08-01;
# na cobertura do mundo a 0x594810 roda 28 vezes e a 0x594ff0 2).
# - 0x594810 FinFirst::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18 -> eax
#   (virtual 0x44): fase com sinal < 1: status 6 add_qualified_for_round_record 0x86bdd0 (0x458, 0x82, 0x28) e 0, senao
#   -1. Fase 1, status 1: add_league_champions_record 0x868c50 e a linha (League::find_club_row 0x68ab70 na liga, senao
#   na fase 0) +0x37 = 5; 0. Fase 2, status 2: a linha +0x37 = 3 e add_relegation_record 0x86a1c0; 0. Fase 3: clube da
#   Veikkausliiga (id da divisao = [0x9cf7e8], sem testar nulo) procurado na tabela dela (sem: 0): status 1 0xfe, senao
#   rebaixamento do registro da Veikkausliiga ([0xae23d0] + id x 0x6b) e 3; devolve [0x9cf7e8]. Outro clube: a linha
#   (sem: 0); status 1 add_promotion_record 0x869480 (0x32) e 0, senao 0xfe; devolve [0x9cf7f8]. Outros: 0. Prologo
#   mov edx, [0x9cf7e8] (6 bytes).
# - 0x594ff0 FinFirst::set_predicted_positions thiscall() ret 0 (virtual 0x5c): sem +8 a caixa (0x593); malloc(0x28)
#   (caixa 0x59b); norte (a liga) e sul (fase 0), cada um com a propria contagem: qsort 0x945333 (0x4b07d0) e
#   Comp_Stats::set_club_final_position 0x4a2540 (clube, 2k + 1, o byte); free. Prologo sub esp, 0x204.
# - 0x595160 FinFirst::record_season_honours thiscall() ret 0 (virtual 0x30): jmp para a virtual 0x30 da fase 1 (a
#   repescagem de acesso, League::record_season_honours 0x684640, ret 0). Prologo mov eax, [ecx+0xc]; mov ecx, [eax+4]
#   (6 bytes). Nao e um thunk de um jmp so: le a fase e salta pela tabela virtual dela; fica com desvio.
# Comparadores: os registros so a original, conferida contra o plano da copia feito antes; as posicoes so a original,
# conferidas contra a ordem da copia; o historico so a original.
# A ordem do registro e do status na fase 2 (o original escreve o status antes do rebaixamento) nao muda nada visto de
# fora: os registros do tecnico nao leem a tabela.
# Iguais no cm0102-1080.exe. Depende do cx09_12 (two_group_predicted_positions).
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('fin_first_update_staff_records_for_status', 0x594810, [0x8b, 0x15, 0xe8, 0xf7, 0x9c, 0x00]),
           ('fin_first_set_predicted_positions', 0x594ff0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('fin_first_record_season_honours', 0x595160, [0x8b, 0x41, 0x0c, 0x8b, 0x48, 0x04])]
DOCS = ('os registros pelo destino, as posições previstas e o histórico da Primeira Divisão finlandesa',
        'the Finnish First Division staff records by outcome, predicted positions and season honours',
        [('fin_first_update_staff_records_for_status', 'FinFirst::update_staff_records_for_status', '0x594810', 'fin_first.cpp', 'thiscall, ret 0x18',
          'nos grupos, a vaga na repescagem vai aos registros do técnico; na repescagem de acesso o vencedor é campeão; na de rebaixamento quem perde cai; contra a Veikkausliiga, o clube da Primeira que vence sobe e o da Veikkausliiga que perde cai',
          "in the groups, a play-off place goes to the manager's records; in the promotion play-off the winner is champion; in the relegation play-off the loser goes down; against the Veikkausliiga, the First Division club that wins goes up and the Veikkausliiga club that loses goes down"),
         ('fin_first_set_predicted_positions', 'FinFirst::set_predicted_positions', '0x594ff0', 'fin_first.cpp', 'thiscall, ret 0',
          'as posições previstas: em cada grupo (norte e sul) os clubes ordenados pela força recebem 1, 3, 5... (2k + 1)',
          'the predicted positions: in each group (north and south) the clubs ordered by strength get 1, 3, 5... (2k + 1)'),
         ('fin_first_record_season_honours', 'FinFirst::record_season_honours', '0x595160', 'fin_first.cpp', 'thiscall, ret 0',
          'o histórico da temporada da Primeira Divisão finlandesa vem da repescagem de acesso (a fase 1)',
          'the Finnish First Division season honours come from the promotion play-off (stage 1)')])
CAMPOS = []
INTERESSA = []
