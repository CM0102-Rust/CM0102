"""Integra a 260a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('faltas_do_dia', 4296912, [131, 236, 28, 83, 85]), ('teve_falta_ao_treino', 4296688, [86, 139, 116, 36, 8]), ('teve_incidente_de_forma', 4296816, [83, 139, 92, 36, 8]), ('marcar_falta_ao_treino', 4301664, [106, 255, 104, 232, 62, 149, 0]), ('fim_da_ausencia', 4298512, [86, 87, 139, 124, 36, 12]), ('marcas_do_contrato', 8811872, [129, 236, 0, 2, 0, 0]), ('recusar_oferta_de_transferencia', 9172608, [81, 83, 139, 92, 36, 12]), ('retirar_oferta_de_transferencia', 9174192, [81, 83, 85, 86, 87]), ('noticia_oferta_recusada', 9261424, [106, 255, 104, 43, 67, 150, 0]), ('desassossegar_pelo_lance', 9179792, [131, 236, 32, 83, 139, 92, 36, 40]), ('noticia_oferta_desfeita', 9299856, [100, 161, 0, 0, 0, 0]), ('noticia_premio_por_jogos', 9287840, [100, 161, 0, 0, 0, 0]), ('efetivar_transferencia', 9212816, [81, 83, 85, 86, 139, 116, 36, 20]), ('noticia_transferencia_concluida', 9289456, [106, 255, 104, 91, 69, 150, 0]), ('noticia_transferencia_acertada', 9307600, [106, 255, 104, 155, 70, 150, 0]), ('emprestimo_da_configuracao', 9171968, [106, 255, 104, 40, 59, 150, 0]), ('noticia_fim_do_contrato', 5088656, [100, 161, 0, 0, 0, 0]), ('noticia_pedido_de_adiamento', 9296528, [100, 161, 0, 0, 0, 0]), ('tatica_do_clube', 9004640, [129, 236, 0, 2, 0, 0]), ('escolher_vaga_de_tatica', 6274592, [139, 68, 36, 12, 129, 236, 0, 2, 0, 0]), ('tatica_ativa_do_humano', 6274304, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('tirar_da_escalacao', 9006176, [129, 236, 12, 2, 0, 0]), ('trocar_vagas_da_escalacao', 9008096, [129, 236, 0, 2, 0, 0]), ('tem_copia_da_escalacao', 9007696, [138, 129, 240, 5, 0, 0]), ('noticia_da_pressao', 6950512, [100, 161, 0, 0, 0, 0]), ('noticia_da_saida_do_tecnico', 6953728, [100, 161, 0, 0, 0, 0]), ('noticia_do_tecnico_novo', 6958592, [100, 161, 0, 0, 0, 0]), ('pagar_premios_da_liga', 6827536, [129, 236, 4, 2, 0, 0]), ('registros_pela_situacao_na_liga', 6842688, [138, 68, 36, 8, 60, 5]), ('anunciar_situacao_na_liga', 6841968, [106, 255, 104, 139, 202, 149, 0]), ('registrar_partida_nas_estatisticas', 8129904, [85, 139, 236, 131, 228, 248]), ('recordes_de_posicao_na_liga', 4486192, [129, 236, 32, 2, 0, 0]), ('noticias_de_recordes', 4492848, [100, 161, 0, 0, 0, 0])]
BASE = 1308
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
# cpw_7 (0x867560 na tabela principal): no modo comparação fica a porta do pós-jogo (PROLOGOS_POS, a fita), como a
# 0x6f5c30 e a 0x70dfd0; e o diagnóstico da marca (CM_DIAG_PESSOA, prova do "Deixar infeliz") deixa de desviá-la
a = 'if comparando && (PROLOGOS[i].endereco == 0x6f5c30 || PROLOGOS[i].endereco == 0x70dfd0) {'
assert s.count(a) == 1
s = s.replace(a, 'if comparando && (PROLOGOS[i].endereco == 0x6f5c30 || PROLOGOS[i].endereco == 0x70dfd0 || PROLOGOS[i].endereco == 0x867560) {')
a = re.search(r'\n        DIAG_MARCA = desvios::desviar\("diag marca 0x867560".*\n', s)
assert a
s = s[:a.start()] + '\n' + s[a.end():]
open(p, 'w', encoding='utf-8').write(s)
print('ok')
