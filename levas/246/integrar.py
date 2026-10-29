"""Integra a 246a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('soltar_suspensoes_guardadas', 5597136, [83, 139, 92, 36, 8]), ('tirar_ofertas_pendentes', 5032944, [86, 87, 139, 249, 139, 71, 52]), ('marcar_decisao_da_transferencia', 5032240, [86, 87, 139, 249, 139, 71, 52]), ('noticia_para_todos_os_tecnicos', 7890816, [129, 236, 0, 2, 0, 0]), ('noticia_para_os_tecnicos_da_divisao', 7889616, [129, 236, 0, 2, 0, 0]), ('noticia_para_os_tecnicos_do_pais', 7889904, [129, 236, 4, 2, 0, 0]), ('oferta_bate_o_recorde', 4490944, [83, 86, 87, 139, 124, 36, 16]), ('condicao_do_jogador', 6473104, [86, 139, 116, 36, 8]), ('limite_de_escalacao', 9238496, [131, 236, 44, 51, 192]), ('limpar_partidas_assistidas', 6285520, [81, 83, 85, 86, 51, 219]), ('alvos_dos_olheiros_do_clube', 8361616, [81, 83, 139, 92, 36, 16]), ('trocar_olheiro_da_busca', 8054944, [83, 86, 139, 241, 131, 203, 255]), ('diretoria_depois_da_partida', 6902240, [83, 86, 139, 116, 36, 12]), ('disciplina_da_partida', 5622272, [83, 86, 139, 116, 36, 12]), ('entradas_do_clube_na_shortlist', 8659168, [131, 236, 8, 83, 139, 92, 36, 16]), ('maior_oferta_pelo_jogador', 9226544, [81, 83, 139, 217, 139, 76, 36, 12])]
BASE = 867
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
