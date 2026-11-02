"""Integra a 268a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('resposta_da_ia_ao_lance', 9207952, [106, 255, 104, 171, 61, 150, 0]), ('negociacao_de_emprestimo', 9170512, [106, 255, 104, 238, 58, 150, 0]), ('opcao_de_troca', 9209952, [106, 255, 104, 7, 62, 150, 0]), ('agendar_fim_do_emprestimo', 5033120, [129, 236, 20, 2, 0, 0]), ('lances_do_clube', 8691744, [85, 139, 236, 131, 228, 248]), ('cqe4_interesse', 8677664, [85, 139, 236, 131, 228, 248]), ('cqe4_construir_contrato', 8774672, [106, 255, 104, 235, 41, 150, 0]), ('cqe5_pode_emprestar', 5741920, [131, 236, 108, 85, 86]), ('cqe5_registrar_emprestimo', 5742544, [139, 68, 36, 4, 139, 21, 180, 35, 174, 0]), ('cqe5_dia_das_regras', 5742624, [131, 236, 8, 86, 106, 255]), ('cqe6_periodo_inglaterra', 5741680, [106, 255, 104, 8, 135, 149, 0]), ('cqe6_pode_emprestar_escocia', 8349264, [139, 13, 80, 16, 174, 0]), ('cqe6_periodo_escocia', 8348976, [86, 87, 185, 144, 44, 174, 0]), ('cqe6_pode_emprestar_italia', 6690672, [131, 236, 100, 85, 86]), ('cqe6_pode_emprestar_irlanda', 6538192, [83, 86, 87, 139, 124, 36, 16]), ('cqe6_pode_emprestar_espanha', 8754736, [131, 236, 100, 86, 87])]
BASE = 1579
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
