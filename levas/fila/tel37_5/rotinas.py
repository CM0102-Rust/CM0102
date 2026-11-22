# The display list reader: Display::process_command_packet (display.cpp; reader tel37, the screens round, 21/11/2026),
# the whole routine (0x564080..0x567a1b, 14 900 bytes; jump tables 0x567a1c (27 commands) and 0x567a88 (11 item
# subcommands)) hooked in one item, in pieces named by command.
# The screen manager writes every screen as command packets and applies them to its local Display here, so it runs on
# every screen, also in a local game (the automatic career's screens too).
# Conventions checked in the disassembly (every push against esp): thiscall(Display, session, player, NetworkBuffer*)
# ret 0xc -> ax (-1; -2 for command 4; the item for command 6); the callers keep eax (0x7fcb71) and read the low word.
# Frame of 0x2af8 bytes (_chkstk): the copy keeps the locals at the original's offsets (CommandFrame) because the reads
# write them with mixed widths (a word into a dword local: the high word is the buffer position's), they persist
# between the items of a packet and some are passed whole as dwords.
# Called by address: Display::reset 0x55f7c0 thiscall(1) ret 4; Display::draw_screen 0x55f300 thiscall(4) ret 0x10;
# Display::add_item 0x55f990 (ret 0x48) and add_area 0x55fba0 (ret 0x2c) (the 1080 exe jumps from both: by address);
# Area::release 0x4031a0 thiscall; Area::convert_colour_to_pixel_format 0x4041d0 thiscall(1) ret 4; Area::set_scroll_row
# 0x403320 thiscall(1) ret 4; Guio::init 0x5ee150 thiscall(21) ret 0x54; Guio::get_fields 0x5ee4a0 thiscall(19) ret 0x4c;
# Guio::convert_colours_pixel_format 0x5eef80 thiscall(2) ret 8; Guio::set_text_and_refresh 0x5eed50 thiscall(2) ret 8;
# guio_set_fixed_rect / guio_set_rect 0x567ac0 / 0x567ae0 thiscall(4) ret 0x10; guio_copy_text_to_caller 0x567b00
# thiscall; Display::send_screen_section 0x562370 thiscall(3) ret 0xc; display_send_status_packet 0x568290 stdcall(3)
# ret 0xc; display_localise_media_path 0x5680f0 cdecl(2); ScreenManager::set_background_picture 0x7feeb0 thiscall(1);
# set_area_selection 0x5680b0 thiscall(2); the sound slot flags 0x802cb0 thiscall(slot, out byte*) ret 8 / 0x802ce0
# thiscall(slot) ret 4; gui_draw_progress_box 0x5e76f0 cdecl(3); set_cursor_mode 0x5e8170 cdecl(1); SoundPlayer
# play_file 0x84cbc0 thiscall(3) ret 0xc, set_slot_volume 0x84ccc0 (2) ret 8, stop_all 0x84cb10, stop_slot 0x84cb70 (1)
# ret 4; print_text_lines 0x84c830 stdcall(3) ret 0xc (the original passes 0xdd7ee8 in ecx, unread); Network::send_packet
# 0x77ac30 thiscall(3) ret 0xc; calloc / realloc / malloc / free.
# The NetworkBuffer reads are inlined (with their own network.h box lines: 143, 144, 146, 147, 148 for an unsigned char,
# 149, 151) or called (NetworkBuffer::get_* of tel37_1): the copy reads both through PacketReader, same room check, line
# and width; the explicit get_sized_block uses tel37_1's length 0 without room.
# The cm0102-1080.exe does not change this routine (0x564080..0x567ac0 byte for byte).
# Depends on tel26_1 (display_list, display_item, display_item_at, display_area_at, display_method(1), tel26_add_item,
# tel26_add_area, DisplayItemFields, tel26_copy_text, tel26_error_box, DISPLAY_SNAPSHOT_POOL, DISPLAY_HOTKEY_PENDING,
# DISPLAY_CLICK_PENDING), tel26_2 (display_set_text), tel29_1 (NETWORK_H, c_string_len), tel37_1 (network_buffer,
# network_buffer_cursor/advance/put_char/put_int, NET_SEND_BUFFER, DISPLAY_PACKET_BUFFER), tel37_3 (display_life,
# display_network_error_box) and tel38_5 (SOUND_PLAYER): it goes after them.
# Defects of the original: with an empty buffer the command byte is an uninitialised local (the copy takes 0); a block
# length read without room leaves the old local as the length copied; a negative block length copies ~4 GB (the copy
# copies nothing and moves the position the same way); command 6 takes a stale local as the first drop coordinate
# without room (DROP_STALE).
TELAS = True
ROTINAS = [('display_process_command_packet', 0x564080, [0xb8, 0xf8, 0x2a, 0x00, 0x00])]
DOCS = ('o leitor da lista de exibição: aplicar a uma exibição os pacotes de comando do gerente de telas',
        "the display list reader: applying the screen manager's command packets to a Display",
        [('display_process_command_packet', 'Display::process_command_packet', '0x564080', 'display.cpp', 'thiscall, ret 0xc',
          'aplica um pacote de comando à exibição (a posição do buffer volta a 0; o primeiro byte é o comando): 0 tela nova (recomeça a exibição, o fundo localizado, as contagens anunciadas, o foco e o formato das cores; responde 5 e 0 ou o pacote de estado 2), 1 as áreas anunciadas (lidas e acrescentadas por Display::add_area, com a cor convertida; responde 5 e 2), 2 os itens anunciados (cada um por uma máscara de campos, partindo do instantâneo ou copiando outro item; acrescentados por Display::add_item, as cores convertidas; o depósito de instantâneos fica com a tabela nova; responde 5 e 1), 3 e 9 desenhar a tela inteira, 4 devolve -2, 5 manda uma seção da tela a um jogador de rede, 6 refaz um item no lugar (e o alvo onde ele foi solto; devolve o item), 7 a caixa de progresso, 8 muda um campo de um item (11 subcomandos), 10 a 14 os sons de um jogador de rede, 15 e 16 a seleção e a rolagem de uma área, 17 a 20 os sinais e itens da entrada, 21 a 26 a lista de impressão; devolve -1 nos outros',
          'applies a command packet to the Display (the buffer position back to 0; the first byte is the command): 0 a new screen (the Display reset, the background localised, the announced counts, the focus and the colour format; answers 5 and 0 or the status packet 2), 1 the announced areas (read and added by Display::add_area, the colour converted; answers 5 and 2), 2 the announced items (each by a field mask, starting from the snapshot or copying another item; added by Display::add_item, the colours converted; the snapshot pool takes the new table; answers 5 and 1), 3 and 9 drawing the whole screen, 4 returns -2, 5 sends a screen section to a network player, 6 rebuilds an item in place (and the target it was dropped on; returns the item), 7 the progress box, 8 changes one field of an item (11 subcommands), 10 to 14 a network player\'s sounds, 15 and 16 an area\'s selection and scroll row, 17 to 20 the input flags and queued items, 21 to 26 the print list; returns -1 for the others')])
