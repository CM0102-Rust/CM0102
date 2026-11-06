"""Integra a 280a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('calendario_da_primeira_inglesa', 5708160, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('calendario_da_premiership', 5715552, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('calendario_da_primera', 8709200, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('cqp7_virada_do_baiano', 4390416, [129, 236, 0, 2, 0, 0]), ('cqp7_fase_seguinte_do_baiano', 4390848, [139, 81, 44, 139, 65, 48]), ('cqp7_virada_do_centro_oeste', 4395152, [129, 236, 0, 2, 0, 0]), ('cqp7_fase_seguinte_do_centro_oeste', 4395584, [139, 81, 44, 139, 65, 48]), ('cqp7_virada_do_gaucho', 4399984, [129, 236, 0, 2, 0, 0]), ('cqp7_fase_seguinte_do_gaucho', 4400416, [139, 81, 44, 139, 65, 48]), ('cqp8_virada_do_goiano', 4404976, [129, 236, 0, 2, 0, 0]), ('cqp8_fase_seguinte_do_goiano', 4405408, [139, 81, 44, 139, 65, 48]), ('cqp8_virada_do_norte', 4415104, [129, 236, 0, 2, 0, 0]), ('cqp8_fase_seguinte_do_norte', 4415536, [139, 81, 44, 139, 65, 48]), ('cqp8_virada_do_nordeste', 4419216, [129, 236, 0, 2, 0, 0]), ('cqp8_fase_seguinte_do_nordeste', 4419648, [139, 81, 44, 139, 65, 48]), ('cqp9_virada_do_pernambucano', 4429008, [129, 236, 0, 2, 0, 0]), ('cqp9_fase_seguinte_do_pernambucano', 4429440, [139, 81, 44, 139, 65, 48]), ('cqp9_virada_do_catarinense', 4439312, [129, 236, 0, 2, 0, 0]), ('cqp9_fase_seguinte_do_catarinense', 4439744, [139, 81, 44, 139, 65, 48]), ('cqp9_virada_do_paulista', 4443456, [129, 236, 0, 2, 0, 0]), ('cqp9_fase_seguinte_do_paulista', 4443888, [139, 81, 44, 139, 65, 48]), ('calendario_da_segunda_espanhola', 8731792, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('calendario_da_divisao_de_baixo_espanhola', 8720992, [138, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('construir_primeira_francesa', 6009184, [106, 255, 104, 129, 153, 149, 0]), ('cqq7_fase_seguinte_da_serie_a', 4368720, [139, 81, 44, 139, 65, 48]), ('cqq7_play_off_da_serie_a', 4368864, [100, 161, 0, 0, 0, 0]), ('cqq7_fase_seguinte_da_serie_b', 4378064, [139, 81, 44, 139, 65, 48]), ('cqq7_play_off_da_serie_b', 4378208, [100, 161, 0, 0, 0, 0]), ('cqq8_virada_da_serie_a', 4367936, [129, 236, 0, 2, 0, 0]), ('cqq8_acesso_e_rebaixamento', 4370064, [161, 252, 173, 173, 0]), ('cqq8_virada_da_serie_b', 4377616, [129, 236, 0, 2, 0, 0]), ('cqq9_fase_seguinte_da_j1', 6699456, [139, 81, 44, 139, 65, 48]), ('cqq9_final_da_j1', 6699600, [100, 161, 0, 0, 0, 0]), ('cqq9_virada_da_j1', 6698704, [129, 236, 0, 2, 0, 0]), ('tabela_das_sedes', 6265200, [106, 255, 104, 91, 175, 149, 0]), ('noticia_das_sedes', 6270656, [100, 161, 0, 0, 0, 0]), ('carregar_sedes', 6271328, [129, 236, 0, 2, 0, 0])]
BASE = 1923
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
