"""Lacunas da 157a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("narracao no painel", "0x6d7b00 (157ª)", "—", "o desenho da narração no painel (cores, quebra e corte da linha, sons do lance e da torcida) só roda com a tela da partida (contador \"narracao no painel\" em zero); o teste é assistir partidas no `JOGAR.bat`")]
EN = [("narracao no painel", "0x6d7b00 (batch 157)", "—", "drawing the commentary on the panel (colours, line wrap and cut, play and crowd sounds) only runs with the match screen (counter \"narracao no painel\" at zero); the test is watching matches in `JOGAR.bat`")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `texto e partida em rede`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(49 hoje)', '(50 hoje)'), ('index.en.md', '(49 today)', '(50 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
