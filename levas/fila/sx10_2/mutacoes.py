# Mutations (sx10_2) by the save proof (mutacoes.py --prova-save N: the .qst written with the routines against the
# base). No 4th field: the base of the save proof runs with the routines off. The test career has attribute masking
# off, so fog.dat holds only the masking byte.
# - the masking byte is written as 1 (the fog.dat item differs);
# - with masking off the 16 presence words are written anyway (fog.dat grows by 64 bytes).
MUTACOES = [
    ('sx10_2_masking_byte',
     'if exe::zwrite(G_ATTRIBUTE_MASKING_ENABLED, 1, 1, z) != 1 { // sx10_2: the masking flag',
     'if exe::zwrite(&present as *const u32 as usize, 1, 1, z) != 1 { // mutation: masking written as on'),
    ('sx10_2_presence_words',
     'if read_u8(G_ATTRIBUTE_MASKING_ENABLED) == 0 { // sx10_2: masking off, nothing more',
     'if read_u8(G_ATTRIBUTE_MASKING_ENABLED) == 0 { for _ in 0..FOG_MAP_SLOTS { exe::zwrite(&absent as *const u32 as usize, 4, 1, z); } // mutation: empty slots written'),
]
