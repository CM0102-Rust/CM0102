"""Documentacao da 174a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 174
R = [('pessoa_na_lista', 'person_in_list', '0x69fc60', 'manager_manager.cpp', 'thiscall', 'a pessoa está na lista (iterador 0x68d1b0, devolvido no fim)?', 'is the person in the list (iterator 0x68d1b0, restored at the end)?'), ('ficha_de_contrato', 'contract_record', '0x4cf6d0', 'contract_manager.cpp', 'thiscall', 'a ficha de contrato (0x50 bytes) da pessoa', "the person's contract record (0x50 bytes)"), ('desgaste_diario', 'daily_wear', '0x62c2f0', 'injury.cpp', 'thiscall', 'o desgaste do dia na palavra +0xc (idade, sorteios de 25 e 13, teto de goleiro)', "the day's wear on word +0xc (age, draws of 25 and 13, goalkeeper cap)")]
ESCAPADAS = {}  # nome: (explicação pt, explicação en), quando houver
tot, ac, esc, ok = cadeia(N)
assert ok, 'cadeia sem a referencia'
assert set(esc) <= set(ESCAPADAS), ('explicar as escapadas', esc)
lac_pt = ' '.join(f'`{e}` escapou: {ESCAPADAS[e][0]}.' for e in esc) + (' (lacuna em `PENDENCIAS.md`)' if esc else '')
lac_en = ' '.join(f'`{e}` escaped: {ESCAPADAS[e][1]}.' for e in esc) + (' (gap in `PENDENCIAS.en.md`)' if esc else '')
M = (f"`mutacoes/leva{N}.py` ({tot}, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): {ac} acusadas. {lac_pt} "
     "Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = (f"`mutacoes/leva{N}.py` ({tot}, by the career to 2002-06-01, confirming the escaped ones in comparison mode): {ac} caught. {lac_en} "
      "Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(N, 395, 395 + 3, 'a pessoa na lista do técnico, a ficha de contrato e o desgaste diário', 'person in the manager list, the contract record and the daily wear',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'a pessoa na lista do técnico, a ficha de contrato e o desgaste diário', 'person in the manager list, the contract record and the daily wear', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
