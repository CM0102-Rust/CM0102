"""Integra a 214a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('ordem_das_linhas_da_tabela', 6828560, [81, 83, 139, 29, 252, 121, 182, 0]), ('montar_grade_de_turnos', 6833328, [139, 76, 36, 8, 131, 236, 16]), ('margem_do_ultimo_dia', 6857936, [15, 191, 129, 169, 0, 0, 0]), ('partida_das_vagas_da_noticia', 4915216, [83, 85, 139, 108, 36, 20]), ('resultado_da_eliminatoria_na_noticia', 4917392, [83, 86, 139, 116, 36, 16]), ('dia_da_rodada_da_copa', 5381904, [102, 139, 76, 36, 12]), ('mapa_de_casas_da_formacao', 5992800, [131, 236, 72, 83, 85]), ('nota_do_clube_para_o_jovem', 8049184, [83, 85, 139, 108, 36, 12]), ('partida_assistida_pelo_humano', 6283920, [129, 236, 0, 2, 0, 0])]
BASE = 534
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
