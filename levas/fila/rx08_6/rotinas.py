# As posicoes previstas das eliminatorias europeias da Copa do Mundo (wc_europe_league.cpp; leitor rx08, rodada 6,
# 25/11/2026). Simulacao, LIGAS 'ENG', ATE 2005-12-31 (a campanha de 2004 para a Copa de 2006): a virtual +0x5c roda no
# dia do sorteio (update_for_date, ja em Rust, pela vtable).
# - 0x91b1b0 WcEuropeLeague::set_predicted_positions thiscall() ret 0: sem estatisticas (+8) a caixa 0x7a0 e nada mais;
#   qsort 0x945333 (+0xee, +0xf2, 4, 0x4b07d0); NationalTeams::release_competitions_of_continent 0x775070 (thiscall em
#   [0xb74318]; o continente [0xae23a4] + 0xc6 x [0x9cfa10] e 0); para cada k (palavra com sinal) < +0xf2: pela
#   contagem de isentas +0xf6 (0..3) a posicao do pote (isentas 0: k < 10 -> 1, < 15 -> 11, < 19 -> 16, < 20 -> 20,
#   < 30 -> 21, < 40 -> 31, senao 41; 1: 9/1, 14/10, 18/15, 27/19, 36/28, 45/37, senao 46; 2: 9/1, 13/10, 16/14, 18/17,
#   27/19, 36/28, 45/37, senao 46; 3: 8/1, 12/9, 15/13, 16/16, 24/17, 32/25, 40/33, senao 41) em
#   Comp_Stats::set_club_final_position 0x4a2540 (thiscall em +8; selecao, posicao); outra contagem, sem posicao; depois
#   NationalTeams::set_current_competition 0x775220 (selecao +0x53, +4). Prologo sub esp, 0x200 (6 bytes).
# Igual no cm0102-1080.exe (inteira). Comparador: so a original (muda as competicoes das selecoes), com o plano (o bolo
# ordenado como uma copia ordenada antes).
# Autossuficiente (usa so WC_EUROPE_EXEMPT, ja no HEAD).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_europe_league_set_predicted_positions', 0x91b1b0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as posições previstas das eliminatórias europeias da Copa do Mundo', 'the European World Cup qualifying predicted positions',
        [('wc_europe_league_set_predicted_positions', 'WcEuropeLeague::set_predicted_positions', '0x91b1b0', 'wc_europe_league.cpp', 'thiscall, ret 0',
          'regra: antes do início das eliminatórias europeias cada seleção do bolo recebe a posição final prevista pelo pote do sorteio (o mais forte primeiro; a primeira posição de cada pote, conforme o número de isentas), e todas as seleções europeias deixam as competições continentais para jogar esta',
          "rule: before the European qualifying starts every nation of the pool gets its predicted final position from its seeding pot (strongest first; each pot's first place, by the number of exempt nations), and all the European national teams leave their continental competitions to play this one")])
CAMPOS = []
INTERESSA = []
