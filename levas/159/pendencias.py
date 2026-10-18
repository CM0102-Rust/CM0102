"""Lacuna da 159a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = "| `formacao` | 0x5b7d20, 0x5b8960 (159ª) | — | mutações `m_nivel` e `v_a20` escapam na carreira e na comparação: no banco a posição natural tem nota 20 (o laço da máscara para no nível 15) e nenhuma vaga com lado aberto é de meia. Exige banco editado (jogador sem posição acima de 14) |"
EN = "| `formacao` | 0x5b7d20, 0x5b8960 (batch 159) | — | mutations `m_nivel` and `v_a20` escape in the career and in the comparison: in the database the natural position is rated 20 (the mask loop stops at level 15) and no slot with an open side is a midfielder's. Needs an edited database (player with no position above 14) |"
for arq, nova in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    L = open(p, encoding='utf-8').read().split('\n')
    k = [i for i, l in enumerate(L) if l.startswith('| `conteiner`')][0]
    L.insert(k + 1, nova)
    open(p, 'w', encoding='utf-8').write('\n'.join(L))
for arq, a, b in (('index.md', '(51 hoje)', '(52 hoje)'), ('index.en.md', '(51 today)', '(52 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
