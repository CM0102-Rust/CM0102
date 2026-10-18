"""Lacuna da 164a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = "| `calendario` | 0x68a030 (164ª) | — | a carreira automática só joga ligas inglesas: o rodízio com a marca 0x80 e a volta no fim do rodízio não rodam (`g_meio`, `g_volta` escapam, mesmo refeitas). Teste: carreira automática com outras ligas (`simular.py --ligas`) |"
EN = "| `calendario` | 0x68a030 (batch 164) | — | the automatic career only plays English leagues: the rotation with flag 0x80 and wrapping at the end of the rotation do not run (`g_meio`, `g_volta` escape, even redone). Test: automatic career with other leagues (`simular.py --ligas`) |"
for arq, nova in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    L = open(p, encoding='utf-8').read().split('\n')
    k = [i for i, l in enumerate(L) if l.startswith('| `tabela tatica`')][0]
    L.insert(k + 1, nova)
    open(p, 'w', encoding='utf-8').write('\n'.join(L))
for arq, a, b in (('index.md', '(54 hoje)', '(55 hoje)'), ('index.en.md', '(54 today)', '(55 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
