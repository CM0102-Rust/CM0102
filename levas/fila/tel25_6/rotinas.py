# The training screens (training_screens.cpp, part 5; reader tel25b, the screens round, 21/11/2026): the training
# screen's input. Uses tel25_5 (the schedule and view constants, training_schedule, training_current_schedule,
# TRAINING_EMPTY_TEXT), tel25_1 (TRAINING_SCREENS_CPP, TRAINING_SQUAD_VIEW), tel25_2 (TRAINING_TEXT_BUFFER), tel26_1
# (tel26_item_id, tel26_item_value, tel26_error_box) and tel22_1 (comp_lang_format): those go first.
# Conventions checked in the disassembly:
# - 0x8b83f0 cdecl(widget) -> eax, 0x56c bytes of locals: the dialogs' name (200 bytes), two 200-byte texts and the
#   0x104-byte path. 0x803320 get_area_scroll_row thiscall(area) ret 4; 0x9466d0 strncpy(name, src, 200).
# - The dialog calls keep their arguments on the stack across the formats (MSVC): 0x58f500 cdecl(title, header, mode,
#   folder, ".tsh", default, 0x8b57e0, p1, p2, code, 0, 0, 0, 0) after langlib_sprintf(header buffer, prompt, the same
#   arguments) and langlib_sprintf(title buffer, title, header, ...); 0x591840 cdecl(title, header, body, question, p1,
#   p2, code) after the formats of body, header and title; the copy passes the same extra arguments to the formats.
# - 0x5fc5b0 thiscall on [0xb63c98](0, 3) ret 8 -> the folder; 0x5fc760 thiscall(param 4, path, 0x104, 0, 0, 3)
#   ret 0x18; 0x8b0e00 load / 0x8b0e70 save thiscall(path) ret 4; 0x944cff (_splitpath) cdecl(file, 0, 0, schedule
#   +0x2e, 0); 0x94643a remove; 0x945b13 _makepath(path, "", folder, name, ".tsh"); 0x944c94 operator delete;
#   0x8b9740 ~TRAINING_EDIT_SESSION thiscall; 0x8b0bf0 set_current_schedule thiscall(value) ret 4; 0x8702a0
#   cdecl(person, 1, 1, 0, 0); 0x8b0dc0 thiscall(person, schedule) ret 8 (reads the low byte; the original pushes a
#   stack dword with three garbage high bytes); 0x944e46 malloc(0x36); 0x7fe600 open_screen thiscall(0x8ba940,
#   0x8baea0, 1, 0, 0); 0x8b97e0 cdecl(session); 0x8b0b40 apply_changes thiscall; 0x7615d0 / 0x7643d0 cdecl(widget).
# - The link is asked again before each test (the item id, 0 -> [0xdd6bde]); nothing between the tests changes it, so
#   the copy asks once after the dialog answers (tel26_item_id).
# - Jump tables 0x8b96dc/0x8b96e4 (filter: 1, 2, 4, 8, 0x10 clear 0x1f; others return) and 0x8b96f4/0x8b9700 (view:
#   1, 2, 4, 8 clear 0x800000f; 0x10, 0x20, 0x40 clear 0x3f0; others return).
# - The 1080 exe changes nothing in 0x8b83f0..0x8b9740.
# Defects of the original: the dialogs' name buffer is left uninitialised when the widget is -1 outside a network
# game (an injected Save would build the path from stack garbage; the copy uses zeros); the coaches copy leaks when
# the panel does not open; Ok calls apply_changes on a null session.
MOTOR = True
TELAS = True
ROTINAS = [('training_screens_handle_event', 0x8b83f0, [0x81, 0xec, 0x6c, 0x05, 0x00, 0x00])]
DOCS = ('as telas de treino (5): a entrada da tela de treino',
        'the training screens (5): the training screen input',
        [('training_screens_handle_event', 'training_screens_handle_event', '0x8b83f0', 'training_screens.cpp', 'cdecl',
          'a entrada da tela de treino: as abas trocam o programa atual; Carregar, Salvar, Salvar como, Apagar e os arquivos recentes passam por janelas de arquivo (perguntando antes se o programa não foi salvo) e pelas respostas pendentes 0xc8..0xcf; arrastar um jogador para uma aba muda o programa dele, para ele mesmo abre o jogador; filtro, ordem e vista mudam a vista do elenco; Treinadores abre o painel, Editar o editor; Cancelar (perguntando se algo mudou) e Ok (aplicando) saem',
          'the training screen input: the tabs change the current schedule; Load, Save, Save As, Delete and the recent files go through file dialogs (asking first when the schedule is not saved) and the pending answers 0xc8..0xcf; a player dragged onto a tab changes his schedule, onto himself opens the player; filter, sort and view change the squad view; Coaches opens the panel, Edit the editor; Cancel (asking when something changed) and Ok (applying) leave')])
