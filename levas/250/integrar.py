"""Integra a 250a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('noticia_pelo_alcance', 7889040, [15, 190, 68, 36, 24]), ('noticia_ao_tecnico_do_clube', 7889376, [139, 84, 36, 4, 129, 236, 0, 2, 0, 0]), ('entregar_noticia_ao_tecnico', 7888192, [129, 236, 0, 2, 0, 0]), ('responder_todas_as_noticias', 7894192, [129, 236, 0, 2, 0, 0]), ('tecnico_tem_noticia_pendente', 7893840, [139, 68, 36, 8, 129, 236, 4, 2, 0, 0]), ('gravar_texto_na_noticia', 7887040, [129, 236, 0, 2, 0, 0]), ('nota_do_jogador_para_a_historia', 7563040, [131, 236, 16, 85, 139, 108, 36, 28]), ('texto_da_crise_financeira', 7647072, [139, 68, 36, 8, 83]), ('noticias_da_caixa', 7891296, [129, 236, 8, 2, 0, 0]), ('aposentar_do_elenco', 8039648, [83, 86, 139, 116, 36, 12]), ('melhor_clube_para_o_jovem', 8048864, [131, 236, 20, 137, 76, 36, 16]), ('clube_da_nacao_para_o_jovem', 8050064, [129, 236, 24, 2, 0, 0]), ('abrir_vagas_que_faltam', 6934624, [129, 236, 8, 2, 0, 0]), ('melhor_nota_de_candidato_a_tecnico', 6931776, [131, 236, 8, 83, 85]), ('agendar_oferta', 9200160, [129, 236, 16, 2, 0, 0]), ('estatuto_no_elenco_pelo_clube', 8691456, [83, 86, 87, 139, 124, 36, 20]), ('estimar_estatuto_no_elenco', 8691040, [131, 236, 8, 83, 85]), ('liberar_alvos_da_lista_do_clube', 8668768, [131, 236, 8, 83, 139, 92, 36, 20]), ('busca_cobre_a_nacao', 8090768, [131, 236, 12, 83, 85]), ('tirar_amistoso_marcado', 6064048, [129, 236, 0, 2, 0, 0]), ('iniciar_pares_proibidos_de_amistoso', 6079184, [129, 236, 0, 2, 0, 0])]
BASE = 946
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
