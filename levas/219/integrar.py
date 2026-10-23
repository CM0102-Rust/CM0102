"""Integra a 219a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('filtro_do_jogo_salvo', 8621584, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('filtro_do_inicio_rapido', 8622736, [129, 236, 0, 2, 0, 0]), ('versao_do_jogo_salvo_valida', 8621744, [100, 161, 0, 0, 0, 0]), ('gravar_regras_fifa', 8623536, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('carregar_regras_fifa', 8623968, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('montar_resumo_do_save', 5840768, [131, 236, 112, 139, 68, 36, 116]), ('gravar_banco_no_save', 5393712, [139, 68, 36, 12, 139, 76, 36, 8]), ('carregar_banco_do_save', 5393680, [139, 68, 36, 8, 139, 76, 36, 4]), ('liberar_dados_do_jogo', 8585024, [83, 185, 80, 68, 183, 0])]
BASE = 565
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
