"""Lacuna da 167a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = "| `partida abandonada` | 0x6d5930 (167ª) | — | nenhuma partida é abandonada na carreira automática (minuto do abandono em −1): a mutação `a_onze` escapa. Teste: uma partida abandonada (clima) no jogo do Lucas comparando |"
EN = "| `partida abandonada` | 0x6d5930 (batch 167) | — | no match is abandoned in the automatic career (abandonment minute at −1): mutation `a_onze` escapes. Test: an abandoned match (weather) in Lucas's game in comparison mode |"
for arq, nova in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    L = open(p, encoding='utf-8').read().split('\n')
    k = [i for i, l in enumerate(L) if l.startswith('| `categoria do contrato`')][0]
    L.insert(k + 1, nova)
    open(p, 'w', encoding='utf-8').write('\n'.join(L))
for arq, a, b in (('index.md', '(58 hoje)', '(59 hoje)'), ('index.en.md', '(58 today)', '(59 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
