# As posicoes previstas do Mundial de Clubes (world_club_champ.cpp; leitor rx10, rodada 6, 25/11/2026). Simulacao com
# as 8 ligas e ATE 2003-08-15 (ver rx10_11): roda no sorteio de 5/fev/2003 (virtual +0x5c, de update_for_date), antes do
# primeiro grupo; o embaralhamento dela e a ordem dos inscritos.
# - 0x92aba0 WorldClubChamp::set_predicted_positions thiscall() ret 0: o molde do Oceania (0x798640): sem
#   estatisticas a caixa 0x429; qsort da lista +0xee (o dword +0xf2) com 0x4b07d0, as posicoes 1, 2, 3, 4, 5, 5, 7, 7,
#   7, 10, 10, 10 (Comp_Stats 0x4a2540) e comp_util_shuffle_pointer_array 0x4afb40(lista, a palavra +0xf2). Prologo
#   sub esp, 0x200 (6 bytes).
# Iguais no cm0102-1080.exe. Comparador: so a original (embaralha), com o plano (predicted_by_reputation_check) e os
# mesmos inscritos.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('world_club_champ_set_predicted_positions', 0x92aba0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as posições previstas do Mundial de Clubes', 'the Club World Championship predicted positions',
        [('world_club_champ_set_predicted_positions', 'WorldClubChamp::set_predicted_positions', '0x92aba0', 'world_club_champ.cpp', 'thiscall, ret 0',
          'regra: as posições previstas do Mundial de Clubes: os doze inscritos pela reputação, o melhor primeiro, nas posições 1, 2, 3, 4, 5, 5, 7, 7, 7, 10, 10, 10; depois a ordem dos inscritos é embaralhada',
          'rule: the Club World Championship predicted positions: the twelve entrants by reputation, the best first, in places 1, 2, 3, 4, 5, 5, 7, 7, 7, 10, 10, 10; then the entrant order is shuffled')])
CAMPOS = []
INTERESSA = []
