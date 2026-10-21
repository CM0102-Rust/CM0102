"""Documentacao da 208a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 208
R = [('organizar_area', 'Area::layout', '0x403350', 'Area.cpp', 'thiscall', 'reparte a largura e a altura da área pelos pesos das colunas e linhas (até 30 de cada; mais dá a caixa Area.cpp 497/507) e liga a barra de rolagem', 'splits the area width and height by the column and row weights (up to 30 each; more raises the Area.cpp 497/507 box) and sets up the scrollbar'), ('iniciar_area', 'Area::init', '0x402ac0', 'Area.cpp', 'thiscall', 'inicia a área: retângulo limitado à tela, campos, barra de rolagem zerada e os pesos das colunas e linhas; as linhas à mostra travadas em 30 (o tamanho da grade), o resto vai para a barra de rolagem', 'initialises the area: rectangle clamped to the screen, fields, cleared scrollbar and the column and row weights; the visible rows clamped at 30 (the grid size), the rest goes to the scroll bar'), ('desenhar_area', 'Area::draw', '0x402e70', 'Area.cpp', 'thiscall', 'desenha a área: o painel, as linhas da grade ou cada item à mostra na sua célula, e a barra de rolagem', 'draws the area: the panel, the grid lines or each visible widget in its cell, and the scrollbar')]
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
fazer(N, 497, 497 + 3, 'as áreas da interface (1): organizar a grade, iniciar e desenhar uma área (com a trava das 30 linhas: o "Area.cpp 507" do relatório em 1080)', 'the interface areas (1): laying out the grid, initialising and drawing an area (with the 30-row clamp: the 1080 report "Area.cpp 507")',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'as áreas da interface (1): organizar a grade, iniciar e desenhar uma área (com a trava das 30 linhas: o "Area.cpp 507" do relatório em 1080)', 'the interface areas (1): laying out the grid, initialising and drawing an area (with the 30-row clamp: the 1080 report "Area.cpp 507")', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
