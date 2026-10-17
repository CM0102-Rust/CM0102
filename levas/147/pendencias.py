"""Lacunas da 147a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("chute bloqueado", "0x702050 (147ª)", "—", "a interceptação no caminho do gol devolve 2 (alguém desvia): 0x1fd3/0x1fd4, o chute muda em sorteio(1000) - sorteio(2000) e, fora do alvo, sai para escanteio; na carreira automática até 2002-06-01 não ocorreu")]
EN = [("chute bloqueado", "0x702050 (batch 147)", "—", "the interception on the way to goal returns 2 (someone deflects): 0x1fd3/0x1fd4, the shot changes by draw(1000) - draw(2000) and, off target, goes out for a corner; it did not happen in the automatic career up to 2002-06-01")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `laco cobrador da bola parada`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(33 hoje)', '(34 hoje)'), ('index.en.md', '(33 today)', '(34 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
