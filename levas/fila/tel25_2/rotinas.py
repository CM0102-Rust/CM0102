# The training screens (training_screens.cpp, part 2; reader tel25, the screens round, 21/11/2026): opening the
# training screen and the schedule editor, the schedule editor's input and the coaches panel (its builder and input).
# Uses the helpers of tel25_1 (TRAINING_SCREENS_CPP, TRAINING_SCHEDULE_SIZE), tel26_1 (tel26_error_box,
# tel26_add_area, tel26_area_button, tel26_area_text, tel26_item_id, tel26_item_value) and tel22_1 (comp_lang_format,
# comp_text_item): those go first.
# Conventions checked in the disassembly:
# - 0x8b5930 cdecl(club), SEH: 0x7fe600 open_screen thiscall(0x8b5a10, 0x8b83f0, 1, 0, 0x8b97a0) ret 0x14; 0x944cf1
#   operator new(0x819); 0x8b0a00 TRAINING_EDIT_SESSION ctor thiscall(club) ret 4 -> this; 0x7ff270 set_screen_param
#   thiscall(k, v, 0) ret 0xc.
# - 0x8b97e0 cdecl(session), SEH: the error box (line 0x6dc = 1756); operator new(0x9b); 0x8b4e10 TRAINING_SCHEDULE
#   ctor thiscall ret 0 -> this; rep movs 0x9b bytes from session + 0xfa + 0x9b x (i8)[session+0x7d9]; open_screen
#   thiscall(0x8b9940, 0x8ba530, 1, 0, 0); set_screen_param(0, session, 0), (1, copy, 0), (2, copy + 9, 1).
# - 0x8ba530 cdecl(widget) -> eax: 0x8b4f30 set_category_level thiscall(category, level) ret 8; 0x8b5090 / 0x8b50c0
#   thiscall(data) ret 4; 0x468b10 Display::get_widget_id thiscall(w) ret 4 -> ax; 0x4739a0 get_widget_data
#   thiscall(w) ret 4; 0x468b40 ScreenManager::get_widget_action_id thiscall(w) ret 4; 0x8b0c90
#   set_current_schedule thiscall(schedule) ret 4; 0x8ba920 thiscall(1) ret 4; 0x944c94 operator delete; 0x7643d0
#   menubar_handle_object_link cdecl(w).
# - 0x8ba940 cdecl: 0x5e4800 graphics_rgb_to_pixel cdecl(r, g, b, 0); 0x41c0d0 background_set_picture cdecl(0);
#   0x55fba0 Display::add_area thiscall(11) ret 0x2c on the current human's Display; 0x66f4e0 langlib_sprintf;
#   0x803fd0 add_text_item thiscall(11) ret 0x2c; 0x8040a0 add_area_button thiscall(11) ret 0x2c; 0x804100
#   add_area_text thiscall(9) ret 0x24; 0x8ac600 draw_pitch cdecl(11); 0x53dba0 cdecl(person, 0xde1f65, 0x7d0, 1);
#   0x8b5520 training_schedule_get_category_name cdecl(category, buf, 0x7d0); jump table 0x8bae84 (6 rows).
#   The 1080 exe patches the full-screen area's edges (push 0x257 at 0x8baa57, push 0x31f at 0x8baa5f): read from the
#   running exe.
# - 0x8baea0 cdecl(widget) -> eax: 0x9452ca crt free; 0x8b0ce0 set_coach_record thiscall(edit) ret 4.
# - Colours: the original pushes some colour words with the high half of whatever register held them (a pointer or a
#   computed index); the copy pushes the word with a zero high half (the display keeps the colour as a word).
# Defects of the original: open_schedule_editor copies into the new schedule without checking the allocation (a
# failed operator new would write at 0); the coaches panel builder's jump table has a default (box 2378) no row
# reaches.
MOTOR = True
TELAS = True
ROTINAS = [('training_screens_open', 0x8b5930, [0x6a, 0xff, 0x68, 0x7b, 0x38, 0x96, 0x00]),
           ('training_screens_open_schedule_editor', 0x8b97e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('training_screens_schedule_editor_handle_event', 0x8ba530, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('training_screens_draw_coaches_panel', 0x8ba940, [0xa0, 0x48, 0x1f, 0xde, 0x00]),
           ('training_screens_coaches_panel_handle_event', 0x8baea0, [0x51, 0x53, 0x55, 0x56, 0x57])]
DOCS = ('as telas de treino (2): abrir a tela de treino e o editor de programa, a entrada do editor e o painel dos treinadores',
        'the training screens (2): opening the training screen and the schedule editor, the editor\'s input and the coaches panel',
        [('training_screens_open', 'training_screens_open', '0x8b5930', 'training_screens.cpp', 'cdecl',
          'abre a tela de treino de um clube: uma sessão de edição nova do treino do elenco no parâmetro 0; 1, 2 e 3 zerados, 4 = 1, 5 = -1',
          'opens a club\'s training screen: a new edit session of the squad\'s training in parameter 0; 1, 2 and 3 cleared, 4 = 1, 5 = -1'),
         ('training_screens_open_schedule_editor', 'training_screens_open_schedule_editor', '0x8b97e0', 'training_screens.cpp', 'cdecl',
          'abre o editor sobre uma cópia do programa atual da sessão (sem sessão, a caixa de erro 1756); parâmetros: a sessão, a cópia e o nome da cópia',
          'opens the editor on a copy of the session\'s current schedule (no session: error box 1756); parameters: the session, the copy and the copy\'s name'),
         ('training_screens_schedule_editor_handle_event', 'training_screens_schedule_editor_handle_event', '0x8ba530', 'training_screens.cpp', 'cdecl',
          'a entrada do editor: os botões de nível põem a categoria em 0, 10, 25 ou 50; 7 e 8 o treino de posição e de lado; nome trocado marca o programa (+0x15 = 8, +0x93 = 0); 2 cancela; Ok guarda a cópia na sessão',
          'the editor\'s input: the level buttons set the category to 0, 10, 25 or 50; 7 and 8 the position and side training; a changed name marks the schedule (+0x15 = 8, +0x93 = 0); 2 cancels; Ok keeps the copy in the session'),
         ('training_screens_draw_coaches_panel', 'training_screens_draw_coaches_panel', '0x8ba940', 'training_screens.cpp', 'cdecl',
          'monta o painel dos treinadores: tira da lista quem não é mais auxiliar nem treinador do clube; cada linha com o nome do treinador e as 5 categorias (amarelas as que ele assume); Cancelar e Ok',
          'builds the coaches panel: drops from the list whoever is no longer the club\'s assistant or coach; each row with the coach\'s name and the 5 categories (yellow those he takes); Cancel and Ok'),
         ('training_screens_coaches_panel_handle_event', 'training_screens_coaches_panel_handle_event', '0x8baea0', 'training_screens.cpp', 'cdecl',
          'a entrada do painel: um botão de categoria liga ou desliga a categoria daquele treinador; Cancelar descarta; Ok grava na sessão',
          'the panel\'s input: a category button toggles that coach\'s category; Cancel discards; Ok writes it into the session')])
