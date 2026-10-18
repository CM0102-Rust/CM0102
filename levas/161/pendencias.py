"""Lacuna da 161a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = "| `comissao` | 0x866f60 (161ª) | — | a faixa 51..75 de +0x44 da comissão (+5 na nota) não aparece nos dados: a mutação `c_idade` escapa mesmo refeita com +15. Exige banco editado |"
EN = "| `comissao` | 0x866f60 (batch 161) | — | the 51..75 range of the staff +0x44 (+5 on the score) does not appear in the data: mutation `c_idade` escapes even redone with +15. Needs an edited database |"
for arq, nova in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    L = open(p, encoding='utf-8').read().split('\n')
    k = [i for i, l in enumerate(L) if l.startswith('| `formacao`')][0]
    L.insert(k + 1, nova)
    open(p, 'w', encoding='utf-8').write('\n'.join(L))
for arq, a, b in (('index.md', '(52 hoje)', '(53 hoje)'), ('index.en.md', '(52 today)', '(53 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
