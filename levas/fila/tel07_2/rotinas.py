# The compare players screen (staff_screens.cpp, part 1; reader tel07, the screens round, 21/11/2026).
# - 0x876520 staff_screens_build_compare_players_screen (16364 bytes, 0x876520..0x87a509; not in the community map: it
#   is the builder 0x870650 passes to open_screen): cdecl(void), ret 0, SEH frame, 0x2644 bytes of locals (a Text, a
#   second Text in the statistics, 25 MenuItem built by the CRT vector iterators 0x9463af/0x9462bb, stdcall, and never
#   used). Split into named pieces: compare_screen_heading, compare_screen_positions, compare_screen_position_table,
#   compare_screen_statistics (with compare_screen_player_stats, compare_screen_foot); the whole routine is hooked.
# Conventions checked at the call sites: 0x55fba0 Display::add_area thiscall ret 0x2c; 0x804100 add_area_text thiscall
#   ret 0x24; 0x803fd0 add_text_item ret 0x2c; 0x55f990 add_item ret 0x48; 0x4aeaf0 Text::draw_wrapped thiscall ret
#   0x20; 0x4afa40 Text::clear, 0x4ae630 ~Text thiscall ret 0; 0x4ae660 set_string ret 4; 0x4ae8a0 add_link ret 0xc;
#   0x54a650 Date::get_short_date_string thiscall ret 0xc; 0x7c25b0 PlayerStats::format_player_stat thiscall ret 0x20;
#   0x7c1660 format_recent_ratings ret 0xc; 0x4cc3a0 write_morale_comparison_text ret 0x10; 0x4cf6d0/0x4cf5a0
#   contracts ret 4; 0x8c1870 free transfer ret 4; 0x7f8610 find_club_search_entry ret 4; 0x8b1750
#   update_player_training ret 8; cdecl: 0x41c790, 0x53dba0, 0x5e5ad0, 0x5e5cd0, 0x75db00, 0x66f4e0, 0x944c9f,
#   0x6181d0, 0x9471cd (toupper), 0x53b3d0, 0x53ee50, 0x53ad00, 0x5428a0, 0x543440, 0x543540, 0x945333 qsort, 0x5ed070,
#   0x5edcc0.
# The original's inline strcpy/strcat/strcmp are pure Rust here (the same bytes).
# Kept from the original (DEFECTS, docs/DEFEITOS_DO_ORIGINAL.md candidates): the masked marks (0xdd7f6c) are not sorted
#   with the three tables, so in a language whose attribute names are not in the numbering order the " -" lands on
#   another row; the Avg. Rating row computes the recent ratings (format_recent_ratings) and shows the statistics'
#   averages instead; the positions buffers get 2000 as their size (they are 200 bytes); with no second player the
#   heading prints stack garbage after "~" (here empty). Stack garbage in the high halves of the colours and of the
#   attribute dword: zero here.
# Comparer: side by side by the display list (compare_match_screen_build) plus the compare tables, the position words
#   and marks and 0xae1080. Both sides call update_player_training (training_manager) for the first player.
# How to reach it: choose two players to compare (the profile's Action menu > "Compare with..." / the scouting), then
#   the compare screen; the position table needs the first one to be a player of the database.
TELAS = True
ROTINAS = [('staff_screens_build_compare_players_screen', 0x876520, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a tela de comparação de jogadores',
        'the compare players screen',
        [('staff_screens_build_compare_players_screen', 'staff_screens_build_compare_players_screen', '0x876520', 'staff_screens.cpp', 'cdecl, SEH',
          'Monta a tela de comparação dos dois jogadores da comparação do olheiro (sem o primeiro, o da tela): o cabeçalho com os dois nomes ligados, idades e nacionalidades (a segunda nação só sem jogos pela seleção e se não for Portugal) e as posições ou o cargo; para um jogador do banco, a tabela das posições (35 para quem pode jogar lá contra quem não pode, duas vezes a diferença quando os dois podem, "n/a" quando nenhum; lateral e ponta pelas regras próprias, somando a diferença dos atributos que contam para cada posição) em estrelas de 6/12/19/27/36, a legenda; e a tabela das estatísticas em três colunas: a diferença de cada um dos 31 atributos (ordenados pelo nome; " -" mascarado), os pés, as notas, a moral, gols ou sofridos (dois goleiros), jogos, assistências, melhor em campo, e o empréstimo ou a transferência livre; sem dois jogadores, "Need two players to compare."',
          'Builds the compare screen of the scouting manager\'s two compare players (without the first, the screen\'s player): the heading with both names linked, ages and nationalities (the second nation only without caps and when it is not Portugal) and the positions or the job; for a player of the database, the position table (35 for one who can play there against one who cannot, twice the difference when both can, "n/a" when neither; full back and winger by their own rules, adding the difference of the attributes each position counts) in stars by 6/12/19/27/36, the legend; and the statistics table in three columns: the difference of each of the 31 attributes (sorted by name; " -" masked), the feet, the ratings, the morale, goals or conceded (two goalkeepers), appearances, assists, man of the match, and the loan or the free transfer; without two players, "Need two players to compare."')])
