# As escapadas da 295a leva (explicadas pelo codigo e pelo diario da base, copias/bancada2/jogo_testes/dias_base.txt).
#
# As quatro `virada_ano_mais_dois` que escaparam tem a mesma explicacao: a virada das copas irlandesas roda uma vez
# so, no dia da entrada de temporada da Irlanda (`key_nation_record::SEASON_END_DAY`, +0x15 das entradas de 72 bytes
# de `G_LEAGUE_TABLE`), e esse dia e 2002-212, o ultimo que a cadeia processa (ATE = 2002-08-01). Depois dele nada
# mais le o ano novo da copa.
_VIRADA_PT = (
    'o argumento trocado e o passo do ano da virada: `cup::YEAR` (+0x40) sobe 2 em vez de 1. O unico leitor de +0x40 '
    'na simulacao e a atualizacao diaria da copa (`Cup::update_for_date` 0x51f890): ela monta a data de cada rodada '
    'com o dia (+0 do registro de 0x68 bytes) e o byte de ano (+2) somado a +0x40 e, enquanto a data nao chegou '
    '(`ano > hoje.ano || (ano == hoje.ano && dia > hoje.dia)`), devolve 0 sem sortear (`exe::cup_draw_round` '
    '0x51c410) nem montar a rodada; os outros usos de +0x40 sao a gravacao e a leitura do `.tmp` da competicao e os '
    'rotulos de texto, que nao voltam para a simulacao. A virada em si roda uma vez so, pela passada das temporadas '
    '(`game_season_update_pass` 0x5d5ae0 -> `season_pass_restart_comp` -> `comp_vtable::START_NEW_SEASON`, que so '
    'recomeca a competicao completa em todos os estagios), no dia da entrada de temporada da Irlanda, e esse dia e '
    '2002-212 (31/7/2002), o ultimo que a cadeia processa: o contador "{ramo}" marcou 1 na base, e as tres mutacoes '
    'do mesmo tipo que a carreira acusou (mwa2, mwa3 e mwa13) divergem do diario da base exatamente em 2002-212 e em '
    'nenhum dia antes. '
)
_VIRADA_EN = (
    "the swapped argument is the year step of the turnover: `cup::YEAR` (+0x40) goes up by 2 instead of 1. The only "
    "reader of +0x40 in the simulation is the cup's daily update (`Cup::update_for_date` 0x51f890): it builds each "
    "round's date from the day (+0 of the 0x68-byte record) and the year byte (+2) added to +0x40 and, while the date "
    "has not arrived (`year > today.year || (year == today.year && day > today.day)`), returns 0 without drawing "
    "(`exe::cup_draw_round` 0x51c410) and without setting the round up; the other uses of +0x40 are writing and "
    "reading the competition's `.tmp` and the text labels, which do not flow back into the simulation. The turnover "
    "itself runs only once, through the season pass (`game_season_update_pass` 0x5d5ae0 -> `season_pass_restart_comp` "
    "-> `comp_vtable::START_NEW_SEASON`, which only restarts a competition complete in every stage), on the day of "
    "the Irish season entry, and that day is 2002-212 (31 July 2002), the last the chain processes: the counter "
    '"{ramo}" read 1 in the base, and the three mutations of the same kind the career did catch (mwa2, mwa3 and '
    "mwa13) diverge from the base's diary exactly on 2002-212 and on no earlier day. "
)

ESCAPADAS = {
    'mwa1_virada_ano_mais_dois': (
        _VIRADA_PT.format(ramo='mwa_1 irish challenge cup turnover') +
        'Na Copa da Irlanda a edicao nova so tem evento no fim do ano: a rodada 0 e sorteada em 1/11 e jogada em '
        '28/11 (`IRISH_CHALLENGE_CUP_ROUNDS`), isto e 1/11/2002 na base e 1/11/2003 com a mutacao, tres meses ou '
        'quinze meses depois do ultimo dia da janela; a comparacao (ate 2001-08-20) nao chega a virada, e o '
        'comparador da rotina (`compare_cup_new_season`, que confere o ano antes e depois) pegaria a troca se a '
        'janela da comparacao passasse de 2002-212 (provado pelo codigo)',
        _VIRADA_EN.format(ramo='mwa_1 irish challenge cup turnover') +
        "In the Irish Challenge Cup the new edition's first event is at the end of the year: round 0 is drawn on 1 "
        'November and played on 28 November (`IRISH_CHALLENGE_CUP_ROUNDS`), that is 1 November 2002 in the base and 1 '
        'November 2003 with the mutation, three or fifteen months past the last day of the window; the comparison (to '
        "2001-08-20) does not reach the turnover, and the routine's comparer (`compare_cup_new_season`, which checks "
        'the year before and after) would catch the swap if the comparison window went past 2002-212 (proved from the '
        'code)'),
    'mwa4_virada_ano_mais_dois': (
        _VIRADA_PT.format(ramo='mwa_4 presidents cup turnover') +
        'A Copa do Presidente tem uma rodada so, sorteada em 5/7 e jogada em 4/8 (`IRISH_PRESIDENTS_CUP_ROUNDS`): o '
        'jogo da edicao nova cai em 1 a 7 de agosto de 2002 (dia 4 ajustado para o dia da semana 4, que move no '
        'maximo tres dias para cada lado), depois do ultimo dia da janela (2002-212), e o sorteio, com a data ja '
        'passada quando a tabela e refeita, so poderia sair na atualizacao diaria do dia seguinte, 2002-213 '
        '(1/8/2002), que a carreira nao alcanca. Medido: o sorteio nao roda dentro da janela, porque o sorteio dessa '
        'rodada (regra de confronto 3, sem 0x400 nem 0x200) cai em `Cup::randomize_round_teams` (0x51e220), que '
        'aloca a lista temporaria em 0x51e262 e chama `utils_random` (0x90cfc0) em 0x51e495 uma vez por confronto — '
        'e o diario da base em 2002-212 tem exatamente os mesmos sorteios (3132438070) e o mesmo estado do alocador '
        '(pointer=0xabdc20, offset=0x75bd) da rodada com a mutacao (provado pelo codigo)',
        _VIRADA_EN.format(ramo='mwa_4 presidents cup turnover') +
        "The President's Cup has a single round, drawn on 5 July and played on 4 August "
        "(`IRISH_PRESIDENTS_CUP_ROUNDS`): the new edition's match falls on 1 to 7 August 2002 (day 4 snapped to "
        'weekday 4, which moves it at most three days either way), after the last day of the window (2002-212), and '
        'the draw, whose date is already past when the table is rebuilt, could only come out on the next day\'s '
        'update, 2002-213 (1 August 2002), which the career never reaches. Measured: the draw does not run inside the '
        'window, because the draw of that round (tie rule 3, with neither 0x400 nor 0x200) goes to '
        '`Cup::randomize_round_teams` (0x51e220), which allocates its temporary list at 0x51e262 and calls '
        '`utils_random` (0x90cfc0) at 0x51e495 once per tie — and the base diary for 2002-212 has exactly the same '
        'draw count (3132438070) and the same allocator state (pointer=0xabdc20, offset=0x75bd) as the mutated run '
        '(proved from the code)'),
    'mwa5_virada_ano_mais_dois': (
        _VIRADA_PT.format(ramo='mwa_5 northern irish charity shield turnover') +
        'A Supercopa da Irlanda do Norte tambem tem uma rodada so, com data de sorteio 5/7 e jogo em 5/8 '
        '(`NORTHERN_IRISH_CHARITY_SHIELD_ROUNDS`): o jogo da edicao nova cai em 2 a 8 de agosto de 2002 (dia 5 '
        'ajustado para o dia da semana 5), depois do ultimo dia da janela, e a rodada nem e sorteada, porque com a '
        'regra de confronto 0x200 `Cup::draw_round` desvia o sorteio em 0x51c5e4 e pula para 0x51c5f6 (so os mandos '
        'de campo, 0x51e870) — e isso tambem so poderia acontecer na atualizacao diaria de 2002-213, que a carreira '
        'nao alcanca. Medido: o diario da base em 2002-212 e igual ao da rodada com a mutacao em sorteios, alocador '
        'e impressoes (provado pelo codigo)',
        _VIRADA_EN.format(ramo='mwa_5 northern irish charity shield turnover') +
        "The Northern Irish Charity Shield also has a single round, drawn on 5 July and played on 5 August "
        "(`NORTHERN_IRISH_CHARITY_SHIELD_ROUNDS`): the new edition's match falls on 2 to 8 August 2002 (day 5 snapped "
        'to weekday 5), after the last day of the window, and the round is not even drawn, because with tie rule '
        '0x200 `Cup::draw_round` skips the draw at 0x51c5e4 and jumps to 0x51c5f6 (only the home sides, 0x51e870) — '
        'and that too could only happen in the daily update of 2002-213, which the career never reaches. Measured: '
        "the base diary for 2002-212 matches the mutated run's in draws, allocator and fingerprints (proved from the "
        'code)'),
    'mwa6_virada_ano_mais_dois': (
        _VIRADA_PT.format(ramo='mwa_6 irish cup turnover') +
        'Na Copa da Irlanda do Norte a edicao nova so comeca no inverno: a rodada 0 da tabela e sorteada em 10/12 e '
        'jogada em 20/1 do ano seguinte (`NORTHERN_IRISH_CUP_ROUNDS`), isto e 10/12/2002 e 20/1/2003 na base, quatro '
        'e seis meses depois do ultimo dia da janela (e um ano mais com a mutacao); o comparador da rotina pegaria a '
        'troca se a comparacao passasse de 2002-212 (provado pelo codigo)',
        _VIRADA_EN.format(ramo='mwa_6 irish cup turnover') +
        "In the Northern Irish Cup the new edition only starts in the winter: round 0 of the table is drawn on 10 "
        'December and played on 20 January of the next year (`NORTHERN_IRISH_CUP_ROUNDS`), that is 10 December 2002 '
        'and 20 January 2003 in the base, four and six months past the last day of the window (and a year later with '
        "the mutation); the routine's comparer would catch the swap if the comparison went past 2002-212 (proved from "
        'the code)'),
    'mwa13_sem_esperar_a_premier': (
        'a espera tirada (`advance_irish_first_stage`, `NirFirst::advance_stage` 0x78bfd0) so adia a fase seguinte da '
        'First Division: quando a propria fase da First esta completa, a atualizacao diaria chama `advance_stage`, '
        'que pergunta a Premier (`comp_vtable::CHECK_COMPLETE`(0, 1)) e, enquanto ela nao acabou, volta sem fazer '
        'nada; sem a espera a fase sobe no primeiro pedido e a repescagem do acesso '
        '(`NirFirst::create_playoff_stage` 0x78c0a0) nasce nesse dia. Na carreira da cadeia os dois dias sao o mesmo: '
        'o contador "mwa_13 irish first division waits for the premier" marcou 1, e a repescagem foi criada no mesmo '
        'dia com e sem a espera — o teste tem alcance para ver isso, porque a outra mutacao da mesma rotina '
        '(`mwa13_virada_ano_mais_dois`) foi acusada so pela impressao das partidas, isto e, basta a data de um jogo '
        'marcado mudar para a carreira divergir, e os jogos da repescagem mudariam de data se ela nascesse em outro '
        'dia. Isto e: na temporada 2001/02 da carreira a Premier da Irlanda do Norte ja estava completa na primeira '
        'vez que a First perguntou, e a espera so muda o resultado numa temporada em que a First acabe antes da '
        'Premier. O comparador da rotina (`compare_irish_first_advance_stage`) nao pode acusar: so a original roda '
        'nele (provado pelo codigo)',
        'the removed wait (`advance_irish_first_stage`, `NirFirst::advance_stage` 0x78bfd0) only delays the First '
        "Division's next stage: once the First's own stage is complete the daily update calls `advance_stage`, which "
        'asks the Premier (`comp_vtable::CHECK_COMPLETE`(0, 1)) and, while it is not over, returns doing nothing; '
        'without the wait the stage goes up at the first request and the promotion play-off '
        '(`NirFirst::create_playoff_stage` 0x78c0a0) is born on that day. In the chain\'s career both days are the '
        'same: the counter "mwa_13 irish first division waits for the premier" read 1, and the play-off was created '
        'on the same day with and without the wait — the test has the reach to see that, because the other mutation '
        'of the same routine (`mwa13_virada_ano_mais_dois`) was caught by the fixtures fingerprint alone, that is, a '
        "scheduled match's date is enough to make the career diverge, and the play-off's matches would change date if "
        'it were born on another day. In other words: in the career\'s 2001/02 season the Northern Irish Premier '
        'Division was already complete the first time the First Division asked, and the wait only changes the result '
        "in a season where the First finishes before the Premier. The routine's comparer "
        '(`compare_irish_first_advance_stage`) cannot catch it: only the original runs in it (proved from the code)'),
}  # nome: (explicacao pt, explicacao en)
