# A tabela e as regras da Serie C2/A italiana (ita_ser_c2a.cpp; leitor cx11, rodada 5, 24/11/2026). Simulacao com a
# Italia e as divisoes inferiores: as duas rodam na criacao do jogo (o construtor do cx11_2) e de novo na virada de 2002
# (o start_new_season do cx11_6). Como no cx11_2, a C2 so existe com a base NORMAL ou MAXIMA (BASE abaixo; ver o
# cx11_2): a prova pede DATABASE=NORMAL na carreira.
# - 0x653a90 ItaSerC2a::create_table thiscall() ret 0: o corpo do ItaSerC1a::create_table (conferido instrucao a
#   instrucao; so o fonte 0x9ea654 e a divisao [0x9cf580] mudam; a linha da caixa 0x1bd e a mesma): 18 clubes, malloc
#   0x3b x 18, init_table_row 0x687430 (thiscall, 4 args), sem fundo de premios nem TV: ita_ser_a_fill_league_table.
#   Prologo sub esp, 0x200 (6 bytes).
# - 0x653b90 ItaSerC2a::set_up_rules thiscall() ret 0 -> 1: o corpo do ItaSerC1a::set_up_rules (conferido instrucao a
#   instrucao) com a divisao de cima [0x9cf57c] (C1/B) e a de baixo [0x9cf584] (C2/B) no lugar de [0x9cf574] e
#   [0x9cf57c] (a lista A, B, C1/A, C1/B, C2/A, C2/B, C2/C que o all_stages_complete percorre); a virtual 0x3c (push 0; push &+0x3a; push &+0xa9; push -1) em +0xba:
#   ita_ser_a_apply_league_rules com serie_c2_rules (aqui; o cx11_4 usa). Prologo push ebx; push esi; mov esi, ecx; mov
#   dl, 3 (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: a tabela so a original contra o plano
# (ita_ser_a_compare_league_table); as regras a seco lado a lado (ita_ser_a_compare_league_rules).
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('ita_ser_c2a_create_table', 0x653a90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('ita_ser_c2a_set_up_rules', 0x653b90, [0x53, 0x56, 0x8b, 0xf1, 0xb2, 0x03])]
DOCS = ('a tabela e as regras da Série C2/A italiana', 'the Italian Serie C2/A table and rules',
        [('ita_ser_c2a_create_table', 'ItaSerC2a::create_table', '0x653a90', 'ita_ser_c2a.cpp', 'thiscall, ret 0',
          'monta a tabela da Série C2/A com os 18 clubes da divisão na ordem do banco; sem fundo de prêmios nem dinheiro da TV',
          "builds the Serie C2/A table with the division's 18 clubs in database order; no prize fund and no TV money"),
         ('ita_ser_c2a_set_up_rules', 'ItaSerC2a::set_up_rules', '0x653b90', 'ita_ser_c2a.cpp', 'thiscall, ret 0',
          'as regras da Série C2/A (18 clubes): ida e volta, 3 pontos por vitória e 1 por empate, desempates 1 e 2; um sobe direto, do 2º ao 5º vão à repescagem de acesso, do 14º ao 17º ao play-out e o último cai; 7 reservas e 3 substituições; acima fica a C1/B e abaixo a C2/B: as divisões italianas formam uma lista (A, B, C1/A, C1/B, C2/A, C2/B, C2/C); depois o calendário da liga',
          'the Serie C2/A rules (18 clubs): home and away, 3 points for a win and 1 for a draw, tie-breaks 1 and 2; one goes up directly, 2nd to 5th go to the promotion play-off, 14th to 17th to the play-out and the last goes down; 7 substitutes and 3 substitutions; above it C1/B and below it C2/B: the Italian divisions form one list (A, B, C1/A, C1/B, C2/A, C2/B, C2/C); then the league schedule')])
CAMPOS = []
INTERESSA = []
