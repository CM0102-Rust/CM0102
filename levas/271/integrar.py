"""Integra a 271a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('assinar_contrato', 5044000, [129, 236, 24, 2, 0, 0]), ('refazer_como_jovem', 8043200, [131, 236, 8, 86, 139, 116, 36, 16]), ('tirar_da_disputa', 5677200, [83, 139, 92, 36, 8]), ('ler_formacao_do_disco', 5987600, [129, 236, 8, 4, 0, 0]), ('trocar_bytes_das_palavras', 4453072, [139, 68, 36, 8, 139, 200]), ('trocar_bytes_dos_dwords', 4453120, [139, 68, 36, 8, 139, 200]), ('montar_os_arbitros', 7978848, [129, 236, 12, 2, 0, 0]), ('ordem_das_vagas_de_competicao', 7984144, [139, 68, 36, 4, 139, 76, 36, 8]), ('cqg10_amistoso_cancelado', 6055280, [100, 161, 0, 0, 0, 0]), ('cqg10_jogo_da_selecao_cancelado', 6076896, [106, 255, 104, 128, 163, 149, 0]), ('cqg10_torneio_cancelado', 6062688, [106, 255, 104, 119, 162, 149, 0]), ('ordem_dos_arbitros_decrescente', 7984352, [139, 84, 36, 4, 86]), ('ordem_dos_arbitros_crescente', 7984256, [139, 84, 36, 4, 86]), ('arquivar_estatisticas', 8151008, [129, 236, 60, 2, 0, 0]), ('pessoa_pelo_nome', 8016112, [83, 139, 92, 36, 20]), ('texto_entre_aspas', 8016016, [85, 139, 108, 36, 12]), ('tipo_da_linha', 8008000, [86, 139, 116, 36, 8])]
BASE = 1629
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
