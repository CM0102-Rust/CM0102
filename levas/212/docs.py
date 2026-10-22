"""Documentacao da 212a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia, REF_JOGO, CARREIRA_PT, CARREIRA_EN

N = 212
R = [('preencher_registro_da_partida', 'Match_Stats::fill_match_record', '0x729ff0', 'match_screens.cpp', 'thiscall', 'preenche o registro de estatística da partida: clubes, datas, público, renda, clima, eventos ordenados, formação, capitão e os 20 jogadores de cada time', 'fills the match statistics record: clubs, dates, attendance, gate receipts, weather, sorted events, formation, captain and the 20 players of each side'), ('ordem_dos_eventos', 'match_stats_compare_events', '0x72b160', 'match_screens.cpp', 'cdecl', 'ordena dois eventos da partida pelo grupo do tipo, depois pelo minuto e pela ordem de chegada (comparador do qsort)', 'orders two match events by type group, then by minute and arrival order (qsort comparator)'), ('tirar_idioma_do_jogador', 'PlayerStats::remove_language', '0x7c6bf0', 'player_stats.cpp', 'thiscall', 'tira um idioma do jogador da tabela de idiomas (busca binária) e desce as entradas seguintes', "removes a language from the player's language table (binary search) and shifts the later entries down")]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'i_ultima': ('na carreira a 0x7c6bf0 só é chamada pelos regens (idiomas 1..64 do jogador recém-criado, que ainda não tem nenhum): a busca nunca acha a entrada, então a última vaga nunca é apagada (o comparador cobre a tabela inteira)', 'in the career 0x7c6bf0 is only called by the regens (languages 1..64 of the freshly created player, who has none yet): the search never finds the entry, so the last slot is never cleared (the comparator covers the whole table)'),
}
tot, ac, esc, ok = cadeia(N)
assert ok, 'cadeia sem a referencia'
assert set(esc) <= set(ESCAPADAS), ('explicar as escapadas', esc)
lac_pt = ' '.join(f'`{e}` escapou: {ESCAPADAS[e][0]}.' for e in esc) + (' (lacuna em `PENDENCIAS.md`)' if esc else '')
lac_en = ' '.join(f'`{e}` escaped: {ESCAPADAS[e][1]}.' for e in esc) + (' (gap in `PENDENCIAS.en.md`)' if esc else '')
M = (f"`mutacoes/leva{N}.py` ({tot}, pela carreira até 2002-06-01, confirmando no modo comparação as que escapam): {ac} acusadas. {lac_pt} "
     f"Jogo real comparando até 2002-01-01: 0 divergências ({REF_JOGO}). Carreiras Rust ×2 e original até 2002-06-01: {CARREIRA_PT}")
ME = (f"`mutacoes/leva{N}.py` ({tot}, by the career to 2002-06-01, confirming the escaped ones in comparison mode): {ac} caught. {lac_en} "
      f"Real game comparing to 2002-01-01: 0 divergences ({REF_JOGO}). Rust ×2 and original careers to 2002-06-01: {CARREIRA_EN}")
fazer(N, 524, 524 + len(R), 'o registro da partida, a ordem dos eventos e o idioma que o jogador perde', 'the match record, the order of the events and the language a player loses',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'o registro da partida, a ordem dos eventos e o idioma que o jogador perde', 'the match record, the order of the events and the language a player loses', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
