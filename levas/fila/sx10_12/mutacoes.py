# Mutations (sx10_12), screens: the routines' comparers catch them in the game comparing (JOGAR_COMPARANDO.bat):
# - the font name shows the file name instead of the full font name (the font choice in the game options, with font
#   files in the FONTS folder);
# - the font file count one higher (the game options and the setup screen ask for it).
MUTACOES = [
    ('sx10_12_file_name',
     'if full != 0 { full } else { read_usize(entry + ff::ENTRY_FILE_NAME) }',
     'read_usize(entry + ff::ENTRY_FILE_NAME) // mutation: always the file name',
     'sx10_12 font file name'),
    ('sx10_12_count',
     'read_u16(font_files::COUNT) as u32',
     'read_u16(font_files::COUNT) as u32 + 1 // mutation'),
]
