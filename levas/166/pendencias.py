"""Lacunas da 166a leva em docs/PENDENCIAS (pt/en) e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
NOVAS = [("| `humano manda no clube` | 0x600c90 (166ª) | — | sem técnico humano o contador fica em zero (`h_forcar` escapa). O teste é o jogo do Lucas comparando, inclusive como técnico de seleção |",
          "| `humano manda no clube` | 0x600c90 (batch 166) | — | with no human manager the counter stays at zero (`h_forcar` escapes). The test is Lucas's game in comparison mode, also as a national team manager |"),
         ("| `categoria do contrato` | 0x4ca470 (166ª) | — | o contrato do tipo 5 e a faixa da situação 0 com satisfação de 40 a 75 não aparecem na carreira (`c_tipo5`, `c_k0` escapam mesmo refeitas). Candidatas à carreira ampla de madrugada |",
          "| `categoria do contrato` | 0x4ca470 (batch 166) | — | type-5 contracts and the status-0 range with happiness 40 to 75 do not appear in the career (`c_tipo5`, `c_k0` escape even redone). Candidates for the overnight wide career |")]
for k, arq in ((0, 'PENDENCIAS.md'), (1, 'PENDENCIAS.en.md')):
    p = os.path.join(R, 'docs', arq)
    L = open(p, encoding='utf-8').read().split('\n')
    i = [j for j, l in enumerate(L) if l.startswith('| `vaga extra apagada`')][0]
    for d, par in enumerate(NOVAS):
        L.insert(i + 1 + d, par[k])
    open(p, 'w', encoding='utf-8').write('\n'.join(L))
for arq, a, b in (('index.md', '(56 hoje)', '(58 hoje)'), ('index.en.md', '(56 today)', '(58 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    assert s.count(a) == 1, (arq, a)
    open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
