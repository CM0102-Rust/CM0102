"""Lacunas da 149a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("escalacao ficha do tecnico", "0x6db500 (149ª)", "—", "a tática copia três campos da ficha do técnico (+0xa, +8, +0xc) para +0, +2, +4; na carreira automática o segundo (+2) não pesa (a mutação que o troca não muda nada)")]
EN = [("escalacao ficha do tecnico", "0x6db500 (batch 149)", "—", "the tactics copy three fields of the manager's record (+0xa, +8, +0xc) to +0, +2, +4; in the automatic career the second (+2) does not matter (the mutation that swaps it changes nothing)")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `prejogo codigo da insatisfacao`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(36 hoje)', '(37 hoje)'), ('index.en.md', '(36 today)', '(37 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
