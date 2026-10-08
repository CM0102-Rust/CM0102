# Mutacoes da leva46 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva46.py
MUTACOES = [
    ('p_nulo', "    if alvo == 0 || pessoa == 0 {", "    if pessoa == 0 {"),
    ('p_pref', "    if pref == 0 {\n        return 0;", "    if pref == 0 {\n        return 1;"),
    ('p_tres', "(0..3).any(|k|", "(0..2).any(|k|"),
    ('p_gosta', "    nas_preferencias(alvo, pessoa, 0x1c)", "    nas_preferencias(alvo, pessoa, 0x20)"),
    ('p_nao', "    nas_preferencias(alvo, pessoa, 0x28)", "    nas_preferencias(alvo, pessoa, 0x24)"),
    ('m_menos1', "*destino = if ficha.is_null() { u32::MAX } else { *ficha };", "*destino = if ficha.is_null() { 0 } else { *ficha };"),
    ('m_187f', "    *((this + 0x187f) as *mut u32) = 0;\n    1", "    1"),
    ('m_base', "let destino = (this as isize + 0x5bf + 4 * posicao as isize) as *mut u32;", "let destino = (this as isize + 0x5bf + 4 * (posicao as u8) as isize) as *mut u32;"),
]
