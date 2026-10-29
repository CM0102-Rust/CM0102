"""Integra a 249a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('decidir_jogo_da_copa', 5357600, [129, 236, 232, 5, 0, 0]), ('decidir_confronto_da_copa', 5370608, [129, 236, 8, 2, 0, 0]), ('ordem_dos_cabecas_de_chave', 5368768, [139, 68, 36, 4, 139, 84, 36, 8]), ('iniciar_linha_da_tabela', 6845488, [129, 236, 8, 2, 0, 0]), ('vetor_dos_clubes_da_liga', 6847008, [129, 236, 0, 2, 0, 0]), ('gravar_posicoes_finais_da_liga', 6859312, [83, 51, 219, 102, 57, 89, 62]), ('mudar_clube_de_divisao', 6828208, [139, 13, 240, 126, 221, 0]), ('embaralhar_ponteiros', 4913984, [86, 87, 15, 191, 116, 36, 16]), ('ordenar_e_embaralhar_os_melhores', 4914128, [129, 236, 0, 2, 0, 0]), ('contar_jogo_da_suspensao', 5598864, [131, 236, 16, 15, 190, 68, 36, 24]), ('soltar_suspensao', 5598464, [129, 236, 12, 2, 0, 0]), ('apagar_historico_de_suspensoes', 5628368, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('sortear_lesao_recorrente', 6481152, [131, 236, 16, 83, 85]), ('recomecar_historico_de_lesoes', 6496144, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('reiniciar_fisico_do_jogador', 6471584, [131, 236, 8, 83, 85]), ('somar_totais_de_transferencia', 4491984, [81, 139, 68, 36, 12]), ('recordes_de_carreira_no_clube', 4492464, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('recorde_de_nota_da_temporada', 4532880, [129, 236, 20, 2, 0, 0]), ('competicao_na_data_do_amistoso', 6039984, [100, 161, 0, 0, 0, 0]), ('amistoso_perto_da_data', 6047136, [106, 255, 104, 0, 160, 149, 0]), ('acrescentar_amistoso_planejado', 6048608, [129, 236, 8, 2, 0, 0])]
BASE = 925
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
