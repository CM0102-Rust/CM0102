"""Integra a 254a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('restricoes_da_partida', 9238192, [81, 83, 85, 86, 139, 116, 36, 24]), ('limite_do_banco_do_dia', 9238656, [131, 236, 12, 85, 139, 108, 36, 24]), ('precisa_de_permissao_de_trabalho', 9205088, [139, 68, 36, 8, 129, 236, 16, 2, 0, 0]), ('clube_pode_chegar_ao_jogador', 9234832, [106, 255, 104, 120, 63, 150, 0]), ('termos_ao_jogador_da_troca', 9183328, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('fila_livre_do_clube', 9181056, [129, 236, 0, 2, 0, 0]), ('transferencia_futura_do_jogador', 9226768, [106, 255, 104, 152, 62, 150, 0]), ('tirar_transferencia_futura', 9224048, [83, 86, 87, 139, 124, 36, 16]), ('acrescentar_transferencia_futura', 9222560, [129, 236, 8, 2, 0, 0]), ('ofertas_abertas_pelos_jogadores_do_clube', 9199872, [81, 85, 86, 139, 241]), ('desfazer_ofertas_pelo_jogador', 9193424, [106, 255, 104, 136, 60, 150, 0]), ('guardar_percentual_da_proxima_venda', 9218800, [129, 236, 0, 2, 0, 0]), ('iniciar_diretorias_dos_clubes', 6877488, [85, 139, 236, 131, 228, 248]), ('apelacoes_de_suspensao', 6995696, [129, 236, 8, 2, 0, 0]), ('pode_ver_a_partida', 6285056, [100, 161, 0, 0, 0, 0]), ('aposentadoria_no_inicio', 8026560, [131, 236, 12, 87, 139, 124, 36, 20]), ('aposentadoria_por_idade', 8024848, [131, 236, 20, 139, 13, 80, 16, 174, 0]), ('lista_inicial_de_aposentadorias', 8018160, [106, 255, 104, 136, 241, 149, 0]), ('novas_aposentadorias', 8017808, [106, 255, 104, 88, 241, 149, 0]), ('sair_dos_cargos', 8040112, [83, 86, 139, 116, 36, 12]), ('registrar_a_liberacao', 8837120, [106, 255, 104, 123, 45, 150, 0]), ('estado_da_lesao', 6472704, [129, 236, 8, 2, 0, 0]), ('marcar_decisao_da_oferta', 5034544, [106, 255, 104, 80, 109, 149, 0]), ('candidatos_a_tecnico_do_ano', 8846000, [106, 255, 104, 8, 46, 150, 0]), ('projetar_posicoes_da_liga', 6859840, [131, 236, 12, 83, 85, 86]), ('ordem_da_projecao_pela_reputacao', 6860720, [139, 68, 36, 4, 139, 8]), ('ordem_da_projecao_pelos_pontos', 6860752, [139, 68, 36, 4, 139, 84, 36, 8]), ('achar_jogos_do_calendario', 5950432, [129, 236, 36, 2, 0, 0]), ('jogos_da_competicao_no_dia', 5946768, [129, 236, 8, 2, 0, 0]), ('competicao_tem_jogo_na_data', 5957328, [129, 236, 4, 2, 0, 0]), ('clube_livre_na_data', 6056816, [106, 255, 104, 49, 161, 149, 0]), ('copa_tem_o_clube', 5384464, [86, 102, 139, 113, 62]), ('nacao_parceira_do_clube', 5961136, [139, 68, 36, 4, 83])]
BASE = 1083
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
