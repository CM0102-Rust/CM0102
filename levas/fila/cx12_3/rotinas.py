# O construtor, a tabela e as regras da Serie C2/C italiana (ita_ser_c2c.cpp; leitor cx12, rodada 5, 24/11/2026).
# Simulacao com a Italia e as divisoes inferiores: o construtor roda uma vez na criacao do jogo (o setup da Italia),
# a tabela e as regras nele e de novo na virada de 2002 (o start_new_season do cx12_4). Como no cx11_2, a C2 so existe
# com a base NORMAL ou MAXIMA (BASE abaixo; ver o cx11_2): a prova pede DATABASE=NORMAL na carreira.
# - 0x65b4e0 ItaSerC2c::ItaSerC2c thiscall(ano, registro) ret 8: o corpo do ItaSerC2a::ItaSerC2a (conferido instrucao a
#   instrucao contra o 0x652e90: so a vtable 0x96c804, as regras 0x65c1e0, a tabela 0x65c0e0, o calendario real
#   0x65c8e0, o fonte 0x9ea83c e o manipulador do SEH mudam; as linhas das caixas 0x49, 0x50, 0x5a, 0x63, 0x6a sao as
#   mesmas): construct_italian_division com SERIE_C2C_DIVISION. Prologo push -1; push imm32 (7 bytes).
# - 0x65c0e0 ItaSerC2c::init_team_list thiscall() ret 0: o corpo do ItaSerC2a::create_table (conferido; so o fonte e a
#   divisao [0x9cf588] mudam; a linha da caixa 0x1bd e a mesma): ita_ser_a_fill_league_table. Prologo sub esp, 0x200.
# - 0x65c1e0 ItaSerC2c::set_up_rules thiscall() ret 0 -> 1: o corpo do ItaSerC2a::set_up_rules com a divisao de cima
#   [0x9cf584] (C2/B) e a de baixo -1 (mov dword [esi+0x20], 0xffffffff no lugar da leitura da chave: a C2/C e a ultima
#   da lista que o all_stages_complete percorre); o resto conferido instrucao a instrucao: serie_c2c_rules (aqui, sobre
#   o serie_c2_rules do cx11_3) e ita_ser_a_apply_league_rules. Prologo push ebx; push esi; mov esi, ecx; mov dl, 3.
# Iguais no cm0102-1080.exe (inteiras). Comparadores: o construtor so a original (compare_italian_constructor), a
# tabela so a original contra o plano (ita_ser_a_compare_league_table), as regras a seco lado a lado
# (ita_ser_a_compare_league_rules).
# Depende do cx11_3 (serie_c2_rules): integrar depois dele.
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('construct_ita_ser_c2c', 0x65b4e0, [0x6a, 0xff, 0x68, 0x19, 0xbd, 0x95, 0x00]),
           ('ita_ser_c2c_create_table', 0x65c0e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ita_ser_c2c_set_up_rules', 0x65c1e0, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03])]
DOCS = ('o construtor, a tabela e as regras da Série C2/C italiana', 'the Italian Serie C2/C constructor, table and rules',
        [('construct_ita_ser_c2c', 'ItaSerC2c::ItaSerC2c', '0x65b4e0', 'ita_ser_c2c.cpp', 'thiscall, ret 8',
          'monta a Série C2/C italiana na criação do jogo (só com a base normal ou máxima), como a C2/A: regra de disciplina 0x11, a liga e duas fases (a repescagem de acesso e o play-out do rebaixamento); sem nada salvo, as regras, a tabela, o calendário real de 2001/02 pelos nomes dos clubes, os jogos, a ordem, as estatísticas e as posições previstas',
          'builds the Italian Serie C2/C at game creation (only with the normal or maximum database), as C2/A: discipline rule 0x11, the league and two stages (the promotion play-off and the relegation play-out); with nothing saved, the rules, the table, the real 2001/02 calendar by club names, the fixtures, the order, the statistics and the predicted positions'),
         ('ita_ser_c2c_create_table', 'ItaSerC2c::init_team_list', '0x65c0e0', 'ita_ser_c2c.cpp', 'thiscall, ret 0',
          'monta a tabela da Série C2/C com os 18 clubes da divisão na ordem do banco; sem fundo de prêmios nem TV',
          "builds the Serie C2/C table with the division's 18 clubs in database order; no prize fund and no TV money"),
         ('ita_ser_c2c_set_up_rules', 'ItaSerC2c::set_up_rules', '0x65c1e0', 'ita_ser_c2c.cpp', 'thiscall, ret 0',
          'as regras da Série C2/C: turno e returno, 3 pontos a vitória e 1 o empate, desempate 1 e 2, 1 sobe direto, 4 na repescagem de acesso, 4 no play-out e 1 cai direto; acima dela a C2/B e abaixo nenhuma (a C2/C é a última divisão italiana); depois o calendário',
          'the Serie C2/C rules: two legs, 3 points a win and 1 a draw, tie-breaks 1 and 2, 1 promoted directly, 4 in the promotion play-off, 4 in the play-out and 1 relegated directly; Serie C2/B above it and nothing below (C2/C is the last Italian division); then the schedule')])
CAMPOS = []
INTERESSA = []
