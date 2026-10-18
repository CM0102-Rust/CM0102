"""Documentacao da 164a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 164
R = [('girar_rodada', 'rotate_round', '0x68a030', 'league.cpp', 'cdecl', 'a próxima posição no rodízio do calendário (liga especial, saltos de 3 por sorteio, voltas em n)', 'the next position in the calendar rotation (special league, jumps of 3 by draw, wrapping at n)'), ('registrar_jogo_da_rodada', 'store_round_match', '0x68a1c0', 'league.cpp', 'cdecl', 'grava o jogo da rodada (três bytes e um dword)', 'stores the round match (three bytes and a dword)'), ('marcar_ano_no_calendario', 'mark_calendar_year', '0x5af180', 'fix_man.cpp', 'cdecl', 'marca o ano do jogo nas entradas dos dois clubes (ano × 2 + bit, regra de 2001)', "marks the match year in both clubs' entries (year × 2 + bit, 2001 rule)")]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'g_meio': ('a carreira automática só joga ligas inglesas, sem a marca 0x80 do rodízio; nem refeita mais forte (`g_meio_b`, `mutacoes/leva164b.py`) é acusada',
               'the automatic career only plays English leagues, without the rotation\'s 0x80 flag; not even redone stronger (`g_meio_b`, `mutacoes/leva164b.py`) is it caught'),
    'g_volta': ('a volta no fim do rodízio não acontece na carreira; nem refeita mais forte (`g_volta_b`) é acusada',
                'wrapping at the end of the rotation does not happen in the career; not even redone stronger (`g_volta_b`) is it caught'),
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
fazer(N, 365, 365 + 3, 'o calendário das ligas', 'the league calendar',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'o calendário das ligas', 'the league calendar', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
