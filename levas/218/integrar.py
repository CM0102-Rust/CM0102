"""Integra a 218a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('casar_nome_com_padrao', 9648640, [81, 83, 85, 139, 108, 36, 20]), ('id_do_conteiner', 9646528, [129, 236, 0, 2, 0, 0]), ('tamanho_dos_itens_do_padrao', 9648224, [129, 236, 4, 2, 0, 0]), ('empacotar_no_conteiner', 9647632, [129, 236, 32, 4, 0, 0]), ('desempacotar_do_conteiner', 9646880, [129, 236, 20, 3, 0, 0]), ('limpar_pasta_temporaria', 9489744, [184, 28, 104, 0, 0]), ('ler_do_conteiner', 9645104, [83, 85, 86, 139, 116, 36, 28]), ('gravar_no_conteiner', 9645584, [83, 85, 86, 139, 116, 36, 28]), ('fechar_conteiner', 9644800, [86, 139, 116, 36, 8])]
BASE = 556
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
