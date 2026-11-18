# As escapadas da 304a leva (itens mwe_1 a mwe_13: Suecia, Finlandia, Noruega e Portugal).
#
# A leva rodou com a confirmacao desligada (`bancada.py cadeia 4 304 -`), entao "escapou" quer dizer que a carreira
# nao mudou; a comparacao nem foi tentada dentro da cadeia. O jogo real desta leva compara ENG SWE FIN NOR POR ate
# 2002-08-01 (levas/304/ATE), a janela inteira, com a virada de temporada dentro dela.
#
# AS DUAS ESCAPADAS SAO A MESMA LACUNA, E ELA ALCANCA UMA TERCEIRA ROTINA NAO MUTADA. O molde
# `advance_first_division_stage` serve a Suecia (`swe_first_advance_stage` 0x88f310) e a Noruega
# (`nor_first_advance_stage`), e as duas usam o comparador `compare_first_division_advance`, que roda so a original e
# confere a transicao de fase; a Finlandia (`fin_first_advance_stage` 0x593f90) tem rotina propria com a mesma forma e
# um comparador com a mesma limitacao. Nenhum dos tres verifica o efeito da atualizacao da liga principal. A mutacao
# norueguesa nao existe nesta leva, de modo que a lacuna esta la sem ninguem ter apontado: quem for consertar deve dar
# o ensaio ao molde (e a rotina finlandesa), nao remendar as duas mutacoes que apareceram.
#
# O JOGO REAL DESTA LEVA TAMBEM ACHOU UM FALSO ALARME DO COMPARADOR, ja corrigido no codigo.rs:
# `fin_first_advance_stage(0x23192290) before=0 after=2`. A expectativa era `after == before + 1`, mas a rotina
# (original e porte, identicos) soma 1 e, quando a fase nova e 1 ou 3, chama o construtor da fase seguinte
# (0x594080 / 0x594610), que avanca de novo: de 0 o original para em 2. A invariante passou a ser "a fase subiu ao
# menos 1", que e mais fraca e verdadeira; a antiga era forte e falsa. A versao forte exige modelar o que aqueles dois
# construtores fazem com a fase, e cabe quando eles forem portados.

ESCAPADAS = {
    'mwe10_sem_atualizar_a_primeira': (
        'LACUNA MEDIDA, nao escapada explicada: a mutacao tira a chamada `update_premier_division_status(premier)` do '
        'avanco de fase da Primeira Divisao sueca (`advance_first_division_stage`, `SweFirst::advance_stage` 0x88f310), '
        'que chama o metodo virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS` da liga principal. **Nem a carreira '
        'nem a comparacao a acusam.** A carreira nao ve pelos dois fatos abaixo; a comparacao nao ve porque o '
        'comparador `compare_first_division_advance` roda **so a original** (a rotina avanca a fase e monta o '
        'mata-mata, efeitos que nao podem rodar duas vezes) e confere apenas a transicao de fase, sem olhar a tabela '
        'da liga principal.\n'
        'O que esta medido: a corrida mutante parou na mesma data com **o mesmo numero de sorteios da base** '
        '(4857694775), isto e, a chamada removida nao consome sorteio nenhum nesta carreira.\n'
        'O que e argumento e nao prova: a chamada so acontece depois de `premier_division_complete(premier)`, com a '
        'liga principal encerrada, e apenas recalcula posicoes e situacoes sobre uma tabela que nao muda mais -- '
        'seria idempotente, porque o proprio avanco de fase da liga principal ja as atualizou quando ela terminou. '
        'Falta provar que as linhas da tabela da principal sao identicas antes e depois da chamada nesse momento '
        '(uma conferencia barata, mas precisa de uma corrida que chegue ao fim da temporada sueca).\n'
        'Caminho para fechar, em ordem de preferencia: (1) provar a idempotencia pela conferencia acima; (2) dar ao '
        'comparador um ensaio que compare as linhas da liga principal em volta da chamada; (3) trocar a mutacao por '
        'uma que o comparador acuse. Enquanto nao fechar, entra em `PENDENCIAS.md` como lacuna medida.',
        'MEASURED GAP, not an explained escape: the mutation removes the call `update_premier_division_status(premier)` '
        'from the stage advance of the Swedish First Division (`advance_first_division_stage`, `SweFirst::advance_stage` '
        '0x88f310), which calls the league\'s virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS` on the Premier '
        'Division. **Neither the career nor the comparison catches it.** The career does not see it for the two facts '
        'below; the comparison does not see it because the comparer `compare_first_division_advance` runs **only the '
        'original** (the routine advances the stage and builds the play-off, effects that cannot run twice) and checks '
        'the stage transition alone, without looking at the Premier Division\'s table.\n'
        'What is measured: the mutation run stopped on the same date with **the same number of draws as the base** '
        '(4857694775), that is, the removed call consumes no draw at all in this career.\n'
        'What is an argument and not a proof: the call only happens after `premier_division_complete(premier)`, with '
        'the Premier Division finished, and it only recomputes positions and statuses over a table that no longer '
        'changes -- it would be idempotent, because the Premier Division\'s own stage advance already updated them '
        'when it ended. What is missing is to prove that the Premier Division\'s table rows are identical before and '
        'after the call at that moment (a cheap check, but it needs a run that reaches the end of the Swedish '
        'season).\n'
        'Ways to close it, in order of preference: (1) prove the idempotence with the check above; (2) give the '
        'comparer a dry run that compares the Premier Division\'s rows around the call; (3) replace the mutation with '
        'one the comparer does catch. Until it is closed, it goes into `PENDENCIAS.en.md` as a measured gap.'),
    'mwe11_sem_atualizar_a_veikkausliiga': (
        'LACUNA MEDIDA, a mesma de `mwe10_sem_atualizar_a_primeira` na rotina irma finlandesa: a mutacao tira a '
        'chamada do metodo virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS` da Veikkausliiga no avanco de fase da '
        'Ykkonen (`fin_first_advance_stage`, `FinFirst::advance_stage` 0x593f90, no `stage == 2`, depois de '
        '`comp_vtable::CHECK_COMPLETE` confirmar que a Veikkausliiga terminou). O comparador '
        '`compare_fin_first_advance_stage` tem a mesma forma do sueco: roda so a original e confere a transicao de '
        'fase, sem olhar a tabela da liga principal. Medido: a corrida mutante parou na mesma data com o mesmo numero '
        'de sorteios da base (4857694775). Fecha junto com a sueca, pelo mesmo caminho.',
        'MEASURED GAP, the same one as `mwe10_sem_atualizar_a_primeira` in the Finnish sibling routine: the mutation '
        'removes the call to the virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS` in the '
        'Ykkonen stage advance (`fin_first_advance_stage`, `FinFirst::advance_stage` 0x593f90, in `stage == 2`, '
        'after `comp_vtable::CHECK_COMPLETE` confirms the Veikkausliiga has finished). The comparer '
        '`compare_fin_first_advance_stage` has the same shape as the Swedish one: it runs only the original and checks '
        'the stage transition, without looking at the Premier Division table. Measured: the mutation run stopped '
        'on the same date with the same number of draws as the base (4857694775). It closes together with the Swedish '
        'one, by the same route.'),
}  # nome: (explicacao pt, explicacao en)
