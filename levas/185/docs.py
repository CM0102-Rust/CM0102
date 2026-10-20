"""Documentacao da 185a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 185
R = [('marcar_contrato_acertado', 'set_agreed_contract_flag', '0x8c5410', 'transfer_manager.cpp', 'thiscall', 'liga a marca de contrato acertado (0x40) da pessoa no gerente de transferências', "sets the person's agreed-contract flag (0x40) in the transfer manager"), ('desmarcar_contrato_acertado', 'clear_agreed_contract_flag', '0x8c5430', 'transfer_manager.cpp', 'thiscall', 'desliga a marca de contrato acertado da pessoa', "clears the person's agreed-contract flag"), ('vaga_da_oferta', 'offer_slot', '0x8c61e0', 'transfer_manager.cpp', 'thiscall', 'o endereço da vaga de uma oferta (fora da faixa, o começo da tabela)', 'the address of an offer slot (out of range, the table start)')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    't_desmarca': ('quando a marca de contrato acertado é desligada, a pessoa nunca tem também a marca 0x80', 'when the agreed-contract flag is cleared, the person never also has flag 0x80'),
    't_vaga': ('a carreira nunca pede a vaga de oferta igual à quantidade (fora da faixa)', 'the career never asks for the offer slot equal to the count (out of range)'),
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
fazer(N, 428, 428 + 3, 'a marca de contrato acertado e a vaga da oferta', 'the agreed-contract flag and the offer slot',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'a marca de contrato acertado e a vaga da oferta', 'the agreed-contract flag and the offer slot', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
