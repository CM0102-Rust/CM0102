



// The virtual method slots the Rust code calls, by name: the offset of each method in the class's vtable, read
// from the exe's .rdata (engenharia-reversa/nomear_vtables.py; levas/reorganizacao/vtables.json; method names from
// the community function map). `read_usize(read_usize(comp) + comp_vtable::CREATE_SCHEDULE)` is the address of
// the competition's create_schedule override.

/// Virtual method slots of Comp (League, Cup and subclasses, Friendly): slots 0x00..0x8c shared by every competition.
#[allow(dead_code)]
pub(crate) mod comp_vtable {
    /// Slot 0x0: 0x48c600 Comp::scalar_deleting_destructor, 0x518670 Cup::`scalar deleting destructor', 0x5230c0 Cup_Stage::`scalar deleting destructor'. Name: verified.
    pub(crate) const SCALAR_DELETING_DESTRUCTOR: usize = 0x0;
    /// Slot 0x4: 0x685d30 League::add_result, 0x51a150 Cup::process_fixture, 0x48c690 Comp::add_result. Name: verified.
    pub(crate) const ADD_RESULT: usize = 0x4;
    /// Slot 0x8: 0x682760 League::start_new_season, 0x5a8f60 shared_noop_ret0_5a8f60, 0x518780 Cup::start_new_season. Name: verified.
    pub(crate) const START_NEW_SEASON: usize = 0x8;
    /// Slot 0x10: 0x689ad0 League::check_complete, 0x521e00 Cup::check_complete, 0x5b9420 stub_return_zero_ret8. Name: verified.
    pub(crate) const CHECK_COMPLETE: usize = 0x10;
    /// Slot 0x14: 0x6847c0 League::update_for_date, 0x51f890 Cup::update_for_date, 0x48ce80 Comp::update_for_date. Name: verified.
    pub(crate) const UPDATE_FOR_DATE: usize = 0x14;
    /// Slot 0x1c: 0x6891c0 League::save_to_temp_file, 0x5210f0 Cup::save_cup_to_disk, 0x48cea0 shared_noop_ret4_48cea0. Name: verified.
    pub(crate) const SAVE_TO_TEMP_FILE: usize = 0x1c;
    /// Slot 0x28: 0x5a8f60 shared_noop_ret0_5a8f60, 0x401e10 AfricanNations::advance_stage, 0x40c070 AsiaClubChamp::advance_stage. Name: verified.
    pub(crate) const ADVANCE_STAGE: usize = 0x28;
    /// Slot 0x30: 0x684640 League::record_season_honours, 0x519a90 Cup::record_season_honours, 0x5a8f60 shared_noop_ret0_5a8f60. Name: verified.
    pub(crate) const RECORD_SEASON_HONOURS: usize = 0x30;
    /// Slot 0x3c: 0x48c6a0 Comp::create_schedule, 0x682650 League::create_schedule, 0x5186a0 Cup::create_round_table. Name: verified.
    pub(crate) const CREATE_SCHEDULE: usize = 0x3c;
    /// Slot 0x40: 0x686670 League::announce_team_status, 0x519690 Cup::add_team_result_news, 0x48c6b0 Comp::announce_team_status. Name: verified.
    pub(crate) const ANNOUNCE_TEAM_STATUS: usize = 0x40;
    /// Slot 0x44: 0x519890 Cup::update_staff_records_for_status, 0x686940 League::update_staff_records_for_status, 0x48c6c0 Comp::update_staff_records_for_status. Name: verified.
    pub(crate) const UPDATE_STAFF_RECORDS_FOR_STATUS: usize = 0x44;
    /// Slot 0x54: 0x68a3e0 League::get_match_importance, 0x5221f0 Cup::get_match_importance, 0x48c5f0 Comp::get_match_importance. Name: verified.
    pub(crate) const GET_MATCH_IMPORTANCE: usize = 0x54;
    /// Slot 0x5c: 0x68a850 League::set_predicted_positions, 0x5223a0 Cup::rank_teams, 0x5a53c0 shared_noop_ret0_5a53c0. Name: verified.
    pub(crate) const SET_PREDICTED_POSITIONS: usize = 0x5c;
    /// Slot 0x60: 0x48e360 Comp::counts_for_fixture_clash, 0x5839d0 shared_return_zero_ret8. Name: verified.
    pub(crate) const COUNTS_FOR_FIXTURE_CLASH: usize = 0x60;
    /// Slot 0x64: 0x5bb550 stub_return_one_ret4, 0x5b9430 stub_return_zero_ret4. Name: read.
    pub(crate) const ACCEPTS_FIXTURE: usize = 0x64;
    /// Slot 0x68: 0x48e380 Comp::set_club_progress, 0x432af0 BraRegGaucho::set_club_progress, 0x4241b0 BelThird::set_club_progress. Name: verified.
    pub(crate) const SET_CLUB_PROGRESS: usize = 0x68;
    /// Slot 0x6c: 0x583470 shared_noop_ret4. Name: read.
    pub(crate) const PAY_STAGE_PRIZE_MONEY: usize = 0x6c;
    /// Slot 0x70: 0x4011c0 Comp::get_qualification_status, 0x911b80 WcAfricanCup::get_qualification_status, 0x57cd30 EuroChamp::get_qualification_status. Name: verified.
    pub(crate) const GET_QUALIFICATION_STATUS: usize = 0x70;
    /// Slot 0x74: 0x4011d0 Comp::get_qualification_status_change, 0x920260 WcOceaniaLeague::get_qualification_status_change, 0x57cd80 EuroChamp::get_qualification_status_change. Name: verified.
    pub(crate) const GET_QUALIFICATION_STATUS_CHANGE: usize = 0x74;
    /// Slot 0x7c: 0x48f2d0 Comp::set_fixture_substitutes, 0x5bb100 FraCup::set_fixture_substitutes, 0x4124e0 AusNsl::set_fixture_substitutes. Name: verified.
    pub(crate) const SET_FIXTURE_SUBSTITUTES: usize = 0x7c;
    /// Slot 0x88: 0x5dce00 shared_return_false, 0x404480 shared_return_true. Name: read.
    pub(crate) const HAS_HOST_COUNTRY: usize = 0x88;
    /// Slot 0x8c: 0x48ce90 shared_return_one, 0x6826f0 League::set_up_rules, 0x518710 Cup::set_up_rounds. Name: verified.
    pub(crate) const SET_UP_RULES: usize = 0x8c;
}

/// Virtual method slots of League and subclasses (League_Stage): slots past Comp, 0x90..0xb0.
#[allow(dead_code)]
pub(crate) mod league_vtable {
    /// Slot 0x90: 0x68aad0 League::points_for_result, 0x43c4a0 BraRegSp::points_for_result, 0x66d770 KorLeagueCup::points_for_result. Name: verified.
    pub(crate) const POINTS_FOR_RESULT: usize = 0x90;
    /// Slot 0x94: 0x684cd0 League::update_positions_and_status, 0x420680 BelSecond::update_positions_and_status, 0x422f70 BelThird::update_positions_and_status. Name: verified.
    pub(crate) const UPDATE_POSITIONS_AND_STATUS: usize = 0x94;
    /// Slot 0x98: 0x684790 League::has_automatic_promotion, 0x404480 shared_return_true. Name: verified.
    pub(crate) const HAS_AUTOMATIC_PROMOTION: usize = 0x98;
    /// Slot 0x9c: 0x6847b0 League::has_relegation, 0x404480 shared_return_true, 0x5dce00 shared_return_false. Name: verified.
    pub(crate) const HAS_RELEGATION: usize = 0x9c;
    /// Slot 0xa4: 0x68aa30 League::record_final_positions, 0x650050 ItaSerC1b::record_final_positions, 0x901ba0 TurSecondB::record_final_positions. Name: verified.
    pub(crate) const RECORD_FINAL_POSITIONS: usize = 0xa4;
    /// Slot 0xb0: 0x689c20 League::process_promotion_relegation_chain, 0x4067f0 ArgPrm::process_promotion_relegation_chain, 0x41ee80 BelFirst::process_promotion_relegation_chain. Name: verified.
    pub(crate) const PROCESS_PROMOTION_RELEGATION_CHAIN: usize = 0xb0;
}

/// Virtual method slots of Cup and subclasses (Cup_Stage): slots past Comp, 0x90..0x9c (different methods from League at the same offsets).
#[allow(dead_code)]
pub(crate) mod cup_vtable {
    /// Slot 0x90: 0x522360 Cup::club_in_higher_division, 0x5bb030 FraCup::club_in_higher_division, 0x5d9930 GerCup::club_in_higher_division. Name: verified.
    pub(crate) const CLUB_IN_HIGHER_DIVISION: usize = 0x90;
    /// Slot 0x94: 0x518790 Cup::map_fixture_tree, 0x5241e0 Cup_Stage::map_fixture_tree. Name: verified.
    pub(crate) const MAP_FIXTURE_TREE: usize = 0x94;
}

/// Virtual method slots of TRANSFER_RULES (per-nation transfer rules, TransferManager +TRANSFER_RULES, 29 vtables of 13 slots).
#[allow(dead_code)]
pub(crate) mod transfer_rules_vtable {
    /// Slot 0x0: 0x583470 shared_noop_ret4, 0x57a020 ENGLAND_RULES::daily_update, 0x40a710 ARGENTINA_RULES::daily_update. Name: verified.
    pub(crate) const DAILY_UPDATE: usize = 0x0;
    /// Slot 0x4: 0x583470 shared_noop_ret4, 0x40a810 ARGENTINA_RULES::record_transfer, 0x5c17d0 FRANCE_RULES::record_transfer. Name: verified.
    pub(crate) const RECORD_TRANSFER: usize = 0x4;
    /// Slot 0x8: 0x583470 shared_noop_ret4, 0x579fd0 ENGLAND_RULES::record_loan, 0x63c4e0 IRELAND_RULES::record_loan. Name: verified.
    pub(crate) const RECORD_LOAN: usize = 0x8;
    /// Slot 0xc: 0x8e8800 TRANSFER_RULES::can_make_offer, 0x424eb0 BELGIUM_RULES::can_make_offer, 0x40a900 ARGENTINA_RULES::can_make_offer. Name: verified.
    pub(crate) const CAN_MAKE_OFFER: usize = 0xc;
    /// Slot 0x10: 0x8e8560 TRANSFER_RULES::can_sign_player, 0x40a860 ARGENTINA_RULES::can_sign_player, 0x424d30 BELGIUM_RULES::can_sign_player. Name: verified.
    pub(crate) const CAN_SIGN_PLAYER: usize = 0x10;
    /// Slot 0x18: 0x8e8a60 TRANSFER_RULES::get_ineligibility_code, 0x66e770 KOREA_RULES::get_ineligibility_code, 0x8595e0 SPAIN_RULES::get_ineligibility_code. Name: verified.
    pub(crate) const GET_INELIGIBILITY_CODE: usize = 0x18;
    /// Slot 0x1c: 0x5832c0 TRANSFER_RULES::fill_squad_restrictions, 0x63c330 IRELAND_RULES::fill_squad_restrictions, 0x579c50 ENGLAND_RULES::fill_squad_restrictions. Name: verified.
    pub(crate) const FILL_SQUAD_RESTRICTIONS: usize = 0x1c;
    /// Slot 0x20: 0x40a600 TRANSFER_RULES::can_loan_player, 0x793900 NORTHERN_IRELAND_RULES::can_loan_player, 0x412dd0 AUSTRALIA_RULES::can_loan_player. Name: verified.
    pub(crate) const CAN_LOAN_PLAYER: usize = 0x20;
    /// Slot 0x24: 0x40a610 TRANSFER_RULES::can_end_loan, 0x5a9ba0 FINLAND_RULES::can_end_loan, 0x5c1ed0 FRANCE_RULES::can_end_loan. Name: verified.
    pub(crate) const CAN_END_LOAN: usize = 0x24;
    /// Slot 0x28: 0x8e8bd0 TRANSFER_RULES::get_loan_period, 0x90f1a0 WALES_RULES::get_loan_period, 0x7ec8e0 RUSSIA_RULES::get_loan_period. Name: verified.
    pub(crate) const GET_LOAN_PERIOD: usize = 0x28;
    /// Slot 0x2c: 0x8e9080 TRANSFER_RULES::post_deadline_news, 0x424fa0 BELGIUM_RULES::post_deadline_news, 0x5c1a70 FRANCE_RULES::post_deadline_news. Name: verified.
    pub(crate) const POST_DEADLINE_NEWS: usize = 0x2c;
    /// Slot 0x30: 0x8e91b0 TRANSFER_RULES::write_deadline_news_text, 0x40aa30 ARGENTINA_RULES::write_deadline_news_text, 0x4250b0 BELGIUM_RULES::write_deadline_news_text. Name: verified.
    pub(crate) const WRITE_DEADLINE_NEWS_TEXT: usize = 0x30;
}

/// Virtual method slots of RulingBody (Discipline rules, 44 vtables of 8 slots).
#[allow(dead_code)]
pub(crate) mod ruling_body_vtable {
    /// Slot 0x4: 0x7e7d50 RulingBody::daily_update. Name: verified.
    pub(crate) const DAILY_UPDATE: usize = 0x4;
}

