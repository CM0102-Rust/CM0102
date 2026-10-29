"""Integra a 248a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('clube_tem_oferta_pelo_jogador', 9231504, [106, 255, 104, 24, 63, 150, 0]), ('oferta_do_clube_pelo_jogador', 9231744, [106, 255, 104, 56, 63, 150, 0]), ('jogador_com_oferta_pendente', 9228624, [106, 255, 104, 216, 62, 150, 0]), ('jogador_sob_oferta', 9231984, [106, 255, 104, 88, 63, 150, 0]), ('clube_do_jogador_com_lei_bosman', 9243248, [106, 255, 104, 91, 64, 150, 0]), ('veto_da_diretoria_na_oferta', 9231232, [86, 139, 116, 36, 8]), ('guardar_transferencia_passada', 9222160, [129, 236, 0, 2, 0, 0]), ('dar_permissao_de_trabalho', 9203824, [129, 236, 0, 2, 0, 0]), ('guardar_premio_por_jogo_no_time', 9219136, [129, 236, 0, 2, 0, 0]), ('vaga_livre_de_contrato', 5011872, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('acrescentar_contrato_da_pessoa', 5011792, [86, 139, 241, 106, 1]), ('acrescentar_data_de_decisao', 5035728, [129, 236, 16, 2, 0, 0]), ('zerar_moral_do_contrato', 8810208, [83, 86, 139, 241, 51, 219]), ('contrato_pelo_atual', 8775056, [129, 236, 8, 2, 0, 0]), ('faixa_do_vencimento', 8813872, [86, 87, 139, 249, 138, 71, 53]), ('registrar_historia_da_midia', 7699024, [129, 236, 8, 2, 0, 0]), ('expirar_historias_da_midia', 7699568, [131, 236, 36, 83, 139, 217]), ('historia_ja_registrada', 7699872, [131, 236, 32, 85, 86]), ('acrescentar_interesse_de_transferencia', 7685952, [129, 236, 36, 2, 0, 0]), ('ordem_das_chaves_de_ids', 7700128, [139, 84, 36, 4, 86]), ('ordem_das_historias', 7700192, [139, 84, 36, 4, 86])]
BASE = 904
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
