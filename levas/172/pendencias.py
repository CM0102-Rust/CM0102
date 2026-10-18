"""Lacunas da 172a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = ["| `próximo evento da narração` | 0x6d78b0 (172ª) | — | só roda com a narração na tela (a carreira automática não mostra partida): as mutações `p_parte` e `p_anel` escapam. Teste: o Lucas assistindo partidas com a narração, comparando |",
      "| `nome do jogador na narração` | 0x6db2d0 (172ª) | — | só roda com a narração na tela: a mutação `n_longa` (a forma longa para os nomes curtos repetidos) escapa. Teste: partida assistida comparando, de preferência com dois jogadores de mesmo nome curto |"]
EN = ["| `next commentary event` | 0x6d78b0 (batch 172) | — | only runs with the commentary on screen (the automatic career shows no match): mutations `p_parte` and `p_anel` escape. Test: Lucas watching matches with commentary, in comparison mode |",
      "| `commentary player name` | 0x6db2d0 (batch 172) | — | only runs with the commentary on screen: mutation `n_longa` (the long form for repeated short names) escapes. Test: a watched match in comparison mode, ideally with two players sharing a short name |"]
for arq, novas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    L = open(p, encoding='utf-8').read().split('\n')
    if novas[0] in L:
        continue
    k = max(i for i, l in enumerate(L) if '(170ª)' in l or 'batch 170)' in l)
    L[k + 1:k + 1] = novas
    open(p, 'w', encoding='utf-8').write('\n'.join(L))
for arq, a, b in (('index.md', '(62 hoje)', '(64 hoje)'), ('index.en.md', '(62 today)', '(64 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    if a in s:
        open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
