"""Lacunas da 152a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("dia linhas do humano", "0x6b4360 (152ª)", "—", "no fechamento do dia, as táticas guardadas dos humanos (0x5fbe20, 0x5fbd00, 0x5b5960) só são refeitas com técnico humano; o contador \"linhas do humano refeitas\" fica em zero na carreira automática (a mutação que tira a chamada escapa; o contador de terminadas, +0x25, é a lacuna já anotada na 151ª)")]
EN = [("dia linhas do humano", "0x6b4360 (batch 152)", "—", "at the end of the day, the humans' stored tactics (0x5fbe20, 0x5fbd00, 0x5b5960) are only redone with a human manager; the counter \"linhas do humano refeitas\" stays at zero in the automatic career (the mutation that removes the call escapes; the finished counter, +0x25, is the gap already noted in batch 151)")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `dia historia`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(41 hoje)', '(42 hoje)'), ('index.en.md', '(41 today)', '(42 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
