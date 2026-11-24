# O construtor da Primeira Divisao finlandesa (Veikkausliiga) e a troca da Liga de Honra portuguesa com as zonas sem a
# Segunda Divisao B (fin_prm.cpp, por_prm.cpp; leitor sbE, rodada 5, 24/11/2026). Simulacao com a Finlandia e Portugal
# (as ligas e a data dos itens mwe): o construtor na criacao do jogo; a troca na virada portuguesa (PorPrm::start_new_season
# 0x7cf9e0, mwe_15, sem a Segunda B, que so existe com a base NORMAL), em junho/julho de 2002.
# - 0x595170 FinPrm::FinPrm thiscall(ano, registro) ret 8: League::League 0x682200, registro, vtable 0x96a4c0, ano,
#   disciplina 10; o estado salvo (0x687b10, push 1, ret 4) termina; senao os clubes dos grupos Norte e Sul da Ykkonen
#   (indice ate clubes - 2 x nacoes) vao para a Ykkonen com o grupo na divisao reserva (+0x60); fase 0xff, atual -1, 0
#   fases e vetor 0; regras 0x595c90 (caixa 0x59), tabela 0x595b70 (0x60), jogos 0x6835c0 (0x67), ordem 0x6827d0 push 0
#   (0x70), estatisticas (new 0x5ce, 0x49ee70; 0x77), posicoes previstas 0x68a850. O quadro SEH so cobre o new. Prologo
#   push -1; push imm32 (7 bytes).
# - 0x7cfd30 PorPrm::exchange_second_division_with_zones thiscall() ret 0 (chamada direta): a lista (malloc de 4 x
#   clubes; caixa 0x28d) dos clubes portugueses (+0x53 = 0x9cf434) das tres zonas da Segunda B; cada linha da Liga de
#   Honra: time B (club_get_linked_b_team 0x540a50, cdecl, a marca 0) com o clube principal tambem na Liga de Honra e o
#   proprio fora da Primeira: status 3 e Comp::send_promotion_relegation_override_news 0x48ece0 (ecx = Liga de Honra;
#   competicao, clube, principal, 1; ret 0x10); linha de status 3: conta (byte) e cai para a zona Norte (0x6831a0); com
#   candidatos suficientes, comp_util_sort_and_shuffle_top_clubs 0x4afbd0 (cdecl) e os primeiros sobem (0x6830b0);
#   free. Prologo sub esp, 0x214 (6 bytes).
# Auxiliares novos (pub(crate)): open_nation_division, build_new_nation_division (os passos do construct_nation_division
# para os construtores que fazem outra coisa no meio; o sbE_9 tambem usa) e move_group_clubs_to_division (o sbE_3
# tambem usa).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: so a original (o construtor pelo compare_nation_division_ctor e
# nenhum clube nos grupos; a troca, os rebaixados na zona Norte).
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('construct_fin_prm', 0x595170, [0x6a, 0xff, 0x68, 0xc9, 0x8f, 0x95, 0x00]),
           ('por_prm_exchange_second_division_with_zones', 0x7cfd30, [0x81, 0xec, 0x14, 0x02, 0x00, 0x00])]
DOCS = ('o construtor da Veikkausliiga e a troca da Liga de Honra portuguesa com as zonas',
        'the Veikkausliiga constructor and the Liga de Honra exchanged with the zones',
        [('construct_fin_prm', 'FinPrm::FinPrm', '0x595170', 'fin_prm.cpp', 'thiscall, ret 8',
          'a Primeira Divisão finlandesa na criação do jogo: sem estado salvo, os clubes dos grupos Norte e Sul da Ykkönen são reunidos na Ykkönen (o grupo fica como divisão reserva); depois as regras, a tabela, os jogos, a ordem, as estatísticas e as posições previstas',
          'the Finnish Premier Division at career creation: with no saved state, the clubs of the Ykkönen\'s North and South groups are gathered into the Ykkönen (the group kept as the reserve division); then the rules, the table, the fixtures, the order, the statistics and the predicted positions'),
         ('por_prm_exchange_second_division_with_zones', 'PorPrm::exchange_second_division_with_zones', '0x7cfd30', 'por_prm.cpp', 'thiscall, ret 0',
          'sem a Segunda Divisão B: o time B da Liga de Honra com o clube principal na mesma divisão é rebaixado (com a notícia); os rebaixados da Liga de Honra caem para a zona Norte e o mesmo número de clubes portugueses das zonas, pela força com os melhores sorteados, sobe',
          'without the Second Division B: a Liga de Honra B team whose senior club is in the same division is relegated (with the news); the Liga de Honra\'s relegated clubs go down to the North zone and as many Portuguese zone clubs, by strength with the best ones shuffled, go up')])
CAMPOS = []
INTERESSA = []
