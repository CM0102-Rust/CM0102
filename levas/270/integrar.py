"""Integra a 270a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('por_oferta_de_transferencia', 9167616, [106, 255, 104, 106, 58, 150, 0]), ('aceitar_emprestimo', 9176672, [139, 68, 36, 8, 85]), ('termos_aceitos', 9183616, [100, 161, 0, 0, 0, 0]), ('noticia_de_jogador_listado', 5075376, [106, 255, 104, 22, 111, 149, 0]), ('contrato_oferecido', 5036656, [106, 255, 104, 104, 109, 149, 0]), ('mundial_sortear_grupos', 9627968, [129, 236, 28, 2, 0, 0]), ('mundial_sorteio_dos_grupos', 9627632, [129, 236, 0, 2, 0, 0]), ('mundial_dia', 9630112, [83, 139, 92, 36, 12]), ('mundial_tabela_do_grupo_um', 9621744, [129, 236, 4, 2, 0, 0]), ('mundial_fase_do_grupo', 9622848, [100, 161, 0, 0, 0, 0]), ('mundial_posicoes_previstas', 9631360, [129, 236, 0, 2, 0, 0]), ('mundial_rodadas_da_fase', 9618064, [129, 236, 0, 2, 0, 0]), ('mundial_nacao_classificada', 9627456, [102, 139, 65, 54, 129, 236, 0, 2, 0, 0]), ('mundial_cabecas_de_chave', 9630256, [129, 236, 28, 2, 0, 0]), ('distribuir_os_treinadores', 9119872, [131, 236, 52, 83, 86]), ('trocar_os_programas_do_elenco', 9118832, [129, 236, 208, 4, 0, 0]), ('nomes_padrao_dos_programas', 9118256, [86, 87, 139, 241, 191, 167, 0, 0, 0]), ('historico_da_carreira', 8142400, [129, 236, 72, 2, 0, 0]), ('anos_desde_o_clube', 8154672, [139, 68, 36, 4, 129, 236, 4, 2, 0, 0]), ('ordem_do_historico_por_pessoa', 8154592, [86, 139, 116, 36, 8]), ('lesao_pelo_nome', 8008416, [86, 139, 116, 36, 8]), ('noticia_da_volta_aos_treinos', 6491872, [100, 161, 0, 0, 0, 0]), ('nome_da_categoria_da_lesao', 6491216, [139, 68, 36, 12, 129, 236, 0, 2, 0, 0]), ('cqf10_olheiro_no_jogo', 8084448, [106, 255, 104, 104, 245, 149, 0]), ('cqf10_historico_do_clube_observado', 8114960, [129, 236, 20, 2, 0, 0]), ('cqf11_relatorio_da_selecao', 7800512, [106, 255, 104, 166, 222, 149, 0]), ('cqf11_noticia_da_convocacao', 7790768, [106, 255, 104, 251, 221, 149, 0]), ('cqf11_adversarios_da_selecao', 7787872, [106, 255, 104, 187, 221, 149, 0]), ('cqf12_olheiros_do_dia', 8357712, [106, 255, 104, 190, 15, 150, 0]), ('cqf12_relatorios_dos_olheiros', 8362128, [129, 236, 16, 2, 0, 0]), ('cqf12_noticia_do_relatorio', 8362592, [106, 255, 104, 59, 16, 150, 0])]
BASE = 1598
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
