"""Documentacao da 197a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 197
R = [('hora_das_ferias', 'HUMAN_MANAGER::get_holiday_stamp', '0x5fdc30', 'human_manager.cpp', 'thiscall', 'a hora guardada quando o técnico humano saiu de férias', 'the clock stamp stored when the human manager went on holiday'), ('escalacao_guardada', 'HUMAN_MANAGER::get_saved_lineup', '0x5fed50', 'human_manager.cpp', 'thiscall', 'o endereço da escalação guardada do técnico humano (16 pares)', "the address of the human manager's saved line-up (16 pairs)"), ('tecnico_da_busca', 'PLAYER_SEARCH::set_manager', '0x7ae570', 'player_search.cpp', 'thiscall', 'grava na busca de jogadores o id do técnico humano', 'stores the human manager id in the player search')]
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
fazer(N, 464, 464 + 3, 'o registro do técnico humano (hora das férias e escalação guardada) e o técnico da busca', 'the human manager record (holiday stamp and saved line-up) and the search manager',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'o registro do técnico humano (hora das férias e escalação guardada) e o técnico da busca', 'the human manager record (holiday stamp and saved line-up) and the search manager', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
