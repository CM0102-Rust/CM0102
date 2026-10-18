"""Documentacao da 170a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 170
R = [('ordem_das_partidas_do_dia', 'day_match_order', '0x6b6510', 'match_eng.cpp', 'cdecl', 'a ordem das partidas do dia (qsort): competição, campos do registro e o nome do clube', "the day's match order (qsort): competition, record fields and the club name"), ('ordem_de_acao', 'action_order', '0x6bd840', 'match_eng.cpp', 'cdecl', 'a ordem dos jogadores na vez do motor (qsort): os marcados primeiro, depois pela força', "the players' order on the engine's turn (qsort): the marked ones first, then by strength"), ('variante_do_evento', 'event_variant', '0x6d5d80', 'match_events.cpp', 'stdcall', 'troca o evento da narração por uma variante, por sorteio e pelo tipo do lance', 'swaps the commentary event for a variant, by a draw and by the play type')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'o_campos': ('na liga inglesa as partidas do dia empatadas até +0x36 nunca divergem em sinal entre +0x38 e +0x34 (falta de dados; vai para a carreira ampla)', 'in the English league the day matches tied up to +0x36 never disagree in sign between +0x38 and +0x34 (missing data; goes to the broad career)'),
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
fazer(N, 383, 383 + 3, 'o fechamento do motor (4): as ordens das partidas e dos jogadores e a variante do evento', 'closing the engine (4): the match and player orderings and the event variant',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'o fechamento do motor (4): as ordens das partidas e dos jogadores e a variante do evento', 'closing the engine (4): the match and player orderings and the event variant', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
