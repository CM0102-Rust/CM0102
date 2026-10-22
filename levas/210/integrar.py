"""Integra a 210a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('historia_punivel_do_jogador', 7559104, [139, 68, 36, 4, 83, 85]), ('escolher_tipo_de_lesao', 6476096, [81, 86, 87, 139, 124, 36, 24]), ('categoria_da_noticia', 7886448, [139, 1, 129, 236, 0, 2, 0, 0]), ('apagar_repetidos_das_convocacoes', 7782496, [81, 83, 85, 50, 192]), ('vaga_do_escalado', 9007808, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('escolher_comentarista', 7518224, [81, 83, 85, 139, 108, 36, 20]), ('regras_da_rodada_no_jogo', 5381472, [85, 139, 108, 36, 8]), ('chave_do_jogo_na_copa', 5348496, [129, 236, 0, 2, 0, 0]), ('pais_da_excursao', 6047360, [81, 139, 68, 36, 8])]
BASE = 505
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
