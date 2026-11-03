"""Integra a 272a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('campeoes_sortear_grupos', 5788304, [106, 255, 104, 222, 138, 149, 0]), ('campeoes_ordem_dos_potes', 5795680, [139, 68, 36, 8, 139, 84, 36, 4]), ('campeoes_posicoes_previstas', 5794048, [129, 236, 32, 2, 0, 0]), ('campeoes_sortear_segunda_fase', 5790384, [106, 255, 104, 254, 138, 149, 0]), ('campeoes_ordem_da_segunda_fase', 5796320, [139, 76, 36, 4, 139, 84, 36, 8]), ('grupo_da_fase', 4915664, [139, 68, 36, 4, 61, 232, 3, 0, 0]), ('campeoes_rodadas_da_fase', 5782608, [100, 161, 0, 0, 0, 0]), ('liga_expectativas_da_temporada', 6859392, [161, 68, 122, 182, 0]), ('liga_rebaixar_clube', 6828448, [86, 139, 116, 36, 8]), ('competicao_todas_completas', 4771344, [86, 139, 241, 131, 126, 28, 255]), ('competicao_lista_de_clubes', 4776384, [129, 236, 8, 2, 0, 0]), ('competicao_progresso_do_clube', 4770928, [131, 236, 12, 83, 85])]
BASE = 1646
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
