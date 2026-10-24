"""Integra a 224a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('primeiro_nome_da_nacao', 5479520, [131, 236, 20, 83, 85]), ('segundo_nome_da_nacao', 5480880, [81, 83, 85, 86, 87]), ('formatar_nome_da_pessoa', 5495712, [129, 236, 0, 2, 0, 0]), ('cores_do_uniforme_do_clube', 5486144, [102, 161, 240, 189, 174, 0]), ('tecnico_do_clube_ou_selecao', 5491344, [129, 236, 0, 2, 0, 0]), ('nome_ingles_do_clube', 5527680, [106, 255, 104, 251, 121, 149, 0]), ('ajustar_moral_da_pessoa', 5523216, [138, 76, 36, 8, 132, 201]), ('vetor_de_atributos_do_nao_jogador', 5482672, [139, 68, 36, 4, 139, 76, 36, 12]), ('marcar_elenco_da_pessoa', 5507456, [138, 76, 36, 8, 139, 68, 36, 4])]
BASE = 595
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
