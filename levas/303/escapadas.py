# As escapadas da 303a leva (itens mwd_27 e mwd_28), explicadas pelo codigo.
#
# A leva rodou com a confirmacao desligada (`bancada.py cadeia 3 303 -`), entao "escapou" quer dizer que a carreira
# nao mudou; a comparacao nem foi tentada dentro da cadeia. O jogo real desta leva compara ENG POL TUR GRE HRV RUS
# ate 2002-08-01 (levas/303/ATE), a janela inteira, com a virada de temporada dentro dela.

ESCAPADAS = {
    'mwd27_classe_da_categoria_b': (
        'a mutacao troca a classe gravada para os 26 inscritos da categoria B da segunda divisao turca '
        '(`tur_cup_collect_entrants`, `TurCup::collect_entrants` 0x8fcb90: `set_cup_entry(this, k, lower[0x19 - k], 1)` '
        'passa a gravar 2). O `set_cup_entry` escreve o clube em +0, a situacao 0 em +5 e a classe no byte +0x4 da '
        'entrada de 6 bytes da tabela de inscritos (`cup_team_entry::SEED_LEVEL` de `cup::TEAM_TABLE`). Nao muda a '
        'carreira porque ninguem le esse byte nesta familia de copas: o sorteio generico das rodadas '
        '(`Cup::draw_round` 0x51c410) indexa a tabela so em +0, o ponteiro do clube (`mov ebx, [esi+0xa7]` com '
        '`eax = i*3` e `cmp dword ptr [ebx + eax*2], 0`, e `mov ecx, [edi + eax]` nas chamadas de texto), e as unicas '
        'rotinas portadas que leem +0x4 sao as das competicoes europeias, que guardam ali a classe de sorteio. '
        'Diferente da semente da copa croata (301a leva), aqui nao ha nem sorteio envolvido: a classe e uma '
        'constante, de modo que a sequencia do gerador fica identica. '
        '**A comparacao acusa:** o comparador `compare_tur_cup_collect_entrants` usa '
        '`compare_entrants_with_seeds` com o predicado `|_| true`, isto e, compara a classe de **todas** as 64 '
        'entradas, e a mutacao muda 26 delas. Nao e lacuna: a carreira nao ve, a comparacao ve.',
        'the mutation changes the class written for the 26 category B entrants of the Turkish second division '
        '(`tur_cup_collect_entrants`, `TurCup::collect_entrants` 0x8fcb90: `set_cup_entry(this, k, lower[0x19 - k], 1)` '
        'now writes 2). `set_cup_entry` writes the club at +0, status 0 at +5 and the class into byte +0x4 of the '
        '6-byte entry of the entrant table (`cup_team_entry::SEED_LEVEL` of `cup::TEAM_TABLE`). It does not change the '
        'career because nothing reads that byte in this family of cups: the generic round draw (`Cup::draw_round` '
        '0x51c410) indexes the table at +0 only, the club pointer (`mov ebx, [esi+0xa7]` with `eax = i*3` and '
        '`cmp dword ptr [ebx + eax*2], 0`, and `mov ecx, [edi + eax]` in the text calls), and the only ported routines '
        'that read +0x4 are those of the European competitions, which keep the draw class there. Unlike the Croatian '
        'cup seed (batch 301), there is not even a draw involved here: the class is a constant, so the generator '
        'stream stays identical. '
        '**The comparison does catch it:** the comparer `compare_tur_cup_collect_entrants` uses '
        '`compare_entrants_with_seeds` with the predicate `|_| true`, that is, it compares the class of **every** one '
        'of the 64 entries, and the mutation changes 26 of them. Not a gap: the career does not see it, the '
        'comparison does.'),
}  # nome: (explicacao pt, explicacao en)
