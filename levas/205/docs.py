"""Documentacao da 205a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 205
R = [('preencher_estatisticas_do_time', 'fill_team_stats_rows', '0x721cd0', 'match_screens.cpp', 'cdecl', 'escreve as 20 linhas de jogador das estatísticas do time (22 colunas pela tabela do parâmetro 0x1c), montando as células ou atualizando o que mudou', 'writes the 20 player rows of the team stats table (22 columns from the param 0x1c table), building the cells or updating what changed'), ('atualizar_ultimos_placares', 'refresh_latest_scores_rows', '0x724520', 'match_screens.cpp', 'cdecl', 'reescreve as linhas dos últimos placares em páginas: fase, jogo, placar, período, autores dos eventos, agregado e o rodapé da página', 'rewrites the latest scores rows in pages: stage, fixture, score, period, event scorers, aggregate and the page footer'), ('esperar_os_outros_jogadores', 'match_screens_handle_waiting_screen', '0x727fb0', 'match_screens.cpp', 'cdecl', 'no jogo em rede, espera os outros humanos da mesma partida ou fecha as telas, avisa e abre a tela da partida', 'in a network game, waits for the other humans of the same fixture or closes the screens, shows the notice and opens the match screen')]
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
fazer(N, 488, 488 + 3, 'as telas da partida (12): as linhas das estatísticas do time, o redesenho dos últimos placares e a espera dos outros jogadores', 'the match screens (12): the team stats rows, the latest scores refresh and the wait for the other players',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'as telas da partida (12): as linhas das estatísticas do time, o redesenho dos últimos placares e a espera dos outros jogadores', 'the match screens (12): the team stats rows, the latest scores refresh and the wait for the other players', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
