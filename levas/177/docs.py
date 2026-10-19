"""Documentacao da 177a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 177
R = [('bits_do_registro', 'record_bits', '0x7a7eb0', 'player_regen.cpp', 'stdcall', 'os bits 4..5 do byte +0xb do registro de 79 bytes da pessoa', "bits 4..5 of byte +0xb of the person's 79-byte record"), ('competicao_marcada', 'competition_flagged', '0x684790', 'league.cpp', 'thiscall', 'a competição tem +0xbe e a marca 2 de +0xd9?', 'does the competition have +0xbe and flag 2 of +0xd9?'), ('registro_da_liga', 'league_record', '0x6826d0', 'league.cpp', 'thiscall', 'o registro (59 bytes) da tabela +0xb1', 'the record (59 bytes) of table +0xb1')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'm_marca': ('nas ligas inglesas as competições com +0xbe ligado nunca têm os bits 1 e 2 de +0xd9 diferentes (falta de dados; vai para a carreira ampla)', 'in the English leagues the competitions with +0xbe set never have bits 1 and 2 of +0xd9 different (missing data; goes to the broad career)'),
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
fazer(N, 404, 404 + 3, 'o bit do contrato, a marca da competição e o registro do clube na liga', 'the contract bit, the competition flag and the club record in the league',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'o bit do contrato, a marca da competição e o registro do clube na liga', 'the contract bit, the competition flag and the club record in the league', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
