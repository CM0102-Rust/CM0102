# As escapadas da 301a leva (explicadas pelo codigo e pelos contadores de ramo da linha "ramos na base" de
# levas/301/cadeia.txt).
#
# A leva rodou com a confirmacao desligada (`cadeia.sh 301 -`), entao "escapou" quer dizer que a carreira nao mudou;
# a comparacao nem foi tentada dentro da cadeia. O jogo real desta leva compara ENG POL TUR GRE HRV RUS ate
# 2002-08-01 (levas/301/ATE), isto e, a janela inteira, com a virada de temporada dentro dela.

ESCAPADAS = {
    'mwd8_semente_menor': (
        'a mutacao estreita a faixa da semente sorteada para os 32 inscritos da Segunda Divisao croata '
        '(`cro_cup_collect_entrants`, `CroCup::collect_entrants` 0x517bb0: `exe::utils_random(0x32)` passa a '
        '`exe::utils_random(0xa)`, semente 0..9 em vez de 0..49), gravada no byte de semente da entrada de 6 bytes '
        'da tabela de inscritos (`cup_team_entry::SEED_LEVEL`, +0x4 de `cup::TEAM_TABLE`). Nao muda a carreira por '
        'duas razoes somadas. Primeira: as duas faixas consomem exatamente um sorteio de `utils_random`, de modo que '
        'a sequencia do gerador fica identica -- a corrida da mutacao parou na mesma data com os mesmos '
        '5767398875 sorteios da base. Segunda: ninguem le esse byte nesta copa. O sorteio generico das rodadas '
        '(`Cup::draw_round` 0x51c410) indexa a tabela so em +0, o ponteiro do clube (`mov ebx, [esi+0xa7]` com '
        '`eax = i*3` e `cmp dword ptr [ebx + eax*2], 0`, e `mov ecx, [edi + eax]` nas chamadas de texto); nenhuma '
        'rotina portada le +0x4 (as unicas que o escrevem sao as competicoes europeias, que guardam ali a classe de '
        'sorteio, e a Supercopa da Espanha, que grava 0). O proprio original confirma: ele nao escreve +0x4 para os '
        '16 inscritos da Primeira Divisao croata, que ficam com o lixo do `crt_malloc` -- um campo de que o sorteio '
        'dependesse nao poderia ficar assim. A semente da copa croata e, portanto, escrita e nunca lida: a leitura '
        'esta na faixa das copas europeias, fora desta leva. '
        '**A comparacao, porem, acusa [medido, 16/11/2026]:** com o comparador corrigido (o cursor de sorteios do '
        '`utils_random` restaurado, `crate::rng::STATE_REGIONS`), as 32 sementes saem do mesmo ponto da tabela nas '
        'duas corridas, e a faixa estreitada faz as listas diferirem. Numa corrida propria na bancada 5 (ENG POL TUR '
        'GRE HRV RUS ate 2001-08-20), a base deu `engine: 10000000000 comparisons, 0 divergences` e a mutacao foi '
        'acusada em 25 s: `DIVERGENCE cro_cup_collect_entrants(...) returned 1 x 1 memory true list false '
        '(48 entries)`. Nao e lacuna: a carreira nao ve, a comparacao ve -- que e o esquema hibrido da cadeia.',
        'the mutation narrows the range of the seed drawn for the 32 Croatian Second Division entrants '
        '(`cro_cup_collect_entrants`, `CroCup::collect_entrants` 0x517bb0: `exe::utils_random(0x32)` becomes '
        '`exe::utils_random(0xa)`, a seed 0..9 instead of 0..49), written into the seed byte of the 6-byte entry of '
        'the entrant table (`cup_team_entry::SEED_LEVEL`, +0x4 of `cup::TEAM_TABLE`). It does not change the career '
        'for two reasons together. First, both ranges consume exactly one `utils_random` draw, so the generator '
        'stream is identical -- the mutation run stopped on the same date with the same 5767398875 draws as the '
        'base. Second, nothing reads that byte in this cup. The generic round draw (`Cup::draw_round` 0x51c410) '
        'indexes the table at +0 only, the club pointer (`mov ebx, [esi+0xa7]` with `eax = i*3` and '
        '`cmp dword ptr [ebx + eax*2], 0`, and `mov ecx, [edi + eax]` in the text calls); no ported routine reads '
        '+0x4 (the only ones that write it are the European competitions, which keep the draw class there, and the '
        'Spanish Super Cup, which writes 0). The original confirms it: it does not write +0x4 for the 16 Croatian '
        'First Division entrants, which keep the `crt_malloc` garbage -- a field the draw depended on could not be '
        'left like that. The Croatian cup seed is therefore written and never read: the reader lives in the European '
        'cups, outside this batch. '
        '**The comparison, however, does catch it [measured, 16/11/2026]:** with the comparer fixed (the '
        '`utils_random` draw cursor restored, `crate::rng::STATE_REGIONS`), the 32 seeds come from the same point of '
        'the table in both runs, and the narrowed range makes the lists differ. In a run of its own on bench 5 (ENG '
        'POL TUR GRE HRV RUS to 2001-08-20), the base gave `engine: 10000000000 comparisons, 0 divergences` and the '
        'mutation was caught in 25 s: `DIVERGENCE cro_cup_collect_entrants(...) returned 1 x 1 memory true list '
        'false (48 entries)`. Not a gap: the career does not see it, the comparison does -- which is the chain\'s '
        'hybrid scheme.'),
}  # nome: (explicacao pt, explicacao en)
