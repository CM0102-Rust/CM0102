"""Documentacao da 183a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 183
R = [('abrir_detalhes_da_partida', 'open_match_details', '0x71b4a0', 'match_screens.cpp', 'cdecl', 'abre a tela de detalhes de uma partida e grava os parâmetros dela', 'opens the details screen of a match and writes its parameters'), ('responder_detalhes_da_partida', 'match_details_input', '0x71bdc0', 'match_screens.cpp', 'cdecl', 'a resposta da tela de detalhes a um clique (troca de aba, tática do clube, redesenho das abas, menu)', "the match details screen's response to a click (tab switch, club tactics, tab redraw, menu)"), ('fechar_detalhes_da_partida', 'close_match_details', '0x71c170', 'match_screens.cpp', 'cdecl', 'fecha a tela de detalhes: destrói e libera os objetos guardados nos parâmetros', 'closes the details screen: destroys and frees the objects kept in the parameters')]
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
fazer(N, 422, 422 + 3, 'as telas da partida (3): a tela de detalhes da partida', 'the match screens (3): the match details screen',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'as telas da partida (3): a tela de detalhes da partida', 'the match screens (3): the match details screen', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
