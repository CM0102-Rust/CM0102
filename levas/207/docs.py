"""Documentacao da 207a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 207
R = [('desenhar_conferencia_da_escalacao', 'match_screens_draw_team_selection', '0x7260f0', 'match_screens.cpp', 'cdecl', 'desenha a conferência da escalação: avisa (com os nomes ligados) os convocados, emprestados, indisponíveis, inelegíveis, suspensos, lesionados e sem permissão de trabalho, o limite do banco ou a falta de titulares; senão pede a confirmação (capitão, cobradores, reservas) com "No" e "Yes"', 'draws the team selection check: warns (with linked names) about players on international duty, on loan, unavailable, ineligible, banned, injured or without a work permit, the bench limit or missing starters; otherwise asks for confirmation (captain, set piece takers, substitutes) with "No" and "Yes"'), ('carregar_estatisticas_da_partida', 'Match_Stats::load', '0x72aa80', 'match_stats.cpp', 'thiscall', 'lê do match_stats.dat do save o índice dos registros, os cursores dos quatro baldes e as contagens', 'reads the record index, the four bucket cursors and the counts from match_stats.dat in the saved game'), ('salvar_estatisticas_da_partida', 'Match_Stats::save', '0x72adf0', 'match_stats.cpp', 'thiscall', 'grava no match_stats.dat do save o índice dos registros, os cursores dos quatro baldes e as contagens', 'writes the record index, the four bucket cursors and the counts to match_stats.dat in the saved game')]
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
fazer(N, 494, 494 + 3, 'as telas da partida (15): a conferência da escalação e o índice das estatísticas da partida no save', 'the match screens (15): the team selection check and the match statistics index in the saved game',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'as telas da partida (15): a conferência da escalação e o índice das estatísticas da partida no save', 'the match screens (15): the team selection check and the match statistics index in the saved game', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
