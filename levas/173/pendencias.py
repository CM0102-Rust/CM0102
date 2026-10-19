"""Lacunas da 173a leva em docs/PENDENCIAS (pt/en), na lista da carreira ampla e a contagem no wiki."""
import os
R = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..')
PT = ["| `regra do elenco` | 0x6e8910 (173ª) | — | na liga inglesa a fase do banco não teve restritos e a regra por idade nunca teve um jogador exatamente no limite: `r_banco` e `r_idade` escapam até comparando. Teste: carreira ampla (Itália, Espanha: regras de estrangeiros) |",
      "| `nota do jogador: veterano sem jogos` | 0x6e6340/0x6e2f20 (173ª) | — | o ajuste do contrato (flag 0x2000000) nunca pegou um veterano (mais de 34) sem jogos: `n_veterano` escapa. Teste: carreira ampla |",
      "| `nota para a vaga: férias` | 0x6e2f20 (173ª) | — | o bônus das férias só roda com a lista de férias de um técnico humano: `v_ferias` escapa. Teste: o Lucas saindo de férias comparando |"]
EN = ["| `squad rule` | 0x6e8910 (batch 173) | — | in the English league the bench phase had no restricted players and the age rule never had a player exactly at the limit: `r_banco` and `r_idade` escape even comparing. Test: broad career (Italy, Spain: foreigner rules) |",
      "| `player score: veteran with no games` | 0x6e6340/0x6e2f20 (batch 173) | — | the contract adjustment (flag 0x2000000) never met a veteran (over 34) with no games: `n_veterano` escapes. Test: broad career |",
      "| `lineup slot score: holidays` | 0x6e2f20 (batch 173) | — | the holiday bonus only runs with a human manager's holiday list: `v_ferias` escapes. Test: Lucas going on holiday in comparison mode |"]
for arq, novas in (('PENDENCIAS.md', PT), ('PENDENCIAS.en.md', EN)):
    p = os.path.join(R, 'docs', arq)
    L = open(p, encoding='utf-8').read().split('\n')
    if novas[0] in L:
        continue
    k = max(i for i, l in enumerate(L) if '(172ª)' in l or 'batch 172)' in l)
    L[k + 1:k + 1] = novas
    open(p, 'w', encoding='utf-8').write('\n'.join(L))
p = os.path.join(R, 'levas', 'carreira_ampla', 'pendentes.txt')
s = open(p, encoding='utf-8').read().rstrip('\n')
if 'leva173b' not in s:
    open(p, 'w', encoding='utf-8').write(s + '\n- leva173b.py: r_banco, r_idade, n_veterano (regra do elenco, veterano sem jogos); v_ferias fica para o Lucas\n')
for arq, a, b in (('index.md', '(64 hoje)', '(67 hoje)'), ('index.en.md', '(64 today)', '(67 today)')):
    p = os.path.join(R, 'wiki', arq)
    s = open(p, encoding='utf-8').read()
    if a in s:
        open(p, 'w', encoding='utf-8').write(s.replace(a, b))
print('ok')
