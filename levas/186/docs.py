"""Documentacao da 186a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 186
R = [('abrir_tela_da_partida', 'open_match_screen', '0x718aa0', 'match_screens.cpp', 'cdecl', 'abre a tela da partida ao vivo, grava os parâmetros dela e marca o humano como espectador', 'opens the live match screen, writes its parameters and marks the human as a spectator'), ('responder_ultimos_placares_da_partida', 'match_screens_handle_latest_scores_cycle', '0x725860', 'match_screens.cpp', 'cdecl', 'o painel dos últimos placares da tela da partida: a troca de página a cada 5 s e os cliques (competição, páginas, parar)', 'the latest scores panel of the match screen: the page switch every 5 s and the clicks (competition, pages, pause)'), ('responder_estatisticas_do_time', 'handle_team_stats_input', '0x722dc0', 'match_screens.cpp', 'cdecl', 'o painel das estatísticas do time: o redesenho com dados novos e os cliques que sobem e descem a linha', 'the team stats panel: the redraw with new data and the clicks that move the row up and down')]
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
fazer(N, 431, 431 + 3, 'as telas da partida (4): abrir a tela da partida e as respostas dos painéis dos últimos placares e das estatísticas do time', 'the match screens (4): opening the match screen and the responses of the latest scores and team stats panels',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'as telas da partida (4): abrir a tela da partida e as respostas dos painéis dos últimos placares e das estatísticas do time', 'the match screens (4): opening the match screen and the responses of the latest scores and team stats panels', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
