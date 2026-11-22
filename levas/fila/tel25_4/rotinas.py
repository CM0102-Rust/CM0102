# The training screens (training_screens.cpp, part 3; reader tel25, the screens round, 21/11/2026): the schedule
# editor's builder (its input and opener are in tel25_2). Uses the helpers of tel25_1 (TRAINING_SCHEDULE_SIZE),
# tel25_2 (TRAINING_TEXT_BUFFER), tel26_1 (tel26_add_area, tel26_add_item, tel26_area_button, tel26_area_text) and
# tel22_1 (comp_lang_format, comp_text_item): those go first.
# Conventions checked in the disassembly:
# - 0x8b9940 cdecl, 0x50 bytes of locals: the column weights [3, 1], [10, 2, 10, 12], [10, 6, 6, 6, 6],
#   [10, 6, 6, 11, 1] and the bit words [1, 2, 4, 8, 0x10, 0x20, 0x40] and [0x80, 0x200, 0x800] on the stack.
#   Calls: 0x5e4800 graphics_rgb_to_pixel cdecl; 0x41c0d0 background_set_picture cdecl(0); 0x55fba0 Display::add_area
#   thiscall(11) ret 0x2c and 0x55f990 Display::add_item thiscall(18) ret 0x48 on the current human's Display; 0x803fd0
#   add_text_item thiscall(11); 0x8040a0 add_area_button thiscall(11) -> eax (the popup owner); 0x804100
#   add_area_text thiscall(9); 0x66f4e0 langlib_sprintf; 0x8ac600 draw_pitch cdecl(11); 0x8b5520
#   training_schedule_get_category_name and 0x8b5690 training_schedule_get_intensity_name cdecl(value, buf, 0x7d0)
#   (they read the value's low byte); 0x8b4f50 TRAINING_SCHEDULE::get_category_level thiscall(category) ret 4 (movsx
#   of the low byte); 0x5b7ab0 formation_get_position_flag_name cdecl(bits, buf, 0x7d0).
# - The original passes the category to 0x8b5520 and 0x8b4f50 with three garbage high bytes (a stack dword whose
#   low byte is the counter) and some colour words with garbage high halves; the callees read the low byte / the
#   word; the copy passes clean values.
# - The 1080 exe patches the full-screen area's edges (push 0x257 at 0x8b9a73, push 0x31f at 0x8b9a7b): read from the
#   running exe.
# Defects of the original: none found.
MOTOR = True
TELAS = True
ROTINAS = [('training_screens_draw_schedule_editor', 0x8b9940, [0xa0, 0x49, 0x1f, 0xde, 0x00])]
DOCS = ('as telas de treino (3): a montagem do editor de programa',
        'the training screens (3): the schedule editor builder',
        [('training_screens_draw_schedule_editor', 'training_screens_draw_schedule_editor', '0x8b9940', 'training_screens.cpp', 'cdecl',
          'monta o editor de programa: a caixa do nome, o nível de cada uma das 5 categorias (0, 10, 25 ou 50: o atual em amarelo, os outros botões verdes), a posição e o lado visados (Nenhum ou Específico, com a lista dos bits); Cancelar, e Ok só com nome; programa com +0x15 = 7 tem o nome apagado antes',
          'builds the schedule editor: the name box, the level of each of the 5 categories (0, 10, 25 or 50: the current one yellow, the others green buttons), the target position and side (None or Specific, with the list of bits); Cancel, and Ok only with a name; a schedule with +0x15 = 7 has its name emptied first')])
