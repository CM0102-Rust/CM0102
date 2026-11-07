"""Integra a 282a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('cqr1_turno_do_carioca', 4435872, [106, 255, 104, 126, 74, 149, 0]), ('cqr1_turno_do_mineiro', 4411824, [106, 255, 104, 14, 72, 149, 0]), ('cqr1_turno_do_paranaense', 4425568, [106, 255, 104, 126, 73, 149, 0]), ('cqr1_turno_da_j1', 6700064, [106, 255, 104, 94, 193, 149, 0]), ('cqr2_virada_do_carioca', 4433568, [129, 236, 0, 2, 0, 0]), ('cqr2_virada_do_mineiro', 4409616, [129, 236, 0, 2, 0, 0]), ('cqr2_virada_do_paranaense', 4423776, [129, 236, 0, 2, 0, 0]), ('cqr3_fase_seguinte_da_nsl', 4267888, [139, 81, 44, 139, 65, 48]), ('cqr3_serie_final_da_nsl', 4268048, [106, 255, 104, 30, 58, 149, 0]), ('cqr3_grande_final_da_nsl', 4268480, [100, 161, 0, 0, 0, 0]), ('cqr4_fase_seguinte_da_mls', 9475072, [139, 81, 44, 139, 65, 48]), ('cqr4_play_off_da_mls', 9475264, [106, 255, 104, 30, 84, 150, 0]), ('cqr4_final_da_mls', 9476048, [106, 255, 104, 62, 84, 150, 0]), ('cqr5_virada_copa_do_brasil', 4357392, [129, 236, 0, 2, 0, 0]), ('cqr5_virada_copa_do_imperador', 6695248, [129, 236, 0, 2, 0, 0]), ('cqr5_virada_copa_da_liga_japonesa', 6708160, [129, 236, 0, 2, 0, 0]), ('cqr6_virada_supercopa_japonesa', 6710496, [129, 236, 0, 2, 0, 0]), ('cqr6_virada_copa_dos_eua', 9484176, [129, 236, 0, 2, 0, 0]), ('cqr6_virada_jogo_das_estrelas', 9480032, [129, 236, 0, 2, 0, 0]), ('cqr6_times_do_jogo_das_estrelas', 9479488, [129, 236, 4, 2, 0, 0]), ('cqr10_libertadores_dia', 4998160, [129, 236, 16, 2, 0, 0]), ('cqr10_libertadores_primeiro_grupo', 4994976, [129, 236, 4, 2, 0, 0]), ('cqr10_libertadores_grupo', 4995856, [100, 161, 0, 0, 0, 0]), ('cqr11_mercosul_dia', 5004432, [129, 236, 16, 2, 0, 0]), ('cqr11_mercosul_primeiro_grupo', 5000800, [129, 236, 4, 2, 0, 0]), ('cqr11_mercosul_grupo', 5001680, [100, 161, 0, 0, 0, 0]), ('cqr12_merconorte_dia', 4992144, [129, 236, 16, 2, 0, 0]), ('cqr12_merconorte_primeiro_grupo', 4988384, [129, 236, 4, 2, 0, 0]), ('cqr12_merconorte_grupo', 4989248, [100, 161, 0, 0, 0, 0]), ('cqr13_copa_america_dia', 6168864, [106, 255, 104, 230, 168, 149, 0]), ('cqr13_copa_america_primeiro_grupo', 6164960, [129, 236, 4, 2, 0, 0]), ('cqr13_copa_america_grupo', 6166192, [100, 161, 0, 0, 0, 0]), ('cqs1_tabela_da_mls', 9471328, [129, 236, 0, 2, 0, 0]), ('cqs1_segunda_divisao_da_mls', 9473984, [100, 161, 0, 0, 0, 0]), ('cqs1_terceira_divisao_da_mls', 9474528, [100, 161, 0, 0, 0, 0]), ('cqs2_virada_da_mls', 9473536, [129, 236, 0, 2, 0, 0]), ('cqs2_posicoes_previstas_da_mls', 9477040, [129, 236, 0, 2, 0, 0]), ('cqs2_clubes_da_mls', 9476672, [129, 236, 0, 2, 0, 0]), ('cqs3_grupo_da_serie_c', 4386432, [106, 255, 104, 190, 69, 149, 0]), ('cqs3_segunda_fase_da_serie_c', 4383792, [106, 255, 104, 78, 69, 149, 0]), ('cqs3_terceira_fase_da_serie_c', 4384512, [106, 255, 104, 124, 69, 149, 0])]
BASE = 1986
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
