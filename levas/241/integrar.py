"""Integra a 241a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('ordenar_tabela_da_liga', 6825936, [131, 236, 76, 83, 85]), ('mini_tabela_do_confronto', 6826800, [129, 236, 16, 2, 0, 0]), ('liberar_tabela_da_liga', 6846832, [81, 83, 85, 139, 108, 36, 16]), ('liga_completa', 6855376, [83, 86, 139, 241, 87]), ('pagar_direitos_de_tv', 6828048, [161, 56, 42, 174, 0]), ('registrar_honras_da_temporada', 6833728, [131, 236, 12, 83, 85]), ('acrescentar_ao_historico', 4914400, [161, 124, 35, 174, 0]), ('tipo_de_confronto_na_liga', 4915936, [85, 139, 236, 131, 228, 248]), ('forca_do_clube', 4982560, [129, 236, 240, 1, 0, 0]), ('clube_colocado_mais_recente', 4914560, [129, 236, 28, 2, 0, 0]), ('amistoso_do_clube', 6064368, [83, 85, 86, 87, 139, 249]), ('lista_da_data', 5957280, [139, 84, 36, 12, 139, 68, 36, 4]), ('proxima_partida', 5949728, [131, 236, 20, 139, 68, 36, 24]), ('estadio_em_uso', 5957744, [131, 236, 20, 139, 68, 36, 24]), ('data_do_amistoso_internacional', 6077904, [102, 15, 190, 1, 139, 84, 36, 4])]
BASE = 773
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
