"""Integra a 243a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('copiar_recorde_de_nota', 4489776, [86, 87, 139, 124, 36, 12]), ('copiar_recorde_de_melhor_em_campo', 4490448, [86, 87, 139, 124, 36, 12]), ('copiar_recorde_de_disciplina', 4490496, [86, 87, 139, 124, 36, 12]), ('copiar_recorde_de_contagem', 4490544, [139, 68, 36, 4, 86]), ('copiar_recorde_do_torcedor', 4490672, [86, 87, 139, 124, 36, 12]), ('construir_recorde_de_ocorrencias', 4525152, [86, 139, 241, 141, 78, 17]), ('construir_recorde_de_contagem', 4525360, [86, 139, 241, 141, 78, 18]), ('copiar_recorde_de_valor', 4490576, [131, 236, 8, 86, 87]), ('copiar_recorde_de_gols_na_partida', 4527472, [86, 87, 139, 124, 36, 12]), ('copiar_noticia', 7888912, [131, 236, 8, 186, 50, 0, 0, 0]), ('copiar_transferencia_passada', 9222464, [139, 68, 36, 4, 131, 236, 8]), ('copiar_transferencia_futura', 9224000, [131, 236, 8, 141, 68, 36, 0]), ('copiar_entrada_da_fila_de_regens', 8031088, [139, 68, 36, 4, 131, 236, 8]), ('esta_na_lista_de_observados', 8659072, [139, 1, 131, 236, 8]), ('escolher_doador_de_dados', 8023648, [81, 161, 100, 35, 174, 0]), ('copiar_registro_de_suspensao', 5598320, [131, 236, 8, 85, 86]), ('salario_padrao_do_contrato', 8791776, [139, 68, 36, 4, 86]), ('condicao_de_jogo_do_jogador', 6472976, [86, 139, 116, 36, 8]), ('dias_ate_o_fim_do_contrato', 8780288, [161, 148, 44, 174, 0]), ('limpar_todas_as_queixas', 8811808, [86, 139, 241, 139, 6]), ('conferir_retirada_do_pedido', 8811072, [86, 139, 241, 87, 139, 6])]
BASE = 808
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
