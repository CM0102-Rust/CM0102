"""Lacunas da 155a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("eventos cfg sons e opcoes", "0x6d9e50 (155ª)", "—", "dos dados do `events_*.cfg`, a carreira automática só enxerga a opção 2 (lance seguinte) e o peso relativo dos sons; pesos, tipos e nomes dos sons e as opções 0, 1 e 4 servem ao áudio, à tela da partida e ao passo em rede (6 mutações escapam, inclusive a refeita na opção 0). No jogo do Lucas (JOGAR.bat, 17/10/2026) a narração e os sons funcionaram")]
EN = [("eventos cfg sons e opcoes", "0x6d9e50 (batch 155)", "—", "of the `events_*.cfg` data, the automatic career only sees option 2 (follow-on play) and the sounds' relative weight; the sounds' weights, types and names and options 0, 1 and 4 serve the audio, the match screen and the network step (6 mutations escape, including the one redone on option 0). In Lucas's game (JOGAR.bat, 2026-10-17) the commentary and sounds worked")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `passo em rede`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(47 hoje)', '(48 hoje)'), ('index.en.md', '(47 today)', '(48 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
