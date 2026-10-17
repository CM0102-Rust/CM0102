"""Lacunas da 153a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("dia primeira volta", "0x6b3c10 (153ª)", "—", "a 1ª volta do dia (+0x33 = 1) só começa partidas cuja competição (+0x63 da partida) tem +0x10, que a montagem cria zerado; na carreira automática a 1ª volta não escolhe nada e tudo cai na 2ª, então começar na 2ª ou apontar outra competição não muda a carreira"),
      ("dia partidas ja marcadas", "0x6b3c10 (153ª)", "—", "a montagem pula as partidas da lista já marcadas -2; na carreira automática nenhuma chega marcada assim (a mutação que as deixa entrar escapa, mesmo a forte)"),
      ("sons dos eventos", "0x6d8c70 (153ª)", "—", "a espera dos sons dos eventos só roda com a tela da partida (0x6b55c0, jogo em rede): o contador \"sons dos eventos\" fica em zero na carreira automática")]
EN = [("dia primeira volta", "0x6b3c10 (batch 153)", "—", "the day's 1st round (+0x33 = 1) only starts matches whose competition (the match's +0x63) has +0x10, which the setup creates zeroed; in the automatic career the 1st round picks nothing and everything falls to the 2nd, so starting in the 2nd or pointing to another competition does not change the career"),
      ("dia partidas ja marcadas", "0x6b3c10 (batch 153)", "—", "the setup skips the list's matches already marked -2; in the automatic career none arrives marked like that (the mutation that lets them in escapes, even the strong one)"),
      ("sons dos eventos", "0x6d8c70 (batch 153)", "—", "waiting for the event sounds only runs with the match screen (0x6b55c0, network game): the counter \"sons dos eventos\" stays at zero in the automatic career")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `dia linhas do humano`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(42 hoje)', '(45 hoje)'), ('index.en.md', '(42 today)', '(45 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
