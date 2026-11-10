# Mutações da 219a leva (itens save_s2_a, save_s2_b, save_s2_c). Uso: python mutacoes.py mutacoes\leva219.py --prova-save 219
MUTACOES = [
    ('sv2_resumo_primeira', '            if first {\n                format(', '            if !first {\n                format('),
    ('sv2_resumo_versao', '    copy_text(summary.add(0x7d0), exe::utils_get_version_string());', '    copy_text(summary.add(0x7d1), exe::utils_get_version_string());'),
    ('sv2_fifa_segundo_byte', '    if (exe::zwrite(G_MAX_CONTRACT_LENGTH_YEARS, (1) as u32, (1) as u32, z) as i32) != 1 {', '    if 1 != 1 {'),
    ('sv2_banco_a4', '    exe::save_database(save, 1, a4, kind)', '    exe::save_database(save, 1, a4 ^ 1, kind)'),
]
