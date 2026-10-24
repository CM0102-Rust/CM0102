"""Integra a 228a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('ler_continentes', 5559776, [139, 68, 36, 16, 129, 236, 0, 2, 0, 0]), ('ler_nacoes', 5560128, [139, 68, 36, 16, 129, 236, 0, 2, 0, 0]), ('ler_cidades', 5560480, [139, 68, 36, 16, 129, 236, 0, 2, 0, 0]), ('ler_estadios', 5560832, [139, 68, 36, 16, 129, 236, 0, 2, 0, 0]), ('ler_pessoas_157', 5564288, [139, 68, 36, 16, 129, 236, 0, 2, 0, 0]), ('ler_historico_de_pessoas', 5569888, [139, 68, 36, 16, 129, 236, 0, 2, 0, 0]), ('ler_historicos_de_competicao', 5571120, [139, 68, 36, 16, 129, 236, 0, 2, 0, 0]), ('ler_competicoes_de_pessoal', 5571456, [139, 68, 36, 16, 129, 236, 0, 2, 0, 0]), ('ler_historico_de_competicoes_de_pessoal', 5571792, [139, 68, 36, 16, 129, 236, 0, 2, 0, 0])]
BASE = 625
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
