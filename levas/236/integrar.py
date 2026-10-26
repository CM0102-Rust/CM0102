"""Integra a 236a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('nome_por_ids', 5497152, [139, 68, 36, 16, 129, 236, 8, 2, 0, 0]), ('nome_oficial', 5499568, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('texto_da_mascara_da_posicao', 5538544, [83, 87, 139, 124, 36, 16]), ('nome_da_posicao', 5530032, [129, 236, 4, 2, 0, 0]), ('descricao_do_jogador', 5531040, [129, 236, 4, 2, 0, 0]), ('melhor_candidato_a_favorito', 5539856, [131, 236, 8, 83, 139, 92, 36, 16]), ('forcas_do_jogador', 5532336, [131, 236, 12, 139, 68, 36, 20]), ('fraquezas_do_jogador', 5533904, [131, 236, 12, 139, 68, 36, 20]), ('contar_fisioterapeutas', 5491120, [129, 236, 0, 2, 0, 0]), ('clube_e_time_b', 5523648, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('pessoas_fora_do_elenco', 5524592, [139, 76, 36, 4, 129, 236, 0, 2, 0, 0]), ('atributo_visivel', 5517952, [139, 68, 36, 4, 86]), ('atributo_para_quem_ve', 5518400, [129, 236, 0, 2, 0, 0]), ('nome_do_atributo_de_tecnico', 5520144, [138, 76, 36, 4, 129, 236, 0, 2, 0, 0]), ('ordem_das_pessoas_pelo_sobrenome', 5504864, [129, 236, 160, 15, 0, 0]), ('ordem_dos_ponteiros_de_clube_pela_reputacao', 5504992, [139, 68, 36, 4, 139, 76, 36, 8]), ('nacao_do_clube_na_cee', 5494736, [139, 68, 36, 4, 139, 64, 83]), ('regioes_pelo_nome', 5529024, [176, 1, 185, 4, 33, 174, 0]), ('ordem_das_regioes_pelo_nome', 5529088, [139, 68, 36, 4, 129, 236, 144, 1, 0, 0]), ('pessoa_da_opiniao', 5545424, [139, 76, 36, 4, 51, 192])]
BASE = 741
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
