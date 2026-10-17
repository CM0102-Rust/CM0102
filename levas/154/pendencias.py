"""Lacunas da 154a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("eventos gravados", "0x6d98b0, 0x6d9c40 (154ª)", "—", "a gravação roda na carreira (contador acima de 2048), mas os arquivos \"events1/2.tmp\" não entram nas impressões, e a leitura de volta (rever os lances) só roda com a tela: o teste é no jogo, revendo os lances de uma partida assistida"),
      ("passo em rede", "0x6b55c0 (154ª)", "—", "a partida do humano em rede (pelo relógio e pelo placar dos humanos) não roda na carreira automática; o contador \"passo em rede\" fica em zero")]
EN = [("eventos gravados", "0x6d98b0, 0x6d9c40 (batch 154)", "—", "recording runs in the career (counter above 2048), but the \"events1/2.tmp\" files are not part of the fingerprints, and reading them back (replaying the plays) only runs with the screen: the test is in the game, replaying the plays of a watched match"),
      ("passo em rede", "0x6b55c0 (batch 154)", "—", "the human's network match (by the clock and the humans' scoreboard) does not run in the automatic career; the counter \"passo em rede\" stays at zero")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `sons dos eventos`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(45 hoje)', '(47 hoje)'), ('index.en.md', '(45 today)', '(47 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
