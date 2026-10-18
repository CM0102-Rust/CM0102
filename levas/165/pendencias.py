"""Lacuna da 165a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = "| `vaga extra apagada` | 0x897ee0 (165ª) | — | o caminho roda (contador alto), mas a segunda vaga extra está sempre vazia ao apagar a primeira nos clubes da IA: a mutação `v_desce` é equivalente nos dados. Teste: técnico humano com as duas vagas extras preenchidas, apagando a primeira |"
EN = "| `vaga extra apagada` | 0x897ee0 (batch 165) | — | the path runs (high counter), but the second extra slot is always empty when the first is cleared in AI clubs: mutation `v_desce` is equivalent in the data. Test: a human manager with both extra slots filled, clearing the first |"
for arq, nova in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    L = open(p, encoding='utf-8').read().split('\n')
    k = [i for i, l in enumerate(L) if l.startswith('| `calendario`')][0]
    L.insert(k + 1, nova)
    open(p, 'w', encoding='utf-8').write('\n'.join(L))
for arq, a, b in (('index.md', '(55 hoje)', '(56 hoje)'), ('index.en.md', '(55 today)', '(56 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
