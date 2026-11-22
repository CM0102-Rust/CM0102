# The Setup screens, part 5 (Setup.cpp, part 2; reader tel12, the screens round, 21/11/2026): the Game Settings box.
# Uses the Setup helpers and names of tel11_1 (setup_clicked_id, setup_clicked_value, setup_colour, setup_answer_regions,
# SETUP_TEXT, FN_OPEN_GAME_SETTINGS_BOX, FN_SELECT_DEFAULT_LEAGUE_BY_LANGUAGE): goes after the tel11 items.
# Conventions checked in the disassembly and at the call sites (all cdecl, ret 0):
# - 0x823890 open_game_settings_box(origin) (SEH frame for the operator new 0x31b + Config::Config 0x5d7ad0 thiscall
#   ret 4); 0x823930 (item) -> 0x823890(2 or 3); 0x823950 build (); 0x8259a0 input(short item) -> int.
# - ScreenManager::add_text_item 0x803fd0 thiscall (11 args) ret 0x2c; add_button_item 0x804030 (12) ret 0x30;
#   add_area_button 0x8040a0 (11) ret 0x2c; add_area_text 0x804100 (9) ret 0x24; Display::add_area 0x55fba0 (11) ret
#   0x2c; Display::add_item 0x55f990 (18) ret 0x48; draw_pitch 0x8ac600 cdecl (11 pushed); the Config methods thiscall
#   (set_defaults 0x5d7bc0, set_language 0x5d8020, set_currency 0x5d8270, set_option 0x5d7c60, set_font 0x5d8b50,
#   set_auto_save_interval 0x5d83e0 ret 4; apply_and_save 0x5d8480, ~Config 0x5d7b70, get_font_index 0x5d8b40 ret 0).
# - The colour words go in registers whose high word is leftover (the menu areas' green even through a stack slot); the
#   callees keep the word: here zero-extended (as tel11).
# - utils_get_version_string 0x90d130 takes nothing: in English the two strings pushed for it ("Nov 21 2002",
#   "13:10:31") stay on the stack and are the 2nd and 3rd arguments of the version text.
# Which routines run in the automatic career: none (the coverage counts 0 for the block; with auto_setup.cfg the
# settings box is never opened).
# DEFECTS of the original kept (candidates for docs/DEFEITOS_DO_ORIGINAL.md):
# - Game Settings, Auto Save row: when no choice matches the Config (bit 0x2000 with an interval other than 7, 14, 30,
#   90, 180 or 360 days, or neither 0x1000 nor 0x2000), the current value shows the name buffer's last text (the last
#   font file name or the last currency).
MOTOR = True
TELAS = True
ROTINAS = [('open_game_settings_box', 0x823890, [0x6a, 0xff, 0x68, 0x6b, 0x13, 0x96, 0x00]),
           ('open_game_settings_from_side_bar', 0x823930, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('game_settings_box_build', 0x823950, [0xa0, 0x0f, 0x71, 0xdd, 0x00]),
           ('game_settings_box_handle_input', 0x8259a0, [0x51, 0x53, 0x55, 0x56, 0x57])]
DOCS = ('as telas de criar o jogo (5): a caixa Game Settings',
        'the game creation screens (5): the Game Settings box',
        [('open_game_settings_box', 'open_game_settings_box', '0x823890', 'Setup.cpp', 'cdecl',
          'abre a caixa Game Settings com uma cópia da configuração viva (parâmetro 0) e a origem (parâmetro 1: 0 a primeira vez, 1 Setup Game, 2/3 a barra lateral)',
          'opens the Game Settings box with a copy of the live Config (param 0) and the origin (param 1: 0 the first time, 1 Setup Game, 2/3 the side bar)'),
         ('open_game_settings_from_side_bar', 'open_game_settings_from_side_bar', '0x823930', 'Setup.cpp', 'cdecl',
          'abre a caixa Game Settings pela barra lateral: origem 2 para o item 0, 3 para os outros',
          'opens the Game Settings box from the side bar: origin 2 for item 0, 3 for the others'),
         ('game_settings_box_build', 'game_settings_box_build', '0x823950', 'Setup.cpp', 'cdecl',
          'monta a caixa Game Settings: título, versão, Reset, Cancel e Ok, e as linhas idioma (com dois ou mais), moeda, fontes, salários, modo de tela, troca de fundo, som, banco de dados, partidas guardadas, gravação automática, gravar comprimido, gravar com segurança, texto piscante, partidas de fundo e imprimir; origem 3 só leitura nas listas, 2 e 3 sem mudar banco e partidas guardadas',
          'builds the Game Settings box: title, version, Reset, Cancel and Ok, and the rows language (with two or more), currency, fonts, wages, screen mode, background changes, sound, database, matches stored, auto save, save compressed, save safely, flashing text, background matches and print to; origin 3 read-only lists, 2 and 3 cannot change the database and matches stored'),
         ('game_settings_box_handle_input', 'game_settings_box_handle_input', '0x8259a0', 'Setup.cpp', 'cdecl',
          'responde à caixa Game Settings: Reset volta a cópia ao padrão; Cancel reaplica a configuração viva e libera a cópia; Ok aplica a cópia e a libera; idioma (na abertura, com banco, escolhe a liga padrão pelo idioma), fonte, moeda, opção e intervalo da gravação automática mudam a cópia',
          'answers the Game Settings box: Reset puts the copy back to the defaults; Cancel applies the live Config again and frees the copy; Ok applies the copy and frees it; language (in Setup, with a database, it picks the default league by language), font, currency, option and auto save interval change the copy')])
