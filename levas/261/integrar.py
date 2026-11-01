"""Integra a 261a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('convocar_selecoes_para_o_jogo', 7781040, [100, 161, 0, 0, 0, 0]), ('noticia_do_convocado', 7798032, [100, 161, 0, 0, 0, 0]), ('noticia_do_desfecho_na_copa', 5346960, [106, 255, 104, 219, 117, 149, 0]), ('contar_vagas_com_lance', 8668480, [131, 236, 20, 83, 139, 92, 36, 28]), ('tirar_da_lista_de_transferencias', 8667776, [139, 68, 36, 4, 139, 13, 80, 16, 174, 0]), ('tirar_da_lista_de_emprestimos', 8667856, [139, 68, 36, 4, 139, 13, 80, 16, 174, 0]), ('situacao_na_lista_de_observacao', 8666640, [106, 255, 104, 24, 37, 150, 0]), ('por_na_lista_de_emprestimos', 8667824, [139, 68, 36, 4, 139, 13, 80, 16, 174, 0]), ('teto_de_preco_pelo_estatuto', 8696304, [139, 68, 36, 8, 86]), ('distribuir_verba_de_transferencias', 8632032, [131, 236, 44, 83, 85]), ('estatuto_permitido', 8697040, [139, 76, 36, 4, 129, 236, 0, 2, 0, 0]), ('nome_da_lista_do_olheiro', 8372560, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('amistosos_internacionais_do_dia', 6077184, [106, 255, 104, 160, 163, 149, 0]), ('lista_escolhida_do_tecnico', 8351280, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('escolher_lista_do_tecnico', 8351520, [139, 68, 36, 8, 129, 236, 0, 2, 0, 0]), ('inscritos_da_liga_dos_campeoes', 5787552, [129, 236, 20, 2, 0, 0]), ('tabela_da_qualificacao_da_liga_dos_campeoes', 5786000, [129, 236, 0, 2, 0, 0]), ('proxima_fase_da_liga_dos_campeoes', 5788112, [139, 81, 44, 139, 65, 48]), ('processar_jogo_da_liga_dos_campeoes', 5782288, [83, 86, 139, 116, 36, 12]), ('progresso_do_clube_na_liga_dos_campeoes', 5794816, [139, 68, 36, 4, 139, 73, 8]), ('conta_para_conflito_de_datas', 4776800, [139, 68, 36, 4, 15, 191, 72, 48]), ('escolher_arbitro_da_partida', 7980976, [131, 236, 8, 83, 85, 86]), ('limpar_arbitros_ocupados', 7978816, [139, 81, 4, 51, 192]), ('renovar_arbitros', 7982816, [129, 236, 8, 2, 0, 0]), ('arbitros_sorteados', 7980480, [129, 236, 0, 2, 0, 0]), ('validar_arbitros', 7981728, [139, 13, 144, 35, 174, 0]), ('copiar_ficha_de_arbitro', 7980336, [131, 236, 8, 86, 87]), ('registrar_premio_do_ano', 8831888, [100, 161, 0, 0, 0, 0]), ('vitoria_maior', 8276528, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('derrota_maior', 8276736, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('jogos_das_permissoes_e_clausulas', 9189872, [129, 236, 200, 1, 0, 0]), ('guardar_premio_por_jogos_na_selecao', 9219472, [129, 236, 0, 2, 0, 0]), ('noticia_transferencia_finalizada', 9283040, [106, 255, 104, 150, 68, 150, 0]), ('veto_dos_fisios_e_da_diretoria', 6908688, [131, 236, 20, 138, 68, 36, 36]), ('noticia_do_ultimato', 6997760, [100, 161, 0, 0, 0, 0]), ('texto_do_ultimato', 6998144, [129, 236, 8, 2, 0, 0]), ('liberar_sem_custo', 9181936, [129, 236, 8, 2, 0, 0]), ('ferias_do_humano', 6279920, [129, 236, 0, 2, 0, 0]), ('noticia_conversa_de_contrato', 9272528, [100, 161, 0, 0, 0, 0])]
BASE = 1341
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
