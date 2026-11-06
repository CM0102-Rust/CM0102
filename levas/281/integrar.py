"""Integra a 281a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('cadeia_de_acessos', 6855712, [83, 139, 217, 87, 139, 67, 28]), ('noticia_do_premio', 4776928, [100, 161, 0, 0, 0, 0]), ('noticia_da_tv', 4778560, [100, 161, 0, 0, 0, 0]), ('cqp10_copa_dos_campeoes_tabela', 4348944, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqp10_merconorte_tabela', 4987232, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqp10_supercopa_europeia_tabela', 5743552, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqp10_supercopa_asiatica_tabela', 4262160, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqp11_asia_clubes_tabela', 4238992, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqp11_asia_recopa_tabela', 4247440, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqp11_trofeu_tabela', 5680608, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqp12_copa_america_tabela', 6162688, [129, 236, 0, 2, 0, 0]), ('cqp12_africana_tabela', 4199136, [129, 236, 0, 2, 0, 0]), ('cqp12_confed_tabela', 5813600, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqp13_intertoto_tabela', 6501120, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqp13_intercontinental_tabela', 9615216, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqp13_interamericana_tabela', 6497088, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqp13_concacaf_tabela', 4984576, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqq10_uefa_tabela', 9446848, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqq10_intertoto_inscritos', 6502080, [129, 236, 4, 2, 0, 0]), ('cqq10_intertoto_lista', 6504304, [129, 236, 0, 2, 0, 0]), ('cqq11_europeia_clubes', 5743872, [129, 236, 0, 2, 0, 0]), ('cqq11_asiatica_clubes', 4262480, [129, 236, 0, 2, 0, 0]), ('cqq11_c1_vagas', 6539728, [129, 236, 0, 2, 0, 0]), ('cqq12_oceania_tabela', 7959088, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqq12_asia_clubes_times', 4240928, [129, 236, 0, 2, 0, 0]), ('cqq12_asia_recopa_times', 4248784, [129, 236, 0, 2, 0, 0])]
BASE = 1960
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
