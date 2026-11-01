"""Integra a 264a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('renda_da_partida', 5875152, [129, 236, 28, 2, 0, 0]), ('renda_do_estadio', 5877568, [129, 236, 8, 2, 0, 0]), ('pagar_multa_da_liga', 5871696, [139, 68, 36, 4, 139, 145, 40, 1, 0, 0]), ('impulso_das_financas', 5890128, [81, 83, 85, 86, 139, 116, 36, 20]), ('troca_do_presidente', 5892416, [85, 139, 236, 131, 228, 248]), ('pagar_outros_custos', 5872096, [139, 68, 36, 4, 139, 145, 32, 1, 0, 0]), ('tabela_das_datas_internacionais', 6071536, [129, 236, 4, 2, 0, 0]), ('resolver_datas_internacionais', 6079760, [81, 102, 161, 146, 44, 174, 0]), ('torneio_do_clube_humano', 6055984, [15, 191, 5, 122, 124, 183, 0]), ('amistosos_dos_clubes', 6047600, [129, 236, 80, 2, 0, 0]), ('acrescentar_torneio', 6057472, [131, 236, 8, 83, 86, 87]), ('convite_aberto_do_clube_humano', 6056112, [86, 139, 241, 87, 139, 134, 200, 52, 0, 0]), ('lesao_nova', 6476368, [129, 236, 32, 2, 0, 0]), ('noticia_da_lesao', 6479008, [100, 161, 0, 0, 0, 0]), ('ausente_sem_licenca', 4296544, [86, 139, 116, 36, 8]), ('parte_dos_clubes_anteriores', 9326368, [106, 255, 104, 171, 72, 150, 0]), ('percentual_da_proxima_venda_pago', 9220336, [131, 236, 20, 83, 85]), ('noticia_do_percentual_da_venda', 9286560, [106, 255, 104, 219, 68, 150, 0]), ('permissao_concedida', 9206096, [131, 236, 20, 83, 85]), ('estatuto_pelo_motivo', 9197568, [139, 13, 80, 16, 174, 0]), ('vaga_da_negociacao', 9200128, [139, 68, 36, 4, 139, 81, 16]), ('reacao_do_elenco_a_historia', 7689024, [129, 236, 244, 0, 0, 0]), ('aviso_ao_rival_de_posicao', 7700496, [131, 236, 16, 83, 87]), ('rotulo_da_resposta_da_midia', 7557472, [139, 68, 36, 12, 133, 192]), ('cumprir_suspensoes_internacionais', 5599520, [106, 255, 104, 200, 124, 149, 0]), ('recurso_da_suspensao', 5599232, [83, 85, 139, 92, 36, 12]), ('texto_da_vaga_da_suspensao', 5624768, [139, 84, 36, 4, 86]), ('tirar_da_lista', 6869488, [139, 193, 83, 85, 86]), ('esvaziar_lista', 6869120, [86, 139, 241, 139, 70, 2]), ('item_do_cursor', 6869728, [139, 65, 10, 133, 192]), ('copa_sortear_pela_semente', 5362384, [129, 236, 44, 2, 0, 0]), ('copa_sortear_separando_nacoes', 5364208, [129, 236, 48, 2, 0, 0]), ('copa_comeco_da_temporada', 5343104, [198, 129, 177, 0, 0, 0, 0]), ('nacao_chave_do_clube', 4775648, [83, 86, 139, 116, 36, 12]), ('nacao_chave_do_estadio', 4775744, [139, 68, 36, 4, 83]), ('entrada_da_nacao_chave_ativa', 4775120, [138, 84, 36, 4, 129, 236, 0, 2, 0, 0]), ('conflito_de_jogos_no_mesmo_dia', 5956480, [106, 255, 104, 22, 152, 149, 0]), ('primeiro_ano_com_jogos', 5959232, [139, 68, 36, 4, 139, 21, 92, 42, 174, 0]), ('ultimo_ano_com_jogos', 5959264, [139, 68, 36, 4, 139, 21, 92, 42, 174, 0])]
BASE = 1458
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
