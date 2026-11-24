# O construtor da Terceira Divisao belga e o grupo B (bel_third.cpp; leitor cx09, rodada 5, 24/11/2026). Simulacao com
# a Belgica ('ENG BEL HOL GER DEN', ate 2002-08-01): o construtor roda na criacao do jogo (a carreira de teste cria o
# jogo do zero) e o grupo B nele e na virada de julho de 2002 (cobertura do mundo: 1 e 2 vezes).
# - 0x422970 BelThird::create_group_b_stage thiscall() ret 0: virtual 0x3c (0, &rodadas, &extra, 0); malloc(+0x3e x 4)
#   (caixa 0x2d8); os clubes (indice dword ate [0xae235c] - 2 x [0xae2350], relido) com +0x60 = [0x9cf5a8] (B) e +0x57 =
#   +4 da liga; passando da contagem: a caixa (0x2e9) e +0x57 do clube = 0 (defeito), e o laco segue; faltando: a caixa
#   (0x2f6). operator new 0xee e League_Stage 0x68b100 (this, n, lista, 2, competicao, tabela, rodadas, +0xc2, +0xc3,
#   +0xc4, &+0xc5, &+0xbe, ano, 0, extra, +0x51, 1, 0, +0xd9, -1, 0, 2; 22 argumentos conferidos) na vaga 0; sem fase a
#   caixa (0x301); senao free da lista e da tabela e +0x30 = 0 (nos erros a tabela e a lista vazam: defeito). Prologo
#   push -1; push imm32.
# - 0x4216f0 BelThird::BelThird thiscall(ano, registro) ret 8: League::League 0x682200, +4, a vtable 0x967b48, +0x40,
#   +0x50 = 4; League::load_from_temp_file 0x687b10 (1): carregada, pronto. Nova: +0x44 0xff, +0x30 -1, +0x2c 10,
#   +0xc = malloc(0x28); set_up_rules 0x4226f0 (caixa 0x4a), fill_league_table 0x4224e0 (0x51), League::sort_table
#   0x6827d0 (0) (0x58, antes do calendario, ao contrario das outras divisoes), operator new 0x5ce e Comp_Stats 0x49ee70
#   em +8 (0x60), League::create_fixtures 0x6835c0 (0x67); depois 0x422970 e 0x424030 (chamadas diretas). Prologo push
#   -1; push imm32.
# Comparadores: so a original; o grupo conferido depois (vaga 0 e fase atual), o construtor por
# compare_nation_division_ctor (do bel_first.rs) com a BEL_THIRD_DIVISION.
# Iguais no cm0102-1080.exe. Depende do cx09_8 e do cx09_12.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('bel_third_create_group_b_stage', 0x422970, [0x6a, 0xff, 0x68, 0x3e, 0x41, 0x95, 0x00]),
           ('construct_bel_third', 0x4216f0, [0x6a, 0xff, 0x68, 0xf9, 0x40, 0x95, 0x00])]
DOCS = ('o construtor e o grupo B da Terceira Divisão belga', 'the Belgian Third Division constructor and group B',
        [('bel_third_create_group_b_stage', 'BelThird::create_group_b_stage', '0x422970', 'bel_third.cpp', 'thiscall, ret 0',
          'o grupo B da Terceira Divisão belga: os 16 clubes vindos da Terceira B, em ida e volta, com as regras da liga (fase 0)',
          'Belgian Third Division group B: the 16 clubs that came from Third Division B, home and away, with the league rules (stage 0)'),
         ('construct_bel_third', 'BelThird::BelThird', '0x4216f0', 'bel_third.cpp', 'thiscall, ret 8',
          'monta a Terceira Divisão belga: do arquivo salvo, ou nova com 10 fases (o grupo B, os seis períodos, a final e as duas repescagens), as regras, a tabela do grupo A, a ordem, as estatísticas, o calendário, o grupo B e as posições previstas',
          'builds the Belgian Third Division: from the saved file, or new with 10 stages (group B, the six periods, the title decider and the two play-offs), the rules, the group A table, the order, the statistics, the fixtures, group B and the predicted positions')])
CAMPOS = []
INTERESSA = []
