"""Documentacao da 189a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 189
R = [('e_competicao_do_amistoso', 'Friendly::is_own_comp_record', '0x5c2990', 'friendly.cpp', 'thiscall', 'diz se a competição é a dos amistosos', 'tells whether the competition is the friendlies one'), ('preencher_data_internacional', 'friendly_international_date_set', '0x5cb9b0', 'friendly.cpp', 'thiscall', 'preenche uma data internacional: dia, mês, duração, ano, continente e competição', 'fills an international date: day, month, span, year, continent and competition'), ('limpar_registro_de_tecnico', 'Staff_Records::clear_manager_record', '0x86fc20', 'staff_records.cpp', 'thiscall', 'zera o registro de técnico e põe −1 nos ids do dono, da liga e dos clubes', 'clears the manager record and sets the owner, league and club ids to −1')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    'r_ids': ('o id da última passagem (+0x72) de um registro de técnico limpo nunca é lido na carreira antes de ser regravado', 'the last spell id (+0x72) of a cleared manager record is never read in the career before being rewritten'),
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
fazer(N, 440, 440 + 3, 'a competição do amistoso, a data internacional e o registro de técnico', 'the friendly competition, the international date and the manager record',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'a competição do amistoso, a data internacional e o registro de técnico', 'the friendly competition, the international date and the manager record', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
