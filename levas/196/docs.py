"""Documentacao da 196a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 196
R = [('ordem_do_confronto', 'league_compare_h2h_rows', '0x6834f0', 'league.cpp', 'cdecl', 'a ordem da minitabela do confronto direto: pontos e os critérios de desempate da liga', 'the head-to-head mini table order: points and the league tie-breakers'), ('atualizar_confronto_direto', 'League::update_head_to_head', '0x6869a0', 'league.cpp', 'thiscall', 'soma um jogo ao confronto direto (jogos, resultado, gols e pontos)', 'adds one match to the head-to-head record (played, result, goals and points)'), ('marcar_termos_oferecidos', 'TRANSFER_MANAGER::note_terms_offered_to_main_player', '0x8c1f80', 'transfer_manager.cpp', 'thiscall', 'marca que a oferta de transferência teve os termos de contrato oferecidos ao jogador', 'marks that the transfer offer had contract terms offered to the player')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'c_saldo': ('nenhuma liga da carreira desempata pelo confronto direto com o saldo (a minitabela não decide)', 'no career league breaks ties by head-to-head goal difference (the mini table does not decide)'),
    'c_empate': ('os pontos por empate do confronto direto não decidem nenhuma classificação da carreira', 'head-to-head draw points decide no standings in the career'),
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
fazer(N, 461, 461 + 3, 'o confronto direto da liga e os termos oferecidos na transferência', 'the league head-to-head and the terms offered in the transfer',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'o confronto direto da liga e os termos oferecidos na transferência', 'the league head-to-head and the terms offered in the transfer', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
