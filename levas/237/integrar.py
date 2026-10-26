"""Integra a 237a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('nome_pelos_textos', 5497568, [129, 236, 0, 2, 0, 0]), ('nome_do_cargo', 5483776, [15, 190, 68, 36, 4]), ('nome_do_idioma', 5492448, [15, 190, 68, 36, 4]), ('nome_da_selecao', 5513472, [139, 68, 36, 16, 129, 236, 0, 2, 0, 0]), ('nome_do_atributo_do_jogador', 5515424, [138, 76, 36, 4, 129, 236, 0, 2, 0, 0]), ('texto_das_posicoes', 5500496, [184, 164, 17, 0, 0]), ('nome_da_regiao', 6385440, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('adjetivo_do_continente', 6387696, [85, 139, 108, 36, 12])]
BASE = 761
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
