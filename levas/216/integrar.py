"""Integra a 216a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('liga_tem_rebaixamento', 6834096, [138, 145, 193, 0, 0, 0]), ('data_da_rodada_da_liga', 6857056, [139, 76, 36, 20, 139, 68, 36, 24]), ('importancia_do_jogo_da_liga', 6857696, [81, 139, 68, 36, 16]), ('copiar_recordista', 4490384, [139, 68, 36, 4, 131, 236, 8]), ('copiar_registro_de_partida', 4489488, [131, 236, 8, 141, 68, 36, 0]), ('copiar_registro_de_sequencia', 4489584, [131, 236, 8, 141, 84, 36, 0]), ('humano_assiste_a_partida', 6284320, [129, 236, 0, 2, 0, 0]), ('humano_deu_apelido', 6293312, [129, 236, 32, 2, 0, 0]), ('copiar_aposentadoria_pendente', 8030080, [131, 236, 8, 86, 87])]
BASE = 545
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
