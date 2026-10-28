"""Integra a 244a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('lugar_no_elenco', 5028704, [81, 83, 85, 86, 139, 116, 36, 20]), ('subir_para_o_principal', 5030064, [83, 139, 92, 36, 8]), ('descer_para_os_reservas', 5030128, [83, 139, 92, 36, 8]), ('copiar_contrato_de_pessoal', 5031952, [131, 236, 8, 83, 85]), ('subir_moral_pela_partida', 5025952, [86, 139, 116, 36, 16]), ('tirar_conversas_de_contrato', 5022144, [81, 139, 193, 83, 51, 219]), ('tirar_interesses_de_transferencia', 7686576, [81, 139, 193, 51, 210]), ('tirar_pedido_de_observacao', 8359648, [131, 236, 28, 83, 139, 217]), ('tirar_apelido_de_todos', 6291248, [139, 68, 36, 4, 131, 236, 36]), ('tirar_aposentadoria_pendente', 8029840, [139, 84, 36, 4, 83]), ('aposentadoria_do_desempregado', 8028240, [161, 84, 31, 222, 0]), ('oferta_precisa_de_permissao', 9204848, [83, 86, 139, 116, 36, 12]), ('atualizar_permissoes_de_trabalho', 9204528, [81, 85, 86, 139, 241]), ('olheiro_da_busca', 8055120, [86, 139, 241, 139, 134, 128, 1, 0, 0]), ('sequencia_atual_do_clube', 6884400, [139, 84, 36, 4, 83]), ('construir_recordista', 4525376, [86, 139, 241, 141, 78, 16]), ('tecnico_rival_para_comentar', 7518528, [131, 236, 80, 83, 86])]
BASE = 829
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
