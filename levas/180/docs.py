"""Documentacao da 180a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 180
R = [('rotulo_do_controle', 'match_control_label', '0x725a70', 'match_screens.cpp', 'cdecl', 'o rótulo do botão de controle da partida (esperando, começar, recomeçar, pausar ou retomar, continuar) e se ele fica desligado', 'the match control button label (waiting, kick off, restart, pause or resume, continue) and whether it is disabled'), ('fechar_tela_da_partida', 'close_match_screen', '0x71b120', 'match_screens.cpp', 'cdecl', 'fecha a tela da partida: destrói e libera os parâmetros 0x2d e 0x1d do gerente de telas', 'closes the match screen: destroys and frees screen manager parameters 0x2d and 0x1d'), ('abrir_tela_de_espera', 'open_waiting_screen', '0x727f00', 'match_screens.cpp', 'cdecl', 'abre a tela de espera do técnico humano no gerente de telas', "opens the human manager's waiting screen in the screen manager")]
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
fazer(N, 413, 413 + 3, 'as telas da partida (1): o rótulo do controle, fechar a tela e a espera', 'the match screens (1): the control label, closing the screen and the waiting screen',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'as telas da partida (1): o rótulo do controle, fechar a tela e a espera', 'the match screens (1): the control label, closing the screen and the waiting screen', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
