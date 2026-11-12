# The mini league table of a set of clubs (sub_league.cpp, in leagues\). Competitions front (reader cqx, 12/11/2026),
# 8-league career.
# - 0x88c6d0 (SubLeague::SubLeague, 9 calls to 2002-06-01: the tables of the qualifying groups' runners-up, March
#   2002). Out of cqv_4 on 07/11: ported and compared without divergence, but the mutation (no games between the clubs,
#   in the fill 0x88c9f0) escaped: those tables only decide after June. Here the comparer is side by side (the copy
#   builds a scratch object from the same arguments; the tables compared), so the mutation is caught when the
#   confirmation in comparison mode reaches the calls (March 2002; see PRONTO.txt).
# - 0x584550 (EuropeanCup::build_fate_news_text, 1 call): the Champions League news text of a club's fate. Comparer:
#   compare_text_writer (main 759c98f9d) on the text-only paths; one mutation (the club link type); the text does not
#   reach the impression, so only the comparer catches it, when the confirmation reaches the call.
# Equal in cm0102-1080.exe (both, whole).
# Conventions checked at the original's call sites: 12 pushes; mov ecx, new; call -> thiscall, ret 0x30. Inside:
# 0x682200 (League::League, mov ecx -> thiscall, ret 0), 0x944e46 (push; add esp, 4 -> cdecl), 0x687430 (push 0; push
# 0; push club; push i -> thiscall, ret 0x10), 0x88c9f0 (7 pushes -> thiscall, ret 0x1c), 0x6827d0 (push 0 -> thiscall,
# ret 4), the error box (show_error_at); 0x4b0870 (push word; add esp, 4 -> cdecl), 0x48c6d0 (10 pushes -> thiscall,
# ret 0x28), 0x4b4590 (6 pushes; add esp, 0x18 -> cdecl), 0x66f4e0 (8 pushes; add esp, 0x20 -> cdecl), 0x4ae660 and
# 0x4ae8a0 (thiscall, ret 4 and 0xc).
# Comparers: the mini league side by side with a scratch object (League::free_table on it, then freed); the news text
# with compare_text_writer on the text-only paths, only the original on the generic news paths.
# Determinism: nothing reads stack, heap or an address as a value.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construct_sub_league', 0x88c6d0, [0x6a, 0xff, 0x68, 0xfb, 0x2f, 0x96, 0x00]),
           ('european_cup_build_fate_news_text', 0x584550, [0x8a, 0x44, 0x24, 0x0c, 0x53])]
DOCS = ('a minitabela de um grupo de clubes; a notícia do destino de um clube na Liga dos Campeões',
        "the mini league table of a set of clubs; the news of a club's fate in the Champions League",
        [('construct_sub_league', 'SubLeague::SubLeague', '0x88c6d0', 'sub_league.cpp', 'thiscall, ret 0x30',
          'regra: uma minitabela de um grupo de clubes (por exemplo, os segundos colocados dos grupos das eliminatórias comparados entre si): a tabela desses clubes preenchida com os jogos que eles fizeram entre si, com os pontos por vitória e empate e os critérios de desempate dados, e depois ordenada',
          'rule: a mini league of a given set of clubs (e.g. the runners-up of the qualifying groups compared among themselves): a table of those clubs filled from the fixtures they played, with the given points per win and draw and tie-breaks, then sorted'),
         ('european_cup_build_fate_news_text', 'EuropeanCup::build_fate_news_text', '0x584550', 'european_cup.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do destino de um clube na Liga dos Campeões: nas preliminares, o clube que passa à fase 1 ganha o seu texto e o que cai vai para a Copa da UEFA ("are entered for the UEFA Cup"), salvo nas rodadas de base 0xfa e 0x104 ou com objeto de notícia (a notícia genérica); na primeira fase de grupos a classificação usa a notícia genérica de copa e a eliminação manda o clube à Copa da UEFA; na segunda fase de grupos a eliminação é "knocked out of"; a fase final vai à notícia genérica',
          'rule: the Champions League news of a club\'s fate: in the qualifying rounds a club going through to phase 1 gets its text and one dropping out goes to the UEFA Cup ("are entered for the UEFA Cup"), unless in the base rounds 0xfa and 0x104 or with a news object (the generic news); in the first group phase qualification uses the generic cup news and elimination sends the club to the UEFA Cup; in the second group phase elimination is "knocked out of"; the final stage goes to the generic news')])
INTERESSA = ['CMRTE']
