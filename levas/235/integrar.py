"""Integra a 235a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('clube_acrescentar_pessoa_favorita', 5535856, [86, 139, 116, 36, 8]), ('clube_acrescentar_pessoa_rejeitada', 5536064, [86, 139, 116, 36, 8]), ('acrescentar_pessoa_favorita', 5536272, [83, 139, 92, 36, 8]), ('acrescentar_clube_rejeitado', 5537392, [86, 87, 139, 124, 36, 12]), ('esmaecer_relacoes', 5537680, [86, 139, 116, 36, 8]), ('pessoas_rejeitadas', 5538032, [139, 68, 36, 12, 86]), ('mascara_da_posicao_natural', 5538208, [81, 139, 76, 36, 8]), ('contar_elenco_com_mascara', 5538128, [139, 68, 36, 4, 83]), ('escolher_companheiro_favorito', 5541280, [131, 236, 16, 83, 139, 92, 36, 24]), ('clube_fora_da_argentina', 5539760, [139, 68, 36, 4, 133, 192]), ('clube_fora_de_argentina_franca_croacia', 5539792, [139, 68, 36, 4, 133, 192]), ('nacao_da_melanesia', 5495040, [139, 68, 36, 4, 139, 13, 28, 244, 156, 0]), ('nacao_da_polinesia', 5495104, [139, 68, 36, 4, 139, 13, 208, 244, 156, 0]), ('nome_do_cargo_de_treino', 5483584, [15, 190, 68, 36, 4]), ('anos_entre_datas_em_texto', 5505856, [129, 236, 0, 2, 0, 0]), ('ordem_dos_clubes_pelo_nome_curto', 5505152, [129, 236, 160, 15, 0, 0]), ('ordem_das_pessoas_pelo_nome_completo', 5505648, [129, 236, 160, 15, 0, 0]), ('pessoa_confere_com_os_nomes', 5499280, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('ordem_dos_clubes_pela_latitude', 5504320, [129, 236, 8, 2, 0, 0]), ('ordem_dos_clubes_pela_longitude', 5504592, [129, 236, 8, 2, 0, 0])]
BASE = 721
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
