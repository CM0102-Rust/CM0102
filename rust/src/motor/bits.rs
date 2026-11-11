// The bits of the records' flag fields by name, from the field readers (engenharia-reversa/nomear_bits.py;
// verdicts in levas/reorganizacao/leitores_de_campo/). Each constant says what the bit means and how far to trust
// the name: verified (concrete evidence), read (one clear reading), inferred (a guess).

/// Bits of `area_fields::FLAGS`.
#[allow(dead_code)]
pub(crate) mod area_fields_flags {
    /// layout: margins (0, 0) when set. Name: read.
    pub(crate) const NO_MARGINS: u32 = 0x1;
    /// layout: top/bottom margin 2 instead of 8 when set. Name: read.
    pub(crate) const THIN_VERTICAL_MARGINS: u32 = 0x10;
    /// community Area::draw draws only grid lines when the area is grid-only; layout/init skip the grid split. Name: verified.
    pub(crate) const GRID_ONLY: u32 = 0x80000;
}

/// Bits of `board_record::FLAGS`.
#[allow(dead_code)]
pub(crate) mod board_record_flags {
    /// set by reset_board_for_new_manager 0x694c80 (0x694c9b, 0x694cb1) around set_season_expectations; with it set_season_expectations skips the tenure exit and the chairman draws and averages the target with the current position, weekly_board_confidence_update skips the club (0x698a87); cleared by set_season_expectations' tail and daily_update 0x68fae1. Name: read.
    pub(crate) const NEW_MANAGER: u8 = 0x1;
    /// set when a manager departure is queued: consider_sacking_manager 0x69aba0 (tests at 0x69abd6, sets 0x69b03d/0x69b32c), resolve_board_ultimatum 0x6ac54a, handle_manager_ultimatum 0x6ad840; consider_sacking returns at once while set. Name: read.
    pub(crate) const DEPARTURE_QUEUED: u8 = 0x2;
    /// cleared on a new manager (reset_board_for_new_manager 0x694c80). Name: read.
    pub(crate) const PROJECTED_ABOVE_TARGET: u8 = 0x4;
    /// weekly_board_confidence_update 0x697c40 sets bit 8 once (0x698ad8, confidence -750) when even the better-form projected position (comp_stats_club +0x74) is worse than the target (+0x72); bit 4 is the mirror (0x698aa0, +750, worse-form projection +0x75 better than the target); departure reason 7 ('poor league position', 0xa07924). Name: read.
    pub(crate) const PROJECTED_BELOW_TARGET: u8 = 0x8;
}

/// Bits of `contract::CONTRACT_TYPE`.
#[allow(dead_code)]
pub(crate) mod contract_type {
    /// copied from the old contract; toggled at 2340/2376 and get_squad_flags_for_club returns 2 when set, 1 when clear. Name: read.
    pub(crate) const RESERVE_SQUAD: u8 = 0x40;
}

/// Bits of `crt_file::FLAGS`.
#[allow(dead_code)]
pub(crate) mod crt_file_flags {
    /// MSVC _IOEOF = 0x10; used as end_of_file while reading the events cfg. Name: verified.
    pub(crate) const EOF: u32 = 0x10;
}

/// Bits of `board_record::DISCIPLINE_COUNTERS`.
#[allow(dead_code)]
pub(crate) mod discipline_counters {
    /// bits 0..2 = FA disciplinary fine level (field comment); & 0xf8 clears it. Name: read.
    pub(crate) const FINE_LEVEL: u8 = 0x7;
    /// bits 3..7 count the ban appeals (+8 per appeal, field comment); & 7 clears them and keeps FINE_LEVEL. Name: read.
    pub(crate) const APPEALS_MADE: u8 = 0xf8;
}

/// Bits of `cup_round::DRAW_RULES`.
#[allow(dead_code)]
pub(crate) mod draw_rules {
    /// swap when virtual +0x90 club_in_higher_division returns club b.4 value 1. Name: verified.
    pub(crate) const LOWER_DIVISION_AT_HOME: i16 = 0x1;
    /// draw keeping clubs of the same nation apart (0x51d9f0 check_round_teams_for_nation).4. Name: verified.
    pub(crate) const SEPARATE_NATIONS: i16 = 0x4;
    /// swap when the level (+4) of a is below b's.4 value 8. Name: verified.
    pub(crate) const HIGHER_LEVEL_AT_HOME: i16 = 0x8;
}

/// Bits of `commentary_slot::EVENT_CODE`.
#[allow(dead_code)]
pub(crate) mod event_code {
    /// advance_commentary_read clears it (&= 0x7fff); next_event: bit 0x8000 marks more '|' parts; commentary_queue::SLOTS comment. Name: read.
    pub(crate) const MORE_PARTS: u32 = 0x8000;
}

/// Bits of `board_record::EXPECTATION_FLAGS`.
#[allow(dead_code)]
pub(crate) mod expectation_flags {
    /// build_board_expectation_text 0x694440 & 1: 'expect European / Continental qualification' (0xa046a0); set when the target is within the UEFA places. Name: verified.
    pub(crate) const EUROPEAN_QUALIFICATION: i16 = 0x1;
    /// build_board_expectation_text 0x694440 & 2: 'mount a serious Championship challenge' (0xa04438); news headline 'Board want the title' (0xa0aba0, 0x6a4b69); set when the target is within the champion's places. Name: verified.
    pub(crate) const TITLE_CHALLENGE: i16 = 0x2;
    /// build_board_expectation_text 0x694440 & 4: 'expect the team to achieve promotion this season' (0xa03f98); set_season_expectations sets it when the target is within PROMOTED_COUNT of a lower division. Name: verified.
    pub(crate) const PROMOTION: i16 = 0x4;
    /// build_board_expectation_text 0x694440 & 8: 'expect the club to reach the playoffs' (0xa03c18); set when the target <= PROMOTION_PLAYOFF_COUNT. Name: verified.
    pub(crate) const PLAYOFFS: i16 = 0x8;
    /// build_board_expectation_text 0x694440 & 0x10: 'stay clear of relegation this season' (0xa03480); the default of a league with relegation and of a vacancy/no division. Name: verified.
    pub(crate) const AVOID_RELEGATION: i16 = 0x10;
    /// build_board_expectation_text 0x694440 & 0x20: 'battle bravely against relegation' (0xa030a4); set when the target is inside the relegation places. Name: verified.
    pub(crate) const RELEGATION_BATTLE: i16 = 0x20;
    /// build_board_expectation_text 0x694440 & 0x40: 'attain a respectable league position' (0xa03828); set for the top half and as the default without a league row. Name: verified.
    pub(crate) const RESPECTABLE_POSITION: i16 = 0x40;
}

/// Bits of `board_record::EXPECTATION_FLAGS` (byte 1).
#[allow(dead_code)]
pub(crate) mod expectation_flags_byte1 {
    /// build_board_expectation_text 0x694440 tests the word & 0x100 for 'achieve cup success' (0xa03958) and 'success both domestically and in Europe' (0xa04840); set by the chairman's second draw. Name: verified.
    pub(crate) const CUP_SUCCESS: u8 = 0x1;
    /// build_board_expectation_text 0x694440 tests the word & 0x400 for 'success in Europe / Continental competition' (0xa04b80, 0xa04268, 0xa03b44...); here byte +1 & 4 = 0x400 doubles the weight of a continental-cup match; set at 0x6a408f when the continental-cup danger position is within the chairman's reach. Name: verified.
    pub(crate) const EUROPEAN_SUCCESS: u8 = 0x4;
}

/// Bits of `staff_state::EXTRA_FLAGS`.
#[allow(dead_code)]
pub(crate) mod extra_flags {
    /// set by record_international_result after an under-21 international (with the nation swap); with it a foreign player cannot be called up by another nation. Name: read.
    pub(crate) const PLAYED_U21_INTERNATIONAL: u8 = 0x2;
}

/// Bits of `fixture::TIE_RULES`.
#[allow(dead_code)]
pub(crate) mod fixture_tie_rules {
    /// part of mask 0x21 (any shoot-out decides a level match); 0x20 is the 35-yard variant, so 0x1 the ordinary penalties. Name: inferred.
    pub(crate) const PENALTY_SHOOT_OUT: u16 = 0x1;
    /// a level match is decided by the extra-time goals only with this bit; league::MATCH_RULES: bit 2 extra time. Name: read.
    pub(crate) const EXTRA_TIME: u16 = 0x2;
    /// changes the shoot-out result mark from 4 to 5; field comment: 0x20 35-yard shoot-out. Name: inferred.
    pub(crate) const SHOOT_OUT_35_YARD: u16 = 0x20;
    /// level at 90 with the bit: extra time with `decides`; a goal then sets GOLDEN_GOAL_SCORED and ends the match. Name: read.
    pub(crate) const GOLDEN_GOAL: u16 = 0x80;
}

/// Bits of `formation_fields::FLAGS`.
#[allow(dead_code)]
pub(crate) mod formation_fields_flags {
    /// set_team_instruction 0x5b50d0 groups 1/2/4/8 (mask 0xfffffff0); instructions() picks 2,1,4,8 for directness <=8,<=14,<=18,<=20, so 8 is the most direct passing. Name: inferred.
    pub(crate) const LONG_BALL: i32 = 0x8;
}

/// Bits of `contract::GRIEVANCES`.
#[allow(dead_code)]
pub(crate) mod grievances {
    /// tested clear before the complaint; set via staff_contract_apply_event_flags(c, 0x8000) right before news_post_player_unhappy_with_training. Name: read.
    pub(crate) const UNHAPPY_WITH_TRAINING: u32 = 0x8000;
    /// set on old-club teammates when an established player is sold (process_transfer_squad_reactions); cleared when a bigger player arrives at the new club. Name: inferred.
    pub(crate) const STAR_TEAMMATE_SOLD: u32 = 0x20000000;
    /// set via apply_event_flags on same-position teammates when the club signs a player; cleared when the squad is weak or the position has fewer than 2 players. Name: inferred.
    pub(crate) const SAME_POSITION_SIGNING: u32 = 0x80000000;
}

/// Bits of `league::LEAGUE_FLAGS`.
#[allow(dead_code)]
pub(crate) mod league_flags {
    /// with the bit the first row not yet out (status 0xfe/0xfc/3) is looked for; field comment: 1 champion status. Name: read.
    pub(crate) const CHAMPION_STATUS: i16 = 0x1;
    /// with it each club goes to the slot the virtual +0xa8 returns before the schedule; field comment: 0x100 fixed table slots. Name: read.
    pub(crate) const FIXED_TABLE_SLOTS: i16 = 0x100;
}

/// Bits of `nation::LEAGUE_SELECTED`.
#[allow(dead_code)]
pub(crate) mod league_selected {
    /// club_can_be_managed requires it; key_nation_get_main_continent picks the first league nation with it; the save summary (file_screens.rs) lists bit-2 nations first and bit-1 nations in the second string. Name: read.
    pub(crate) const PLAYABLE: u8 = 0x2;
    /// set: the Conference is played and its clubs enter by category; eng_prm/ita_ser_a/spa_first use the same bit to link the division below the bottom one (Conference, Serie C2, Segunda B). Name: read.
    pub(crate) const LOWER_DIVISIONS_LOADED: u8 = 0x4;
}

/// Bits of `awol_fields::PERSON_FLAGS`.
#[allow(dead_code)]
pub(crate) mod person_flags {
    /// per-person byte of the PERSON_FLAGS table: AWOL::end_absence flips ^ 1. Name: verified.
    pub(crate) const ABSENT_WITHOUT_LEAVE: usize = 0x1;
    /// per-person byte of the PERSON_FLAGS table: closing a missed-training case flips ^ 2. Name: verified.
    pub(crate) const MISSED_TRAINING: usize = 0x2;
}

/// Bits of `formation_fields::PLAYER_INSTRUCTIONS`.
#[allow(dead_code)]
pub(crate) mod player_instructions {
    /// mark_stars sets it for players rating over 8500 (vision, movement, finishing, form); exclusive with 0x400000 in set_square; game meaning of the instruction not identified. Name: inferred.
    pub(crate) const STANDOUT_PLAYER: u32 = 0x800000;
}

/// Bits of `staff::PLAYING_SQUAD`.
#[allow(dead_code)]
pub(crate) mod playing_squad {
    /// docs/MAPA_ESTATISTICAS_E_ELENCO B.2: bit 0 club first team. Name: verified.
    pub(crate) const FIRST_TEAM: u8 = 0x1;
    /// docs/MAPA_ESTATISTICAS_E_ELENCO B.2: bit 1 club reserves (pick_team skips 0x02). Name: verified.
    pub(crate) const RESERVES: u8 = 0x2;
    /// docs/MAPA_ESTATISTICAS_E_ELENCO B.2: bit 2 called up to the national side. Name: verified.
    pub(crate) const NATIONAL_SQUAD: u8 = 0x4;
    /// docs/MAPA_ESTATISTICAS_E_ELENCO B.2: bit 3 called up to the B national side. Name: verified.
    pub(crate) const NATIONAL_B_SQUAD: u8 = 0x8;
}

/// Bits of `board_record::REQUEST_STATE`.
#[allow(dead_code)]
pub(crate) mod request_state {
    /// request 5 'Extra time to rebuild the squad' (0xa0fce8): 0x6a91d6 already set gives reply 6, 0x6a925d sets it with confidence +1250. Name: verified.
    pub(crate) const EXTRA_TIME_GRANTED: u32 = 0x1;
    /// bits 1..4 (signed 4-bit) hold the id of the last board request: handle_board_request 0x6a892c..0x6a8961. Name: read.
    pub(crate) const LAST_REQUEST: u32 = 0x1e;
}

/// Bits of `search_result::FLAGS`.
#[allow(dead_code)]
pub(crate) mod search_result_flags {
    /// contains_player returns 0 for a found result with bit 0 set (the port calls it removed). Name: inferred.
    pub(crate) const REMOVED: u32 = 0x1;
}

/// Bits of `shortlist_slot::FLAGS`.
#[allow(dead_code)]
pub(crate) mod shortlist_slot_flags {
    /// existing comment: bit 0 loan slot; loan slots end in loan_listed_value_end. Name: read.
    pub(crate) const LOAN_SLOT: u8 = 0x1;
    /// with the bit the price is doubled; existing comment bit 5 bid marker. Name: inferred.
    pub(crate) const BID_MARKER: u8 = 0x20;
}

/// Bits of `formation_fields::SLOT_CELLS`.
#[allow(dead_code)]
pub(crate) mod slot_cells {
    /// same position code as position_mask (player_sheet DEFENDER 4, DEFENSIVE_MIDFIELDER 8, LEFT_SIDE 0x80, CENTRAL 0x200, RIGHT_SIDE 0x800); split by formation_split_position_code into sector and side. Name: read.
    pub(crate) const DEFENDER: u32 = 0x4;
    /// same position code as position_mask (player_sheet DEFENDER 4, DEFENSIVE_MIDFIELDER 8, LEFT_SIDE 0x80, CENTRAL 0x200, RIGHT_SIDE 0x800); split by formation_split_position_code into sector and side. Name: read.
    pub(crate) const DEFENSIVE_MIDFIELDER: u32 = 0x8;
    /// same position code as position_mask (player_sheet DEFENDER 4, DEFENSIVE_MIDFIELDER 8, LEFT_SIDE 0x80, CENTRAL 0x200, RIGHT_SIDE 0x800); split by formation_split_position_code into sector and side; 0x880 = either flank. Name: read.
    pub(crate) const LEFT: u32 = 0x80;
    /// same position code as position_mask (player_sheet DEFENDER 4, DEFENSIVE_MIDFIELDER 8, LEFT_SIDE 0x80, CENTRAL 0x200, RIGHT_SIDE 0x800); split by formation_split_position_code into sector and side. Name: read.
    pub(crate) const CENTRE: u32 = 0x200;
    /// same position code as position_mask (player_sheet DEFENDER 4, DEFENSIVE_MIDFIELDER 8, LEFT_SIDE 0x80, CENTRAL 0x200, RIGHT_SIDE 0x800); split by formation_split_position_code into sector and side; 0x880 = either flank. Name: read.
    pub(crate) const RIGHT: u32 = 0x800;
}

/// Bits of `league::SQUAD_FLAGS`.
#[allow(dead_code)]
pub(crate) mod squad_flags {
    /// with it the league assigns squad numbers (squad_manager_assign_league_squad_numbers) days before the first round. Name: read.
    pub(crate) const FIXED_SQUAD_NUMBERS: u8 = 0x2;
}

/// Bits of `fixture::SQUAD_RULES`.
#[allow(dead_code)]
pub(crate) mod squad_rules {
    /// refresh_minor_squads: bit 0x100 refreshes the B/Olympic squad (type 1). Name: read.
    pub(crate) const B_SQUAD_MATCH: u16 = 0x100;
    /// refresh_minor_squads: bit 0x200 refreshes the under-21 squad (type 2). Name: read.
    pub(crate) const U21_SQUAD_MATCH: u16 = 0x200;
}

/// Bits of `staff_state::STATUS_FLAGS`.
#[allow(dead_code)]
pub(crate) mod staff_state_status_flags {
    /// == 0: not on any AI shortlist. Name: read.
    pub(crate) const AI_SHORTLIST_COUNT: u8 = 0xf;
    /// & 0x30 == 0x10 skips the player; existing comment: 0x10 announced. Name: read.
    pub(crate) const RETIREMENT_ANNOUNCED: u8 = 0x10;
    /// set (0x10 cleared) when the person retires; docs/MARCA_DO_REGEN 0x20 retired. Name: verified.
    pub(crate) const RETIRED: u8 = 0x20;
    /// docs/MARCA_DO_REGEN: bits 0x30 retirement state (0x10 announced, 0x20 retired); tested as a whole mask. Name: verified.
    pub(crate) const RETIREMENT_STATE: u8 = 0x30;
    /// docs/MARCA_DO_REGEN: bit 0x40 marks the regen; set when the youth regen is created. Name: verified.
    pub(crate) const REGEN_MARK: u8 = 0x40;
}

/// Bits of `tactics_fields::FLAGS`.
#[allow(dead_code)]
pub(crate) mod tactics_fields_flags {
    /// pressiona = ... || est & 0x4000 (team-wide pressing). Name: inferred.
    pub(crate) const PRESSING: u32 = 0x4000;
}

/// Bits of `engine::TACTICS_FLAGS`.
#[allow(dead_code)]
pub(crate) mod tactics_flags {
    /// tested on the defending side: the offside distance grows by one; match_pl narrates BREAKS_THE_OFFSIDE_TRAP (events_eng.cfg) when it is set. Name: verified.
    pub(crate) const OFFSIDE_TRAP: u32 = 0x400;
    /// with it set the player's row follows the ball line (row +-2..4); in off_ball_decide (line 7938) est & 0x4000 alone makes the player press. Name: inferred.
    pub(crate) const PRESSING: u32 = 0x4000;
}

/// Bits of `cup_round::TIE_RULES`.
#[allow(dead_code)]
pub(crate) mod tie_rules {
    /// no fixture news here: the matches move to the fixed slot dates (line 3016 tests the same bit) and get their news there.4 test at 0x51EC41. Name: verified.
    pub(crate) const FIXED_DATES: i16 = 0x100;
    /// no draw: ties in the order built.4, CAMPOS.md. Name: verified.
    pub(crate) const FIXED_BRACKET: i16 = 0x200;
    /// calls 0x51d2d0 sort_round_teams, the seeded draw. Name: verified.
    pub(crate) const SEEDED_DRAW: i16 = 0x400;
}

/// Bits of `contract::TRANSFER_STATUS`.
#[allow(dead_code)]
pub(crate) mod transfer_status {
    /// staff_is_loan_listed (database.rs) tests TRANSFER_STATUS & 2. Name: verified.
    pub(crate) const LOAN_LISTED: u8 = 0x2;
    /// staff_is_transfer_listed_or_requested tests & 9 and staff_is_transfer_listed & 1, so 8 is the request. Name: inferred.
    pub(crate) const TRANSFER_REQUESTED: u8 = 0x8;
}

/// Bits of `match_spectators::UPDATE_FLAGS`.
#[allow(dead_code)]
pub(crate) mod update_flags {
    /// set when engine.score() exceeds the home goals in the summary (match_summary::SIDES). Name: read.
    pub(crate) const HOME_GOAL: u8 = 0x1;
    /// set when engine.score_side1() exceeds the away goals (match_record AWAY_TEAM). Name: read.
    pub(crate) const AWAY_GOAL: u8 = 0x2;
    /// set when engine clock / 11 passes the summary MINUTE. Name: read.
    pub(crate) const MINUTE_CHANGED: u8 = 0x4;
    /// set (state = 8) when an event is shown; while set the step waits for the commentary line to finish, then shows the next event. Name: read.
    pub(crate) const EVENT_PENDING: u8 = 0x8;
    /// set when +0x115 is set: the shown event had priority 2 (match_events_get_event_priority). Name: read.
    pub(crate) const HIGHLIGHT: u8 = 0x10;
    /// set when the engine step state is 4 (half time per the routine header). Name: read.
    pub(crate) const HALF_TIME: u8 = 0x20;
    /// set when the day match STATE is 8 (or 2 at line 1741). Name: read.
    pub(crate) const MATCH_ENDED: u8 = 0x40;
    /// set when the engine restart type is not OPEN_PLAY. Name: read.
    pub(crate) const SET_PIECE: u8 = 0x80;
}
