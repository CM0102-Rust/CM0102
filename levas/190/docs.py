"""Documentacao da 190a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 190
R = [('tirar_contrato_de_emprestimo', 'CONTRACT_MANAGER::remove_loan_contract', '0x4d0bd0', 'contract_manager.cpp', 'thiscall', 'libera a vaga do contrato de empréstimo da pessoa, se houver', "frees the person's loan contract slot, if any"), ('desmarcar_lances_pelo_jogador', 'Shortlist_Manager::clear_bid_markers_for_player', '0x84aeb0', 'shortlist_manager.cpp', 'thiscall', 'tira a marca de lance das vagas da lista de observados do clube que miram o jogador', 'clears the bid marker of the club shortlist slots that target the player'), ('tecnico_substituto', 'human_manager_get_stand_in_manager', '0x600e90', 'human_manager.cpp', 'cdecl', 'o auxiliar do clube ou, sem ele, o membro da comissão de maior reputação', "the club's assistant or, without one, the highest-reputation staff member")]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    's_empate': ('o técnico substituto só é pedido para o clube humano, e na comissão dele nunca há empate de reputação', 'the stand-in manager is only asked for the human club, and its staff never tie on reputation'),
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
fazer(N, 443, 443 + 3, 'o contrato de empréstimo, as marcas de lance da lista de observados e o técnico substituto', 'the loan contract, the shortlist bid markers and the stand-in manager',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'o contrato de empréstimo, as marcas de lance da lista de observados e o técnico substituto', 'the loan contract, the shortlist bid markers and the stand-in manager', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
