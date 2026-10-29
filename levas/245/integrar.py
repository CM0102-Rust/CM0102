"""Integra a 245a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('construir_recorde_de_valor_pago', 4490704, [106, 255, 104, 91, 84, 149, 0]), ('construir_recorde_de_valor_recebido', 4490784, [106, 255, 104, 123, 84, 149, 0]), ('construir_recorde_de_sequencia', 4490864, [106, 255, 104, 155, 84, 149, 0]), ('construir_recorde_de_vitorias_seguidas', 4524752, [106, 255, 104, 155, 86, 149, 0]), ('construir_recorde_de_derrotas_seguidas', 4524832, [106, 255, 104, 187, 86, 149, 0]), ('construir_recorde_de_invencibilidade', 4524912, [106, 255, 104, 219, 86, 149, 0]), ('construir_recorde_sem_vencer', 4524992, [106, 255, 104, 251, 86, 149, 0]), ('construir_recorde_de_gols_na_partida', 4525072, [106, 255, 104, 27, 87, 149, 0]), ('construir_recorde_do_mais_novo', 4525168, [106, 255, 104, 67, 87, 149, 0]), ('texto_do_fim_do_prazo', 9307552, [86, 87, 139, 124, 36, 12]), ('pode_encerrar_emprestimo', 9241904, [83, 139, 92, 36, 12]), ('pow_do_game', 6117920, [221, 68, 36, 4, 221, 68, 36, 12]), ('tirar_ofertas_guardadas', 5043552, [86, 139, 241, 139, 70, 56]), ('guardar_suspensoes_do_emprestado', 5596144, [129, 236, 16, 2, 0, 0]), ('preencher_vaga_da_comissao', 9221840, [129, 236, 0, 2, 0, 0]), ('noticia_para_os_tecnicos_da_nacao', 7890208, [129, 236, 0, 2, 0, 0]), ('missao_do_olheiro', 8056784, [129, 236, 0, 2, 0, 0]), ('copa_completa', 5381632, [86, 139, 241, 87, 138, 134, 177, 0, 0, 0]), ('construir_recorde_do_mais_velho', 4525264, [106, 255, 104, 99, 87, 149, 0]), ('permissao_de_trabalho_de', 9204320, [139, 68, 36, 4, 131, 236, 16]), ('passa_na_permissao', 8696192, [86, 139, 116, 36, 8])]
BASE = 846
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
