"""Documentacao da 172a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 172
R = [('proximo_evento', 'next_event', '0x6d78b0', 'match_events.cpp', 'thiscall', 'o próximo evento da narração a mostrar, parte por parte do texto (separadas por |), avançando a leitura da fila', 'the next commentary event to show, part by part of the text (split by |), advancing the queue reading'), ('nome_do_jogador_na_narracao', 'commentary_player_name', '0x6db2d0', 'match_events.cpp', 'cdecl', 'o nome do jogador na narração: a forma curta, ou a longa quando outro jogador dos dois clubes tem o mesmo nome curto', 'the player name in the commentary: the short form, or the long one when another player of either club has the same short name'), ('habilidade_vista', 'perceived_ability', '0x6e86d0', 'match_man.cpp', 'cdecl', 'a habilidade atual ou potencial do jogador como um observador a vê (reputação, ruído e conhecimento do observador)', "a player's current or potential ability as an observer sees it (reputation, noise and the observer's knowledge)")]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'p_parte': ('a 0x6d78b0 só roda com a narração na tela, que a carreira automática não mostra (vai para a partida assistida do Lucas)', '0x6d78b0 only runs with the commentary on screen, which the automatic career does not show (goes to a match watched by Lucas)'),
    'p_anel': ('a 0x6d78b0 só roda com a narração na tela, que a carreira automática não mostra', '0x6d78b0 only runs with the commentary on screen, which the automatic career does not show'),
    'n_longa': ('a 0x6db2d0 só roda com a narração na tela, que a carreira automática não mostra', '0x6db2d0 only runs with the commentary on screen, which the automatic career does not show'),
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
fazer(N, 389, 389 + 3, 'o fechamento do motor (6): a leitura da narração, o nome do jogador e a habilidade vista', 'closing the engine (6): reading the commentary, the player name and the perceived ability',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'o fechamento do motor (6): a leitura da narração, o nome do jogador e a habilidade vista', 'closing the engine (6): reading the commentary, the player name and the perceived ability', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
