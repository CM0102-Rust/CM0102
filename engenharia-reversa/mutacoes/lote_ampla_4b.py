# Lote 4b da carreira ampla (22/10/2026, de dia, intercalado com as levas): o lote 3 e as escapadas
# por dados das levas 206, 210 a 213. Uso: python mutacoes.py mutacoes\lote_ampla_4b.py --carreira 2002-03-31 --ligas "ENG ESP ITA FRA BRA JPN AUS USA" --limite 7200
MUTACOES = [
    ('f_fase', '        0xf => (*((o + 0x2e) as *const u8) >= 3) as u32,', '        0xf => (*((o + 0x2e) as *const u8) >= 2) as u32,'),
    ('m_alguma', '    alguma_noticia(midia, *id, |_| true) as u8', '    alguma_noticia(midia, *id, |r| *((r + 0x10) as *const u8) != 0) as u8'),
    ('d_sem_regra', '    if k == -1 {\n        return;\n    }\n    let regras', '    if k <= 0 {\n        return;\n    }\n    let regras'),
    ('d_43', '    for k in 0..43usize {', '    for k in 0..42usize {'),
    ('t_desmarca', '    *marcas_da_pessoa(gerente, id) &= 0xbf;', '    *marcas_da_pessoa(gerente, id) &= 0x3f;'),
    ('t_vaga', '    if k < *((tabela + 4) as *const i32) {', '    if k <= *((tabela + 4) as *const i32) {'),
    ('r_ids', '    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {', '    for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e] {'),
    ('s_empate', '        if melhor == 0 || reputacao(c) > reputacao(melhor) {', '        if melhor == 0 || reputacao(c) >= reputacao(melhor) {'),
    ('c_clube', '    *clube = (*(0xae23b4 as *const usize) as isize + *((contrato + 4) as *const i32) as isize * 581) as usize;', '    *clube = (*(0xae23b4 as *const usize) as isize + *((contrato + 4) as *const i32) as isize * 580) as usize;'),
]
