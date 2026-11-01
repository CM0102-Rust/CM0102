"""Integra a 265a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('gravar_financas', 5878704, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('carregar_financas', 5878432, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('gravar_gerente_de_elencos', 8756416, [129, 236, 8, 2, 0, 0]), ('gravar_sementes_da_uefa', 9458960, [129, 236, 0, 2, 0, 0]), ('gravar_sementes_da_conmebol', 5008240, [129, 236, 0, 2, 0, 0]), ('gravar_paises_anfitrioes', 6271776, [129, 236, 0, 2, 0, 0]), ('gravar_historico_dos_clubes', 4472656, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('gravar_estatisticas_dos_arbitros', 7977856, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('gravar_estatisticas_simuladas', 8701568, [129, 236, 4, 2, 0, 0]), ('gravar_datas_dos_clubes', 5959744, [161, 92, 42, 174, 0]), ('carregar_datas_dos_clubes', 5959312, [161, 92, 42, 174, 0]), ('gravar_conferencia_das_pernas', 5960864, [129, 236, 4, 2, 0, 0])]
BASE = 1497
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
