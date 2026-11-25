# The shortlist manager loaded from the saved game (shortlist_manager.cpp; reader rxf, round 6, 25/11/2026).
# - 0x83a9d0 Shortlist_Manager::load(file): thiscall, ret 4, no value (eax not set on the way out; the constructor
#   0x839a80, sx20_5, ignores it). The argument is the open save stream (zread's handle), as the TRANSFER_MANAGER's
#   loading constructor passes it (0x8bc7c5 -> 0x839a80 -> here). Prologue `mov eax, fs:[0]; push -1` (8 bytes, SEH
#   frame for the 25 old local slots: constructor 0x83b360 = Date::Date() at +0x1a, destructor 0x5a53c0 = empty
#   shared routine, through 0x9463af / 0x9462bb; not reproduced, the read fills them whole). Last ret at 0x83b246, the
#   error tails to 0x83b357; 0x83b360 is another routine (the local slot constructor).
# - Calls: zread 0x932c30 cdecl(data, size, n, file); malloc 0x944e46; free 0x9452ca; qsort 0x945333 cdecl(base, n, 8,
#   0x844740); Date::operator= 0x417c10 thiscall(dst, scratch, src) ret 8; Date::Date(1, 0x7cd) 0x549ef0 (inside
#   shortlist_slot_reset, sx20_5); build_position_lists 0x839e60 thiscall(0) ret 4; the "Error" box (non-fatal, the
#   load stops and the game goes on) and the fatal one for the position lists (0x337: box, 0x5cc7d0, 0x5a8f60,
#   exit(-1)).
# - The version is |[0xdd7110]| (G_GAME_DATA_VERSION, cdq/xor/sub): < 0x3e016 old 21-byte block; <= 0x3e016 16th count
#   0 and the 36-byte slots; < 0x3e017 the 16th lists not read and build_position_lists(0) at the end.
# Uses from sx20_5 (INTEGRAR_ANTES): SHORTLIST_MANAGER_SOURCE, SHORTLIST_SLOT_SIZE, SHORTLIST_SLOTS,
# SHORTLIST_CLUB_ENTRY_SIZE (pub(crate)) and, private there, shortlist_slot_reset and SHORTLIST_BUILD_POSITION_LISTS: both
# routines go to shortlist_manager.rs, so the private names stay visible; if esvaziar_motor.py splits them, make those
# two pub(crate) in sx20_5.
# Same bytes in cm0102-1080. Comparer: only the original (allocates and reads the stream).
# Defects of the original (DEFEITOS): below 0x3e017 the -1 goes twice into [this+0x54]'s list (the person group's 16th)
# and the key group's 16th list ([this+0x94]) keeps what malloc left; the old 21-byte block's buffer leaks when its read
# is short.
SAVE = True
CARGA = True  # the save proof also loads the FeriasTesteSave with and without the routines (prova_save.py)
ROTINAS = [('shortlist_manager_load', 0x83a9d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x6a, 0xff])]
DOCS = ('o gerente de listas curtas lido do save',
        'the shortlist manager loaded from the save',
        [('shortlist_manager_load', 'Shortlist_Manager::load', '0x83a9d0', 'shortlist_manager.cpp', 'thiscall, ret 4',
          'lê do save os pares de listas curtas (copiados e ordenados por clube), as 32 listas por posição e as 40 vagas de lista curta de cada clube que tinha uma, com a cópia da reputação do clube subida à atual; os saves antigos têm um bloco de registros de 21 bytes descartado, vagas de 36 bytes (25 por clube, convertidas; as outras 15 vazias) e sem a 16ª lista, e as listas por posição são refeitas no fim',
          "reads from the save the shortlist pairs (copied and sorted by club), the 32 position lists and the 40 shortlist slots of each club that had one, with the club reputation copy raised to the current one; old saves carry a discarded block of 21-byte records, 36-byte slots (25 per club, converted; the other 15 empty) and no 16th list, and the position lists are rebuilt at the end")])
