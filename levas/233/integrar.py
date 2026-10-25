"""Integra a 233a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('clube_em_temporada', 5535600, [139, 68, 36, 4, 133, 192]), ('clube_em_nacao_grande', 5545552, [139, 68, 36, 4, 133, 192]), ('nacao_grande', 5545632, [139, 68, 36, 4, 133, 192]), ('limpar_vinculos_de_opiniao', 5545104, [139, 84, 36, 4, 133, 210]), ('juntar_vinculo_de_opiniao', 5545200, [139, 84, 36, 4, 86]), ('tirar_vinculo_de_opiniao', 5545312, [139, 76, 36, 4, 133, 201]), ('auxiliar_do_clube', 5491584, [139, 76, 36, 4, 129, 236, 0, 2, 0, 0]), ('treinadores_do_clube', 5491760, [129, 236, 0, 2, 0, 0]), ('contar_treinadores_do_clube', 5490896, [129, 236, 0, 2, 0, 0]), ('sigla_inglesa_da_nacao', 5526720, [106, 255, 104, 123, 121, 149, 0]), ('nome_ingles_da_competicao', 5526944, [106, 255, 104, 155, 121, 149, 0]), ('nome_ingles_da_competicao_de_pessoal', 5527200, [106, 255, 104, 187, 121, 149, 0]), ('nome_ingles_do_estadio', 5527952, [106, 255, 104, 27, 122, 149, 0]), ('arquivo_do_idioma_existe', 5462320, [139, 68, 36, 4, 133, 192]), ('liberar_tabelas_em_ingles', 5525296, [86, 139, 241, 139, 70, 32]), ('gerar_comissao_extra', 5403200, [129, 236, 24, 2, 0, 0]), ('cortar_tabela_de_clubes', 5400512, [129, 236, 40, 6, 0, 0]), ('podar_historico_de_competicoes', 5479136, [139, 13, 124, 35, 174, 0]), ('resolver_indices_do_banco', 5445024, [129, 236, 4, 6, 0, 0]), ('ordenar_clubes_pela_reputacao', 5495616, [138, 68, 36, 4, 132, 192])]
BASE = 681
m = re.search(r'\];\nconst N: usize = (\d+);', s)
assert m and int(m.group(1)) == BASE, ('o motor.rs mudou desde a materialização', m and m.group(1), BASE)
pro = ''.join('    Prologo { endereco: %#x, bytes: &[%s] },\n' % (e, ', '.join('0x%02x' % b for b in bs)) for _, e, bs in ROTINAS)
s = s[:m.start()] + pro + '];\nconst N: usize = %d;' % (BASE + len(ROTINAS)) + s[m.end():]
fins = [x.start() for x in re.finditer(r' as \*const \(\) as usize,\n    \];', s)]
assert len(fins) == 2, fins
for k, prefixo in ((1, 'comparar_'), (0, '')):
    i = fins[k] + len(' as *const () as usize,\n')
    s = s[:i] + ''.join('        %s%s as *const () as usize,\n' % (prefixo, a) for a, _, _ in ROTINAS) + s[i:]
anc = '\n    ];\n    for i in 0..N {'
assert s.count(anc) == 1
s = s.replace(anc, ' ' + ' '.join('"motor_%s",' % a for a, _, _ in ROTINAS) + anc)
anc = 'comparar_valor!(comparar_criterio_da_comissao, criterio_da_comissao, 357, () -> u32);\n'
assert s.count(anc) == 1
s = s.replace(anc, anc + '\n' + open(os.path.join(aqui, 'codigo.rs'), encoding='utf-8').read().strip('\n') + '\n')
open(p, 'w', encoding='utf-8').write(s)
print('ok')
