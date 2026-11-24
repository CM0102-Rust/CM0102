# A tabela e as regras da Serie C2/B italiana (ita_ser_c2b.cpp; leitor cx11, rodada 5, 24/11/2026). Simulacao com a
# Italia e as divisoes inferiores: as duas rodam na criacao do jogo (o construtor do cx11_2) e de novo na virada de 2002
# (o start_new_season do cx11_6). Como no cx11_2, a C2 so existe com a base NORMAL ou MAXIMA (BASE abaixo; ver o
# cx11_2): a prova pede DATABASE=NORMAL na carreira.
# - 0x657f50 ItaSerC2b::init_team_list (o create_table da vtable, slot do construtor) thiscall() ret 0: o corpo do
#   ItaSerC1a::create_table (conferido instrucao a instrucao; so o fonte 0x9ea75c e a divisao [0x9cf584] mudam; a linha
#   da caixa 0x1bd e a mesma): ita_ser_a_fill_league_table. Prologo sub esp, 0x200 (6 bytes).
# - 0x658050 ItaSerC2b::set_up_rules thiscall() ret 0 -> 1: o corpo do ItaSerC1a::set_up_rules (conferido instrucao a
#   instrucao) com a divisao de cima [0x9cf580] (C2/A) e a de baixo [0x9cf588] (C2/C), a anterior e a seguinte na lista
#   das divisoes italianas: serie_c2_rules do cx11_3. Prologo push ebx; push esi; mov esi, ecx; mov dl, 3 (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: a tabela so a original contra o plano
# (ita_ser_a_compare_league_table); as regras a seco lado a lado (ita_ser_a_compare_league_rules).
# Depende do cx11_3 (serie_c2_rules): integrar depois dele.
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('ita_ser_c2b_create_table', 0x657f50, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ita_ser_c2b_set_up_rules', 0x658050, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03])]
DOCS = ('a tabela e as regras da Série C2/B italiana', 'the Italian Serie C2/B table and rules',
        [('ita_ser_c2b_create_table', 'ItaSerC2b::init_team_list', '0x657f50', 'ita_ser_c2b.cpp', 'thiscall, ret 0',
          'monta a tabela da Série C2/B com os 18 clubes da divisão na ordem do banco; sem fundo de prêmios nem dinheiro da TV',
          "builds the Serie C2/B table with the division's 18 clubs in database order; no prize fund and no TV money"),
         ('ita_ser_c2b_set_up_rules', 'ItaSerC2b::set_up_rules', '0x658050', 'ita_ser_c2b.cpp', 'thiscall, ret 0',
          'as regras da Série C2/B, as mesmas da C2/A; na lista das divisões italianas, acima fica a C2/A e abaixo a C2/C',
          'the Serie C2/B rules, the same as C2/A; in the list of Italian divisions, C2/A is above it and C2/C below')])
CAMPOS = []
INTERESSA = []
