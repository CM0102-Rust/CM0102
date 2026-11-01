"""Integra a 258a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('passar_a_oferta_aos_clubes', 9187152, [131, 236, 12, 83, 85]), ('noticia_da_oferta_aceita', 9270992, [100, 161, 0, 0, 0, 0]), ('noticia_do_emprestimo_oferecido', 9312128, [100, 161, 0, 0, 0, 0]), ('recusa_dos_termos', 9185472, [129, 236, 20, 2, 0, 0]), ('noticia_da_confianca_da_diretoria', 6966336, [106, 255, 104, 107, 206, 149, 0]), ('noticia_das_expectativas_da_diretoria', 6965792, [106, 255, 104, 75, 206, 149, 0]), ('ofertas_em_espera_na_janela', 9195024, [139, 68, 36, 4, 131, 236, 12]), ('noticia_das_expectativas_da_federacao', 6964384, [106, 255, 104, 43, 206, 149, 0]), ('noticia_do_protesto_da_torcida', 6970640, [106, 255, 104, 235, 206, 149, 0]), ('guardar_a_negociacao', 9169360, [100, 161, 0, 0, 0, 0]), ('noticia_do_boato_do_tecnico', 6946128, [100, 161, 0, 0, 0, 0]), ('noticia_da_recusa_em_se_mudar', 9274128, [100, 161, 0, 0, 0, 0]), ('receita_do_merchandising', 5879776, [81, 83, 86, 139, 116, 36, 16]), ('ingressos_da_temporada', 5872160, [131, 236, 20, 139, 21, 180, 35, 174, 0]), ('melhorar_o_centro_de_treino', 5879344, [83, 86, 87, 139, 124, 36, 16]), ('eventos_financeiros_da_semana', 5893488, [129, 236, 8, 2, 0, 0]), ('diretoria_paga_a_divida', 5889344, [85, 139, 236, 131, 228, 248]), ('injecao_de_dinheiro', 5891488, [131, 236, 12, 83, 85]), ('teto_salarial_do_pessoal', 5864880, [131, 236, 24, 139, 65, 8]), ('pagar_os_bonus', 5871968, [139, 68, 36, 4, 139, 145, 56, 1, 0, 0]), ('ajustar_o_teto_salarial', 5866304, [15, 190, 68, 36, 4]), ('efeitos_fisicos_da_partida', 6471808, [131, 236, 32, 83, 85]), ('batida_do_jogador', 6471520, [86, 139, 241, 87, 139, 70, 40]), ('jogador_de_ferias', 6493520, [139, 68, 36, 4, 139, 13, 40, 61, 182, 0]), ('contratar_o_regen', 8023824, [100, 161, 0, 0, 0, 0]), ('aposentar_pela_lesao', 8027936, [106, 255, 104, 240, 241, 149, 0]), ('noticia_da_aposentadoria_anunciada', 8037184, [100, 161, 0, 0, 0, 0]), ('fim_da_lesao', 6481664, [129, 236, 0, 2, 0, 0]), ('noticia_do_pedido_de_desculpas', 7519216, [100, 161, 0, 0, 0, 0]), ('esvaziar_caixa_de_entrada', 7894576, [129, 236, 0, 2, 0, 0]), ('tirar_convocado_machucado', 7822544, [129, 236, 16, 2, 0, 0]), ('noticia_da_saida_do_convocado', 7796688, [100, 161, 0, 0, 0, 0]), ('noticia_do_pedido_de_substituto', 7799280, [100, 161, 0, 0, 0, 0]), ('pode_mudar_a_convocacao', 7789312, [106, 255, 104, 219, 221, 149, 0]), ('esta_na_convocacao', 7789184, [86, 139, 116, 36, 8]), ('marcar_adversario_do_amistoso', 6051792, [139, 84, 36, 4, 83]), ('registrar_premio_do_mes', 8833216, [100, 161, 0, 0, 0, 0]), ('faixa_de_reputacao_do_amistoso', 6050512, [139, 68, 36, 4, 102, 139, 128, 128, 0, 0, 0]), ('folga_do_amistoso', 6059216, [15, 190, 68, 36, 4])]
BASE = 1233
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
