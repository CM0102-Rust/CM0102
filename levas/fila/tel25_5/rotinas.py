# The training screens (training_screens.cpp, part 4; reader tel25b, the screens round, 21/11/2026): the training
# screen's builder (10.7 KB), whole, in named pieces (training_schedule_tabs, training_schedule_menu,
# training_view_menu, training_edit_and_filter, training_squad_list, training_squad_row, training_status_cell,
# training_passes_filter). Uses the helpers of tel25_1 (TRAINING_SCHEDULE_SIZE, TRAINING_SQUAD_VIEW), tel25_2
# (TRAINING_TEXT_BUFFER), tel26_1 (tel26_add_item, tel26_add_area, tel26_add_button, tel26_area_button,
# tel26_area_text, tel26_copy_text, display_area_at, job_colour, JOB_COLOUR_*) and tel22_1 (comp_lang_format,
# comp_crt_format, comp_text_item, CompFormatFn) and tel07_2 (training_update_pending): those go first. The comparer
# runs only the original when a listed player's training is due (update_player_training would roll the week; the
# regions restore his entry, so the copy would roll it again).
# Conventions checked in the disassembly:
# - 0x8b5a10 cdecl, SEH, 0x1768 bytes of locals (chkstk): a local LLIST (0x68d020 / 0x68d040, unused) and 25 tabs of
#   0xd4 bytes (0x9463af stdcall vector constructor with 0x4568f0 / 0x5a53c0; 0x9462bb at the end).
#   Calls: 0x7ff010/0x7ff270 screen params; 0x53b640 cdecl(club, 1, &p1, &p2); 0x8b0d60 thiscall(&list, comparer)
#   ret 8 (writes session + 0x80a and re-sorts the 50 entries, 0x8b5000); 0x41c8e0 cdecl(6, 0); 0x803f30
#   thiscall(p2, p1, text) ret 0xc; 0x75db00 cdecl(4, 0); 0x8b50f0 thiscall ret 0; 0x8b0d10 thiscall(k) ret 4 (reads
#   the low byte); 0x944c9f crt_sprintf; 0x5ed780 gui_add_tab_bar cdecl(tabs, 10, current, &top, &bottom, 1);
#   0x55fba0 Display::add_area thiscall(11) (columns and rows read as bytes), 0x55f990 Display::add_item thiscall(18),
#   0x8040a0 / 0x804100 / 0x804030 / 0x803fd0 ScreenManager items; 0x5fc760 thiscall on [0xb63c98](k, buf, 0x104,
#   mode, 0, 3) ret 0x18 (reads the low byte of k); 0x8b5820 cdecl(path, 0x6a-byte buffer); 0x9466d0 strncpy;
#   0x944cf1 operator new(0xe); 0x68d1b0/0x68d1d0 get_next/get_prev, 0x68d2e0 get_current thiscall ret 0; 0x68d0d0
#   add_tail thiscall(node, 5) ret 8; 0x5b7d20 cdecl(player sheet) -> ax; 0x5e5cd0 cdecl(2); 0x945640 _ftol of
#   height / rows + 0.5; 0x945333 crt_qsort(table, 9, 1, 0x81ae60); 0x5428a0 cdecl(attribute, buf, 0x7d0, 1);
#   0x8b1750 thiscall on [0xde1f44](person, 0) ret 8; 0x4849f0 cdecl(person, club, 0) -> al; 0x53b900 cdecl(club);
#   0x7757d0 thiscall on [0xb74318](person, 0xae2c90, club, 0, 0) ret 0x14; 0x5e4800 cdecl(0xb4, 0xb4, 0, 0) twice;
#   0x53dba0 cdecl(person, buf, 0x7d0, 3); 0x53ee50 cdecl(sheet, buf, 0xc8, 1); 0x7c1660 thiscall on [0xb74350](sheet,
#   buf, 0x7d0) ret 0xc; 0x4cc2e0 stdcall(person, buf, 0xc8) (the original loads ecx with [0xae1050], unread);
#   0x62c400 thiscall on [0xb63d28](person) -> al; 0x62c590 thiscall(person, 1) -> ax; 0x5431e0 cdecl(person,
#   attribute); 0x8b37c0 thiscall on [0xde1f44](person, attribute) -> al; 0x43f460 Cash::get_display_amount
#   thiscall(5) ret 4 -> edx:eax and 0x441c10 cdecl(buf, 0x7d0, amount, 5); 0x403320 Area::set_scroll_row
#   thiscall(row) on display + 4 + 0xbf1 x area.
# - Garbage the copy does not reproduce (the callees read the low byte or word): the k of 0x8b0d10 and 0x5fc760, the
#   rows of the list areas (the high bytes of the filtered list pointer), the attribute bytes, the colour words with
#   the high half of a register.
# - The 1080 exe changes nothing in 0x8b5a10..0x8b83e2 (byte compare with cm0102-1080.exe; no 0x31f/0x257 here: the
#   areas end at 0x30c/0x316 and 0x24e).
# Defects of the original: the Attributes view without a group bit (0x10..0x200) uses as attribute table a stack
# dword whose low byte holds the rows count (the copy keeps it); the new list for the filtered squad is not checked
# (add_tail on 0 if operator new fails); the attribute groups are sorted in place in .data on every build.
MOTOR = True
TELAS = True
ROTINAS = [('training_screens_draw', 0x8b5a10, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as telas de treino (4): a montagem da tela de treino',
        'the training screens (4): the training screen builder',
        [('training_screens_draw', 'training_screens_draw', '0x8b5a10', 'training_screens.cpp', 'cdecl',
          'monta a tela de treino: as 10 abas dos programas (nome, ou "nome (n)" com quantos seguem cada um; o pré-definido em cinza), os menus Programa (Carregar, Salvar, Salvar como, Apagar e os 4 arquivos recentes), Vista (Geral ou Atributos, e o grupo de atributos), Editar ou Treinadores e Filtro (posições e lados), a descrição do programa e a lista do elenco filtrada e ordenada pela coluna escolhida (situação, nome, posição, idade, forma, moral, condição ou os atributos com a cor da mudança no treino, e o valor); a lista volta à linha rolada; Cancelar e Ok',
          'builds the training screen: the 10 schedule tabs (name, or "name (n)" with how many follow each; the preset grey), the Schedule (Load, Save, Save As, Delete and the 4 recent files), View (General or Attributes, and the attribute group), Edit or Coaches and Filter (positions and sides) menus, the schedule\'s description and the squad list filtered and sorted by the chosen column (status, name, position, age, form, morale, condition or the attributes coloured by the training change, and the value); the list goes back to its scroll row; Cancel and Ok')])
