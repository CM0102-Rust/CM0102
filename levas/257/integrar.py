"""Integra a 257a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('linha_da_nacao', 6722368, [139, 84, 36, 4, 50, 192]), ('linha_da_nacao_do_clube', 6722400, [129, 236, 0, 2, 0, 0]), ('ano_da_temporada_do_clube', 6723920, [81, 86, 139, 116, 36, 12]), ('clube_na_temporada', 6724560, [106, 255, 104, 88, 198, 149, 0]), ('continente_principal', 6723008, [161, 8, 45, 174, 0]), ('nacao_principal', 6723072, [161, 8, 45, 174, 0]), ('estatistica_da_categoria', 8134192, [129, 236, 12, 2, 0, 0]), ('ordem_dos_idiomas', 8154432, [139, 84, 36, 4, 86]), ('ordem_da_historia_por_clube', 8154512, [86, 139, 116, 36, 8]), ('clausula_de_rescisao_ativa', 8813392, [138, 81, 28, 176, 2]), ('periodo_protegido', 8815120, [106, 255, 104, 8, 43, 150, 0]), ('tipo_de_bonus_permitido', 8813216, [139, 68, 36, 4, 139, 64, 97]), ('adiar_pelo_clima', 9582128, [83, 86, 139, 116, 36, 12]), ('financas_do_clube', 5899664, [129, 236, 4, 2, 0, 0]), ('ordem_das_reservas_do_estadio', 8774528, [83, 139, 92, 36, 8]), ('clima_da_partida', 9580560, [81, 83, 85, 139, 233]), ('temperatura_do_jogo', 9581248, [15, 190, 76, 36, 4]), ('gramado_do_jogo', 9582960, [139, 76, 36, 8, 83]), ('faixa_de_riqueza', 5867184, [83, 86, 139, 241, 139, 21, 180, 35, 174, 0]), ('ordem_por_habilidade', 6119968, [139, 68, 36, 4, 139, 76, 36, 8]), ('ordem_pela_capacidade', 8774480, [139, 68, 36, 4, 139, 72, 12]), ('programas_padrao', 9125184, [129, 236, 12, 12, 0, 0]), ('tipo_do_programa', 9129552, [15, 190, 68, 36, 4]), ('programa_do_humano', 9124144, [139, 84, 36, 4, 161, 100, 35, 174, 0]), ('fichas_de_treino_iniciais', 9124944, [161, 108, 35, 174, 0]), ('guardar_jogador_no_banco', 8029008, [129, 236, 0, 2, 0, 0]), ('tem_reserva_de_tecnico', 8023584, [139, 65, 32, 133, 192]), ('suspensao_do_vermelho', 8293904, [131, 236, 8, 83, 85]), ('nova_temporada_do_orgao', 8290000, [131, 236, 8, 86, 139, 241]), ('compactar_disciplina_do_orgao', 8294480, [129, 236, 4, 2, 0, 0]), ('coordenada_da_casa', 5977440, [129, 236, 128, 2, 0, 0]), ('posicao_repetida_na_formacao', 5977184, [138, 84, 36, 4, 129, 236, 0, 2, 0, 0]), ('tipo_de_marcacao', 5986256, [138, 68, 36, 8, 129, 236, 0, 2, 0, 0]), ('sortear_ordem_dos_clubes', 6842992, [129, 236, 104, 2, 0, 0]), ('clubes_do_lado_do_jogo', 5379984, [129, 236, 0, 2, 0, 0]), ('clubes_possiveis_do_jogo', 5380528, [139, 84, 36, 12, 129, 236, 0, 2, 0, 0]), ('nota_alvo_do_clube', 8112784, [131, 236, 8, 83, 139, 92, 36, 16]), ('jogo_do_historico', 8116272, [138, 84, 36, 4, 138, 129, 161, 1, 0, 0]), ('gols_do_historico', 8116336, [138, 129, 161, 1, 0, 0])]
BASE = 1194
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
