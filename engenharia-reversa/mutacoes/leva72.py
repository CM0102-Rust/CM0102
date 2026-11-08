# Mutacoes da leva72 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva72.py
MUTACOES = [
    ('t_9b23', "        Some((_, r)) => *((r as isize + 0x9b23 + i as isize) as *const u8),", "        Some((_, r)) => *((r as isize + 0x9b24 + i as isize) as *const u8),"),
    ('g_4', "    is_in_preferences(clube, person, 4)\n", "    is_in_preferences(clube, person, 8)\n"),
    ('g_10', "    is_in_preferences(clube, person, 0x10)\n", "    is_in_preferences(clube, person, 0x14)\n"),
    ('g_ordem', "unsafe extern \"C\" fn likes_club(person: usize, clube: usize) -> u32 {\n    is_in_preferences(clube, person, 4)", "unsafe extern \"C\" fn likes_club(person: usize, clube: usize) -> u32 {\n    is_in_preferences(person, clube, 4)"),
]
