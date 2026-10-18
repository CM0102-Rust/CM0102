"""Documentacao da 166a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 166
R = [('registro_do_observador', 'scout_record', '0x7f6cd0', 'scout_manager.cpp', 'thiscall', 'o registro de observação 0..9 do técnico humano (0x2b0 bytes)', "the human manager's scouting record 0..9 (0x2b0 bytes)"), ('humano_manda_no_clube', 'human_runs_club', '0x600c90', 'human_manager.cpp', 'cdecl', 'o técnico humano da vez manda no clube (ou na seleção principal da nação)?', "does the current human manager run the club (or the nation's main team)?"), ('categoria_do_contrato', 'contract_category', '0x4ca470', 'contract_manager.cpp', 'stdcall', 'a categoria 1..8 do jogador no contrato (situação, satisfação, idade, ambição, reputação)', "the player's category 1..8 in the contract (status, happiness, age, ambition, reputation)")]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'h_forcar': ('o contador \"humano manda no clube\" fica em zero: sem técnico humano, a rotina nunca devolve 1 na carreira', 'counter \"humano manda no clube\" stays at zero: with no human manager the routine never returns 1 in the career'),
    'c_k0': ('limite exato; nem refeita mais forte (`c_k0_b`, `mutacoes/leva166b.py`) é acusada: a faixa da situação 0 com satisfação de 40 a 75 não aparece', 'exact limit; not even redone stronger (`c_k0_b`, `mutacoes/leva166b.py`) is it caught: the status-0 range with happiness 40 to 75 does not appear'),
    'c_k5': ('limite exato; refeita como `c_k5_b` e acusada', 'exact limit; redone as `c_k5_b` and caught'),
    'c_tipo5': ('contrato do tipo 5 não aparece na carreira; nem refeita mais forte (`c_tipo5_b`) é acusada', 'type-5 contracts do not appear in the career; not even redone stronger (`c_tipo5_b`) is it caught'),
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
fazer(N, 371, 371 + 3, 'o observador do humano, o humano no clube e a categoria do contrato', "the human's scout, the human at the club and the contract category",
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'o observador do humano, o humano no clube e a categoria do contrato', "the human's scout, the human at the club and the contract category", [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
