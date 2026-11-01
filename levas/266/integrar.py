"""Integra a 266a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('fim_de_temporada_da_nacao', 5885392, [129, 236, 8, 2, 0, 0]), ('fim_de_temporada_do_clube', 5885856, [129, 236, 28, 2, 0, 0]), ('noticia_do_dividendo', 5897744, [106, 255, 104, 43, 145, 149, 0]), ('teto_salarial', 5859536, [131, 236, 52, 83, 86]), ('noticia_das_obras', 5898160, [106, 255, 104, 107, 145, 149, 0]), ('noticia_da_obra_recusada', 5896928, [100, 161, 0, 0, 0, 0]), ('conflitos_do_dia', 5954464, [106, 255, 104, 235, 151, 149, 0]), ('conflitos_do_jogo', 5953408, [106, 255, 104, 201, 151, 149, 0]), ('carregar_conferencia_das_pernas', 5960592, [129, 236, 20, 2, 0, 0]), ('nova_data_do_jogo', 5944592, [106, 255, 104, 7, 151, 149, 0]), ('jogo_no_calendario', 5940864, [106, 255, 104, 36, 150, 149, 0]), ('ano_do_calendario', 5939920, [106, 255, 104, 234, 149, 149, 0]), ('copa_criar_jogos_do_confronto', 5360528, [106, 255, 104, 17, 119, 149, 0]), ('copa_noticia_do_confronto_pendente', 5384528, [106, 255, 104, 251, 119, 149, 0]), ('janelas_avisos_do_dia', 9202624, [106, 255, 104, 72, 61, 150, 0]), ('copa_levar_jogos_as_datas', 5368800, [106, 255, 104, 90, 119, 149, 0]), ('tirar_jogador_da_troca', 9191872, [106, 255, 104, 104, 60, 150, 0]), ('permissoes_da_oferta', 9205392, [81, 83, 85, 86, 139, 116, 36, 20]), ('confianca_depois_da_partida', 6902336, [131, 236, 44, 139, 68, 36, 60]), ('noticia_da_reacao_da_diretoria', 6971296, [106, 255, 104, 22, 207, 149, 0]), ('noticia_da_avaliacao_da_federacao', 6969952, [106, 255, 104, 203, 206, 149, 0]), ('expectativas_da_temporada', 6961008, [131, 236, 16, 83, 85, 86]), ('vaga_de_tecnico', 6928976, [106, 255, 104, 160, 204, 149, 0]), ('noticia_da_oferta_de_emprego', 6948160, [100, 161, 0, 0, 0, 0]), ('boatos_de_transferencia', 7686704, [106, 255, 104, 187, 216, 149, 0]), ('noticia_nao_vende', 5797376, [100, 161, 0, 0, 0, 0]), ('noticia_do_titulo', 7717776, [100, 161, 0, 0, 0, 0]), ('amistosos_entre_selecoes', 6077936, [106, 255, 104, 198, 163, 149, 0]), ('convite_de_amistoso_valido', 6070416, [100, 161, 0, 0, 0, 0]), ('convite_de_torneio_valido', 6070928, [129, 236, 16, 2, 0, 0]), ('dia_das_selecoes', 7778560, [106, 255, 104, 8, 221, 149, 0]), ('busca_atual_sem_clube', 8356896, [15, 191, 5, 122, 124, 183, 0]), ('clube_nas_buscas_da_sessao', 8357120, [15, 191, 5, 122, 124, 183, 0]), ('escolher_elenco_da_selecao', 7782688, [106, 255, 104, 122, 221, 149, 0]), ('olheiros_na_partida', 8358912, [106, 255, 104, 216, 15, 150, 0]), ('sessao_de_edicao', 8350688, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('escolher_sessao_de_edicao', 8350448, [139, 68, 36, 8, 129, 236, 0, 2, 0, 0]), ('dia_das_transferencias', 9197696, [100, 161, 0, 0, 0, 0]), ('processar_oferta_vencida', 9200544, [106, 255, 104, 0, 61, 150, 0]), ('restricoes_da_oferta', 9239696, [106, 255, 104, 248, 63, 150, 0])]
BASE = 1509
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
