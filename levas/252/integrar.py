"""Integra a 252a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('comparar_datas_de_decisao', 5097744, [83, 139, 92, 36, 8]), ('decidir_as_datas_vencidas', 5036480, [83, 85, 86, 87, 139, 249]), ('ofertas_de_contrato_do_clube', 5022464, [129, 236, 8, 2, 0, 0]), ('assinar_contrato_de_emprestimo', 5046752, [106, 255, 104, 27, 110, 149, 0]), ('encerrar_o_emprestimo', 5048288, [129, 236, 8, 2, 0, 0]), ('trocar_lugar_no_elenco', 5029152, [81, 83, 139, 92, 36, 16]), ('diretoria_do_tecnico_novo', 6900864, [131, 236, 8, 83, 85]), ('motivo_da_saida_do_tecnico', 6927184, [131, 236, 20, 83, 85]), ('registro_do_tecnico', 8844720, [129, 236, 16, 2, 0, 0]), ('serve_para_o_cargo', 6944784, [85, 86, 139, 116, 36, 12]), ('pode_assumir_o_cargo', 5028240, [15, 190, 68, 36, 4]), ('baixar_moral_pela_partida', 5026384, [131, 236, 8, 86, 87]), ('montar_vagas_da_formacao', 8634512, [81, 139, 68, 36, 8]), ('refazer_lista_do_clube', 8628016, [86, 87, 139, 124, 36, 12]), ('suspender_clubes_que_miram_o_jogador', 8661760, [106, 255, 104, 219, 36, 150, 0]), ('por_jogador_nas_listas_dos_clubes', 8662016, [131, 236, 20, 83, 85]), ('por_jogador_a_venda', 8659744, [83, 85, 86, 87, 139, 124, 36, 20]), ('pagar_premio_por_jogos', 9220928, [131, 236, 16, 83, 85]), ('clube_recusa_a_oferta', 9189040, [86, 139, 241, 139, 76, 36, 8]), ('soltar_jogadores_da_oferta', 9202400, [106, 255, 104, 24, 61, 150, 0]), ('montar_oferta_em_dinheiro', 9209664, [106, 255, 104, 206, 61, 150, 0]), ('acrescentar_surto_de_doenca', 6481952, [129, 236, 8, 2, 0, 0]), ('espalhar_surtos_de_doenca', 6482400, [106, 255, 104, 136, 177, 149, 0]), ('jogador_fora_da_temporada', 6475312, [106, 255, 104, 32, 177, 149, 0]), ('forma_inicial_dos_elencos', 6465824, [106, 255, 104, 144, 176, 149, 0]), ('registros_iniciais_de_lesao', 6475632, [129, 236, 28, 2, 0, 0]), ('listas_de_clubes_por_nacao', 8048576, [106, 255, 104, 107, 242, 149, 0]), ('jogador_do_ano_da_torcida', 4533664, [85, 139, 236, 131, 228, 248]), ('arquivar_estatisticas_da_nacao', 8131408, [129, 236, 12, 2, 0, 0]), ('recordes_de_valor_de_transferencia', 4491104, [131, 236, 16, 83, 139, 92, 36, 24]), ('par_de_nacoes_sem_amistoso', 6079456, [81, 138, 129, 6, 53, 0, 0]), ('ordem_das_nacoes_pela_reputacao', 6079984, [139, 68, 36, 4, 139, 8]), ('ordem_dos_amistosos_pela_data', 6080016, [139, 68, 36, 4, 86]), ('pode_marcar_jogo_do_clube', 7827392, [106, 255, 104, 72, 223, 149, 0]), ('ordem_das_convocacoes_pela_data', 7835280, [86, 139, 116, 36, 12]), ('acrescentar_nacao_classificada', 7829936, [139, 84, 36, 4, 131, 236, 8]), ('ordem_dos_cabecas_de_chave_decrescente', 5382544, [139, 76, 36, 4, 139, 84, 36, 8]), ('ordem_dos_cabecas_de_chave_crescente', 5382592, [139, 76, 36, 4, 139, 84, 36, 8]), ('teto_da_divisao_dos_clubes', 6859136, [86, 87, 139, 249, 51, 246])]
BASE = 1006
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
