"""Integra a 267a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('aceitar_lance', 9175168, [106, 255, 104, 72, 59, 150, 0]), ('noticia_da_oferta_feita', 9253008, [106, 255, 104, 198, 66, 150, 0]), ('pode_emprestar', 9240640, [106, 255, 104, 27, 64, 150, 0]), ('liga_dia', 6834112, [106, 255, 104, 33, 202, 149, 0]), ('liga_criar_tabela', 6829504, [106, 255, 104, 129, 201, 149, 0]), ('liga_promover_e_rebaixar', 6855808, [129, 236, 16, 2, 0, 0]), ('calendario_dia', 5943040, [106, 255, 104, 112, 150, 149, 0]), ('calendario_lista', 5947152, [100, 161, 0, 0, 0, 0]), ('calendario_carregar_ano', 5948336, [129, 236, 84, 2, 0, 0]), ('copa_arvore_dos_confrontos', 5343120, [129, 236, 24, 2, 0, 0]), ('competicao_primeira_fase_do_grupo', 4772528, [129, 236, 0, 2, 0, 0]), ('competicao_fase_atual', 4771520, [129, 236, 0, 4, 0, 0]), ('atualizar_o_treino_do_jogador', 9115472, [129, 236, 52, 4, 0, 0]), ('programa_de_treino_do_jogador', 9124336, [129, 236, 0, 2, 0, 0]), ('nivel_da_categoria_do_programa', 9129808, [15, 190, 68, 36, 4]), ('tirar_ou_devolver_o_progresso', 9123392, [129, 236, 8, 2, 0, 0]), ('mudanca_do_atributo_pelo_treino', 9123776, [129, 236, 8, 2, 0, 0]), ('aptidao_para_a_posicao', 9127792, [15, 191, 68, 36, 8]), ('aposentadorias_do_dia', 8018400, [106, 255, 104, 168, 241, 149, 0]), ('noticia_da_aposentadoria', 8032992, [100, 161, 0, 0, 0, 0]), ('noticia_do_jovem_promovido', 8031136, [106, 255, 104, 11, 242, 149, 0]), ('limpar_ausencia', 4298400, [85, 139, 108, 36, 8]), ('limpar_falta_ao_treino', 4298688, [85, 139, 108, 36, 8]), ('noticia_da_falta_ao_treino', 4302384, [100, 161, 0, 0, 0, 0]), ('troca_com_compensacao', 8697584, [83, 139, 92, 36, 20]), ('ordem_das_vagas', 8632000, [139, 68, 36, 4, 139, 76, 36, 8]), ('marcar_sob_oferta', 8697216, [106, 255, 104, 232, 37, 150, 0]), ('campeao_da_liga_nos_registros', 8817744, [106, 255, 104, 118, 43, 150, 0]), ('campeao_da_copa_nos_registros', 8824832, [106, 255, 104, 38, 44, 150, 0]), ('limpar_registro_do_humano', 8846448, [139, 68, 36, 4, 139, 21, 100, 35, 174, 0])]
BASE = 1549
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
