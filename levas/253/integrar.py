"""Integra a 253a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('pensar_em_demitir_o_tecnico', 6925216, [131, 236, 8, 161, 204, 110, 221, 0]), ('dar_o_ultimato_da_diretoria', 6997344, [106, 255, 104, 136, 208, 149, 0]), ('tirar_o_tecnico_do_cargo', 6928272, [131, 236, 8, 83, 85]), ('avaliar_a_selecao_na_competicao', 6968848, [131, 236, 8, 83, 85]), ('clube_em_apuro_financeiro', 6941520, [86, 139, 116, 36, 8]), ('faixa_da_confianca_da_diretoria', 6884208, [139, 68, 36, 4, 139, 9]), ('registrar_o_acesso', 8819840, [106, 255, 104, 150, 43, 150, 0]), ('registrar_o_rebaixamento', 8823232, [106, 255, 104, 6, 44, 150, 0]), ('registrar_a_classificacao_na_fase', 8830416, [100, 161, 0, 0, 0, 0]), ('registrar_a_transferencia', 8834416, [106, 255, 104, 91, 45, 150, 0]), ('registrar_a_eliminacao', 8830976, [100, 161, 0, 0, 0, 0]), ('registrar_a_nao_classificacao', 8831440, [100, 161, 0, 0, 0, 0]), ('data_de_conclusao_da_oferta', 9236080, [129, 236, 16, 2, 0, 0]), ('primeiro_dia_permitido_da_oferta', 9236704, [106, 255, 104, 168, 63, 150, 0]), ('janela_fixa_de_transferencias', 9243568, [106, 255, 104, 144, 64, 150, 0]), ('compensacao_pela_pessoa', 9242496, [106, 255, 104, 59, 64, 150, 0]), ('pagar_transferencia_e_clausulas', 9218208, [131, 236, 24, 83, 85]), ('clube_vendedor_deve_aceitar', 9228864, [106, 255, 104, 0, 63, 150, 0]), ('permissao_de_trabalho_na_mudanca', 9207328, [100, 161, 0, 0, 0, 0]), ('concluir_oferta_de_contrato_aceita', 9189552, [100, 161, 0, 0, 0, 0]), ('oferecer_contrato_ao_jogador_da_oferta', 9182784, [106, 255, 104, 200, 59, 150, 0]), ('desfazer_oferta', 9192672, [86, 139, 116, 36, 8]), ('fazer_oferta_de_emprestimo', 9170272, [106, 255, 104, 195, 58, 150, 0]), ('concluir_transferencias_futuras', 9223136, [129, 236, 12, 2, 0, 0]), ('nota_do_doador_de_dados', 8024400, [131, 236, 8, 86, 139, 116, 36, 16]), ('ordem_das_aposentadorias', 8053408, [83, 86, 87, 139, 124, 36, 16]), ('ordem_da_fila_de_regens', 8053312, [86, 139, 116, 36, 8]), ('registro_da_categoria', 8141088, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('criar_estatisticas', 8150400, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('aposentadoria_marcada', 8053584, [139, 68, 36, 4, 86]), ('copa_sortear_times_da_rodada', 5366304, [129, 236, 40, 2, 0, 0]), ('copa_separar_estadio_compartilhado', 5383696, [131, 236, 20, 86, 87]), ('escala_pela_rodada', 4978768, [102, 139, 76, 36, 16]), ('lado_do_jogo_para_o_clube', 4913792, [86, 139, 116, 36, 12]), ('ordem_dos_coeficientes_das_nacoes', 4914080, [139, 76, 36, 4, 139, 84, 36, 8]), ('pedido_de_observacao', 8359280, [106, 255, 104, 251, 15, 150, 0]), ('limpar_vaga_da_escalacao', 9006736, [129, 236, 0, 2, 0, 0]), ('marcar_batedor_de_bola_parada', 9010096, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0])]
BASE = 1045
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
