# As escapadas da 298a leva (explicadas pelo codigo e pelos contadores de ramo da linha "ramos na base" de
# levas/298/cadeia.txt; os contadores temporarios do mata-mata da Copa Premier sao de uma rodada na bancada 5,
# copias/bancada5/esc298.log, registrada em levas/298/trocas_prova.txt).
#
# A leva rodou com a confirmacao desligada (`cadeia.sh 298 -`), entao "escapou" quer dizer que a carreira nao mudou;
# a comparacao nem foi tentada. O jogo real desta leva compara ENG SCO WAL ate 2002-08-01 (levas/298/ATE), isto e, a
# janela inteira.
ESCAPADAS = {
    'mwb20_premier_confronto_direto_primeiro': (
        'a mutacao gira a ordem dos quatro codigos de desempate da Copa Premier galesa (`league::TIEBREAKS`, +0xc5: '
        '1 saldo, 2 gols pro, 4 confronto direto viram 4, 1, 2), escritos uma vez por temporada por '
        '`wel_prm_cup_set_up_rules` (0x928160; o contador "mwb_20 welsh premier cup rules" marcou 2 na base, as duas '
        'temporadas da janela). Quem le esses bytes e so a ordenacao da tabela: `League::sort_table` (0x6827d0) '
        'escolhe o caminho (com o codigo 4 no primeiro lugar monta a minitabela do confronto direto, '
        '`league_rank_by_head_to_head`, ordena por `league_compare_table_rows_h2h` 0x683410 e passa a bolha; com ele '
        'depois, ordena so por `league_compare_table_rows` 0x683210) e o comparador de linhas aplica os codigos em '
        'ordem, mas so DEPOIS dos pontos (+0x19), que ele compara primeiro e que a mutacao nao toca. Nos dois grupos '
        'de quatro clubes da Copa Premier (6 jogos por clube) nenhum empate em pontos desta carreira mudou a ordem, e '
        'nada mais le os bytes: a montagem do mata-mata (`WelPrmCup::create_knockout_stage` 0x928650) le as linhas 0 '
        'e 1 ja ordenadas das duas tabelas (deslocamentos 0 e 0x3b), nao os codigos. A rotina nao fica sem prova: o '
        'comparador (`eng_prm_compare_league_rules`) roda a original e a copia lado a lado e confere byte a byte '
        'cada regra escrita, os quatro codigos de desempate inclusive, e a comparacao desta leva alcanca as duas '
        'chamadas (ENG SCO WAL ate 2002-08-01)',
        'the mutation rotates the four tie-break codes of the Welsh Premier Cup (`league::TIEBREAKS`, +0xc5: '
        '1 goal difference, 2 goals for, 4 head-to-head become 4, 1, 2), written once per season by '
        '`wel_prm_cup_set_up_rules` (0x928160; the counter "mwb_20 welsh premier cup rules" read 2 in the base, the '
        "window's two seasons). The only reader of those bytes is the table sort: `League::sort_table` (0x6827d0) "
        'picks the path (with code 4 first it builds the head-to-head mini table, `league_rank_by_head_to_head`, '
        'sorts with `league_compare_table_rows_h2h` 0x683410 and runs the bubble pass; with it later, it sorts only '
        'with `league_compare_table_rows` 0x683210) and the row comparer applies the codes in order, but only AFTER '
        'the points (+0x19), which it compares first and which the mutation does not touch. In the two four-club '
        "groups of the Premier Cup (6 games each) no points tie in this career changed the order, and nothing else "
        'reads the bytes: the knockout build (`WelPrmCup::create_knockout_stage` 0x928650) takes rows 0 and 1 of the '
        'two already sorted tables (offsets 0 and 0x3b), not the codes. The routine is not left unproved: the '
        'comparer (`eng_prm_compare_league_rules`) runs the original and the copy side by side and checks every rule '
        'byte written, the four tie-break codes included, and this batch\'s comparison reaches both calls (ENG SCO '
        'WAL to 2002-08-01)'),
    'mwb21_mata_mata_em_5_rodadas': (
        'trocada por `mwb21_mata_mata_na_fase_zero` (a antiga punha o peso do mata-mata da Copa Premier em 5 rodadas '
        'em vez de 3, isto e `comp_util_scale_by_round` 0x4bf850, (alto - baixo) x rodada / (rodadas - 1), dando 12 e '
        '25 em vez de 25 e 50 nas rodadas 1 e 2; o peso so chega aos pontos do tecnico '
        '(`Staff_Records::add_qualified_for_round_record` 0x86bdd0 e `add_knocked_out_record` 0x86c000: reputacao^2 x '
        'peso no ano, no mes e na temporada). O ramo roda dentro da janela - contadores temporarios na bancada 5: '
        '"esc298 prm ko 8", "esc298 prm ko round over 0 4" -, logo a mutacao muda mesmo os pontos e ainda assim a '
        'carreira nao mudou, e o comparador da rotina '
        '(`compare_wel_prm_cup_update_staff_records_for_status`) so roda a original e confere o retorno 0, de modo '
        'que nada a pegaria), acusada pela carreira em 2002-131 numa rodada propria (`trocas_prova.txt`)',
        'replaced by `mwb21_mata_mata_na_fase_zero` (the old one set the Premier Cup knockout weight over 5 rounds '
        'instead of 3, that is `comp_util_scale_by_round` 0x4bf850, (high - low) x round / (rounds - 1), giving 12 '
        'and 25 instead of 25 and 50 in rounds 1 and 2; the weight only reaches the manager points '
        '(`Staff_Records::add_qualified_for_round_record` 0x86bdd0 and `add_knocked_out_record` 0x86c000: '
        'reputation^2 x weight in the year, the month and the season). The branch does run inside the window - '
        'temporary counters on bench 5: "esc298 prm ko 8", "esc298 prm ko round over 0 4" -, so the mutation really '
        'does change the points and the career still did not move, and the routine\'s comparer '
        '(`compare_wel_prm_cup_update_staff_records_for_status`) only runs the original and checks the return 0, so '
        'nothing would catch it), caught by the career on 2002-131 in a run of its own (`trocas_prova.txt`)'),
}
