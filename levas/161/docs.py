"""Documentacao da 161a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 161
R = [('serve_para_a_vaga', 'shortlist_slot_fits', '0x844300', 'shortlist_manager.cpp', 'stdcall', 'o jogador serve para a vaga 0..15 da lista de observação (nota 15 no setor e no lado; a 15 pede contrato, id e a 0x600a50 em zero); devolve só `al`', 'the player fits shortlist slot 0..15 (rating 15 in sector and side; slot 15 needs a contract, an id and 0x600a50 at zero); returns only `al`'), ('codigo_da_posicao', 'position_code', '0x83bd00', 'shortlist_manager.cpp', 'stdcall', 'o código da posição pela máscara de setor e lado (28 combinações; o resto 0xe); devolve só `al`', 'the position code from the sector and side mask (28 combinations; the rest 0xe); returns only `al`'), ('criterio_da_comissao', 'staff_criterion', '0x866f60', 'staff_contracts.cpp', 'thiscall', 'o critério da comissão técnica: uma nota em 8 bits (id mod 7, notas, idade, reputação, número de funções) decide pelas funções se devolve 1', 'the staff criterion: an 8-bit score (id mod 7, ratings, age, reputation, number of roles) decides by the roles whether it returns 1')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    's_id': ('mudava só o limite exato do id (−16 para −17); refeita como `s_id_b` em `mutacoes/leva161b.py` e acusada (`mutacoes_refeitas.txt`)',
             'it only changed the exact id limit (−16 to −17); redone as `s_id_b` in `mutacoes/leva161b.py` and caught (`mutacoes_refeitas.txt`)'),
    'c_rep': ('mudava só a reputação exata 0x1c53; refeita como `c_rep_b` e acusada',
              'it only changed the exact reputation 0x1c53; redone as `c_rep_b` and caught'),
    'c_idade': ('nem refeita com +15 (`c_idade_b`) é acusada: nenhuma pessoa da comissão tem +0x44 entre 51 e 75 nos dados',
                'not even redone with +15 (`c_idade_b`) is it caught: no staff member has +0x44 between 51 and 75 in the data'),
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
fazer(N, 356, 356 + 3, 'lista de observação, código da posição e critério da comissão', 'shortlist slot, position code and staff criterion',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'lista de observação, código da posição e critério da comissão', 'shortlist slot, position code and staff criterion', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
