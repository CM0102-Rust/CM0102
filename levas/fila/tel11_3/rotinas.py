# The Setup screens, part 3 (Setup.cpp; reader tel11, the screens round, 21/11/2026): Enter Name, Select
# Nationality and Select Team, the new manager's three screens. Uses the helpers of tel11_1 and two FN_ names of
# tel11_2 (goes with both).
# Conventions checked in the disassembly and at the call sites (all cdecl, ret 0): the builders () -> void, the
# handlers (short item) -> int; opened by setup_add_human_manager (Enter Name), the Enter Name handler (Select
# Nationality) and the Select Nationality handler (Select Team).
# - NATION_MENU_SPLIT 0x66a280 thiscall(split, 1, order, 0) ret 0xc, part_count 0x66a350 -> al, format_header 0x66a360
#   thiscall(split, buffer, 0x7d0, part) ret 0xc, in_second_part 0x66a430 thiscall(split, nation) ret 4 -> al; the
#   object is 0x10 bytes on the original's stack (0x40 here);
# - HUMAN_MANAGER::get_password 0x5fde40 / get_password_entry 0x5fdf40 thiscall(HM, 0 = the current one) ret 4;
#   ScreenManager::remove_manager_record 0x7ff990 thiscall(sm, key, -1) ret 8; toupper 0x9471cd, malloc 0x944e46,
#   free 0x9452ca, qsort 0x945333 cdecl (comparers 0x839530 nations, 0x540080 clubs).
# Which routines run in the automatic career: none (the career creates its manager in create_game_from_setup_file).
# How to reach them in the game: Start New Game -> leagues -> Next -> Manager Status -> Add Manager: Enter Name (type
# the names, Next) -> Select Nationality (Filter: All/Major) -> Next -> Select Team (Nation chooser with more than one
# league; a club opens the take control box of tel11_4).
# DEFECTS of the original kept (candidates for docs/DEFEITOS_DO_ORIGINAL.md):
# - Select Nationality: the chosen row lives in the filter's local, so with no nation chosen the centring uses the
#   filter value (0 or 1) as the row;
# - Select Nationality: the list holds 250 nations on a stack array whose last 4 entries overlap the saved registers.
# - 1920x1080 exe: it rewrites the column layout of both lists (Select Nationality at 0x821d1d and 0x821f60;
#   Select Team at 0x82259d, a jump to its own table at 0x601ef7, and 0x8225b2/0x82283c): their entries are for
#   the original exe only (EXE). assinaturas.py does not read EXE and the prologues are the same, so the DLL still
#   detours them in the 1080 exe: each copy checks the first patched byte and calls the exe's routine there.
EXE = {0x821900: 'original', 0x822260: 'original'}
MOTOR = True
TELAS = True
ROTINAS = [('setup_build_enter_name_screen', 0x820f50, [0x83, 0xec, 0x0c, 0x53, 0x56]),
           ('setup_handle_enter_name_input', 0x821520, [0x81, 0xec, 0xc8, 0x00, 0x00, 0x00]),
           ('setup_build_select_nationality_screen', 0x821900, [0x81, 0xec, 0x04, 0x06, 0x00, 0x00]),
           ('setup_handle_select_nationality_input', 0x822040, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('setup_build_select_team_screen', 0x822260, [0x81, 0xec, 0xfc, 0x02, 0x00, 0x00]),
           ('select_team_screen_handle_input', 0x8228f0, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('as telas de criar o jogo (3): o nome, a nacionalidade e o time do técnico novo',
        'the game creation screens (3): the new manager\'s name, nationality and team',
        [('setup_build_enter_name_screen', 'setup_build_enter_name_screen', '0x820f50', 'Setup.cpp', 'cdecl',
          'monta Enter Name: primeiro e segundo nome (26 letras), senha opcional e redigitação (só com senha), Cancel e Next; o campo do parâmetro 0 fica em foco',
          'builds Enter Name: first and second name (26 characters), optional password and re-typing (only with a password), Cancel and Next; the entry of param 0 has the focus'),
         ('setup_handle_enter_name_input', 'setup_handle_enter_name_input', '0x821520', 'Setup.cpp', 'cdecl',
          'responde a Enter Name: a senha apaga a redigitação e passa o foco; Cancel libera o registro; Next confere os nomes e as senhas (caixas de nome em branco e senhas diferentes) e abre Select Nationality',
          'answers Enter Name: the password empties the re-typing and moves the focus; Cancel frees the record; Next checks the names and the passwords (blank name and password mismatch boxes) and opens Select Nationality'),
         ('setup_build_select_nationality_screen', 'setup_build_select_nationality_screen', '0x821900', 'Setup.cpp', 'cdecl',
          'monta Select Nationality: o filtro (todas ou as 33 principais), as nações com continente em ordem, duas por linha, a escolhida destacada e centralizada na primeira vez',
          'builds Select Nationality: the filter (all or the 33 major ones), the nations with a continent in order, two per row, the chosen one marked and centred the first time'),
         ('setup_handle_select_nationality_input', 'setup_handle_select_nationality_input', '0x822040', 'Setup.cpp', 'cdecl',
          'responde a Select Nationality: o filtro, a nação clicada vira a do técnico; Next abre Select Team na nação principal',
          'answers Select Nationality: the filter, the clicked nation becomes the manager\'s; Next opens Select Team on the main nation'),
         ('setup_build_select_team_screen', 'setup_build_select_team_screen', '0x822260', 'Setup.cpp', 'cdecl',
          'monta Select Team: o seletor de nação (com mais de uma liga, em duas partes quando a lista é grande) e os clubes que podem ser dirigidos, em ordem, dois por linha, com a nação e a divisão',
          'builds Select Team: the nation chooser (with more than one league, in two parts when the list is long) and the clubs that can be managed, in order, two per row, with the nation and the division'),
         ('select_team_screen_handle_input', 'select_team_screen_handle_input', '0x8228f0', 'Setup.cpp', 'cdecl',
          'responde a Select Team: a nação escolhida no seletor troca a lista; o clube vai pelo menu (a caixa de assumir o controle)',
          'answers Select Team: the nation picked in the chooser changes the list; the club goes through the menu (the take control box)')])
