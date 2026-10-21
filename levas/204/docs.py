"""Documentacao da 204a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 204
R = [('painel_das_zonas_de_acao', 'draw_action_zones_panel', '0x71f500', 'match_screens.cpp', 'cdecl', 'desenha as zonas de ação: o seletor de período, o campo dividido em três zonas com as barras das cores dos dois clubes, ou a mensagem de que não há zonas', "draws the action zones: the period selector, the pitch split into three zones with both clubs' colour bars, or the no-zones message"), ('painel_do_relatorio_da_partida', 'draw_match_report_panel', '0x721000', 'match_screens.cpp', 'cdecl', 'desenha o relatório da partida: os minutos e os textos dos lances (criando o relatório fora da partida), ou a mensagem de que não há relatório', 'draws the match report: the minutes and the event texts (building the report outside the match), or the no-report message'), ('montar_jogos_do_dia', 'build_day_fixtures_screen', '0x717db0', 'match_screens.cpp', 'cdecl', 'monta a tela dos jogos do dia: título, árvore das competições do dia, a tabela das rodadas e partidas e a rolagem até a partida do humano', "builds the day fixtures screen: title, the day's competition tree, the rounds and matches table and the scroll to the human's match")]
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
fazer(N, 485, 485 + 3, 'as telas da partida (10): os painéis das zonas de ação e do relatório da partida e a tela dos jogos do dia', 'the match screens (10): the action zones and match report panels and the day fixtures screen',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'as telas da partida (10): os painéis das zonas de ação e do relatório da partida e a tela dos jogos do dia', 'the match screens (10): the action zones and match report panels and the day fixtures screen', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
