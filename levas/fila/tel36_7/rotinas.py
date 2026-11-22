# The staff note screen (notes.cpp; reader tel36, the screens round, 21/11/2026). Uses tel36_1 (tel36_staff_param,
# tel36_langlib), tel36_5 (TRANSFER_RULES_DEFAULT_CLUB_KEY), tel26_1 (tel26_add_area, tel26_add_item,
# tel26_area_button, tel26_area_text, tel26_item_id, tel26_item_value, job_colour), tel26_4 (manager_menu_bar) and
# tel35_1 (tel35_text_draw_wrapped, TEL35_NETWORK_FLAG).
# Conventions checked in the disassembly: the open cdecl(Staff*); the builder cdecl(void) with an SEH frame (a local
# Text); the handler cdecl(short) -> eax with an SEH frame (two local Dates). On G_NOTES (0xb7433c), thiscall:
# find_note 0x795750 (human, person) ret 8; clear_edit_text 0x795a60 (human) ret 4; set_edit_text 0x795a10 (human,
# text) ret 8; get_edit_text 0x7959e0 (human) ret 4; notes_get_reminder_label 0x795a90 (out, 2000, k) ret 0xc;
# notes_get_reason_label 0x795b90 (out, 2000, k, 2) ret 0x10; notes_is_reason_valid_for_staff 0x796100 (human, person,
# k) ret 0xc -> al; 0x795710 (human, person, text, &date, &date, reason) ret 0x18; 0x7956d0 and 0x7953d0 (human,
# person, text, choice, reason) ret 0x14; 0x795660 (human, person) ret 8. Date::days_since 0x54cdb0 thiscall(date, day,
# leap) ret 8; Date::get_long_date_string 0x54ba20 thiscall(date, out, 1000, 0xff) ret 0xc. The handler pushes the
# choice and the reason (and the builder and the open the loop counters) as dwords whose high bytes are the frame's;
# the callees read the bytes (the copy passes the bytes). Display::add_area reads the rows as a byte.
# Equal in cm0102-1080.exe.
# Comparers: the builder side by side over the display list; the handler's choices side by side over the screen
# params; the open, saving, deleting and the menu bar only the original.
TELAS = True
ROTINAS = [('notes_open_staff_note_screen', 0x7961b0, [0x51, 0x6a, 0x00, 0x68, 0x60, 0x8f, 0x5a, 0x00]),
           ('notes_draw_staff_note_screen', 0x796340, [0x6a, 0xff, 0x68, 0x5b, 0xea, 0x95, 0x00]),
           ('notes_staff_note_screen_handler', 0x796b30, [0x6a, 0xff, 0x68, 0x80, 0xea, 0x95, 0x00])]
DOCS = ('a tela da anotação sobre uma pessoa',
        'the staff note screen',
        [('notes_open_staff_note_screen', 'notes_open_staff_note_screen', '0x7961b0', 'notes.cpp', 'cdecl',
          'abre a tela para criar a anotação do humano sobre a pessoa, ou editar a existente: o texto, o motivo e, quando o lembrete cai 7 ou 28 dias depois da escrita, a escolha de data (0xff); parâmetros: a pessoa, hoje, a escolha, o motivo, se edita, o texto, as duas datas da anotação e a escolha de novo',
          'opens the screen to create the human\'s note on the person, or to edit the existing one: the text, the reason and, when the reminder falls 7 or 28 days after the writing, the date choice (0xff); params: the person, today, the choice, the reason, whether editing, the text, the note\'s two dates and the choice again'),
         ('notes_draw_staff_note_screen', 'notes_draw_staff_note_screen', '0x796340', 'notes.cpp', 'cdecl',
          'monta a tela: a pessoa nas cores do clube ("Create" ou "Edit Staff Note"), a frase com o link dela, a caixa do texto (254 caracteres), as três escolhas de lembrete e, editando, a data do lembrete; os cinco motivos (os que não valem para a pessoa apagados) e Cancelar / Confirmar (e Apagar editando)',
          'builds the screen: the person in the club colours ("Create" or "Edit Staff Note"), the sentence with his link, the text entry (254 characters), the three reminder choices and, editing, the reminder date; the five reasons (those not valid for the person greyed) and Cancel / Confirm (and Delete when editing)'),
         ('notes_staff_note_screen_handler', 'notes_staff_note_screen_handler', '0x796b30', 'notes.cpp', 'cdecl',
          'um lembrete ou motivo clicado vira o escolhido (remonta); Confirmar grava a anotação (nova, ou a editada, com as datas quando a escolha é a data); Apagar remove; Cancelar fecha; no jogo em rede sempre fecha; o resto vai para a barra de menus',
          'a clicked reminder or reason becomes the chosen one (rebuilds); Confirm saves the note (new, or the edit, with the dates when the choice is the date); Delete removes it; Cancel closes; in a network game it always closes; the rest goes to the menu bar')])
