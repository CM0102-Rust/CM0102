"""Documentacao da 168a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 168
R = [('zagueiros_do_lado_sem_conflito', 'side_centre_backs_without_conflict', '0x6d5b20', 'match_eng.cpp', 'thiscall', 'a 0x5b8dc0 na formação do lado', "0x5b8dc0 on the side's formation"), ('construir_arbitro', 'construct_referee', '0x6e9460', 'match_official.cpp', 'thiscall', 'o construtor do árbitro (zera +0 e +4)', 'the referee constructor (zeroes +0 and +4)'), ('destruir_jogador_da_partida', 'destroy_match_player', '0x6ebd60', 'match_pl.cpp', 'thiscall', 'o destrutor do jogador da partida (zera +0x19e)', 'the match player destructor (zeroes +0x19e)')]
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
fazer(N, 377, 377 + 3, 'o fechamento do motor (2): os zagueiros do lado, o árbitro e o jogador da partida', "closing the engine (2): the side's centre-backs, the referee and the match player",
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'o fechamento do motor (2): os zagueiros do lado, o árbitro e o jogador da partida', "closing the engine (2): the side's centre-backs, the referee and the match player", [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
