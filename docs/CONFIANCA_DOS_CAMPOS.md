*[English version](CONFIANCA_DOS_CAMPOS.en.md).*

# Confiança dos nomes dos campos

Gerado por `engenharia-reversa/confianca_campos.py` a partir do `rust/src/motor/fields.rs`. O nível vem da
origem do nome, que o comentário de cada campo já diz:

- **comunidade** (`// 2 byte(s), UInt16`): os leiautes dos editores da comunidade (docs/FORMATOS.md), anos de uso;
- **conferido** (`[verified: como]`): confirmado por uma segunda fonte independente (outra leitura do original, o mapa
  de funções da comunidade, uma observação no jogo);
- **lido** (descrição sem marca): proposto pela leitura do código original, com a evidência na descrição;
- **inferido** (`[inference]`): proposto pela leitura, um palpite plausível.

A largura de cada acesso foi conferida contra a instrução original (rodada 28, `larguras_campos.py --original`);
o nível aqui é o do **nome** (o significado), não o da largura. Cada método do `records.rs` traz o nível no comentário.

## Totais

| Nível | Campos |
|---|---|
| comunidade | 247 |
| conferido | 165 |
| lido | 1545 |
| inferido | 163 |

## Por registro

| Registro | comunidade | conferido | lido | inferido |
|---|---|---|---|---|
| `african_nations` | 0 | 0 | 2 | 0 |
| `age_record` | 0 | 0 | 2 | 0 |
| `appearance_clause` | 0 | 0 | 3 | 0 |
| `area_fields` | 0 | 6 | 14 | 0 |
| `asia_club_champ_fields` | 0 | 1 | 0 | 0 |
| `award_fields` | 0 | 3 | 0 | 0 |
| `awol_fields` | 0 | 0 | 0 | 1 |
| `awol_fitness_incident` | 0 | 0 | 0 | 1 |
| `awol_missed_training` | 0 | 0 | 1 | 0 |
| `ban_appeal` | 0 | 0 | 8 | 0 |
| `ban_record` | 0 | 1 | 7 | 0 |
| `ban_slot` | 0 | 0 | 1 | 1 |
| `board_record` | 0 | 1 | 16 | 5 |
| `bra_nat_third` | 0 | 0 | 1 | 0 |
| `call_up_entry` | 0 | 0 | 1 | 0 |
| `career_entry` | 0 | 0 | 8 | 0 |
| `city` | 6 | 0 | 0 | 0 |
| `club` | 43 | 0 | 2 | 0 |
| `club_discipline_record` | 0 | 0 | 3 | 0 |
| `club_history_fields` | 0 | 1 | 0 | 0 |
| `club_match_record` | 0 | 4 | 5 | 0 |
| `club_record` | 0 | 0 | 25 | 1 |
| `club_record_entry` | 0 | 0 | 3 | 0 |
| `club_records_fields` | 0 | 0 | 10 | 0 |
| `club_shortlist` | 0 | 0 | 0 | 2 |
| `cm_date` | 2 | 0 | 0 | 0 |
| `colour` | 0 | 0 | 3 | 0 |
| `commentary_queue` | 0 | 0 | 11 | 1 |
| `commentary_slot` | 0 | 0 | 1 | 0 |
| `comp_history_row` | 0 | 0 | 3 | 1 |
| `comp_stats_club` | 0 | 4 | 7 | 0 |
| `competition` | 9 | 0 | 0 | 0 |
| `con_merc_cup` | 0 | 0 | 2 | 0 |
| `config` | 0 | 1 | 0 | 0 |
| `conmebol_liber` | 0 | 0 | 1 | 1 |
| `conmebol_merc` | 0 | 0 | 1 | 1 |
| `conmebol_seeding` | 0 | 1 | 2 | 0 |
| `continent` | 0 | 1 | 0 | 0 |
| `contract` | 19 | 3 | 6 | 3 |
| `contract_manager_fields` | 0 | 0 | 10 | 1 |
| `contract_offer` | 0 | 0 | 8 | 3 |
| `contract_talks` | 0 | 0 | 2 | 0 |
| `count_record` | 0 | 0 | 2 | 0 |
| `crt_file` | 0 | 1 | 0 | 0 |
| `cup` | 0 | 0 | 25 | 2 |
| `cup_round` | 0 | 0 | 23 | 1 |
| `cup_team_entry` | 0 | 0 | 3 | 0 |
| `cup_tie` | 0 | 2 | 10 | 0 |
| `day_competition` | 0 | 0 | 5 | 1 |
| `day_group` | 0 | 0 | 5 | 0 |
| `day_match` | 0 | 0 | 17 | 1 |
| `db_index_entry` | 0 | 2 | 0 | 0 |
| `decision_date` | 0 | 0 | 3 | 0 |
| `discipline_fields` | 0 | 0 | 11 | 0 |
| `discipline_record` | 0 | 0 | 3 | 0 |
| `display` | 0 | 0 | 3 | 1 |
| `eng_auto_cup` | 0 | 0 | 1 | 1 |
| `eng_cc_cup` | 0 | 0 | 1 | 0 |
| `eng_fa_cup` | 0 | 0 | 3 | 0 |
| `engine` | 0 | 1 | 165 | 22 |
| `english_name_tables` | 0 | 0 | 7 | 0 |
| `euro_champ_qual` | 0 | 0 | 2 | 0 |
| `european_club_entry` | 0 | 0 | 1 | 0 |
| `event_type` | 0 | 0 | 3 | 0 |
| `event_variant` | 0 | 2 | 0 | 0 |
| `exchange_player` | 0 | 0 | 2 | 0 |
| `fans_player_record` | 0 | 0 | 1 | 0 |
| `fifa_ranking_entry` | 0 | 0 | 7 | 0 |
| `fifa_rankings_fields` | 0 | 0 | 2 | 0 |
| `fifa_result_entry` | 0 | 0 | 2 | 0 |
| `file_find_state` | 0 | 0 | 1 | 0 |
| `finance` | 0 | 0 | 40 | 2 |
| `fixed_fixture` | 0 | 0 | 1 | 0 |
| `fixture` | 0 | 0 | 35 | 6 |
| `fixture_leg_check` | 0 | 0 | 4 | 0 |
| `fixture_manager` | 0 | 0 | 7 | 0 |
| `formation_fields` | 0 | 5 | 9 | 0 |
| `fra_first` | 0 | 0 | 2 | 0 |
| `free_transfer_entry` | 0 | 0 | 2 | 0 |
| `friendly` | 0 | 0 | 19 | 2 |
| `friendly_match` | 0 | 0 | 5 | 0 |
| `future_transfer` | 0 | 0 | 1 | 0 |
| `goals_in_match_record` | 0 | 0 | 3 | 0 |
| `goldcup` | 0 | 0 | 2 | 0 |
| `grid_cell` | 0 | 0 | 2 | 0 |
| `guio` | 0 | 0 | 1 | 0 |
| `h2h_entry` | 0 | 0 | 1 | 0 |
| `held_ban` | 0 | 0 | 2 | 0 |
| `host_entry` | 0 | 0 | 7 | 0 |
| `human_manager_fields` | 0 | 2 | 0 | 0 |
| `human_manager_record` | 0 | 11 | 0 | 0 |
| `human_manager_sheet` | 0 | 0 | 2 | 1 |
| `illness_outbreak` | 0 | 0 | 1 | 0 |
| `injury_entry` | 0 | 4 | 6 | 0 |
| `injury_fields` | 0 | 0 | 10 | 0 |
| `instalment` | 0 | 0 | 3 | 0 |
| `intertoto_cup` | 0 | 0 | 1 | 0 |
| `intl_date_entry` | 0 | 0 | 6 | 1 |
| `ita_cup_fields` | 0 | 0 | 2 | 0 |
| `italian_league` | 0 | 0 | 1 | 0 |
| `job_vacancy` | 0 | 0 | 1 | 0 |
| `key_nation_record` | 0 | 7 | 2 | 1 |
| `language_entry` | 0 | 0 | 2 | 0 |
| `league` | 0 | 5 | 44 | 6 |
| `league_projection` | 0 | 0 | 5 | 0 |
| `league_round_date` | 0 | 0 | 7 | 1 |
| `league_table_row` | 0 | 0 | 29 | 0 |
| `lineup_row` | 0 | 4 | 11 | 3 |
| `live_match` | 0 | 0 | 15 | 2 |
| `llist` | 0 | 0 | 4 | 0 |
| `llist_node` | 0 | 0 | 4 | 0 |
| `loan_negotiation` | 0 | 0 | 1 | 0 |
| `man_of_match_record` | 0 | 0 | 2 | 0 |
| `manager_manager_fields` | 0 | 0 | 5 | 0 |
| `manager_record` | 0 | 1 | 47 | 0 |
| `match_day_fields` | 0 | 0 | 17 | 0 |
| `match_events_fields` | 0 | 0 | 3 | 0 |
| `match_incident` | 0 | 0 | 6 | 0 |
| `match_manager` | 0 | 0 | 15 | 7 |
| `match_player` | 0 | 3 | 111 | 22 |
| `match_record` | 0 | 6 | 69 | 11 |
| `match_record_row` | 0 | 7 | 25 | 6 |
| `match_record_team` | 0 | 0 | 28 | 4 |
| `match_spectators` | 0 | 0 | 9 | 1 |
| `match_stats` | 0 | 0 | 5 | 0 |
| `match_stats_entry` | 0 | 0 | 4 | 0 |
| `match_stats_record` | 0 | 0 | 7 | 0 |
| `match_summary` | 0 | 0 | 1 | 0 |
| `media_fields` | 0 | 0 | 6 | 0 |
| `month_award_fields` | 0 | 4 | 0 | 0 |
| `month_rating_entry` | 0 | 1 | 1 | 0 |
| `month_ratings_fields` | 0 | 2 | 0 | 0 |
| `names` | 0 | 3 | 0 | 0 |
| `nation` | 34 | 3 | 1 | 0 |
| `nation_club_bucket` | 0 | 0 | 2 | 0 |
| `nation_index_item` | 0 | 0 | 1 | 0 |
| `nation_name_index` | 0 | 0 | 9 | 0 |
| `national_squad_record` | 0 | 9 | 8 | 1 |
| `national_teams_fields` | 0 | 0 | 9 | 0 |
| `news_fields` | 0 | 0 | 3 | 0 |
| `news_item` | 0 | 0 | 8 | 0 |
| `non_player` | 27 | 0 | 0 | 0 |
| `oceania_club_champ` | 0 | 0 | 1 | 1 |
| `offer` | 0 | 0 | 18 | 7 |
| `official` | 0 | 12 | 5 | 0 |
| `official_stats` | 0 | 0 | 6 | 0 |
| `officials_comp_slot` | 0 | 0 | 4 | 0 |
| `officials_manager_fields` | 0 | 0 | 4 | 0 |
| `officials_nation_block` | 0 | 0 | 6 | 0 |
| `past_transfer` | 0 | 0 | 5 | 3 |
| `pending_ban` | 0 | 1 | 0 | 0 |
| `person_transfer_record` | 0 | 0 | 4 | 0 |
| `physical_record` | 0 | 0 | 10 | 0 |
| `planned_friendly` | 0 | 0 | 4 | 0 |
| `player_category_stats` | 0 | 0 | 18 | 3 |
| `player_regen_fields` | 0 | 0 | 12 | 0 |
| `player_search_fields` | 0 | 0 | 14 | 2 |
| `player_sheet` | 58 | 0 | 0 | 0 |
| `player_stats_entry` | 0 | 0 | 6 | 0 |
| `player_stats_fields` | 0 | 0 | 13 | 0 |
| `preferences` | 7 | 0 | 0 | 0 |
| `preseason_candidate` | 0 | 0 | 7 | 0 |
| `pundit` | 0 | 4 | 0 | 0 |
| `rating_record` | 0 | 0 | 2 | 1 |
| `record_holder` | 0 | 1 | 1 | 0 |
| `record_news_item` | 0 | 0 | 2 | 0 |
| `recovery_news_entry` | 0 | 0 | 5 | 0 |
| `regen_pool_item` | 0 | 0 | 3 | 0 |
| `regen_queue_entry` | 0 | 0 | 2 | 0 |
| `retirement_entry` | 0 | 1 | 8 | 0 |
| `ruling_body` | 0 | 3 | 5 | 0 |
| `ruling_body_entry` | 0 | 3 | 2 | 1 |
| `run_record` | 0 | 0 | 2 | 0 |
| `save_summary` | 0 | 1 | 0 | 0 |
| `scheduled_friendly_slot` | 0 | 0 | 4 | 0 |
| `scout_manager_fields` | 0 | 0 | 4 | 0 |
| `scout_report` | 0 | 3 | 6 | 4 |
| `scout_request` | 0 | 0 | 5 | 0 |
| `scouted_match_entry` | 0 | 1 | 2 | 0 |
| `scouted_player_slot` | 0 | 0 | 2 | 0 |
| `screen_manager` | 0 | 1 | 3 | 1 |
| `screen_node` | 0 | 1 | 0 | 0 |
| `search_result` | 0 | 0 | 2 | 0 |
| `sell_on_clause` | 0 | 0 | 3 | 0 |
| `shortlist_manager_fields` | 0 | 0 | 10 | 0 |
| `shortlist_pair` | 0 | 0 | 1 | 0 |
| `shortlist_slot` | 0 | 0 | 8 | 1 |
| `side_stats` | 0 | 0 | 5 | 0 |
| `spa_first_fields` | 0 | 0 | 1 | 0 |
| `stadium` | 10 | 0 | 1 | 0 |
| `stadium_obj` | 0 | 0 | 7 | 0 |
| `stadium_venue` | 0 | 1 | 0 | 0 |
| `staff` | 32 | 4 | 2 | 0 |
| `staff_history` | 0 | 3 | 0 | 0 |
| `staff_records_fields` | 0 | 0 | 7 | 0 |
| `staff_state` | 0 | 0 | 7 | 1 |
| `story_log_entry` | 0 | 0 | 6 | 0 |
| `tactic_stats` | 0 | 0 | 7 | 3 |
| `tactics_fields` | 0 | 0 | 10 | 2 |
| `tactics_row` | 0 | 2 | 4 | 0 |
| `tally_record` | 0 | 0 | 1 | 0 |
| `tournament_slot` | 0 | 1 | 8 | 3 |
| `training_manager_fields` | 0 | 0 | 6 | 0 |
| `training_schedule` | 0 | 0 | 1 | 0 |
| `transfer_fee_record` | 0 | 0 | 4 | 1 |
| `transfer_interest` | 0 | 0 | 7 | 0 |
| `transfer_manager_fields` | 0 | 0 | 35 | 0 |
| `transfer_rules` | 0 | 2 | 3 | 1 |
| `transfer_window` | 0 | 0 | 2 | 0 |
| `uefa_cup` | 0 | 0 | 1 | 0 |
| `uefa_seeding` | 0 | 0 | 5 | 0 |
| `uefa_seeding_nation` | 0 | 0 | 5 | 0 |
| `undo_slot` | 0 | 0 | 4 | 0 |
| `wc_oceania_league_fields` | 0 | 1 | 0 | 0 |
| `wc_south_american_league_fields` | 0 | 2 | 0 | 0 |
| `work_permit` | 0 | 0 | 4 | 0 |
| `world_cup_fields` | 0 | 0 | 4 | 0 |
| `zfile` | 0 | 4 | 0 | 0 |
| `zip_header` | 0 | 1 | 0 | 0 |

## Os nomes inferidos (a conferir)

| Campo | Desloc. | Descrição |
|---|---|---|
| `contract::PREVIOUS_WAGE` | +0x36 | 4 byte(s): Copy of the wage taken when the contract is set up [inference] |
| `contract::UNHAPPY_FLAG` | +0x3e | 1 byte(s): Byte cleared when a transfer request is withdrawn or morale is reset [inference] |
| `contract::MATCHES_LEFT_OUT` | +0x3f | 1 byte(s): Count of matches the person was left out; 5 or more outside the human club sets the squad mark [inference] |
| `contract_manager_fields::DECISIONS_ADDED` | +0x1c | 4 byte(s): Counter bumped each time an offer decision is scheduled [inference] |
| `cup::IMPORTANCE_BIAS` | +0x51 | 1 byte(s): Added to (match round - ROUND_COUNT) to give a cup match's importance (minimum 1); passed on to its stages (inherited from Comp) [inference] |
| `cup::RUN_BY_STAGES` | +0xb1 | 1 byte(s): Set when the cup has no knockout tree of its own (first round without ties) and is driven by its stages +0xc; cleared at season start [inference] |
| `eng_auto_cup::SELECTED_COUNT` | +0xb6 | 4 byte(s): Number of club slots in SELECTED_CLUBS filled by select_section_clubs (compared up to 0x38) [inference] |
| `friendly::PRESEASON_PLAN_DAY_A` | +0x34e8 | 2 byte(s): One of the two preseason planning day numbers; also compared with the current year for the Intertoto check [inference] |
| `friendly::PRESEASON_PLAN_DAY_B` | +0x34ea | 2 byte(s): The other preseason planning day number checked by update_for_date [inference] |
| `league::FIXED_STADIUM_ID` | +0x24 | 4 byte(s): stadium id used for the fixtures; the club whose stadium (+0x69) matches plays at home; -1 = none [inference] |
| `league::CLUB_SIDES` | +0x42 | 1 byte(s): 1 = played by club sides (written by every set_up_rules) [inference] |
| `league::ACTIVE` | +0x45 | 4 byte(s): nonzero when the competition has clubs/is active; zero makes the day update, sort and completion checks skip [inference] |
| `league::STOPPED` | +0xb5 | 1 byte(s): nonzero = league stopped/closed: no daily update, never complete [inference] |
| `league::REQUIRED_CAPACITY` | +0xe2 | 2 byte(s): capacity a club's stadium must have in this division (-1 none); passed to Finance::upgrade_stadium_for_league [inference] |
| `league::REQUIRED_SEATS` | +0xe4 | 2 byte(s): seated places a club's stadium must have in this division, passed to Finance::upgrade_stadium_for_league [inference] |
| `player_search_fields::ASSIGNED_BASE` | +0x199 | 4 byte(s): Scout assignment flag: scout the region of the club's base [inference] |
| `player_search_fields::ASSIGNED_OPPONENT` | +0x19d | 4 byte(s): Scout assignment flag: scout the next opposition [inference] |
| `offer::TERMS_STATE` | +0x10 | 1 byte(s): Personal-terms progress (0 none, 2 target accepted terms, 4 collapsed) [inference] |
| `offer::JOB_TYPE` | +0x11 | 1 byte(s): Job the target would take at the buyer (11..15 = player jobs), set when terms are accepted [inference] |
| `offer::PLAYER_AWARE` | +0x2d | 1 byte(s): Set to 1 when the target is told of or unsettled by the bid (2 on a new offer becomes 1); non-zero counts as under offer [inference] |
| `offer::RESPONSE_ROUND` | +0x2e | 1 byte(s): Response counter; with status 0xf, >= 3 means closed [inference] |
| `offer::PRE_CONTRACT` | +0x2f | 1 byte(s): Agreement that completes at the end of the target's contract: the seller's approval is skipped and the news gives the contract end [inference] |
| `offer::OFFER_KIND` | +0x31 | 4 byte(s): Dword whose low nibble is the offer kind (copied from the incoming offer, news slot 0xe; kinds 1 and 2 excluded when valuing the best bid) [inference] |
| `offer::INSTALMENTS` | +0x40 | 1 byte(s): Instalments of the fee (cleared when the fee is 0) [inference] |
| `engine::SIDE_OUT_OF_POSITION` | +0xaf | 2 byte(s): per-side count of players marked as playing out of their position during the strength rating; cleared by side_strength [inference] |
| `engine::PENDING_INCIDENT_PLAY` | +0xba | 1 byte(s): first pending incident entry +9: play code of the incident (minus 0x25 selects the codes without commentary) [inference] |
| `engine::SIDE_PLAYED_BALL` | +0x1a5 | 2 byte(s): per-side flag set when a player of that side deliberately plays or takes the ball (cleared for the other side and on interceptions); while clear the goalkeeper may pick it up [inference: back-pass rule] |
| `engine::SHOT_PENDING` | +0x1a7 | 1 byte(s): set when a shot is taken, cleared when the ball is released [inference] |
| `engine::SIDE_PITCH_HANDICAP` | +0x1ad | 1 byte(s): side 1 pitch handicap (0..10; floor 1 when the venue is not a home ground of either club) [inference] |
| `engine::SHOT_REBOUND` | +0x1cd | 1 byte(s): set when a shot leaves the ball loose (blocked, saved or stopped); picking it up in front of goal gets the rebound commentary [inference] |
| `engine::BALL_TIME_THIRD_1` | +0x1f6 | 10 byte(s): five i16 counters (indexed by side, 0..4) of ball time in the first pitch third; copied to the match record +0xc [inference] |
| `engine::BALL_TIME_THIRD_2` | +0x200 | 10 byte(s): five i16 counters of ball time in the second pitch third; copied to the match record +0x16 [inference] |
| `engine::BALL_TIME_THIRD_3` | +0x20a | 10 byte(s): five i16 counters of ball time in the third pitch third; copied to the match record +0x20 [inference] |
| `engine::ADJUSTED_PLAYER_ID` | +0x477a | 4 byte(s): id (first dword of the record) of the last player adjusted by the disposition adjust [inference] |
| `engine::MOMENTUM_A` | +0x4782 | 4 byte(s): signed momentum score changed by shots and goals (i32) [inference] |
| `engine::MOMENTUM_B` | +0x4786 | 4 byte(s): second signed momentum score changed by shots and goals (i32) [inference] |
| `engine::FOULED_PLAYER` | +0x478a | 4 byte(s): player who suffered the last foul/tackle (0 none) [inference] |
| `engine::MATCH_ALREADY_OVER` | +0x8eba | 1 byte(s): the match ended before being played: preparation returns 0 when set after the squads [inference] |
| `engine::SUB_TICK` | +0x8ed2 | 2 byte(s): step counter within the current clock tick; when it reaches 1 the clock advances and it is cleared [inference] |
| `engine::REFEREE_RATING` | +0xf5ba | 2 byte(s): referee base rating used for the post-match grade (/1000); referee object +8 [inference] |
| `engine::FREE_KICK_PLAY_GOALS` | +0xf5c4 | 1 byte(s): side 0 goals from free-kick plays (set piece kind 7 or -1); side 1 at +0x36 [inference] |
| `engine::SET_PIECES` | +0xf5c6 | 1 byte(s): side 0 set pieces lined up by line_up_set_piece (side 1 at +0x36); SIDE_STATS +0xa [inference: free kicks] |
| `engine::SIDE_FOUL_TALLY` | +0xf5c9 | 1 byte(s): side 0 foul tally (side 1 at +0xf5ff; SIDE_STATS +0xd): incremented per foul and compared with the referee's +0x27 to raise strictness [inference] |
| `engine::TEMPO_ROLL_A` | +0xf628 | 4 byte(s): f32 3..9 rolled by the match summary at kick-off and half time (6.0 at preparation) [inference: match tempo] |
| `engine::TEMPO_ROLL_B` | +0xf62c | 4 byte(s): f32 3..9 rolled with TEMPO_ROLL_A (6.0 at preparation) [inference] |
| `engine::TEMPO_GOALS_ROLL` | +0xf630 | 4 byte(s): f32 from random(5) plus the goals so far, rerolled when not below 10 (5.0 at preparation) [inference] |
| `match_player::SIDE_INDEX` | +0x0 | 1 byte(s): player's index within his side's block of 20 (-1 empty); marks shootout takers [inference] |
| `match_player::TACKLES` | +0x4 | 1 byte(s): tackles attempted [inference] |
| `match_player::TACKLES_WON` | +0x5 | 1 byte(s): tackles/challenges won [inference] |
| `match_player::KEY_TACKLES` | +0x6 | 1 byte(s): tackles won near his own goal or behind the defensive line (+300 rating) [inference] |
| `match_player::CHALLENGES` | +0x7 | 1 byte(s): contested balls taken part in (both players counted) [inference] |
| `match_player::CHALLENGES_WON` | +0x8 | 1 byte(s): contested balls won [inference] |
| `match_player::KEY_CHALLENGES_WON` | +0x9 | 1 byte(s): contested balls won in a dangerous situation (+300 rating) [inference] |
| `match_player::DRIBBLES` | +0x11 | 1 byte(s): times he beat an opponent with the ball [inference] |
| `match_player::BALLS_CONTESTED` | +0x12 | 1 byte(s): count of loose balls contested by the player (incremented in respond) [inference] |
| `match_player::MATCH_FORM` | +0x17 | 1 byte(s): 1..20 in-match form byte, nudged up or down by events (adjust_17) and floored by the performance word [inference] |
| `match_player::INJURY_FORCES_OFF` | +0x1c | 1 byte(s): set when his pending injury forces him off: the substitution pass removes him from the pitch; stamina recovery skips the injury cap while set [inference] |
| `match_player::RED_CARDS` | +0x23 | 1 byte(s): incremented when he leaves the pitch (player_left_pitch); added to YELLOW_CARDS for his card total and to the side summary cards [inference: sendings-off] |
| `match_player::PREFERRED_FOOT` | +0x2c | 1 byte(s): foot used when the angle needs no particular one (0 left, 1 right) [inference] |
| `match_player::MATCH_TEMPER` | +0x31 | 2 byte(s): temper/discipline word: 0 at setup, +10 if his club favours him, -20 if it dislikes him, -10 against a rival club; drops with each foul he commits (-3..-200 by severity) and -10 when he injures someone; random(v) < -10 triggers the agitation comments and confidence swings; feeds the moment attributes in update_attributes [inference] |
| `match_player::MATCH_RATING` | +0x35 | 2 byte(s): performance word (code calls it disposition): starts at 6100, rises with good play, drops with fouls and cards [inference] |
| `match_player::EFFECTIVE_ABILITY` | +0x3b | 2 byte(s): match ability word (current ability adjusted by morale, captain, contract) [inference] |
| `match_player::GOAL_COUNTS` | +0x3d | 10 byte(s): five i16 goal tallies (+0x3d..+0x45), incremented with each goal and undone when it is disallowed; read by the scorer milestone commentaries 0x200a..0x2012 [inference: career/season counts] |
| `match_player::INJURY_CULPRIT` | +0x55 | 4 byte(s): player who caused his injury (named in the injury commentary 0x20f6) [inference] |
| `match_player::ANGER` | +0xff | 1 byte(s): anger byte: +1 when a foul against him passes his anger check, decays each moment toward a floor from FRUSTRATION and his dirtiness [inference] |
| `match_player::FRUSTRATION` | +0x100 | 1 byte(s): frustration byte: grows for a goalkeeper after conceding; raises his anger floor and lowers some commentary chances [inference] |
| `match_player::BALL_CONTROL` | +0x19c | 2 byte(s): control margin word, clamped to -200..200 [inference] |
| `match_player::CONFIDENCE` | +0x1a2 | 4 byte(s): f32 0..100 (docs: condition), floor 2 x (25 - temperament) [inference] |
| `fixture::ROUND_DRAW_RULES` | +0x36 | 2 byte(s): round rule word copied from the round (cup +0xb, league +0xdb) [inference: draw type] |
| `fixture::HOME_SIDE_MARK` | +0x40 | 1 byte(s): per-side byte of the home club (league 0, cup 1; find_fixtures side filter); 0 sets the match manager's +0x19 [inference] |
| `fixture::AWAY_SIDE_MARK` | +0x41 | 1 byte(s): per-side byte of the away club (same use as HOME_SIDE_MARK) [inference] |
| `fixture::HOME_AGGREGATE` | +0x4b | 1 byte(s): two-legged tie: home side's first-leg/aggregate goals (0xff none) [inference] |
| `fixture::AWAY_AGGREGATE` | +0x4c | 1 byte(s): two-legged tie: away side's first-leg/aggregate goals (0xff none) [inference] |
| `fixture::FLAGS` | +0x4e | 1 byte(s): flags byte: bits 0-1 national-team call-up flags, bit 2 cup leg flag, bit 3 last league round [inference] |
| `match_manager::KEY_PLAYER_1` | +0x6 | 4 byte(s): first of the three highest-reputation selected players (person pointer) [inference: key players] |
| `match_manager::KEY_PLAYER_2` | +0xa | 4 byte(s): second key player (person pointer) [inference] |
| `match_manager::KEY_PLAYER_3` | +0xe | 4 byte(s): third key player (person pointer) [inference] |
| `match_manager::SELECTION_FLAG` | +0x16 | 1 byte(s): byte flag that sets mask bit 0x800000 in automatic selection and changes the substitution mode [inference] |
| `match_manager::TACTIC_SLOT` | +0x19 | 1 byte(s): 1 when the fixture side mark is 0, else 0; tactic slot passed to the human manager and the tactics table; headers call it 'human' [inference] |
| `match_manager::SELECTION_MODE` | +0x1a | 1 byte(s): selection mode: 0/1/2 rotation weight of today's match, 3 no rotation [inference] |
| `match_manager::BEST_PLAYER` | +0x3b | 4 byte(s): best-rated selected player (best_in_list) [inference: captain candidate] |
| `finance::INSTALMENT_BALANCE` | +0x1c | 4 byte(s): Running share of transfer instalments still owed (buyer negative, seller positive); cleared when the club has none left [inference] |
| `finance::CRISIS_STATE` | +0x165 | 1 byte(s): financial state: 0 normal, 1 crisis, 2 [inference: administration] |
| `ban_slot::VALUE` | +0x1 | 4 byte(s): Value of the slot (-1 when free) [inference: link to the ruling body entry] |
| `board_record::STAFF_CONFIDENCE` | +0xa | 2 byte(s): the staff's confidence in the manager (+0.5 x the match adjustment); departure reason 8, 'poor relationship with his staff' (0xa07540) [inference] |
| `board_record::TRANSFER_MOOD` | +0x14 | 2 byte(s): board mood changed by the transfer deal reasons (signed) [inference] |
| `board_record::FAVOURITES_MOOD` | +0x16 | 2 byte(s): board mood changed by sales of favoured or rejected players and media stories, floor -10000; below -750 is departure reason 4, 'overwhelming supporter pressure' (0xa0764c) [inference] |
| `board_record::EXTRA_FUNDS_GRANTED` | +0x28 | 4 byte(s): money the board granted on a funds request or a funds ultimatum (0 none; cleared at season start and on a manager change) [inference] |
| `board_record::SPENDING` | +0x30 | 8 byte(s): double spending figure, reputation squared / 4 at the season start [inference] |
| `club_record::SEASON_RATING_YEAR` | +0x20e | 2 byte(s): SEASON_RATING + rating_record YEAR (0 in the season record) [inference] |
| `club_shortlist::ACTIVE` | +0x4 | 1 byte(s): Shortlist state of the club (1 active; 2 also seen) [inference] |
| `club_shortlist::DAYS` | +0x8 | 1 byte(s): Day counter; set to 100 when the club suspends a player, bids skipped at 100 or more [inference] |
| `commentary_queue::LOOP_CHANNELS` | +0x20 | 16 byte(s): channels of the crowd loops (-1 none; side's loop at +0x20 + side) [inference] |
| `contract_offer::DECISION_VALID` | +0x62 | 1 byte(s): 1 when the offer counts for a decision (set outside type 6 contracts and when scheduled for a transfer) [inference] |
| `contract_offer::RESERVED_RECORD_ARG1` | +0x67 | 4 byte(s): First extra argument returned with RESERVED_RECORD (add_spare_non_player_record) [inference] |
| `contract_offer::RESERVED_RECORD_ARG2` | +0x6b | 4 byte(s): Second extra argument returned with RESERVED_RECORD [inference] |
| `cup_round::SINGLE_LEG_FLAG` | +0x20 | 1 byte(s): positive with fewer than 2 legs sets fixture SQUAD_RULES bit 0x400 [inference] |
| `day_competition::PANEL` | +0x14 | 64 byte(s): 0x40 bytes cleared when the day's matches are picked [inference] |
| `day_match::SCOREBOARD` | +0x5b | 4 byte(s): humans' network scoreboard (0x11d bytes) [inference] |
| `human_manager_sheet::CLUB` | +0xe6 | 4 byte(s): club the manager leads in that match [inference] |
| `intl_date_entry::WEEKDAY` | +0x1 | 1 byte(s): day byte of the international date as written by build_international_date_table [inference] |
| `key_nation_record::SEASON_SPANS_YEAR` | +0x1d | 1 byte(s): Nonzero when the season ends in the following year [inference] |
| `league_round_date::TV_SLOT_DATA` | +0x5 | 4 byte(s): dword of TV slot 0 (8 slots of 7 bytes from +5) [inference] |
| `lineup_row::DRIBBLES` | +0x28 | 1 byte(s): times he beat an opponent in the match [inference] |
| `lineup_row::RED_CARD` | +0x2d | 1 byte(s): red cards in the match [inference] |
| `lineup_row::TACKLES_WON` | +0x42 | 1 byte(s): tackles won in the match [inference] |
| `live_match::DAY_ENTRY` | +0x63 | 4 byte(s): match-day competition entry of the match (0x54 bytes; screen param 0xe of the match screen) [inference] |
| `live_match::STATE` | +0x67 | 1 byte(s): live state: 0xff = not live; 2 = finished (the score is already in the aggregate); set to 1 by the match screen [inference] |
| `match_record::BALL_TIME_THIRD_1` | +0xc | 10 byte(s): five i16 (indexed by side, 0..4): ball time in the first pitch third, shown as a possession percentage [inference] |
| `match_record::BALL_TIME_THIRD_2` | +0x16 | 10 byte(s): five i16: ball time in the second pitch third [inference] |
| `match_record::BALL_TIME_THIRD_3` | +0x20 | 10 byte(s): five i16: ball time in the third pitch third [inference] |
| `match_record::MAN_OF_THE_MATCH_ID` | +0x3c | 4 byte(s): player id highlighted in the ratings and team stats (compared with the rows' PLAYER_ID); -1 none [inference] |
| `match_record::HOME_HEADERS` | +0x138 | 2 byte(s): headers contested (the away percentage is 100 minus the home one) [inference] |
| `match_record::HOME_RECENT_POSSESSION` | +0x142 | 1 byte(s): home possession of the last five minutes (the overview possession bar) [inference] |
| `match_record::ROW_FIRST_NAME_ID` | +0x198 | 4 byte(s): first name id of a player row (same base) [inference] |
| `match_record::ROW_SECOND_NAME_ID` | +0x19c | 4 byte(s): second name id of a player row (same base) [inference] |
| `match_record::ROW_COMMON_NAME_ID` | +0x1a0 | 4 byte(s): common name id of a player row (same base) [inference] |
| `match_record::AWAY_HEADERS` | +0x75d | 2 byte(s): away headers contested [inference] |
| `match_record::AWAY_RECENT_POSSESSION` | +0x767 | 1 byte(s): away possession of the last five minutes [inference] |
| `match_record_row::FIRST_NAME_ID` | +0x8 | 4 byte(s): first name id (checked against the person) [inference] |
| `match_record_row::SECOND_NAME_ID` | +0xc | 4 byte(s): second name id [inference] |
| `match_record_row::COMMON_NAME_ID` | +0x10 | 4 byte(s): common name id [inference] |
| `match_record_row::DRIBBLES` | +0x28 | 1 byte(s): times he beat an opponent (copy of match_player DRIBBLES +0x11) [inference] |
| `match_record_row::RED_CARD` | +0x2d | 1 byte(s): red card (drawn in red on the ratings) [inference] |
| `match_record_row::STAMINA_CEILING` | +0x36 | 2 byte(s): stamina ceiling of the player when the record was filled (truncated f32); row +0x36 [inference] |
| `match_record_team::HEADERS` | +0x19 | 2 byte(s): headers contested [inference] |
| `match_record_team::POSSESSION` | +0x1f | 5 byte(s): possession percentages; [4] is the last five minutes [inference] |
| `match_record_team::FORMATION_X` | +0x35 | 22 byte(s): 11 u16: first coordinate of each formation slot (formation slot position, axis 0) [inference] |
| `match_record_team::FORMATION_Y` | +0x4b | 22 byte(s): 11 u16: second coordinate of each formation slot (axis 1) [inference] |
| `match_spectators::UPDATE_FLAGS` | +0x0 | 1 byte(s): bits telling the match screens what to redraw (4 data panels, 0x10/0x20 view changes) [inference] |
| `national_squad_record::RESERVE_GROUPS` | +0xd | 42 byte(s): two 0x15-byte reserve level groups (senior, under-21) [inference] |
| `past_transfer::TO_CLUB_ID` | +0x10 | 4 byte(s): Club the person moved to (-1 when released) [inference] |
| `past_transfer::FROM_CLUB_ID` | +0x14 | 4 byte(s): Club the person left (-1 when none) [inference] |
| `past_transfer::FEE` | +0x18 | 4 byte(s): Fee of the move (0 for loans and releases) [inference] |
| `player_category_stats::RED_CARDS` | +0xc | 1 byte(s): red cards summed over the matches [inference] |
| `player_category_stats::TACKLES_WON` | +0x14 | 2 byte(s): tackles won summed over the matches [inference] |
| `player_category_stats::DRIBBLES` | +0x16 | 2 byte(s): times he beat an opponent summed over the matches [inference] |
| `rating_record::YEAR` | +0x1d | 2 byte(s): Season year of the record [inference] |
| `ruling_body_entry::PERSON_ID` | +0x0 | 4 byte(s): Person id of the entry (-1 = free) [inference] |
| `scout_report::ELIGIBILITY_NOTE` | +0x58 | 1 byte(s): eligibility note: 1 needs a work permit (outcome in WORK_PERMIT_GRANTED), 2 eligible through a second nationality of membership group 2, 0 none [inference] |
| `scout_report::CURRENT_RATING` | +0x5c | 1 byte(s): scout's 0-5 grade of the player's current score; the news packs it times 8 [inference] |
| `scout_report::POTENTIAL_RATING` | +0x5d | 1 byte(s): scout's 0-5 grade of the player's projected score; the news packs it in the low 3 bits [inference] |
| `scout_report::LEAGUE_MATCH` | +0x62 | 4 byte(s): 1 when the watched fixture's HOME_SIDE_MARK byte (+0x40) is 0 (league match by that field's reading) [inference] |
| `shortlist_slot::VALUATION` | +0x3 | 4 byte(s): Club's valuation of the target, compared with PRICE in compensation deals [inference] |
| `staff_state::INJURY_COUNTER` | +0xa | 1 byte(s): byte of the 79-byte general.dat record raised by injuries up to 100 [inference] |
| `tactics_fields::FORMATION` | +0x187b | 4 byte(s): formation (shape) object of the tactic, compared with the manager's favourite formation [inference] |
| `tactics_fields::POSITION_MARKS` | +0x188b | 88 byte(s): per tactical position mark bits, one u64 per position (11), read by position_marks [inference: same block layout as the engine tactics] |
| `tournament_slot::COMP_FLAGS_A` | +0xf | 2 byte(s): first rule-flags word for the tournament competition; value 0x80 also adds flag 2 [inference] |
| `tournament_slot::COMP_FLAGS_B` | +0x11 | 2 byte(s): second rule-flags word, OR-ed with COMP_FLAGS_A into the last flags argument of the MiniCup/MiniLeague constructor [inference] |
| `tournament_slot::IN_USE` | +0x29 | 4 byte(s): nonzero once the tournament is under way; find_human_club_tournament only returns slots with it set [inference] |
| `transfer_fee_record::FLAG` | +0x28 | 1 byte(s): Transfer flag: 1 when the transfer's +4 was set (loan or part exchange) [inference] |
| `transfer_rules::SCOPE` | +0x12 | 1 byte(s): Scope of the squad restriction rules (2 continental; otherwise compared with the fixture rules type) [inference] |
| `awol_fields::PERSON_FLAGS` | +0xc | 4 byte(s): Pointer to one absence flag byte per staff id (bit 1 absent without leave, bit 2 missed training; flipped by the mark routines) [inference] |
| `comp_history_row::HOST` | +0x16 | 4 byte(s): Host of the competition, fourth value of season_honours [inference] |
| `conmebol_liber::DRAWN_CLUB_COUNT` | +0xf2 | ? byte(s): Number of clubs in DRAWN_CLUBS [inference] |
| `conmebol_merc::DRAWN_CLUB_COUNT` | +0xf2 | ? byte(s): Number of clubs in DRAWN_CLUBS [inference] |
| `oceania_club_champ::DRAWN_CLUB_COUNT` | +0xf2 | ? byte(s): Number of clubs in DRAWN_CLUBS, zeroed with the list [inference] |
| `display::FOCUS_FLAGS` | +0x12f4fe | 1 byte(s): flag byte written with the focus (0x20 here) [inference] |
| `screen_manager::NETWORK_SLOT` | +0x7d4 | 2 byte(s): nonzero when slot k's screen updates go over the network (+0x300 x k) [inference] |
| `tactic_stats::ALL_OUT_ATTACK_SCORE` | +0x10 | 8 byte(s): f64 score of the tactic used when the stance is 1 (all-out attack) [inference] |
| `tactic_stats::ATTACK_SCORE` | +0x18 | 8 byte(s): f64 score of the tactic used for stance 2 (attack) and any other value [inference] |
| `tactic_stats::DEFENSIVE_SCORE` | +0x20 | 8 byte(s): f64 score of the tactic used for stances 3..5 (balanced, defend, park the bus) [inference] |
| `awol_fitness_incident::REPEAT_CASE` | +0x10 | 1 byte(s): nonzero takes the repeat path (professionalism/temperament draws); never set by report_poor_fitness (stack leftover) [inference] |
