"""Integra a 234a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('clube_tem_defensores', 5542144, [81, 86, 139, 116, 36, 12]), ('clube_tem_meias', 5542400, [81, 86, 139, 116, 36, 12]), ('clube_tem_atacantes', 5542592, [81, 86, 139, 116, 36, 12]), ('clube_tem_esforcados', 5542848, [81, 86, 139, 116, 36, 12]), ('clube_tem_goleiro', 5543024, [139, 68, 36, 4, 85]), ('clube_tem_meias_que_roubam', 5544016, [81, 86, 139, 116, 36, 12]), ('faltam_atacantes_pelo_alto', 5543120, [81, 86, 139, 116, 36, 12]), ('faltam_zagueiros_pelo_alto', 5543344, [81, 86, 139, 116, 36, 12]), ('faltam_zagueiros_rapidos', 5543568, [81, 86, 139, 116, 36, 12]), ('clube_tem_atacante_rapido', 5543792, [81, 86, 139, 116, 36, 12]), ('clube_tem_meias_criativos', 5544256, [81, 86, 139, 116, 36, 12]), ('faixa_de_forca_do_elenco', 5544496, [129, 236, 204, 0, 0, 0]), ('reputacao_do_clube_para_pessoa', 5529232, [131, 236, 8, 83, 85]), ('contar_rejeitados_no_clube', 5536912, [139, 76, 36, 4, 50, 192]), ('contar_favoritos_no_clube', 5536992, [139, 76, 36, 4, 50, 192]), ('tirar_clube_favorito', 5537616, [139, 68, 36, 4, 51, 210]), ('clube_tirar_pessoa_favorita', 5536720, [139, 68, 36, 4, 51, 210]), ('tirar_pessoa_favorita', 5536784, [139, 68, 36, 4, 51, 210]), ('tirar_pessoa_rejeitada', 5536848, [139, 68, 36, 4, 51, 210]), ('pessoas_favoritas', 5537936, [139, 68, 36, 12, 86])]
BASE = 701
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
