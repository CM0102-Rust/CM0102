# As escapadas da 300a leva (explicadas pelo codigo, pelo diario da base (jogo_testes/dias_base.txt) e pelos
# contadores de ramo da linha "ramos na base" de levas/300/cadeia.txt).
#
# A leva rodou com a confirmacao desligada (`cadeia.sh 300 -`), entao "escapou" quer dizer que a carreira nao mudou;
# a comparacao nem foi tentada. O jogo real desta leva compara ENG BEL HOL GER DEN ate 2002-08-01 (levas/300/ATE),
# isto e, a janela inteira.
#
# A janela: `CM_PARAR_EM` e (2002, 212); o dia do ano conta de 1 (2001-205 e 24/7/2001, o dia do save de ferias), de
# modo que o ultimo dia que a cadeia processa e 31/7/2002 e 1/8/2002 nao e simulado. As duas escapadas de virada sao do tipo
# "a virada da copa pula um ano", o mesmo padrao ja provado na 295a, 297a e 298a leva: o ano novo da copa so e lido
# quando a primeira rodada da edicao seguinte chega, e na Copa da Belgica e na da Dinamarca esse sorteio cai em agosto,
# depois do fim da janela.
_VIRADA_PT = (
    'o argumento trocado e o passo do ano da virada (`usa_mls_all_stars_cup_start_new_season`: o ano `cup::YEAR` '
    '(+0x40) sobe 2 em vez de 1). O unico leitor de +0x40 na simulacao e a atualizacao diaria da copa '
    '(`Cup::update_for_date` 0x51f890): ela monta a data de cada rodada com o dia do ano (`cup_round::DAY`, +0x3 do '
    'registro de 0x68 bytes) e o deslocamento de ano (`cup_round::YEAR_OFFSET`, +0x5) somado a +0x40 e, enquanto a '
    'data nao chegou (`ano > hoje.ano || (ano == hoje.ano && dia > hoje.dia)`), devolve 0 sem sortear '
    '(`exe::cup_draw_round` 0x51c410) nem montar a rodada; os outros usos de +0x40 sao a gravacao e a leitura do '
    '`.tmp` da competicao e os rotulos de texto, que nao voltam para a simulacao. A virada roda uma vez so, pela '
    'passada das temporadas de 2002 (`game_season_update_pass` 0x5d5ae0 -> `season_pass_restart_comp` -> '
    '`comp_vtable::START_NEW_SEASON`): o contador "{ramo}" marcou 1 na base. Logo depois dela o que e refeito no '
    'mesmo dia (os inscritos, a tabela de rodadas e a arvore dos confrontos) nao le o ano: a tabela de rodadas grava '
    'so o dia do ano e o deslocamento de ano, e e a atualizacao diaria que soma +0x40. '
)
_VIRADA_EN = (
    'the swapped argument is the year step of the turnover (`usa_mls_all_stars_cup_start_new_season`: `cup::YEAR` '
    '(+0x40) goes up by 2 instead of 1). The only reader of +0x40 in the simulation is the cup\'s daily update '
    '(`Cup::update_for_date` 0x51f890): it builds each round\'s date from the day of the year (`cup_round::DAY`, +0x3 '
    'of the 0x68-byte record) and the year offset (`cup_round::YEAR_OFFSET`, +0x5) added to +0x40 and, while the date '
    'has not arrived (`year > today.year || (year == today.year && day > today.day)`), returns 0 without drawing '
    '(`exe::cup_draw_round` 0x51c410) or building the round; the other uses of +0x40 are the writing and reading of '
    "the competition's `.tmp` and the text labels, which do not flow back into the simulation. The turnover runs only "
    'once, in the 2002 season pass (`game_season_update_pass` 0x5d5ae0 -> `season_pass_restart_comp` -> '
    '`comp_vtable::START_NEW_SEASON`): the counter "{ramo}" read 1 in the base. What is rebuilt right after it on the '
    'same day (the entrants, the round table and the tie tree) does not read the year: the round table stores only '
    'the day of the year and the year offset, and it is the daily update that adds +0x40. '
)
_CONTRASTE_PT = (
    'O contraste mede a discriminacao do teste: a mesma troca na Copa da Alemanha (`mwc14_gercup_ano_mais_2`, a mesma '
    'rotina com o mesmo argumento), cuja rodada 0 (`GER_CUP_LATER_ROUNDS`, codigo 10) e sorteada em 10/7 e jogada em '
    '26/8, foi acusada em 2002-194 (14/7/2002)'
)
_CONTRASTE_EN = (
    "The contrast measures the test's reach: the same swap in the German Cup (`mwc14_gercup_ano_mais_2`, the same "
    'routine with the same argument), whose round 0 (`GER_CUP_LATER_ROUNDS`, code 10) is drawn on 10 July and played '
    'on 26 August, was caught on 2002-194 (14 July 2002)'
)
ESCAPADAS = {
    'mwc14_belfacup_ano_mais_2': (
        _VIRADA_PT.format(ramo='mwc_14 bel fa cup turnover') +
        'A Copa da Belgica so e sorteada em agosto: a rodada 0 da tabela (`BEL_FA_CUP_ROUNDS`, codigo 30) e sorteada '
        'em 6/8 e jogada em 12/8 do ano da copa, isto e 6/8/2002 na edicao nova (e 6/8/2003 com a mutacao), cinco '
        'dias depois do ultimo dia inteiro da carreira (31/7/2002); nenhuma das oito rodadas da edicao nova e '
        'sorteada dentro da janela. ' + _CONTRASTE_PT,
        _VIRADA_EN.format(ramo='mwc_14 bel fa cup turnover') +
        'The Belgian Cup is only drawn in August: round 0 of the table (`BEL_FA_CUP_ROUNDS`, code 30) is drawn on 6 '
        "August and played on 12 August of the cup's year, that is 6 August 2002 in the new edition (and 6 August "
        '2003 with the mutation), five days past the last full day of the career (31 July 2002); none of the new '
        "edition's eight rounds is drawn inside the window. " + _CONTRASTE_EN),
    'mwc14_dencup_ano_mais_2': (
        _VIRADA_PT.format(ramo='mwc_14 den cup turnover') +
        'A Copa da Dinamarca passa de raspao: a rodada 0 da tabela (`DEN_CUP_ROUNDS`, codigo 10) e sorteada em 1/8 e '
        'jogada em 12/8 do ano da copa, isto e 1/8/2002 na edicao nova (e 1/8/2003 com a mutacao), um dia depois do '
        'ultimo dia que a cadeia processa (31/7/2002 = 2002-212): o sorteio da edicao nova nao chega a acontecer, e a '
        'impressao da escapada e a mesma da base (5181706551 sorteios, partidas 33535/66583e9acda9849b, banco '
        '677d8bc468ae5ab0). Um dia a mais de carreira alcancaria o sorteio e bastaria para acusar a troca: foi so pela '
        'impressao das partidas que a carreira acusou '
        '`mwc15_belsuper_ano_mais_2` (a edicao nova da Supercopa belga, sorteada em 5/7/2002, cuja unica partida, em '
        '5/8/2002, nem chega a ser jogada). ' + _CONTRASTE_PT,
        _VIRADA_EN.format(ramo='mwc_14 den cup turnover') +
        'The Danish Cup gets through by a whisker: round 0 of the table (`DEN_CUP_ROUNDS`, code 10) is drawn on 1 '
        "August and played on 12 August of the cup's year, that is 1 August 2002 in the new edition (and 1 August "
        '2003 with the mutation), one day past the last day the chain processes (31 July 2002 = 2002-212): the new '
        "edition's draw never happens, and the escape's fingerprint is the base's (5181706551 draws, matches "
        '33535/66583e9acda9849b, bank 677d8bc468ae5ab0). One more day of career '
        'would reach the draw and would be enough to catch the swap: it was by the match fingerprint alone that the '
        'career caught `mwc15_belsuper_ano_mais_2` (the new edition of the Belgian Super Cup, drawn on 5 July 2002, '
        'whose single match, on 5 August 2002, is never even played). ' + _CONTRASTE_EN),
    'mwc17_belsuper_disciplina_inglesa': (
        'trocada por `mwc17_belsuper_banco_de_3` (a antiga punha a regra de disciplina inglesa '
        '(`discipline_rule::ENGLAND`, 9) no lugar da das copas belgas (`BELGIUM_CUP`, 5) no construtor, isto e em '
        '`cup::DISCIPLINE_INDEX` (+0x50). Esse byte e lido no lance de cartao da partida (0x556210 escolhe a entidade '
        'da competicao e entrega o cartao a ela) e na conferencia da escalacao (`is_player_banned` 0x55d270, o mesmo '
        'byte), ver `docs/MAPA_DISCIPLINA.md`; a Supercopa belga joga uma partida por edicao e so uma cai na janela '
        '(5/8/2001; a da edicao de 2002, em 5/8/2002, fica depois do fim), e os cartoes dessa partida unica, '
        'lancados na entidade inglesa em vez da das copas belgas, nao chegaram a mudar nenhuma suspensao da janela. '
        'Mesmo assim ela poderia discriminar - a mesma troca na Copa da Liga alema, que joga rodadas dentro da '
        'temporada, foi acusada em 2002-061 -, e o construtor e a rotina mais provada da leva: o comparador '
        '(`compare_nation_cup_ctor`) roda a original e confere o +0x50 dela contra a constante da copia, de modo que '
        'a comparacao desta leva pegaria a troca), acusada pela carreira em 2001-216 (5/8/2001, o dia da unica '
        'partida da Supercopa belga na janela) numa rodada propria (`trocas_prova.txt`)',
        'replaced by `mwc17_belsuper_banco_de_3` (the old one put the English discipline rule '
        '(`discipline_rule::ENGLAND`, 9) in place of the Belgian cups\' one (`BELGIUM_CUP`, 5) in the constructor, '
        'that is in `cup::DISCIPLINE_INDEX` (+0x50). That byte is read on a card in a match (0x556210 picks the '
        "competition's ruling body and hands the card to it) and on the squad check (`is_player_banned` 0x55d270, the "
        'same byte), see `docs/MAPA_DISCIPLINA.md`; the Belgian Super Cup plays one match per edition and only one '
        'falls inside the window (5 August 2001; the 2002 edition\'s, on 5 August 2002, is past the end), and that '
        "single match's cards, booked with the English body instead of the Belgian cups' one, did not change any "
        'suspension of the window. It could discriminate all the same - the same swap in the German League Cup, '
        'which plays rounds inside the season, was caught on 2002-061 -, and the constructor is the best proved '
        "routine of the batch: the comparer (`compare_nation_cup_ctor`) runs the original and checks its +0x50 "
        "against the copy's constant, so this batch's comparison would catch the swap), caught by the career on "
        "2001-216 (5 August 2001, the day of the Belgian Super Cup's only match in the window) in a run of its own "
        '(`trocas_prova.txt`)'),
    'mwc17_holsuper_disciplina_inglesa': (
        'trocada por `mwc17_holsuper_inscritos_belgas` (a antiga punha a regra inglesa no lugar da holandesa '
        '(`discipline_rule::HOLLAND`, 15) em `cup::DISCIPLINE_INDEX` (+0x50), com a mesma explicacao da belga: a '
        'Supercopa holandesa joga uma partida por edicao e so a de 8/8/2001 cai na janela (a da edicao de 2002 e em '
        '8/8/2002), e os cartoes dela na entidade inglesa nao mudaram nenhuma suspensao; o construtor da Supercopa '
        'holandesa nao escreve o banco (`bench: None`), de modo que a troca foi para o outro argumento do item, a '
        'rotina dos inscritos, que passa a ser a da Supercopa belga (`BelSuper::select_teams` 0x4212f0 em vez de '
        '`HolSuper::select_teams` 0x5f8c10: o campeao e o vencedor da copa da Belgica no lugar dos da Holanda)), '
        'acusada pela carreira em 2001-001, no dia em que a competicao e construida, numa rodada propria '
        '(`trocas_prova.txt`)',
        'replaced by `mwc17_holsuper_inscritos_belgas` (the old one put the English rule in place of the Dutch one '
        '(`discipline_rule::HOLLAND`, 15) in `cup::DISCIPLINE_INDEX` (+0x50), with the same explanation as the '
        'Belgian one: the Dutch Super Cup plays one match per edition and only the 8 August 2001 one falls inside '
        "the window (the 2002 edition's is on 8 August 2002), and its cards under the English body changed no "
        "suspension; the Dutch Super Cup's constructor does not write the bench (`bench: None`), so the swap went to "
        "the item's other argument, the entrants routine, which becomes the Belgian Super Cup's "
        '(`BelSuper::select_teams` 0x4212f0 instead of `HolSuper::select_teams` 0x5f8c10: the Belgian champion and '
        'cup winner in place of the Dutch ones)), caught by the career on 2001-001, the day the competition is '
        'built, in a run of its own (`trocas_prova.txt`)'),
}  # nome: (explicacao pt, explicacao en)
