"""Lacuna da 162a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = "| `tabela tatica` | 0x896a90, 0x8968f0 (162ª) | — | na carreira automática as vagas da tabela tática ficam vazias (−1) e não há instruções pendentes: a pessoa da vaga (ficha, nação na seleção) e a limpeza das pendentes só rodam com técnico humano. O teste é o jogo do Lucas comparando (montar a tática, convocar a seleção) |"
EN = "| `tabela tatica` | 0x896a90, 0x8968f0 (batch 162) | — | in the automatic career the tactics table slots stay empty (−1) and there are no pending instructions: the slot person (record, nation in a national team) and the pending cleanup only run with a human manager. The test is Lucas's game in comparison mode (setting tactics, picking the national team) |"
for arq, nova in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    L = open(p, encoding='utf-8').read().split('\n')
    k = [i for i, l in enumerate(L) if l.startswith('| `comissao`')][0]
    L.insert(k + 1, nova)
    open(p, 'w', encoding='utf-8').write('\n'.join(L))
for arq, a, b in (('index.md', '(53 hoje)', '(54 hoje)'), ('index.en.md', '(53 today)', '(54 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
