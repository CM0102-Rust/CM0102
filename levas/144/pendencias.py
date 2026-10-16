"""Lacunas da 144a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("conducao goleiro sem marca", "0x7109f0 (144ª)", "—", "quem guarda a posição 1 do adversário sem a marca de goleiro (tabela &1 = 0) dá +5000 à nota do chute; na carreira automática o guarda da posição 1 é sempre goleiro")]
EN = [("conducao goleiro sem marca", "0x7109f0 (batch 144)", "—", "the opponent's position-1 guard without the goalkeeper mark (table &1 = 0) gives +5000 to the shot score; in the automatic career the position-1 guard is always a goalkeeper")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `movimento sem mais adiantado`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(30 hoje)', '(31 hoje)'), ('index.en.md', '(30 today)', '(31 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
