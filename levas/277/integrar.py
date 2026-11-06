"""Integra a 277a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('regras_noruega', 7947504, [106, 255, 104, 43, 233, 149, 0]), ('regras_irlanda_do_norte', 7943888, [106, 255, 104, 187, 231, 149, 0]), ('regras_pais_de_gales', 9498752, [106, 255, 104, 75, 86, 150, 0]), ('regras_turquia', 9445536, [106, 255, 104, 171, 81, 150, 0]), ('regras_polonia', 8181808, [106, 255, 104, 171, 249, 149, 0]), ('regras_finlandia', 5937488, [106, 255, 104, 75, 149, 149, 0]), ('janelas_argentina', 4236832, [129, 236, 0, 2, 0, 0]), ('janelas_portugal', 8207856, [129, 236, 0, 2, 0, 0]), ('regras_coreia', 6743600, [106, 255, 104, 155, 200, 149, 0]), ('janelas_grecia', 6213408, [129, 236, 0, 2, 0, 0]), ('janelas_russia', 8308704, [129, 236, 0, 2, 0, 0]), ('janelas_belgica', 4344832, [129, 236, 0, 2, 0, 0]), ('inscricao_europeia', 8759728, [131, 236, 16, 138, 68, 36, 20]), ('ranking_fifa_inicial', 5823824, [161, 80, 35, 174, 0]), ('lista_de_estadios_neutros', 8773568, [129, 236, 48, 2, 0, 0]), ('cql7_tabela_da_serie_a', 4367488, [129, 236, 0, 2, 0, 0]), ('cql7_tabela_da_serie_b', 4377200, [129, 236, 0, 2, 0, 0]), ('cql7_tabela_da_serie_c', 4381840, [129, 236, 0, 2, 0, 0]), ('cql8_tabela_do_baiano', 4390160, [129, 236, 0, 2, 0, 0]), ('cql8_tabela_do_centro_oeste', 4394736, [129, 236, 0, 2, 0, 0]), ('cql8_tabela_do_gaucho', 4399568, [129, 236, 0, 2, 0, 0]), ('cql9_tabela_do_goiano', 4404544, [129, 236, 0, 2, 0, 0]), ('cql9_tabela_do_mineiro', 4409360, [129, 236, 0, 2, 0, 0]), ('cql9_tabela_do_norte', 4414848, [129, 236, 0, 2, 0, 0]), ('cqm7_tabela_do_nordeste', 4418960, [129, 236, 0, 2, 0, 0]), ('cqm7_tabela_do_paranaense', 4423360, [129, 236, 0, 2, 0, 0]), ('cqm7_tabela_do_pernambucano', 4428592, [129, 236, 0, 2, 0, 0]), ('cqm8_tabela_do_carioca', 4433152, [129, 236, 0, 2, 0, 0]), ('cqm8_tabela_do_catarinense', 4438896, [129, 236, 0, 2, 0, 0]), ('cqm8_tabela_do_paulista', 4443200, [129, 236, 0, 2, 0, 0]), ('cqm9_tabela_da_j1', 6698208, [129, 236, 0, 2, 0, 0]), ('cqm9_tabela_da_j2', 6705200, [129, 236, 0, 2, 0, 0]), ('cqm9_tabela_da_nsl', 4266864, [129, 236, 0, 2, 0, 0]), ('cqn7_mata_mata_do_baiano', 4390992, [106, 255, 104, 46, 70, 149, 0]), ('cqn7_mata_mata_do_norte', 4415680, [106, 255, 104, 126, 72, 149, 0]), ('cqn7_mata_mata_do_nordeste', 4419792, [106, 255, 104, 238, 72, 149, 0]), ('cqn8_mata_mata_do_pernambucano', 4429584, [106, 255, 104, 238, 73, 149, 0]), ('cqn8_mata_mata_do_catarinense', 4439888, [106, 255, 104, 238, 74, 149, 0]), ('cqn8_mata_mata_do_paulista', 4444032, [106, 255, 104, 94, 75, 149, 0])]
BASE = 1810
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
