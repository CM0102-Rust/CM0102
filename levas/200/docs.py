"""Documentacao da 200a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 200
R = [('atribuir_bloco_do_time', 'match_team_block_assign', '0x71dd70', 'match_screens.cpp', 'thiscall', 'copia o bloco do time da partida (0x625 bytes) campo a campo, com as 20 linhas de jogador', 'copies the match team block (0x625 bytes) field by field, with its 20 player rows'), ('atribuir_linha_do_jogador', 'match_player_row_assign', '0x71df00', 'match_screens.cpp', 'thiscall', 'copia uma linha de jogador do bloco do time (0x49 bytes), a data pela Date::operator=', 'copies one player row of the team block (0x49 bytes), the date through Date::operator='), ('contar_placar_ao_vivo', 'match_screens_tally_live_score', '0x728e90', 'match_screens.cpp', 'cdecl', 'conta os gols e os gols nos pênaltis de uma partida até o minuto atual da partida ao vivo', 'counts the goals and shoot-out goals of a match up to the current minute of the live match')]
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
fazer(N, 473, 473 + 3, 'as telas da partida (9): a cópia do bloco do time e da linha do jogador e o placar ao vivo', 'the match screens (9): the team block and player row copies and the live score',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'as telas da partida (9): a cópia do bloco do time e da linha do jogador e o placar ao vivo', 'the match screens (9): the team block and player row copies and the live score', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
