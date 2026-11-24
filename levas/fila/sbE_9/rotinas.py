# Os construtores da Primeira e da Segunda Divisao polonesas (pol_first.cpp, pol_second.cpp; leitor sbE, rodada 5,
# 24/11/2026). Simulacao com a Polonia (as ligas e a data dos itens mwd): rodam na criacao do jogo (o teste cria o jogo
# do zero a cada execucao).
# - 0x7c8a80 PolFirst::PolFirst thiscall(ano, registro) ret 8: League::League 0x682200, registro, vtable 0x96e360,
#   +0xee = 0, ano, disciplina 31; o estado salvo (0x687b10, push 1): com +0x30 >= 1, a fase 1 recebe 0x7ca450 em +0xa3
#   (a ordem da fase final), fim. Senao fase 0xff, atual -1, 2 fases, malloc 8; regras 0x7c9690 (caixa 0x52), tabela
#   0x7c9450 (0x59), jogos (0x60), ordem push 0 (0x69), estatisticas (0x70); PolFirst::create_second_group_stage 0x7c9900
#   (chamada direta); o PolFirst::set_predicted_positions inline (sem estatisticas a caixa 0x346, nunca alcancada aqui;
#   qsort 0x945333 de +0xee com 0x4b07d0 e Comp_Stats::set_club_final_position 0x4a2540). O quadro SEH so cobre o new.
#   Prologo push -1; push imm32 (7 bytes).
# - 0x7cb1d0 PolSecond::PolSecond thiscall(ano, registro) ret 8: o construct_nation_division (vtable 0x96e4b4,
#   disciplina 31, 1 fase, regras 0x7cbf40, tabela 0x7cbdf0, pol_second.cpp 0xa7860c, caixas 0x48, 0x4f, 0x56, 0x5f,
#   0x66), conferido instrucao a instrucao. Prologo push -1; push imm32 (7 bytes).
# Depende do sbE_2 (open_nation_division, build_new_nation_division) e do cx13_5 (pol_first_set_predicted_positions):
# integrar depois deles.
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (compare_nation_division_ctor; a ordem da fase final
# no carregamento).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('construct_pol_first', 0x7c8a80, [0x6a, 0xff, 0x68, 0x39, 0xf7, 0x95, 0x00]),
           ('construct_pol_second', 0x7cb1d0, [0x6a, 0xff, 0x68, 0x39, 0xf8, 0x95, 0x00])]
DOCS = ('os construtores da Primeira e da Segunda Divisão polonesas', 'the Polish First and Second Division constructors',
        [('construct_pol_first', 'PolFirst::PolFirst', '0x7c8a80', 'pol_first.cpp', 'thiscall, ret 8',
          'a Primeira Divisão polonesa na criação do jogo: a regra de disciplina polonesa; salva na fase final, a ordem da fase final volta; nova, as regras, a tabela do primeiro grupo, os jogos, a ordem, as estatísticas, o segundo grupo e as posições previstas',
          'the Polish First Division at career creation: the Polish discipline rule; saved in its final stage, the final stage order comes back; new, the rules, the first group table, the fixtures, the order, the statistics, the second group and the predicted positions'),
         ('construct_pol_second', 'PolSecond::PolSecond', '0x7cb1d0', 'pol_second.cpp', 'thiscall, ret 8',
          'a Segunda Divisão polonesa na criação do jogo: a regra de disciplina polonesa e uma fase além da liga (a repescagem); nova, as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas',
          'the Polish Second Division at career creation: the Polish discipline rule and one stage besides the league (the play-off); new, the rules, the table, the fixtures, the order, the statistics and the predicted positions')])
CAMPOS = []
INTERESSA = []
