"""Documentacao da 176a leva. Uso: python docs.py"""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer, leia, cadeia

N = 176
R = [('recuperar_lesao', 'recover_injury', '0x62bd10', 'injury.cpp', 'thiscall', 'o dia de recuperação (abaixo de −1000 em x87 com −0.0035 e 2.5; senão o ritmo ou sorteio)', 'the recovery day (below −1000 in x87 with −0.0035 and 2.5; otherwise the pace or a draw)'), ('pessoa_na_lista_de_transferencias', 'person_in_transfer_list', '0x8c1870', 'transfer_manager.cpp', 'thiscall', 'a pessoa está na lista +0x92c?', 'is the person in list +0x92c?'), ('punicao_da_tabela', 'table_penalty', '0x55cae0', 'discipline.cpp', 'thiscall', 'a punição da linha e da coluna da tabela, se o tipo bater', 'the penalty at the table row and column, if the type matches')]
ESCAPADAS = {  # nome: (explicação pt, explicação en)
    't_item': ('a lista +0x92c do gerente de transferências nunca tem a pessoa procurada na carreira automática (a rotina devolve sempre 0); vai para a carreira ampla', 'the transfer manager list +0x92c never holds the person looked up in the automatic career (the routine always returns 0); goes to the broad career'),
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
fazer(N, 401, 401 + 3, 'a recuperação da lesão, a pessoa na lista de transferências e a punição da disciplina', 'injury recovery, the person in the transfer list and the discipline penalty',
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, d, _ in R],
      [f"| `{a}()` | {e} | {c} | {f}: {d} |" for a, _, e, f, c, _, d in R],
      M, ME, [(a, g) for a, g, *_ in R], "'posicao_no_conteiner': 'position_in_container',", secao=('Simulação', 'Simulation'))
leia(N, 'a recuperação da lesão, a pessoa na lista de transferências e a punição da disciplina', 'injury recovery, the person in the transfer list and the discipline penalty', [(a, e, f + ', ' + c, dp, de) for a, _, e, f, c, dp, de in R], tot, ac, lac_pt, lac_en)
