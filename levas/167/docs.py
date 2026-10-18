"""Documentacao da 167a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 167
R = [('bola_na_area', 'ball_goal_end', '0x6d58f0', 'match_eng.cpp', 'thiscall', '1 com a bola na área de cima, −1 na de baixo, senão 0; devolve só `al`', '1 with the ball in the top box, −1 in the bottom one, otherwise 0; returns only `al`'), ('partida_abandonada', 'match_abandoned', '0x6d5930', 'match_eng.cpp', 'thiscall', 'o minuto do abandono × 11 abaixo do relógio (16 bits); devolve só `al`', 'the abandonment minute × 11 below the clock (16-bit); returns only `al`'), ('vagas_do_lado_sem_conflito', 'side_slots_without_conflict', '0x6d5af0', 'match_eng.cpp', 'thiscall', 'a 0x5b8960 na formação do lado', "0x5b8960 on the side's formation")]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'a_onze': ('nenhuma partida é abandonada na carreira automática (o minuto do abandono fica em −1)', 'no match is abandoned in the automatic career (the abandonment minute stays at −1)'),
}
tot, ac, esc, ok = cadeia(N)
assert ok, 'cadeia sem a referencia'
assert set(esc) <= set(ESCAPADAS), ('explicar as escapadas', esc)
lac_pt = ' '.join(f'`{e}` escapou: {ESCAPADAS[e][0]}.' for e in esc) + (' (lacuna em `PENDENCIAS.md`)' if esc else '')
lac_en = ' '.join(f'`{e}` escaped: {ESCAPADAS[e][1]}.' for e in esc) + (' (gap in `PENDENCIAS.en.md`)' if esc else '')
M = (f"`mutacoes/leva{N}.py` ({tot}, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): {ac} acusadas. {lac_pt} "
     "Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = (f"`mutacoes/leva{N}.py` ({tot}, by the career to 2002-06-01, confirming the escaped ones in comparison mode): {ac} caught. {lac_en} "
      "Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(N, 374, 374 + 3, 'o fechamento do motor (1): o gol da bola, a partida abandonada e as vagas do lado', "closing the engine (1): the ball's goal end, the abandoned match and the side's slots",
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'o fechamento do motor (1): o gol da bola, a partida abandonada e as vagas do lado', "closing the engine (1): the ball's goal end, the abandoned match and the side's slots", [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
