# Mutações da 219a leva (itens save_s2_a, save_s2_b, save_s2_c). Uso: python mutacoes.py mutacoes\leva219.py --prova-save 219
MUTACOES = [
    ('sv2_resumo_primeira', '            if first {\n                format(', '            if !first {\n                format('),
    ('sv2_resumo_versao', '    copy_text(summary.add(0x7d0), version());', '    copy_text(summary.add(0x7d1), version());'),
    ('sv2_fifa_segundo_byte', '    if write(0xa83608, 1, 1, z) != 1 {', '    if 1 != 1 {'),
    ('sv2_banco_a4', '    write_bench(save, 1, a4, kind)', '    write_bench(save, 1, a4 ^ 1, kind)'),
]
