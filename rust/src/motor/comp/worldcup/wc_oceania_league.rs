use super::*;

/// rule: in the Oceania qualifier groups, a team going through gets the qualification (weight 20) and one going out the
/// failed-to-qualify record and ends its campaign; in the Oceania final, the winner gets the qualification (weight 30)
/// and the loser the knock-out; in the play-off (against South America [inference]), the winner gets the World Cup
/// place (weight 100 record, the entry in its list and status 4) and the loser the knock-out (weight 100) and the end
/// of the campaign
/// Original: `WcOceaniaLeague::update_staff_records_for_status` (0x91e840, wc_oceania_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, _a6: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    let eliminar = |weight: u32| {
        // simplification: +0x30 and +0x32 go in registers with garbage in the high 16 bits; 0x86c000 only uses the words
        european_cup_add_knocked_out_record(club, comp, read_i16(game + fixture::ROUND_PREFIX) as u16 as u32, read_i16(game + fixture::ROUND_NAME) as u16 as u32, weight);
        clear_national_team_competition(Club(club));
    };
    if f < 1 {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x82, 0x14);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x14);
            clear_national_team_competition(Club(club));
        }
    } else if f == 1 {
        if d == 1 {
            european_cup_add_qualified_record(club, comp, 0xa0, 0, 0x1e);
        } else if d == 2 || d == 0xff {
            eliminar(0x1e);
        }
    } else if f == 2 {
        if d == 1 {
            covered!("cqj_10 oceania in the world cup");
            let id = read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP);
            european_cup_add_qualified_record(club, read_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(0x6b)), 0, 0, 0x64);
            exe::worldcup_add_qualified_team(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(id.wrapping_mul(4))), club);
            write_u8(out, 4);
        } else if d == 2 {
            eliminar(0x64);
        }
    }
    0
}

/// rule: the Oceania qualifier groups are single round robin, 3/1 points, tiebreak 1, 2, 3, one qualifier per group (to
/// the Oceania final), category 10, 7 substitutes and 3 changes, starting on 15 of month 0 on weekday 5
/// Original: `WcOceaniaLeague::set_up_rules` (0x91ecf0, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_set_up_rules(this: usize) -> u32 {
    covered!("cqk_10 oceania rules");
    goldcup_write_rule_fields(this, &OCEANIA_RULE_FIELDS, 3);
    let day = 0xf; // cqk10: the start day in Oceania
    let word = competition_date_word(League(this), day, 0, 0, 5);
    finish_rules_setup(League(this), word, 0)
}
