"""Integra a 231a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('gravar_historico_de_pessoas', 5570128, [139, 68, 36, 8, 86]), ('gravar_competicoes', 5571024, [139, 68, 36, 8, 86]), ('gravar_historicos_de_competicao', 5571360, [139, 68, 36, 8, 86]), ('gravar_competicoes_de_pessoal', 5571696, [139, 68, 36, 8, 86]), ('gravar_arbitros', 5572272, [139, 68, 36, 8, 86]), ('gravar_registros_58', 5572608, [139, 68, 36, 8, 86]), ('gravar_nomes', 5572944, [139, 68, 36, 8, 86]), ('abrir_arquivo_do_banco', 5559680, [139, 68, 36, 8, 139, 76, 36, 4]), ('criar_arquivo_do_banco', 5559728, [139, 68, 36, 8, 139, 76, 36, 4])]
BASE = 652
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
