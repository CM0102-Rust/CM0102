# Mutations (tel25_6). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat; club screen > Training): the filter, sort and view links run side by side.
# - a position filter also clears the "all sides" bit (Filter > Goalkeepers with All Sides ticked);
# - the attribute sort remembers the next attribute (clicking an attribute header in the Attributes view).
MUTACOES = [
    ('tel25_6_filter_clears_sides', '1 | 2 | 4 | 8 | 0x10 => filter & 0xffff_ffe0,', '1 | 2 | 4 | 8 | 0x10 => filter & 0xffff_ffc0,'),
    ('tel25_6_sort_attribute', 'write_u8(view + 4, attribute);', 'write_u8(view + 4, attribute.wrapping_add(1));'),
]
