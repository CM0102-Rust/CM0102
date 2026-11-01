"""Integra a 263a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('copa_premio_da_rodada', 5382720, [131, 236, 8, 139, 68, 36, 12]), ('copa_mandantes_da_rodada', 5367920, [139, 68, 36, 8, 131, 236, 8]), ('copa_progresso_do_time', 5380912, [102, 139, 65, 60, 83]), ('amistoso_responder_pedido', 6053696, [106, 255, 104, 187, 160, 149, 0]), ('amistoso_enviar_pedido', 6051184, [100, 161, 0, 0, 0, 0]), ('amistoso_lista_das_nacoes', 6071264, [129, 236, 0, 2, 0, 0]), ('copa_sortear_rodada', 5358608, [100, 161, 0, 0, 0, 0]), ('nome_da_fase_e_rodada', 4980672, [139, 68, 36, 12, 129, 236, 200, 2, 0, 0]), ('clube_da_divisao_mais_alta', 5383008, [139, 68, 36, 4, 139, 72, 87]), ('copa_registros_pelo_desfecho', 5347472, [83, 86, 139, 241, 87]), ('copa_honras_da_temporada', 5347984, [131, 236, 20, 83, 85]), ('copa_semifinalistas_para_a_uefa', 5349312, [131, 236, 16, 83, 85]), ('recusar_oferta_de_emprestimo', 9173552, [139, 68, 36, 4, 85]), ('vaga_no_elenco_para_a_oferta', 9229392, [129, 236, 32, 2, 0, 0]), ('percentual_da_proxima_venda_achado', 9244112, [131, 236, 16, 86, 87]), ('jogador_na_busca', 8058368, [106, 255, 104, 134, 243, 149, 0]), ('sincronizar_listas_dos_olheiros', 8352368, [129, 236, 48, 2, 0, 0]), ('vaga_do_jogador_observado', 8115968, [129, 236, 0, 2, 0, 0]), ('valor_do_jogador_para_o_clube', 8654208, [85, 139, 236, 131, 228, 248]), ('entrada_do_jogador_na_busca_do_clube', 8357392, [15, 191, 5, 122, 124, 183, 0]), ('premio_de_jogos_na_selecao_achado', 9244416, [131, 236, 16, 86, 87]), ('contratos_depois_do_jogo', 5023440, [85, 139, 236, 131, 228, 248]), ('moral_da_ficha', 5030608, [139, 68, 36, 4, 139, 72, 97]), ('texto_da_moral', 5030624, [139, 68, 36, 4, 139, 72, 97]), ('valor_do_jogador', 5049440, [85, 139, 236, 131, 228, 248]), ('diferenca_de_moral_em_texto', 5030816, [139, 68, 36, 4, 139, 84, 36, 8]), ('marcas_de_elenco_para_a_tela', 5030192, [129, 236, 0, 2, 0, 0]), ('jogadores_que_a_diretoria_cita', 6991584, [129, 236, 220, 0, 0, 0]), ('veredito_da_nota_na_partida', 6993520, [106, 255, 104, 27, 208, 149, 0]), ('texto_da_diretoria_sobre_os_jogadores', 6992592, [139, 76, 36, 4, 129, 236, 100, 2, 0, 0]), ('resultado_nos_registros_dos_tecnicos', 8816688, [131, 236, 8, 83, 85]), ('posicoes_no_fim_da_liga', 8822896, [106, 255, 104, 219, 43, 150, 0]), ('jogador_do_ano_da_torcida_no_registro', 8838192, [106, 255, 104, 187, 45, 150, 0]), ('vice_da_copa_nos_registros', 8827056, [106, 255, 104, 70, 44, 150, 0]), ('terceiro_da_copa_nos_registros', 8828688, [106, 255, 104, 102, 44, 150, 0]), ('troca_de_tecnico_nos_registros', 8821408, [106, 255, 104, 193, 43, 150, 0]), ('reacoes_dos_elencos_a_transferencia', 7700992, [131, 236, 24, 83, 85]), ('noticia_da_comemoracao_da_contratacao', 7714528, [100, 161, 0, 0, 0, 0]), ('noticia_do_companheiro_desassossegado', 7720896, [100, 161, 0, 0, 0, 0])]
BASE = 1419
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
