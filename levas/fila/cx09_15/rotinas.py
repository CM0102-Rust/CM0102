# A virada inglesa e a cadeia de acesso e rebaixamento da Football League (eng_prm.cpp; leitor cx09, rodada 5,
# 24/11/2026). Simulacao da Inglaterra com ATE = 2002-08-01: a virada roda uma vez, no meio de 2002 (cobertura do mundo:
# 0x574c10 = 1); nao precisa de outras ligas.
# - 0x574c10 EngPrm::start_new_season thiscall() ret 0 (virtual 8): +0x4c = 0; virtual 0xb0 (1); com a Conference
#   ([[0xadadfc] + 4 x [0x9cf69c]]) 0x5750a0, senao 0x574e60; League::free_table 0x687970 (0); free(+0xba) e zera;
#   Comp_Stats::archive_season_and_reset 0x4a1c50 (1); +0x40 + 1; virtual 0x8c (caixa 0x267), 0x574a20 (0x26e),
#   League::create_fixtures 0x6835c0 (0x274), League::sort_table 0x6827d0 (0) (0x27b); virtual 0x5c;
#   League::limit_club_division_tier 0x68a980 na Third (1, 3, -3, 1) e na Conference (2, 0, -3, 1); virtual 8 da First,
#   da Second, da Third e da Conference (se houver); League::set_club_season_expectations 0x68aa80 e
#   OfficialsManager::renew_officials 0x79cee0 ([0xb74340], +4). Prologo sub esp, 0x200 (6 bytes).
# - 0x5752e0 EngPrm::process_promotion_relegation_chain thiscall(x) ret 4 (virtual 0xb0): virtual 0xa4 da Premier
#   (this), First, Second e Third; league_promote_and_relegate 0x689c80 (this; Premier/First, First/Second,
#   Second/Third; 1, x, -1, -1); com a Conference: a virtual 0xa4 dela, o campeao (League::get_table_row 0x6826d0 (0)),
#   FinanceTable::get_club_finance 0x5a0590 e Finance::upgrade_stadium_for_league 0x599e00 (+0xe4, +0xe2 da Third, id
#   dela, 1; o original empilha esses argumentos antes das duas chamadas de dentro): apto, Third/Conference; senao a
#   ultima linha da Third +0x37 = 0xfe e Comp::send_promotion_relegation_override_news 0x48ece0 (Third; registro da
#   Conference, campeao, 0, 2). Prologo mov eax, [0xadadfc] (5 bytes).
# Fica de fora a 0x5752f0 (o "corpo" do mapa): e o meio desta rotina (push esi depois do prologo de 0x5752e0), sem
# referencia nenhuma; a rotina inteira e a 0x5752e0 (a vaga 0xb0 da vtable 0x969d1c).
# Comparadores: so a original (movem clubes e reconstroem as ligas); a virada confere o ano + 1.
# Iguais no cm0102-1080.exe. Sem dependencias (comp_by_global, comp_record_of_key, rebuild/expectations publicados).
SAVE = False
ATE = '2002-08-01'
ROTINAS = [('eng_prm_start_new_season', 0x574c10, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('eng_prm_process_promotion_relegation_chain', 0x5752e0, [0xa1, 0xfc, 0xad, 0xad, 0x00])]
DOCS = ('a virada inglesa e a cadeia de acesso e rebaixamento da Football League',
        'the English turnover and the Football League promotion and relegation chain',
        [('eng_prm_start_new_season', 'EngPrm::start_new_season', '0x574c10', 'eng_prm.cpp', 'thiscall, ret 0',
          'a virada inglesa, puxada pela Premier: a cadeia de acesso e rebaixamento, a troca com a Conference, a Premier remontada (ano seguinte, regras, tabela, calendário, ordem e posições previstas), o teto de divisão dos clubes da Third e da Conference, e as viradas da First, da Second, da Third e da Conference; as expectativas da diretoria e os árbitros',
          'the English turnover, led by the Premier: the promotion and relegation chain, the Conference exchange, the Premier rebuilt (next year, rules, table, fixtures, order and predicted positions), the division tier cap of the Third Division and Conference clubs, and the First, Second, Third and Conference turnovers; the board expectations and the referees'),
         ('eng_prm_process_promotion_relegation_chain', 'EngPrm::process_promotion_relegation_chain', '0x5752e0', 'eng_prm.cpp', 'thiscall, ret 4',
          'a cadeia da Football League: as posições finais das quatro divisões, as trocas Premier–First, First–Second e Second–Third; com a Conference carregada, o campeão dela só sobe se o estádio servir à Third, senão o último da Third fica',
          'the Football League chain: the final positions of the four divisions, the Premier–First, First–Second and Second–Third exchanges; with the Conference loaded, its champion only goes up if its ground suits the Third Division, otherwise the Third Division bottom club stays')])
CAMPOS = []
INTERESSA = []
