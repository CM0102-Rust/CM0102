"""Integra a 223a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('ordem_do_historico_de_competicao', 5506944, [139, 84, 36, 4, 86]), ('vetor_de_atributos_do_jogador', 5482000, [139, 68, 36, 12, 186, 63, 0, 0, 0]), ('nacao_britanica_ou_irlanda', 5523152, [139, 68, 36, 4, 139, 13, 228, 242, 156, 0]), ('clube_basco', 5494768, [139, 76, 36, 4, 133, 201]), ('pessoa_listada_pelo_clube', 5522592, [86, 87, 139, 124, 36, 12]), ('pessoa_listada_para_emprestimo', 5522832, [86, 139, 116, 36, 8]), ('pessoa_listada_a_pedido', 5522944, [86, 139, 116, 36, 8]), ('nota_relativa_do_atributo', 5522304, [139, 68, 36, 4, 15, 190, 84, 36, 8]), ('clube_pode_ter_reservas', 5515072, [129, 236, 0, 2, 0, 0])]
BASE = 586
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
