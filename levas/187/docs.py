"""Documentacao da 187a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 187
R = [('painel_dos_ultimos_placares', 'draw_latest_scores_panel', '0x723d20', 'match_screens.cpp', 'cdecl', 'desenha o painel dos últimos placares: título, selo, grupos e partidas do dia, tabela e botões', "draws the latest scores panel: title, badge, the day's groups and matches, table and buttons"), ('sem_tela_seguinte', 'ScreenManager::is_forward_unavailable', '0x7febb0', 'scrman.cpp', 'thiscall', 'o gerente de telas não tem tela seguinte para avançar', 'the screen manager has no next screen to go forward to'), ('cor_do_titulo', 'get_heading_colour', '0x839590', 'Setup.cpp', 'cdecl', 'a cor do texto do título (duas cores pela marca)', 'the heading text colour (two colours by the flag)')]
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
fazer(N, 434, 434 + 3, 'as telas da partida (5): o painel dos últimos placares, a tela seguinte e a cor do título', 'the match screens (5): the latest scores panel, the next screen and the heading colour',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'as telas da partida (5): o painel dos últimos placares, a tela seguinte e a cor do título', 'the match screens (5): the latest scores panel, the next screen and the heading colour', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
