"""Integra a 232a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('liberar_historico_de_pessoas', 5406432, [161, 204, 35, 174, 0]), ('liberar_historico_de_premios', 5406496, [161, 228, 35, 174, 0]), ('crescer_historico_de_competicoes', 5406560, [161, 124, 35, 174, 0]), ('ordem_dos_clubes_pela_reputacao', 5505056, [139, 68, 36, 4, 139, 76, 36, 8]), ('ordem_do_elenco_pela_habilidade', 5505776, [139, 68, 36, 4, 139, 76, 36, 8]), ('ordem_do_elenco_pela_habilidade_atual', 5545008, [139, 68, 36, 4, 139, 76, 36, 8]), ('normalizar_atributos_fisicos_do_jogador', 5476032, [129, 236, 120, 2, 0, 0]), ('segundo_nome_pelo_peso', 5480672, [81, 139, 68, 36, 8]), ('normalizar_atributos_do_jogador', 5475264, [129, 236, 116, 2, 0, 0]), ('ler_clubes_v1', 5561488, [81, 139, 68, 36, 20]), ('ler_nao_jogadores_v1', 5564832, [139, 68, 36, 16, 83]), ('ler_jogadores_v1', 5566800, [139, 68, 36, 16, 83]), ('ler_competicoes_v1', 5570512, [139, 68, 36, 16, 83]), ('nome_ingles_da_nacao', 5526512, [106, 255, 104, 91, 121, 149, 0]), ('nome_curto_ingles_do_clube', 5527408, [106, 255, 104, 219, 121, 149, 0]), ('cortar_elencos_de_a_e_b', 5512800, [161, 92, 35, 174, 0]), ('juntar_o_time_b_ao_a', 5523824, [131, 236, 16, 161, 92, 35, 174, 0]), ('montar_indice_de_nomes', 5478224, [161, 80, 35, 174, 0]), ('tirar_pessoa_do_clube', 5489040, [129, 236, 8, 2, 0, 0]), ('olheiros_do_clube', 5492064, [129, 236, 0, 2, 0, 0])]
BASE = 661
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
