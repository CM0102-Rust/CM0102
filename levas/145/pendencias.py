"""Lacunas da 145a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("laco ficha sem motor", "0x6b98c0 (145ª)", "—", "ficha em campo (+0x19 >= 0) sem motor (+0x19e = 0) é tirada (-5) no início do laço; na carreira automática não acontece"),
      ("laco cobrador da bola parada", "0x6b98c0", "—", "bola parada sem dono (ou dono expulso) escolhe o cobrador (0x6ceb90) e o vira para o gol; o reinício já deixa um dono na carreira automática")]
EN = [("laco ficha sem motor", "0x6b98c0 (batch 145)", "—", "an on-field record (+0x19 >= 0) with no engine (+0x19e = 0) is removed (-5) at the start of the loop; does not happen in the automatic career"),
      ("laco cobrador da bola parada", "0x6b98c0", "—", "a set piece with no ball owner (or a sent-off one) picks the taker (0x6ceb90) and turns him to goal; the restart already leaves an owner in the automatic career")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `conducao goleiro sem marca`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(31 hoje)', '(33 hoje)'), ('index.en.md', '(31 today)', '(33 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
