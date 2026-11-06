"""Integra a 276a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('cql10_copa_da_liga_tabela', 5686640, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cql10_copa_da_inglaterra_tabela', 5698336, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cql10_supercopa_tabela', 5690944, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cql11_copa_da_franca_tabela', 6005744, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cql11_copa_da_liga_francesa_tabela', 6016064, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cql11_copa_do_rei_tabela', 8704544, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cql12_supercopa_da_franca_tabela', 6026704, [138, 68, 36, 4, 129, 236, 12, 2, 0, 0]), ('cql12_supercopa_da_espanha_tabela', 8751456, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cql12_supercopa_do_japao_tabela', 6709280, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqm10_copa_da_liga_inscritos', 5688512, [129, 236, 0, 2, 0, 0]), ('cqm10_trofeu_inscritos', 5682112, [129, 236, 0, 2, 0, 0]), ('cqm10_supercopa_clubes', 5691264, [129, 236, 0, 2, 0, 0]), ('cqm11_copa_da_franca_inscritos', 6007376, [129, 236, 4, 2, 0, 0]), ('cqm11_copa_da_liga_inscritos', 6017376, [129, 236, 4, 2, 0, 0]), ('cqm11_supercopa_clubes', 6027120, [129, 236, 0, 2, 0, 0]), ('cqm12_supercopa_da_espanha_clubes', 8751776, [129, 236, 0, 2, 0, 0]), ('cqm12_supercopa_do_japao_clubes', 6709600, [129, 236, 0, 2, 0, 0]), ('cqm12_jogo_das_estrelas_tabela', 9479168, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqn10_copa_da_liga_dia', 5688048, [129, 236, 0, 2, 0, 0]), ('cqn10_copa_da_inglaterra_dia', 5702992, [100, 161, 0, 0, 0, 0]), ('cqn10_jogo_das_estrelas_dia', 9480448, [106, 255, 104, 168, 84, 150, 0]), ('cqn11_copa_da_liga_japonesa_tabela', 6706752, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqn11_copa_do_imperador_tabela', 6692848, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqn11_copa_dos_eua_tabela', 9482608, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('cqn12_copa_da_liga_inscritos', 6707712, [129, 236, 4, 2, 0, 0]), ('cqn12_copa_dos_eua_inscritos', 9483568, [129, 236, 152, 3, 0, 0]), ('cqn12_copa_do_imperador_inscritos', 6694128, [129, 236, 200, 4, 0, 0]), ('numero_de_camisa', 8759136, [83, 139, 92, 36, 8]), ('clube_usa_numeros', 8760784, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('gravar_numero_de_camisa', 8759376, [129, 236, 0, 2, 0, 0]), ('dar_numero_de_camisa', 8758384, [129, 236, 8, 1, 0, 0]), ('numerar_elenco', 8757216, [129, 236, 144, 2, 0, 0]), ('numerar_no_comeco', 8756672, [85, 139, 233, 87, 51, 255]), ('numerar_liga', 8756768, [129, 236, 216, 0, 0, 0]), ('inscrever_na_europa', 8760032, [129, 236, 156, 2, 0, 0]), ('regras_holanda', 6264464, [106, 255, 104, 59, 175, 149, 0]), ('regras_croacia', 5342464, [106, 255, 104, 187, 117, 149, 0]), ('regras_irlanda', 6537744, [106, 255, 104, 171, 183, 149, 0])]
BASE = 1772
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
