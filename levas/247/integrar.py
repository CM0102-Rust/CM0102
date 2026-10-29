"""Integra a 247a leva ao motor.rs (gerado por fila.py: as âncoras são achadas no motor.rs)."""
import os
import re
aqui = os.path.dirname(os.path.abspath(__file__))
p = os.path.join(aqui, '..', '..', 'rust', 'src', 'motor.rs')
s = open(p, encoding='utf-8').read()
ROTINAS = [('recordes_de_sempre', 4531984, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('recordes_da_temporada', 4532432, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('guardar_noticia_de_recorde', 4533424, [129, 236, 0, 2, 0, 0]), ('acrescentar_reserva_de_tecnico', 8028576, [129, 236, 0, 2, 0, 0]), ('acrescentar_aposentadoria', 8029456, [129, 236, 0, 2, 0, 0]), ('guardar_vaga_para_regen', 8030160, [129, 236, 0, 2, 0, 0]), ('acionar_clausulas_de_rebaixamento', 5034032, [131, 236, 12, 83, 139, 92, 36, 20]), ('tirar_oferta_de_contrato', 5032560, [139, 68, 36, 8, 83]), ('esta_suspenso', 5624432, [83, 85, 86, 139, 116, 36, 16]), ('escopo_das_regras_da_partida', 9252768, [139, 21, 132, 246, 156, 0]), ('codigo_de_inelegibilidade', 9237984, [131, 236, 8, 83, 85]), ('vaga_do_amistoso', 6055744, [106, 255, 104, 248, 160, 149, 0]), ('noticia_para_os_tecnicos_do_continente', 7890496, [129, 236, 4, 2, 0, 0]), ('noticia_para_a_regiao_da_competicao', 7891040, [129, 236, 0, 2, 0, 0]), ('noticia_importante_para_a_pessoa', 7884464, [129, 236, 0, 2, 0, 0]), ('texto_do_dado_da_noticia', 7887392, [138, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('noticia_de_apuro_financeiro', 7646816, [100, 161, 0, 0, 0, 0]), ('olheiros_cobrem_a_nacao', 8360336, [129, 236, 0, 2, 0, 0]), ('fluencia_no_idioma', 8153136, [131, 236, 8, 83, 86]), ('recomecar_historico_da_pessoa', 8846208, [139, 68, 36, 4, 129, 236, 0, 2, 0, 0]), ('posicao_na_formacao', 5974944, [138, 68, 36, 4, 129, 236, 28, 2, 0, 0])]
BASE = 883
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
