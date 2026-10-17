"""Lacunas da 156a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("texto e partida em rede", "0x6d8d10, 0x6b5260 (156ª)", "—", "a montagem da linha de narração e a confirmação da partida em rede só rodam com a tela da partida (contadores \"texto do evento\" e \"partida em rede confirmada\" em zero). O texto tem comparação de verdade: o teste é assistir partidas no `JOGAR_COMPARANDO`")]
EN = [("texto e partida em rede", "0x6d8d10, 0x6b5260 (batch 156)", "—", "building the commentary line and confirming the network match only run with the match screen (counters \"texto do evento\" and \"partida em rede confirmada\" at zero). The text has a real comparison: the test is watching matches in `JOGAR_COMPARANDO`")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `eventos cfg sons e opcoes`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(48 hoje)', '(49 hoje)'), ('index.en.md', '(48 today)', '(49 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
