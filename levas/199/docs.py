"""Documentacao da 199a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 199
R = [('estatistica_da_liga_do_jogador', 'PlayerStats::get_club_league_stat', '0x7c47b0', 'player_stats.cpp', 'thiscall', 'as partidas (campo 1) ou os gols (campo 2) de liga do jogador no clube, como número de ponto flutuante', "the player's league appearances (field 1) or league goals (field 2) at the club, as a float"), ('entrada_do_jogo_bate', 'Match_Stats::fixture_entry_matches', '0x729500', 'match_screens.cpp', 'thiscall', 'diz se uma entrada das estatísticas tem os dois clubes, em qualquer ordem, e as datas dadas', 'reports whether a match statistics entry holds the two given clubs in either order and the given date'), ('balde_do_registro', 'match_stats_get_record_bucket', '0x729bd0', 'match_screens.cpp', 'stdcall', 'o balde (0 a 3) em que o registro de estatística do jogo é guardado: tipo da competição, seleção e marcas do jogo', 'the bucket (0 to 3) where the match statistics record is stored: competition type, national team and fixture flags')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'e_data': ('na carreira a segunda data da entrada do jogo nunca decide (quando a primeira bate, a segunda também bate)', 'in the career the fixture entry second date never decides (when the first matches, the second matches too)'),
    'b_tipo': ('o ramo sem a marca +0x40 do balde do registro não roda na carreira automática', 'the record bucket branch without the +0x40 flag does not run in the automatic career'),
}
tot, ac, esc, ok = cadeia(N)
assert ok, 'cadeia sem a referencia'
assert set(esc) <= set(ESCAPADAS), ('explicar as escapadas', esc)
lac_pt = ' '.join(f'`{e}` escapou: {ESCAPADAS[e][0]}.' for e in esc) + (' (lacuna em `PENDENCIAS.md`)' if esc else '')
lac_en = ' '.join(f'`{e}` escaped: {ESCAPADAS[e][1]}.' for e in esc) + (' (gap in `PENDENCIAS.en.md`)' if esc else '')
M = (f"`mutacoes/leva{N}.py` ({tot}, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): {ac} acusadas. {lac_pt} "
     "Jogo real comparando até 2002-01-01: 0 divergências (963cea289f055ac3). Carreiras Rust ×2 e original até 2002-06-01: 2.382.029.053 sorteios, impressão 034ff3eb3e55dfd3, partidas b3ee24bdc00da55f, banco 7f2c15024cd99e7e")
ME = (f"`mutacoes/leva{N}.py` ({tot}, by the career to 2002-06-01, confirming the escaped ones in comparison mode): {ac} caught. {lac_en} "
      "Real game comparing to 2002-01-01: 0 divergences (963cea289f055ac3). Rust ×2 and original careers to 2002-06-01: 2,382,029,053 draws, fingerprint 034ff3eb3e55dfd3, matches b3ee24bdc00da55f, database 7f2c15024cd99e7e")
fazer(N, 470, 470 + 3, 'as estatísticas da partida: a estatística de liga do jogador, a entrada do jogo e o balde do registro', 'the match statistics: the player league stat, the fixture entry and the record bucket',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'as estatísticas da partida: a estatística de liga do jogador, a entrada do jogo e o balde do registro', 'the match statistics: the player league stat, the fixture entry and the record bucket', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
