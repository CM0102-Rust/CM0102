"""Documentacao da 201a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 201
R = [('responder_continuar_da_partida', 'match_screens_handle_continue_command', '0x725ca0', 'match_screens.cpp', 'cdecl', 'o Continuar da partida: o comando pendente da escalação (em rede, a vaga livre do clube ou a espera pelos outros), as partidas da rede e a escalação', "the match Continue: the pending team selection command (online, the club's free slot or waiting for the others), the network matches and team selection"), ('abrir_escalacao_da_partida', 'match_screens_open_team_selection', '0x725f70', 'match_screens.cpp', 'cdecl', 'abre a tela da escalação da próxima partida do humano e grava o clube, a partida e se joga em casa', "opens the Team Selection screen for the human's next match and stores the club, the match and the home flag"), ('responder_escalacao_da_partida', 'match_screens_handle_team_selection_input', '0x727c30', 'match_screens.cpp', 'cdecl', 'os botões da escalação: nomeia o capitão (o melhor titular) se faltar e deixa o comando pendente; os outros vão à resposta comum', 'the Team Selection buttons: appoints the captain (the best starter) when missing and stores the pending command; the rest go to the common handler')]
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
fazer(N, 476, 476 + 3, 'as telas da partida (7): o Continuar da partida, abrir a escalação e a resposta da escalação', 'the match screens (7): the match Continue, opening team selection and the team selection response',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'as telas da partida (7): o Continuar da partida, abrir a escalação e a resposta da escalação', 'the match screens (7): the match Continue, opening team selection and the team selection response', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
