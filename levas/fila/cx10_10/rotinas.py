# Os codigos de progresso da Divisao Principal holandesa e da Regionalliga alema (hol_prm.cpp, ger_regional.cpp; leitor
# cx10, rodada 5, 24/11/2026). Competicoes de outras nacoes: a Eredivisie roda na carreira com a Holanda (a cada rodada
# de liga); a Regionalliga nao e carregada no teste (0 chamadas ate 2002-08-01 na cobertura do mundo; o mwc_21 diz o
# mesmo), entao a GerRegional::set_club_progress entra sem prova pela carreira: a comparacao a provaria com a
# Regionalliga carregada.
# - 0x5f87e0 HolPrm::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (slot SET_CLUB_PROGRESS):
#   Comp_Stats::find_club_entry 0x4a4850 (mov ecx, [this+8]; push clube -> thiscall, ret 4, a entrada sem conferir);
#   melhor 0x10..=0x11 vira 0xf, pior 0x10..=0x11 vira 0x12; os tres bytes em +0x73, +0x74, +0x75.
# - 0x5de2a0 GerRegional::set_club_progress: o mesmo com cada codigo c virando 2c - 1 (byte).
# Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes). Iguais no cm0102-1080.exe (inteiras).
# Comparadores: compare_club_progress do cx02_4 (lado a lado sobre os tres bytes da entrada).
# Depende do cx02_4 (compare_club_progress) e do cx03_9 (store_club_progress): integrar depois deles.
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ROTINAS = [('hol_prm_set_club_progress', 0x5f87e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('ger_regional_set_club_progress', 0x5de2a0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os códigos de progresso da Divisão Principal holandesa e da Regionalliga alemã',
        'the progress codes of the Dutch Premier Division and the German Regional League',
        [('hol_prm_set_club_progress', 'HolPrm::set_club_progress', '0x5f87e0', 'hol_prm.cpp', 'thiscall, ret 0x14',
          'guarda os três códigos de progresso do clube nas estatísticas; os códigos da repescagem 16 e 17 viram 15 no melhor e 18 no pior',
          'stores the club\'s three progress codes in the statistics; the play-off codes 16 and 17 become 15 in the best and 18 in the worst'),
         ('ger_regional_set_club_progress', 'GerRegional::set_club_progress', '0x5de2a0', 'ger_regional.cpp', 'thiscall, ret 0x14',
          'guarda os três códigos de progresso do clube nas estatísticas como ímpares: cada código c vira 2c − 1',
          'stores the club\'s three progress codes in the statistics as odd values: each code c becomes 2c − 1')])
CAMPOS = []
INTERESSA = []
