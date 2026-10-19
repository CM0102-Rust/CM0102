"""Documentacao da 179a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 179
R = [('oferta_de_contrato', 'contract_offer', '0x4cc9e0', 'contract_manager.cpp', 'thiscall', 'a oferta de contrato da pessoa para o clube, na lista do gerente de contratos', "the person's contract offer for the club, in the contract manager's list"), ('oferta_acertada', 'agreed_contract_offer', '0x4cca30', 'contract_manager.cpp', 'thiscall', 'a oferta de contrato acertada (estado 2) da pessoa', "the person's agreed contract offer (state 2)"), ('oferta_fechada', 'transfer_offer_closed', '0x8d0d90', 'transfer_offer.cpp', 'stdcall', 'a oferta de transferência está encerrada (pelo estado e, no estado 0xf, pela fase)', 'whether the transfer offer is closed (by its state and, in state 0xf, by its phase)')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'f_estado': ('na carreira as ofertas de transferência consultadas nunca estão no estado 4', 'in the career the transfer offers asked about are never in state 4'),
    'f_fase': ('nenhuma oferta no estado 0xf está na fase 2 quando é consultada', 'no offer in state 0xf is in phase 2 when it is asked about'),
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
fazer(N, 410, 410 + 3, 'as ofertas de contrato e a oferta de transferência fechada', 'the contract offers and the closed transfer offer',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'as ofertas de contrato e a oferta de transferência fechada', 'the contract offers and the closed transfer offer', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
