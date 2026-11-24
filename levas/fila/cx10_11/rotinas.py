# Os codigos de progresso da Primeira Divisao finlandesa (fin_prm.cpp; leitor cx10, rodada 5, 24/11/2026). Competicao de
# outra nacao: roda a cada rodada da liga finlandesa (abril a outubro: a carreira de julho de 2001 pega o fim de 2001 e
# o comeco de 2002).
# - 0x596020 FinPrm::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (slot SET_CLUB_PROGRESS):
#   Comp_Stats::find_club_entry 0x4a4850 (mov ecx, [this+8]; push clube -> thiscall, ret 4, a entrada sem conferir); o
#   pior 11 vira 12; os tres bytes em +0x73, +0x74, +0x75. Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# Iguais no cm0102-1080.exe (inteira).
# Comparador: compare_club_progress do cx02_4 (lado a lado sobre os tres bytes da entrada).
# Depende do cx02_4 (compare_club_progress) e do cx03_9 (store_club_progress): integrar depois deles.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ROTINAS = [('fin_prm_set_club_progress', 0x596020, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os códigos de progresso da Primeira Divisão finlandesa',
        'the progress codes of the Finnish Premier Division',
        [('fin_prm_set_club_progress', 'FinPrm::set_club_progress', '0x596020', 'fin_prm.cpp', 'thiscall, ret 0x14',
          'guarda os três códigos de progresso do clube nas estatísticas; o pior código 11 vira 12',
          'stores the club\'s three progress codes in the statistics; the worst code 11 becomes 12')])
CAMPOS = []
INTERESSA = []
