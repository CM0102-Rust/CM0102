"""Integra a 255a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('pedido_de_transferencia', 5080592, [100, 161, 0, 0, 0, 0]), ('cargo_com_contrato', 5028640, [138, 68, 36, 4, 60, 11]), ('fora_da_diretoria', 5028192, [139, 76, 36, 4, 138, 65, 61]), ('iniciar_permissoes_de_trabalho', 9203296, [106, 255, 104, 104, 61, 150, 0]), ('contagem_da_clausula', 9190992, [138, 68, 36, 8, 86]), ('premio_por_jogo_do_jogador', 9244320, [131, 236, 16, 86, 87]), ('fila_da_transferencia_livre', 9181376, [106, 255, 104, 176, 59, 150, 0]), ('proxima_janela_aberta', 9237616, [106, 255, 104, 214, 63, 150, 0]), ('emprestimo_negociado', 9204432, [83, 86, 139, 241, 87]), ('oferta_com_clausula', 9211856, [100, 161, 0, 0, 0, 0]), ('motivo_da_recusa_vale', 9196896, [83, 86, 87, 139, 124, 36, 20]), ('motivo_pelo_status_no_elenco', 9196736, [139, 68, 36, 4, 86]), ('relacoes_do_elenco', 6122768, [131, 236, 8, 83, 85, 86]), ('atrito_com_o_tecnico', 6123920, [131, 236, 8, 83, 85, 86]), ('moral_pela_lideranca', 6086832, [131, 236, 28, 161, 92, 35, 174, 0]), ('faltas_ao_treino', 6122144, [81, 139, 13, 100, 35, 174, 0]), ('competicoes_do_dia', 6086752, [161, 248, 173, 173, 0]), ('mudar_confianca_da_diretoria', 6086112, [139, 84, 36, 4, 133, 210]), ('estadios_do_dia', 8763424, [131, 236, 20, 83, 139, 92, 36, 28]), ('reservar_estadio', 8773152, [129, 236, 8, 2, 0, 0]), ('estadio_livre_na_data', 8773040, [131, 236, 20, 86, 139, 241]), ('jogo_com_observado', 8361760, [131, 236, 12, 83, 139, 92, 36, 20]), ('na_lista_do_olheiro', 8352912, [139, 84, 36, 4, 129, 236, 4, 2, 0, 0]), ('registrar_a_primeira_convocacao', 8834128, [106, 255, 104, 59, 45, 150, 0]), ('gols_na_carreira', 8142112, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('registrar_o_emprestimo', 8837664, [106, 255, 104, 155, 45, 150, 0]), ('ordem_da_volta_aos_treinos', 6496320, [139, 68, 36, 4, 86]), ('gols_pelo_clube', 8141744, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('registrar_a_selecao_do_ano', 8832816, [100, 161, 0, 0, 0, 0]), ('ordem_pelos_pontos_do_tecnico', 8847904, [139, 68, 36, 4, 139, 76, 36, 8]), ('jogo_guardado_igual', 5949344, [100, 161, 0, 0, 0, 0]), ('guardar_jogo_no_calendario', 5943856, [100, 161, 0, 0, 0, 0]), ('tirar_jogo_do_calendario', 5946416, [100, 161, 0, 0, 0, 0]), ('criar_amistosos_planejados', 6040288, [106, 255, 104, 16, 158, 149, 0]), ('acrescentar_amistoso', 6057728, [106, 255, 104, 92, 161, 149, 0]), ('planejar_amistoso_em_data_livre', 6046592, [106, 255, 104, 228, 159, 149, 0]), ('atualizar_copa_no_dia', 5372048, [106, 255, 104, 152, 119, 149, 0]), ('montar_rodada_da_copa', 5359616, [106, 255, 104, 217, 118, 149, 0]), ('risco_de_doenca_na_viagem', 5944256, [100, 161, 0, 0, 0, 0])]
BASE = 1116
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
