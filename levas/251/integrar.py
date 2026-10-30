"""Integra a 251a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('avaliar_o_negocio_da_transferencia', 6907248, [131, 236, 8, 137, 76, 36, 0]), ('confianca_da_diretoria_na_transferencia', 6912240, [81, 83, 86, 87, 139, 124, 36, 20]), ('resolver_o_ultimato_da_diretoria', 6997104, [131, 236, 8, 83, 86]), ('tirar_o_contrato_principal', 5049168, [86, 139, 116, 36, 8]), ('reenviar_ofertas_guardadas', 5043360, [85, 86, 87, 139, 249]), ('mes_do_fim_da_temporada_do_clube', 5064176, [106, 255, 104, 64, 110, 149, 0]), ('pontos_dos_tecnicos_no_dia', 8838560, [86, 87, 139, 124, 36, 12]), ('pontos_dos_tecnicos_no_fim_da_competicao', 8836832, [139, 84, 36, 4, 83]), ('tecnico_do_mes_na_liga', 8845648, [129, 236, 32, 2, 0, 0]), ('saidas_pendentes_dos_tecnicos', 6927952, [129, 236, 4, 2, 0, 0]), ('tirar_jogador_do_elenco_do_clube', 6883120, [83, 86, 139, 116, 36, 12]), ('tirar_pessoa_do_cargo_no_clube', 6883472, [139, 13, 80, 16, 174, 0]), ('construir_noticia', 7846880, [106, 255, 104, 254, 224, 149, 0]), ('ajustar_confianca_da_torcida', 7570240, [100, 161, 0, 0, 0, 0]), ('ajustar_confianca_da_diretoria', 7571184, [100, 161, 0, 0, 0, 0]), ('resposta_do_tecnico_da_maquina', 7559280, [81, 83, 85, 86, 139, 116, 36, 20]), ('marcar_noticia_lida', 7893312, [129, 236, 4, 2, 0, 0]), ('achar_noticia_pelo_id', 7891824, [129, 236, 0, 2, 0, 0]), ('noticia_de_gratidao_do_jogador', 7548032, [100, 161, 0, 0, 0, 0]), ('noticia_de_pedido_de_apoio', 7615344, [100, 161, 0, 0, 0, 0]), ('noticia_de_reacao_a_critica', 7617104, [100, 161, 0, 0, 0, 0]), ('montar_listas_de_posicao', 8625760, [129, 236, 4, 2, 0, 0]), ('ordenar_listas_de_posicao', 8628096, [81, 83, 85, 86, 87]), ('clubes_interessados_no_jogador', 8659328, [131, 236, 20, 83, 85]), ('acrescentar_na_lista_de_observacao', 8659952, [129, 236, 8, 2, 0, 0]), ('prever_resposta_do_vendedor', 8695552, [139, 13, 80, 16, 174, 0]), ('periodo_do_emprestimo', 9242000, [131, 236, 16, 50, 210]), ('prazo_da_resposta_a_oferta', 9225648, [129, 236, 20, 2, 0, 0]), ('pagar_prestacoes_das_transferencias', 9219808, [131, 236, 12, 161, 92, 35, 174, 0]), ('tirar_oferta_de_transferencia', 9244512, [83, 86, 139, 116, 36, 12]), ('acrescentar_suspensao', 5597504, [129, 236, 4, 2, 0, 0]), ('tirar_suspensoes_de_fora_do_clube', 5595760, [129, 236, 8, 2, 0, 0]), ('restaurar_suspensoes_guardadas', 5596528, [131, 236, 32, 83, 85]), ('acrescentar_idioma', 8153648, [129, 236, 0, 2, 0, 0]), ('melhorar_idioma', 8154272, [131, 236, 8, 83, 86]), ('zerar_estatisticas_do_jogador', 8152656, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('definir_competicao_da_selecao', 7819808, [139, 84, 36, 4, 129, 236, 4, 2, 0, 0]), ('marcar_clubes_com_jogo', 7828928, [161, 80, 35, 174, 0]), ('tirar_jogador_da_disciplina', 5597232, [131, 236, 8, 83, 85])]
BASE = 967
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
