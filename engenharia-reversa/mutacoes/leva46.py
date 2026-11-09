# Mutacoes da leva46 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva46.py
MUTACOES = [
    ('p_nulo', "    if target == 0 || person.is_null() {", "    if person.is_null() {"),
    ('p_pref', "    if pref == 0 {\n        return 0;", "    if pref == 0 {\n        return 1;"),
    ('p_tres', "(0..3).any(|k|", "(0..2).any(|k|"),
    ('p_gosta', "    is_in_preferences(target, Staff(person), 0x1c)", "    is_in_preferences(target, Staff(person), 0x20)"),
    ('p_nao', "    is_in_preferences(target, Staff(person), 0x28)", "    is_in_preferences(target, Staff(person), 0x24)"),
    ('m_menos1', "*target = if sheet.is_null() { u32::MAX } else { *sheet };", "*target = if sheet.is_null() { 0 } else { *sheet };"),
    ('m_187f', "    *((this + 0x187f) as *mut u32) = 0;\n    1", "    1"),
    ('m_base', "let target = (this as isize + 0x5bf + 4 * position as isize) as *mut u32;", "let target = (this as isize + 0x5bf + 4 * (position as u8) as isize) as *mut u32;"),
]
