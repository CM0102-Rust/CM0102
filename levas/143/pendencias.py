"""Lacunas da 143a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = [("movimento sem vaga na grade", "0x6f09f0 (143ª)", "—", "o jogador que anda sem vaga na casa (+0x28 negativo) entra na casa nova; na carreira automática todos já entram em campo com vaga"),
      ("movimento sem mais adiantado", "0x6f09f0", "—", "o lado 1 sem o mais adiantado registrado (+0xf59a vazio) usa a linha 9; na carreira automática o registro nunca está vazio quando alguém anda")]
EN = [("movimento sem vaga na grade", "0x6f09f0 (batch 143)", "—", "a player moving with no slot in the cell (+0x28 negative) enters the new cell; in the automatic career everyone already takes the field with a slot"),
      ("movimento sem mais adiantado", "0x6f09f0", "—", "side 1 with no furthest-forward player recorded (+0xf59a empty) uses row 9; in the automatic career the record is never empty when someone moves")]
for arq, linhas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    s = open(p, encoding='utf-8').read()
    anc = [l for l in s.split('\n') if l.startswith('| `chute a gol empate tardio`')][0]
    s = s.replace(anc, anc + '\n' + '\n'.join(f'| `{a}` | {b} | {c} | {d} |' for a, b, c, d in linhas), 1)
    open(p, 'w', encoding='utf-8').write(s)
for arq, a, b in (('index.md', '(28 hoje)', '(30 hoje)'), ('index.en.md', '(28 today)', '(30 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
