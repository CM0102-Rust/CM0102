# A tabela, as regras e as duas fases do campeonato da Primeira Divisao argentina (arg_prm.cpp; leitor cx08, rodada 5,
# 24/11/2026). Simulacao com a Argentina (as ligas do cx01_8, ate 2002-08-01): as tres rodam na criacao do jogo (o
# construtor do mwf_23) e a tabela e as regras de novo na virada de 2002.
# - 0x405540 ArgPrm::create_table thiscall() ret 0: 20 clubes, malloc 0x49c (caixa 0x1e7), init_table_row 0x687430
#   (thiscall, 4 args), calc_prize_fund 0x682f70 (push 0xc350), +0xe1 = 1, pay_tv_rights 0x683010 (push 0; push
#   0x7a120). O molde eng_prm_fill_league_table (conferido instrucao a instrucao). Prologo sub esp, 0x200.
# - 0x405670 ArgPrm::set_up_rules thiscall() ret 0 -> 1: as gravacoes na ordem da original e a virtual 0x3c (push 0;
#   push &+0x3a; push &+0xa9; push -1) em +0xba (fra_first_apply_league_rules). Prologo push ebx; push esi; mov esi,
#   ecx; mov ebx, 1 (9 bytes).
# - 0x405950 ArgPrm::create_stages thiscall() ret 0: a lista dos clubes (malloc, caixa 0x2a1), para as fases 0 e 1 a
#   virtual 0x3c (push 0; push &rotulo; push &rodadas; push fase), operator new 0x944cf1 (0xee) e League_Stage 0x68b100
#   (thiscall, ret 0x58, 22 args: a fase 0 com +0x3c, +0xd9 e +0x28; a fase 1 com 2, 0 e 0), free da tabela de rodadas,
#   caixas 0x2c2 e 0x2d2; no fim free da lista e +0x30 = 1. Prologo mov eax, fs:[0] (6 bytes).
# Defeito do original: depois de qualquer caixa do create_stages a lista dos clubes nao e liberada (80 bytes perdidos).
# Comparadores: a tabela so a original contra o plano (compare_nation_league_table); as regras a seco
# (compare_nation_league_rules); as fases so a original, com as duas fases criadas e +0x30 = 1.
# Iguais no cm0102-1080.exe.
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('arg_prm_create_table', 0x405540, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('arg_prm_set_up_rules', 0x405670, [0x53, 0x56, 0x8b, 0xf1, 0xbb, 0x01, 0x00, 0x00, 0x00]),
           ('arg_prm_create_stages', 0x405950, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a tabela, as regras e as fases do campeonato argentino', 'the Argentine Premier Division table, rules and stages',
        [('arg_prm_create_table', 'ArgPrm::create_table', '0x405540', 'arg_prm.cpp', 'thiscall, ret 0',
          'monta a tabela da Primeira Divisão argentina com os 20 clubes da divisão na ordem do banco; cheia, o fundo de prêmios na cota de 50.000 por posição e 500.000 da TV para cada clube',
          "builds the Argentine Premier Division table with the division's 20 clubs in database order; when full, the prize fund by the 50,000 share per place and 500,000 of TV money to each club"),
         ('arg_prm_set_up_rules', 'ArgPrm::set_up_rules', '0x405670', 'arg_prm.cpp', 'thiscall, ret 0',
          'as regras da Primeira Divisão argentina: turno único (cada par se enfrenta uma vez), 3 pontos por vitória e 1 por empate, sem vagas de acesso, repescagem ou rebaixamento direto (o rebaixamento vai pelos promédios), a Segunda Divisão abaixo, desempates 1, 2 e 3, 5 reservas e 3 substituições; depois o calendário do Apertura',
          'the Argentine Premier Division rules: a single round (each pair meets once), 3 points for a win and 1 for a draw, no promotion, play-off or direct relegation places (relegation goes by the averages), the Second Division below, tie-breakers 1, 2 and 3, 5 substitutes and 3 substitutions; then the Apertura schedule'),
         ('arg_prm_create_stages', 'ArgPrm::create_stages', '0x405950', 'arg_prm.cpp', 'thiscall, ret 0',
          'cria as duas fases do campeonato argentino com os 20 clubes: a fase 0 com o calendário do Clausura e as regras da liga, a fase 1 sem calendário, em ida e volta e sem regras nem prêmios; a fase atual vira 1. Defeito do original: depois de uma caixa de erro a lista dos clubes não é liberada',
          "creates the two stages of the Argentine championship with the 20 clubs: stage 0 with the Clausura schedule and the league's rules, stage 1 with no schedule, home and away and with no rules nor prizes; the current stage becomes 1. Defect of the original: after an error box the club list is never freed")])
CAMPOS = []
INTERESSA = []
