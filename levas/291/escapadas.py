# As escapadas da 291ª explicadas pelo código (para o ESCAPADAS do docs.py; formato da levas/284/docs.py).
ESCAPADAS = {
    'cqx9_ano_mais_dois': (
        'o ano da copa é a base do calendário da edição nova, não uma data: a virada (`ConChamp::start_new_season` '
        '0x4c1440) refaz as regras (`ConChamp::set_up_rules` 0x4c15f0 → a tabela de rodadas 0x4c0f00), e cada rodada '
        'guarda o dia do ano e o ano RELATIVO ao ano da copa — `cup_set_round_draw_date` (0x521e60) e '
        '`cup_set_round_match_date` (0x521eb0) gravam `ano − base` com `base` = copa +0x40, e quem lê soma o +0x40 de '
        'volta (ano absoluto = `cup.year()` + o deslocamento da rodada). Subir o ano move a edição inteira junto, sem '
        'mudar o calendário relativo, e a primeira data da edição nova é o sorteio de 1 de julho (0x4c0fbe) e o primeiro '
        'jogo em 28 de agosto (0x4c0fd9) do ano da copa: com +1 (1/7/2002) ou com +2 (1/7/2003) nada dela acontece até '
        '2002-06-01. A virada roda uma vez só na carreira (contador `cqx_9 concacaf champions cup turnover`: 1, no dia '
        '2002-051) e o ano em si não consome sorteio (provado pelo código)',
        "the cup year is the base of the new edition's calendar, not a date: the turnover (`ConChamp::start_new_season` "
        '0x4c1440) redoes the rules (`ConChamp::set_up_rules` 0x4c15f0 → the round table 0x4c0f00), and each round keeps '
        'the day of the year and the year RELATIVE to the cup year — `cup_set_round_draw_date` (0x521e60) and '
        '`cup_set_round_match_date` (0x521eb0) store `year − base` with `base` = cup +0x40, and the readers add +0x40 '
        'back (absolute year = `cup.year()` + the round offset). Moving the year moves the whole edition with it, '
        "without changing the relative calendar, and the new edition's first date is the draw of 1 July (0x4c0fbe) and "
        'the first match on 28 August (0x4c0fd9) of the cup year: with +1 (1/7/2002) or with +2 (1/7/2003) none of it '
        'happens before 2002-06-01. The turnover runs once in the career (counter `cqx_9 concacaf champions cup '
        'turnover`: 1, on day 2002-051) and the year itself draws no random numbers (proved from the code)'),
    'cqx12_semifinalistas_trocados': (
        'o par de semifinalistas só vai para a tabela de 50 vagas da semeadura da UEFA (+0x440, `uefa_seeding::CUP_FINALS`) '
        'e o único leitor dela no exe é `UefaSeeding::assign_uefa_cup_places` (0x904580), chamado de um lugar só '
        '(0x90376a, dentro de `UefaSeeding::update_season` 0x9035a0), que não faz nada enquanto o ano do jogo for igual ao '
        '`uefa_seeding::SEASON_YEAR`; os chamadores do `update_season` são os construtores e as viradas das quatro copas '
        'europeias (0x5837c0/0x584d60, 0x5cc850/0x5cccd0, 0x632fb0/0x633920, 0x9022e0/0x902ee0). Na carreira a semeadura '
        'vira uma vez só, na criação do jogo (contador `cqi_3 qualification rollover`: 1, no dia 2001-000), enquanto a '
        'gravação só acontece quando uma copa nacional encerra a temporada (`Cup::record_season_honours` 0x519a90 → '
        '`Cup::register_semi_final_losers_for_uefa` 0x519fc0), na primavera de 2002 (contador `cqx_12 semi-final losers`, '
        'entre 2002-121 e 2002-141): o que a mutação escreve fica guardado '
        'para as vagas da temporada seguinte e não é lido até 2002-06-01 (o outro uso da tabela é a gravação do '
        '`uefa_seeding.tmp`, 0x905510, que não volta para a simulação) (provado pelo código)',
        'the pair of semi-finalists only goes into the 50-slot table of the UEFA seeding (+0x440, '
        '`uefa_seeding::CUP_FINALS`) and its only reader in the exe is `UefaSeeding::assign_uefa_cup_places` (0x904580), '
        'called from a single place (0x90376a, inside `UefaSeeding::update_season` 0x9035a0), which does nothing while the '
        "game year equals `uefa_seeding::SEASON_YEAR`; `update_season`'s callers are the constructors and the turnovers of "
        'the four European cups (0x5837c0/0x584d60, 0x5cc850/0x5cccd0, 0x632fb0/0x633920, 0x9022e0/0x902ee0). In the career '
        'the seeding rolls over only once, at game creation (counter `cqi_3 qualification rollover`: 1, on day 2001-000), '
        'while the write only happens when a national cup closes its season (`Cup::record_season_honours` 0x519a90 → '
        '`Cup::register_semi_final_losers_for_uefa` 0x519fc0), in the spring of 2002 (counter `cqx_12 semi-final losers`, '
        'between days 2002-121 and 2002-141): what the mutation writes is kept for '
        "the next season's places and is not read before 2002-06-01 (the table's other use is writing `uefa_seeding.tmp`, "
        '0x905510, which does not flow back into the simulation) (proved from the code)'),
    'cqx14_vitoria_vale_1': (
        'a minitabela é um objeto de rascunho e, na carreira, nasce vazia: `WcEuropeLeague` (0x91a240) monta a SubLeague '
        'com os 4 primeiros de um grupo, preenchida só com os jogos daquela competição (0x88c9f0 com o id do grupo), copia '
        'a linha do segundo colocado (0x3b bytes, 0x91a3fa) para a tabela dos segundos colocados e apaga a minitabela logo '
        'depois (destrutor em 0x91a440, `operator delete` em 0x91a447); ela não sorteia nada. Todas as chamadas da carreira '
        '(contador `cqx_14 sub league row`, 32 ou mais) caem todas num dia só, 2002-067, na montagem das eliminatórias '
        'europeias da Copa do Mundo, cujas rodadas só começam em 25 de março (`WcEuropeLeague::set_up_rules` 0x9191e0: dia 0x19 do '
        'mês 2, +0x3c = 2 turnos, 3 pontos a vitória): nenhum jogo dessa competição foi disputado ainda, as linhas saem '
        'todas zeradas e os pontos por vitória não entram em conta; e a tabela dos segundos só decide vagas quando as '
        'eliminatórias acabam, bem depois de 2002-06-01 (provado pelo código)',
        'the mini table is a scratch object and, in the career, is born empty: `WcEuropeLeague` (0x91a240) builds the '
        "SubLeague with the top 4 of a group, filled only from that competition's fixtures (0x88c9f0 with the group id), "
        "copies the runner-up's row (0x3b bytes, 0x91a3fa) into the runners-up table and deletes the mini table right after "
        '(destructor at 0x91a440, `operator delete` at 0x91a447); it draws no random numbers. All the calls in the career '
        '(counter `cqx_14 sub league row`, 32 or more) fall on a single day, 2002-067, when the European World Cup '
        'qualifiers are set up, and their rounds only start on 25 March (`WcEuropeLeague::set_up_rules` 0x9191e0: day 0x19 of month 2, '
        '+0x3c = 2 legs, 3 points a win): no fixture of that competition has been played yet, every row comes out at zero '
        'and the points per win never count; and the runners-up table only decides places when the qualifiers end, well '
        'after 2002-06-01 (proved from the code)'),
}
