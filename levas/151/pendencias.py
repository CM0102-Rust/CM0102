"""Lacunas da 151a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os, re
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("dia passo a passo", "0x6b5ce0 (151ª)", "—", "um passo do motor por chamada (sem `ate_o_fim`) não roda na carreira automática: o contador de ramo \"andar um passo\" fica em zero; lá o laço do dia joga cada partida até o fim"),
      ("dia contador de terminadas", "0x6b5ce0 (151ª)", "—", "o +0x25 do dia (partidas terminadas) só alimenta a porcentagem da barra; contar 2 em vez de 1 não muda a carreira"),
      ("dia rodada do humano", "0x6b5f00 (151ª)", "—", "a rodada (+0x2d de 0x4a4850) e a troca dos lados no 0x6b79a0 só existem com técnico humano; o contador \"fechar rodada do humano\" fica em zero na carreira automática"),
      ("dia historia", "0x6b5f00 (151ª)", "—", "o método 1 de [0xb74434] (a história) roda (contador \"fechar historia\" acima de 2048), mas a história não entra nas impressões da carreira (sorteios, partidas, banco): tirar a chamada não muda nada nelas")]
EN = [("dia passo a passo", "0x6b5ce0 (batch 151)", "—", "one engine step per call (without `ate_o_fim`) does not run in the automatic career: the branch counter \"andar um passo\" stays at zero; there the day loop plays each match to the end"),
      ("dia contador de terminadas", "0x6b5ce0 (batch 151)", "—", "the day's +0x25 (finished matches) only feeds the bar's percentage; counting 2 instead of 1 does not change the career"),
      ("dia rodada do humano", "0x6b5f00 (batch 151)", "—", "the round (+0x2d from 0x4a4850) and the swap of sides in 0x6b79a0 only exist with a human manager; the counter \"fechar rodada do humano\" stays at zero in the automatic career"),
      ("dia historia", "0x6b5f00 (batch 151)", "—", "[0xb74434]'s method 1 (the history) runs (counter \"fechar historia\" above 2048), but the history is not part of the career fingerprints (draws, matches, database): removing the call changes nothing in them")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `escalacao ficha do tecnico`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(37 hoje)', '(41 hoje)'), ('index.en.md', '(37 today)', '(41 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
