# The staff profile's Profile tab (staff_screens.cpp, part 2; reader tel08, the screens round, 21/11/2026).
# - 0x87a510 staff_screens_draw_profile_tab (10358 bytes, 0x87a510..0x87cd85): cdecl(int top, int bottom), ret 0, SEH
#   frame (a Text, 0x1f8 bytes of locals). Called by the staff profile builder (0x870700) for its Profile tab. Split
#   into named pieces: profile_tab_player_attributes, profile_tab_non_player (with profile_tab_chairman and
#   profile_tab_formation) and profile_tab_match_history; the whole routine is hooked.
# Conventions checked at the call sites: as tel08_1, plus 0x7f8610 find_club_search_entry thiscall ret 4 (ecx =
#   [G_SCOUT_MANAGER]); 0x7bd830 get_match_entry thiscall ret 4; 0x549ef0 Date::Date thiscall ret 8; 0x729500
#   fixture_entry_matches thiscall ret 0x14; 0x8b1750 update_player_training thiscall ret 8; 0x8c1870 free transfer
#   thiscall ret 4; 0x7c1660 format_recent_ratings thiscall ret 0xc; 0x4cc2e0 write_morale_text stdcall ret 0xc;
#   0x62c400/0x62c590 injury status/condition thiscall; 0x4afa40 Text::clear; 0x55f990 Display::add_item ret 0x48;
#   cdecl: 0x5428a0, 0x543440, 0x543540, 0x543b10, 0x543fd0, 0x544410, 0x7e4ca0, 0x945333 qsort (comparer 0x88bc50).
# Uses the helpers of tel26_1 and tel08_1 (staff_tab_*, contract_tab_lookup): it goes after both.
# Not in the community map's split: the map names it staff_screens_draw_profile_tab (3153 instructions); the queue
#   list gave it 10368 bytes (the ret is at 0x87cd85).
# Kept from the original: stack garbage in the high halves of the colour words and of the byte counts (zero here);
#   the On Loan row's empty item is added even when neither the loan nor the free transfer line follows.
# Comparer: side by side by the display list (compare_match_screen_build) plus the attribute table 0xddf570 (36 rows)
#   and G_CONTRACT_LOOKUP_RESULT. Both sides call update_player_training for a player (as tel07_2).
# How to reach it: a person's profile (click a name) > Profile tab: a player (attributes, foot, form, morale,
#   condition, on loan / free transfer), a coach (non-player attributes and the formation/style table), a chairman
#   (the chairman table; needs the state bit 0x40), and the scouting view (parameter 4 = 2) of a scouted player (the
#   match history, the << >> arrows and the four statistics views).
TELAS = True
ROTINAS = [('staff_screens_draw_profile_tab', 0x87a510, [0x6a, 0xff, 0x68, 0xd6, 0x2e, 0x96, 0x00])]
DOCS = ('a aba Perfil do perfil da pessoa',
        "the staff profile's Profile tab",
        [('staff_screens_draw_profile_tab', 'staff_screens_draw_profile_tab', '0x87a510', 'staff_screens.cpp', 'cdecl, SEH',
          'Desenha a aba Perfil (a pessoa do parâmetro 0, modo no parâmetro 4) entre top e bottom: sem perfil ("There is no profile available.") para cargo 2 ou id a partir da contagem de pessoas − 16; no modo 2 (olheiro) o histórico de jogos quando há a entrada da busca (data, placar e adversário ligados, mando, as seis estatísticas da visão do parâmetro 0x13 e a nota, até 10 jogos), senão os atributos; no modo 0 os atributos do jogador: atualiza o treino, os 31 atributos (o valor que o humano vê com o mascaramento ligado) ordenados pelo nome e as linhas de pé preferido, forma, moral, condição e emprestado/transferência livre, em três pares de colunas de 12 linhas; no modo 1 o perfil de quem não é jogador: os 12 atributos (o 10º só com cargo 10 ou valor ≥ 10) e o estilo de treino em duas colunas, a tabela do presidente (negócios, disciplina, interferência, trato, paciência, recursos, ambição, com as descrições por faixa) quando o bit 0x40 do estado está ligado, e a formação e o estilo preferidos (ataque e objetividade) com marcação, linha de impedimento e pressão',
          'Draws the Profile tab (the person of parameter 0, view in parameter 4) between top and bottom: no profile ("There is no profile available.") for job 2 or an id from the person count - 16 up; in view 2 (scouting) the match history when there is a search entry (date, score and opposition linked, venue, the six statistics of the view of parameter 0x13 and the rating, up to 10 matches), else the attributes; in view 0 the player attributes: updates the training, the 31 attributes (the value the human sees with masking on) sorted by name and the rows of preferred foot, form, morale, condition and on loan/free transfer, in three column pairs of 12 rows; in view 1 the non-player profile: the 12 attributes (the 10th only with job 10 or value >= 10) and the coaching style in two columns, the chairman table (business, discipline, interference, man handling, patience, resources, ambition, with the descriptions by band) when the state bit 0x40 is set, and the preferred formation and style (attacking and directness) with marking, offside trap and pressing')])
