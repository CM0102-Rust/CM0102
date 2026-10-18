"""Lacuna da 170a leva em docs/PENDENCIAS (pt/en), na lista da carreira ampla e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = "| `ordem das partidas do dia` | 0x6b6510 (170ª) | — | na liga inglesa as partidas do dia que empatam na competição, +0x46 e +0x36 nunca divergem em sinal entre +0x38 e +0x34: a mutação `o_campos` (troca a ordem dos dois) escapa até comparando. Teste: carreira ampla (várias ligas) |"
EN = "| `day match order` | 0x6b6510 (batch 170) | — | in the English league the day's matches tied on competition, +0x46 and +0x36 never disagree in sign between +0x38 and +0x34: mutation `o_campos` (swaps their order) escapes even comparing. Test: broad career (several leagues) |"
for arq, nova in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    L = open(p, encoding='utf-8').read().split('\n')
    if nova in L:
        continue
    k = max(i for i, l in enumerate(L) if 'batch 169)' in l or '(169ª)' in l)
    L.insert(k + 1, nova)
    open(p, 'w', encoding='utf-8').write('\n'.join(L))
p = os.path.join(R, 'levas', 'carreira_ampla', 'pendentes.txt')
s = open(p, encoding='utf-8').read().rstrip('\n')
open(p, 'w', encoding='utf-8').write(s + '\n- leva170.py: o_campos (ordem das partidas do dia, desempate +0x38/+0x34)\n')
for arq, a, b in (('index.md', '(61 hoje)', '(62 hoje)'), ('index.en.md', '(61 today)', '(62 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
