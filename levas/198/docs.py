"""Documentacao da 198a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 198
R = [('abrir_jogos_do_dia', 'open_day_fixtures_screen', '0x717cc0', 'match_screens.cpp', 'cdecl', 'abre a tela dos jogos do dia numa vaga com o grupo de jogos (caixa de erro sem grupo)', 'opens the day fixtures screen in a slot with the fixture group (error box without a group)'), ('responder_jogos_do_dia', 'handle_day_fixtures_input', '0x718930', 'match_screens.cpp', 'cdecl', 'a resposta da tela dos jogos do dia: outro grupo de jogos, o andamento da competição ou a barra de menu', 'the day fixtures screen input: another fixture group, the competition progress or the menu bar'), ('vaga_do_mais_bem_avaliado', 'match_screens_find_best_rated_starter_slot', '0x727e20', 'match_screens.cpp', 'cdecl', 'a vaga dos onze com a maior nota da comissão (o capitão padrão), −1 sem ninguém', 'the first eleven slot with the highest staff rating (the default captain), -1 for none')]
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
fazer(N, 467, 467 + 3, 'as telas da partida (6): a tela dos jogos do dia e o capitão padrão', 'the match screens (6): the day fixtures screen and the default captain',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Motor de partidas', 'Match engine'))
leia(N, 'as telas da partida (6): a tela dos jogos do dia e o capitão padrão', 'the match screens (6): the day fixtures screen and the default captain', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
