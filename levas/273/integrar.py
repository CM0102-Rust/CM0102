"""Integra a 273a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('uefa_rodar_coeficientes', 9452368, [85, 139, 236, 131, 228, 248]), ('uefa_vagas_por_nacao', 9452736, [129, 236, 4, 2, 0, 0]), ('uefa_enesimo_clube_da_nacao', 9457872, [131, 236, 12, 15, 191, 68, 36, 20]), ('uefa_juntar_clubes_europeus', 9453632, [129, 236, 8, 2, 0, 0]), ('uefa_vagas_liga_dos_campeoes', 9454096, [129, 236, 100, 2, 0, 0]), ('uefa_vagas_intertoto', 9457200, [131, 236, 16, 83, 85]), ('uefa_vagas_copa_da_uefa', 9454976, [129, 236, 72, 2, 0, 0]), ('uefa_voluntario_ingles', 9451664, [106, 255, 104, 59, 82, 150, 0]), ('uefa_virada_da_classificacao', 9450912, [100, 161, 0, 0, 0, 0]), ('conmebol_vagas_libertadores', 5006384, [161, 60, 246, 156, 0]), ('conmebol_enesimo_clube_da_nacao', 5007152, [131, 236, 12, 15, 191, 68, 36, 20]), ('conmebol_convidado_da_mercosul', 5007504, [139, 68, 36, 4, 133, 192]), ('conmebol_juntar_clubes', 5005952, [161, 92, 35, 174, 0]), ('conmebol_virada_do_ano', 5005744, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('conmebol_campeao_dos_campeoes', 5007712, [139, 68, 36, 4, 139, 16]), ('sede_da_edicao', 6268720, [139, 1, 83, 85, 86]), ('proxima_sede', 6268880, [139, 1, 86, 102, 139, 113, 4]), ('calendario_das_sedes', 6269008, [106, 255, 104, 120, 175, 149, 0]), ('regras_inglaterra', 5741360, [106, 255, 104, 235, 134, 149, 0]), ('regras_eua', 9484608, [106, 255, 104, 59, 85, 150, 0]), ('regras_australia', 4271232, [106, 255, 104, 27, 59, 149, 0]), ('janelas_espanha', 8754288, [129, 236, 0, 2, 0, 0]), ('janelas_italia', 6689776, [129, 236, 0, 2, 0, 0]), ('janelas_franca', 6034912, [129, 236, 0, 2, 0, 0]), ('regras_brasil', 4452576, [106, 255, 104, 219, 78, 149, 0]), ('janelas_alemanha', 6160352, [129, 236, 0, 2, 0, 0]), ('regras_escocia', 8348560, [106, 255, 104, 235, 14, 150, 0]), ('cqi10_intertoto_dia', 6503744, [129, 236, 4, 2, 0, 0]), ('cqi10_intertoto_registros', 6503440, [138, 68, 36, 8, 83]), ('cqi10_uefa_registros', 9449696, [139, 68, 36, 8, 139, 84, 36, 4]), ('cqi11_concacaf_classificados', 4985248, [129, 236, 160, 9, 0, 0]), ('cqi11_concacaf_dia', 4986400, [129, 236, 0, 2, 0, 0]), ('cqi11_concacaf_rodadas', 4986352, [86, 139, 241, 106, 0]), ('cqi12_copa_italia_regras', 6550352, [83, 86, 139, 241, 176, 4]), ('cqi12_copa_c_regras', 6543072, [83, 86, 139, 241, 176, 4]), ('cqi12_copa_dos_campeoes_regras', 4350512, [83, 86, 139, 241, 176, 3]), ('cqj10_copa_africana_registros', 4201376, [138, 68, 36, 12, 86]), ('cqj10_copa_america_registros', 6165424, [138, 68, 36, 12, 86]), ('cqj10_eliminatoria_oceania_registros', 9562176, [138, 68, 36, 12, 86])]
BASE = 1658
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
