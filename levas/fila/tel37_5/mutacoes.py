# Mutations (tel37_5). The automatic career opens no screen: the chain runs none; these prove the comparer in a session
# of Lucas's (JOGAR_COMPARANDO.bat): every screen sends a new screen packet, its areas and its items.
# - the areas get the column count where the row count goes (side by side over the Display: any area with rows and
#   columns that differ);
# - the items take a packet's colour as the highlight colour (side by side: any item that sends its colour);
# - the new screen packet's colour format read as a word (only the original runs it: the plan's buffer position and
#   fields are off on every screen).
MUTACOES = [
    ('tel37_5_area_rows', 'fr.u32(s::ROWS), fr.addr(s::ROW_WEIGHTS) as u32', 'fr.u32(s::COLUMNS), fr.addr(s::ROW_WEIGHTS) as u32'),
    ('tel37_5_item_colour', 'read(r, fr, 0x200, s::COLOUR, 2, GET_USHORT);', 'read(r, fr, 0x200, s::HIGHLIGHT_COLOUR, 2, GET_USHORT);'),
    ('tel37_5_new_screen_format', 'r.byte_to(fr.addr(frame::FLAG_BYTE), GET_CHAR);', 'r.word_to(fr.addr(frame::FLAG_BYTE), GET_SHORT);'),
]
