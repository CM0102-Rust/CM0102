"""Integra a 283a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('cqs4_final_da_serie_c', 4385152, [100, 161, 0, 0, 0, 0]), ('cqs4_virada_da_serie_c', 4382448, [129, 236, 0, 2, 0, 0]), ('cqs4_posicoes_previstas_da_serie_c', 4386976, [129, 236, 4, 2, 0, 0]), ('cqs5_trofeu_clubes', 5682992, [129, 236, 128, 2, 0, 0]), ('cqs5_trofeu_fase_das_chaves', 5683888, [100, 161, 0, 0, 0, 0]), ('cqs5_trofeu_fase_seguinte', 5684304, [139, 81, 44, 139, 65, 48]), ('cqs6_trofeu_final', 5684448, [100, 161, 0, 0, 0, 0]), ('cqs6_trofeu_classificacao', 5685264, [129, 236, 16, 2, 0, 0]), ('cqs7_fase_de_copa', 5385728, [106, 255, 104, 59, 120, 149, 0]), ('cqs7_fim_da_fase_de_copa', 5387008, [86, 139, 241, 139, 134, 167, 0, 0, 0]), ('cqs10_libertadores_mata_mata', 4996368, [106, 255, 104, 190, 106, 149, 0]), ('cqs10_merconorte_semifinais', 4989744, [106, 255, 104, 14, 106, 149, 0]), ('cqs10_merconorte_fase_seguinte', 4989104, [139, 81, 44, 139, 65, 48]), ('cqs11_merconorte_convidados', 4990304, [129, 236, 36, 4, 0, 0]), ('cqs11_mercosul_fase_seguinte', 5001536, [139, 81, 44, 139, 65, 48]), ('cqs11_copa_america_fase_seguinte', 6166048, [139, 81, 44, 139, 65, 48]), ('cqs12_copa_america_selecoes', 6167760, [129, 236, 16, 2, 0, 0]), ('cqs12_copa_america_previstas', 6169408, [129, 236, 0, 2, 0, 0]), ('cqs12_copa_america_temporada_nova', 6165808, [129, 236, 0, 2, 0, 0]), ('cqt1_construir_baiano', 4387536, [106, 255, 104, 233, 69, 149, 0]), ('cqt1_construir_centro_oeste', 4392112, [106, 255, 104, 89, 70, 149, 0]), ('cqt1_construir_gaucho', 4396992, [106, 255, 104, 201, 70, 149, 0]), ('cqt2_construir_goiano', 4402000, [106, 255, 104, 57, 71, 149, 0]), ('cqt2_construir_mineiro', 4406608, [106, 255, 104, 169, 71, 149, 0]), ('cqt2_construir_norte', 4412224, [106, 255, 104, 57, 72, 149, 0]), ('cqt3_construir_nordeste', 4416496, [106, 255, 104, 169, 72, 149, 0]), ('cqt3_construir_paranaense', 4420608, [106, 255, 104, 25, 73, 149, 0]), ('cqt3_construir_pernambucano', 4425968, [106, 255, 104, 169, 73, 149, 0]), ('cqt4_construir_carioca', 4430400, [106, 255, 104, 25, 74, 149, 0]), ('cqt4_construir_catarinense', 4436272, [106, 255, 104, 169, 74, 149, 0]), ('cqt4_construir_paulista', 4440704, [106, 255, 104, 25, 75, 149, 0]), ('cqt4_atualizar_media_de_publico', 6120064, [85, 139, 236, 131, 228, 248]), ('cqt10_oceania_dia', 7961408, [106, 255, 104, 22, 235, 149, 0]), ('cqt10_oceania_grupo_a', 7960768, [129, 236, 4, 2, 0, 0]), ('cqt10_oceania_grupo_b', 7962240, [100, 161, 0, 0, 0, 0]), ('cqt11_oceania_clubes', 7963168, [129, 236, 252, 3, 0, 0]), ('cqt11_oceania_fase_seguinte', 7962096, [139, 81, 44, 139, 65, 48]), ('cqt11_oceania_semifinais', 7962704, [100, 161, 0, 0, 0, 0])]
BASE = 2027
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
