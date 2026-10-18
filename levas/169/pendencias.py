"""Lacunas da 169a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = ["| `fila da narração: +0x2db0` | 0x6d5bf0 (169ª) | — | mutação equivalente (`c_tempo`): o valor inicial 125 só é comparado com 20 e 101 (0x6b5686, 0x6d8ace) e as telas da partida o regravam (0x719c4e, 0x71a3d5); 124 toma os mesmos caminhos |",
      "| `idioma dos eventos` | 0x6d5b50 (169ª) | — | mutação equivalente (`l_fim`): 0xa1599c guarda o idioma dos eventos carregados e a 0x6d9e50, que chama a liberação, o regrava logo depois (0x6d9eb8) |"]
EN = ["| `commentary queue: +0x2db0` | 0x6d5bf0 (batch 169) | — | equivalent mutation (`c_tempo`): the initial 125 is only compared with 20 and 101 (0x6b5686, 0x6d8ace) and the match screens rewrite it (0x719c4e, 0x71a3d5); 124 takes the same paths |",
      "| `events language` | 0x6d5b50 (batch 169) | — | equivalent mutation (`l_fim`): 0xa1599c holds the language of the loaded events and 0x6d9e50, which calls the freeing, rewrites it right after (0x6d9eb8) |"]
for arq, novas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    L = open(p, encoding='utf-8').read().split('\n')
    k = [i for i, l in enumerate(L) if l.startswith('| `partida abandonada`')][0]
    L[k + 1:k + 1] = novas
    open(p, 'w', encoding='utf-8').write('\n'.join(L))
for arq, a, b in (('index.md', '(59 hoje)', '(61 hoje)'), ('index.en.md', '(59 today)', '(61 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
