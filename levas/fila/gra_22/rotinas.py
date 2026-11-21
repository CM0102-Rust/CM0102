# The European Championship qualifying draw (euro_champ_qual.cpp, in comp\intercomp\). Reader r5_C, round 4 (the big
# routines), 21/11/2026. Marked 8L in gra.txt.
# - 0x57dab0 EuroChampQual::draw_groups (3376 bytes, 2701 of them the fixed 1998 table): 1 call in the 8-league career to
#   2002-06-01 (docs/cobertura_competicoes.txt), from the qualifiers' daily update (0x57fe3c, right after the entrants
#   0x57f600, which sets +0xf2 = 51; +0xff = 7 from the constructor 0x57ce10, so 9 groups), on the draw date (+0xb6:
#   10 March of the edition year, set_up_rules [inference: 2002-03-10]). Only the random draw runs (the edition is not
#   1998).
# Equal in cm0102-1080.exe (whole).
# Conventions checked in the disassembly (0x57dab0..0x57e7de) and at the caller (0x57fe3a: mov ecx, esi; call -> thiscall,
# ret 0):
# - 0x944e46 (malloc): 1 push; add esp, 4 -> cdecl; the error box (show_error_at, line 0x262);
# - 0x53b3d0 (database_get_national_team): 1 push, the add esp batched every 16 calls -> cdecl;
# - 0x90cfc0 (utils_random): 1 push each, add esp, 8 after the pair -> cdecl;
# - 0x57d9d0 (EuroChampQual::get_group_team_count): push eax (low byte g - 1); mov ecx, esi -> thiscall, ret 4; the
#   return compared as a word.
# Defect of the original (DEFEITOS): each group's room is asked for the previous group (g - 1), so in format 1 the groups
# of six are 1, 3 and 6 instead of 0, 2 and 5, and group 0 asks for group -1 (5).
# Comparer: the original runs for real; from the generator state before it the copy draws again into a local array
# without counting the draws; the generator state after both and every slot the copy wrote must match.
# Proof: the draw does not reach the career before 2002-06-01 (the qualifiers start in September 2002): the mutations
# are caught by the comparer, in a comparison of the 8 leagues that passes the draw date (2002-03-10 [inference]).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('euro_champ_qual_draw_groups', 0x57dab0, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00])]
DOCS = ('o sorteio das eliminatórias da Eurocopa',
        'the European Championship qualifying draw',
        [('euro_champ_qual_draw_groups', 'EuroChampQual::draw_groups', '0x57dab0', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'O sorteio dos grupos das eliminatórias da Eurocopa: em 1998, a tabela fixa (49 seleções em 9 grupos, os grupos 1, 3, 6 e 8 com seis); nos outros anos, 9 grupos (+0xff + 2) e até seis potes: a ordem dos grupos é embaralhada com 20 trocas sorteadas e cada grupo, na ordem, recebe a próxima seleção enquanto tiver vaga; defeito: a vaga de cada grupo é a do grupo anterior (g − 1), então no formato 1 os grupos de seis ficam 1, 3 e 6',
          'The groups draw of the European Championship qualifiers: in 1998, the fixed table (49 national teams in 9 groups, groups 1, 3, 6 and 8 with six); in other years, 9 groups (+0xff + 2) and up to six pots: the group order is shuffled by 20 drawn swaps and each group in turn takes the next national team while it has room; defect: each group room is that of the previous group (g − 1), so in format 1 the groups of six are 1, 3 and 6')])
