"""Documentacao da 191a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 191
R = [('clube_do_contrato_acertado', 'TRANSFER_MANAGER::get_agreed_contract_club', '0x8c5450', 'transfer_manager.cpp', 'thiscall', 'o clube do contrato acertado da pessoa (desliga a marca quando o contrato sumiu)', "the club of the person's agreed contract (clears the flag when the contract is gone)"), ('registrar_jogo_pela_selecao', 'TRANSFER_MANAGER::record_international_appearance', '0x8ce920', 'transfer_manager.cpp', 'thiscall', 'registra o jogo do jogador pela seleção nos bits dos últimos jogos', "records the player's national team match in the recent-matches bits"), ('punicao_guardada', 'Discipline::find_held_ban', '0x55cb60', 'discipline.cpp', 'thiscall', 'a punição guardada do jogador numa vaga, ou nenhuma', "the player's held ban in a slot, or none")]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'c_clube': ('na carreira o clube do contrato acertado nunca é pedido com a marca ligada e o contrato achado', 'in the career the agreed contract club is never asked for with the flag set and the contract found'),
    'p_vaga': ('nenhum jogador tem duas punições guardadas em vagas diferentes quando a busca roda', 'no player has two held bans in different slots when the search runs'),
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
fazer(N, 446, 446 + 3, 'o clube do contrato acertado, o jogo pela seleção e a punição guardada', 'the agreed contract club, the national team appearance and the held ban',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'o clube do contrato acertado, o jogo pela seleção e a punição guardada', 'the agreed contract club, the national team appearance and the held ban', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
