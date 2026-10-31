"""Integra a 256a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('jogos_da_selecao_no_periodo', 5822224, [139, 84, 36, 12, 83]), ('atualizar_ranking_da_fifa', 5821536, [131, 236, 24, 83, 139, 92, 36, 32]), ('ordem_dos_resultados_por_pontos', 5825056, [139, 68, 36, 4, 139, 84, 36, 8]), ('resultado_no_historico', 5824160, [100, 161, 0, 0, 0, 0]), ('faixa_da_selecao', 5824912, [139, 68, 36, 4, 86]), ('pontuacoes_do_ranking', 5824448, [131, 236, 28, 161, 80, 35, 174, 0]), ('nacao_chave_do_jogo', 4775280, [139, 13, 116, 44, 174, 0]), ('e_copa_do_mundo', 6724112, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('e_torneio_internacional', 6724320, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('indice_da_nacao_chave', 6722672, [129, 236, 0, 2, 0, 0]), ('ordem_do_ranking', 5821968, [129, 236, 4, 2, 0, 0]), ('ordem_das_pontuacoes', 5825088, [139, 68, 36, 4, 139, 84, 36, 8]), ('tempo_das_cidades', 9580256, [81, 86, 87, 139, 249]), ('ler_regiao_do_clima', 9579584, [129, 236, 208, 7, 0, 0]), ('ler_periodo_do_clima', 9579808, [139, 68, 36, 4, 129, 236, 224, 10, 0, 0]), ('coeficientes_das_nacoes', 6119200, [85, 139, 236, 131, 228, 248]), ('dia_das_aposentadorias', 6120016, [102, 139, 68, 36, 4]), ('soma_presa', 6119888, [102, 15, 190, 76, 36, 4]), ('nota_do_estadio_neutro', 8772096, [131, 236, 12, 161, 228, 242, 156, 0]), ('fator_da_distancia', 8771744, [131, 236, 16, 139, 76, 36, 20]), ('ler_o_clima', 9578512, [129, 236, 240, 5, 0, 0]), ('fator_do_treinador', 9126352, [131, 236, 24, 139, 68, 36, 28]), ('comissao_do_humano', 9129168, [139, 84, 36, 4, 129, 236, 0, 2, 0, 0]), ('carga_do_programa', 9129824, [81, 83, 86, 87, 139, 249]), ('aplicar_o_progresso', 9120736, [129, 236, 12, 2, 0, 0]), ('achar_o_treino_de_posicao', 9128736, [139, 68, 36, 4, 129, 236, 8, 2, 0, 0]), ('aprende_depressa', 9129392, [81, 139, 84, 36, 8]), ('avancar_a_semana', 9115440, [139, 68, 36, 4, 86]), ('tirar_a_ficha_reservada', 8023600, [139, 65, 28, 86, 139, 116, 36, 12]), ('lesao_que_volta', 6483248, [139, 84, 36, 4, 86]), ('atualizar_amistosos_do_dia', 6038624, [100, 161, 0, 0, 0, 0]), ('planejar_pre_temporada_da_ia', 6039456, [129, 236, 8, 2, 0, 0]), ('janelas_da_pre_temporada', 6040896, [106, 255, 104, 78, 158, 149, 0]), ('marcar_jogos_da_excursao', 6045760, [106, 255, 104, 158, 159, 149, 0]), ('ordem_pela_faixa', 6050416, [139, 68, 36, 4, 139, 84, 36, 8]), ('ordem_pela_nacao_e_faixa', 6050448, [139, 68, 36, 4, 139, 84, 36, 8]), ('posicoes_previstas_da_copa', 5383072, [129, 236, 16, 2, 0, 0]), ('ordenar_clubes_pela_forca', 4983024, [129, 236, 4, 2, 0, 0]), ('gravar_posicao_do_clube', 4859200, [129, 236, 0, 2, 0, 0])]
BASE = 1155
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
