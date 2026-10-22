"""Integra a 211a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('fechar_propostas_da_oferta', 9203120, [86, 139, 116, 36, 8]), ('enfileirar_transferencia_livre', 9180848, [129, 236, 12, 2, 0, 0]), ('negociacoes_de_troca_do_jogador', 9199552, [131, 236, 12, 139, 68, 36, 16]), ('tirar_jogador_da_selecao', 7822240, [131, 236, 8, 83, 86, 87]), ('faixa_do_horario', 5960432, [139, 68, 36, 4, 133, 192]), ('teto_da_observacao', 8696368, [15, 190, 68, 36, 12]), ('ofertas_pelo_jogador', 9198832, [81, 139, 68, 36, 8]), ('reputacao_alvo_do_clube', 8634176, [86, 139, 145, 220, 0, 0, 0]), ('gravar_jogo_na_noticia', 4914864, [83, 85, 139, 108, 36, 20])]
BASE = 514
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
