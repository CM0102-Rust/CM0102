use super::*;

/// Rule: the Belgian Third Division turnover (called by the First Division's, after the Third Division was regrouped):
/// the season reopened, last season's stages destroyed, the next year; the rules, the table, the order and the
/// fixtures rebuilt; the group B stage built again; the predicted positions and the officials (no board expectations).
///
/// `league::SEASON_CLOSED` = 0; League::free_table (`exe::league_free_table`(0)); the round dates freed and cleared;
/// with statistics, `exe::comp_stats_archive_season_and_reset`(1); with a current stage ≥ 0, the stages 0..=current
/// (re-read on each step) destroyed by their virtual 0 with 1; the year + 1 and the current stage −1. Then, each with
/// the box of bel_third.cpp (0x989d04) at its line and 0 returned when it returns 0, in this order (the order and the
/// fixtures swapped against the other leagues): the rules (virtual `comp_vtable::SET_UP_RULES`, 0x29e),
/// BelThird::fill_league_table (0x4224e0, thiscall, ret 0; 0x2a5), League::sort_table (`exe::league_sort_table`(0),
/// 0x2ac) and League::create_fixtures (0x2b2). Then BelThird::create_group_b_stage (0x422970, thiscall, ret 0), the
/// predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`) and OfficialsManager::renew_officials on
/// `G_OFFICIALS_MANAGER` with the record, whose eax is returned.
/// Original: `BelThird::start_new_season` (0x4227b0, bel_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_third_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    this.set_year(this.year().wrapping_add(1));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    let file = 0x989d04;
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, 0x29e);
        return 0;
    }
    if exe::belthird_fill_league_table(this) == 0 {
        show_error_at(file, 0x2a5);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x2ac);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x2b2);
        return 0;
    }
    covered!("mwc_19 belgian third division turnover");
    exe::belthird_create_group_b_stage(this);
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32
}

/// Comparer for `bel_third_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_bel_third_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_THIRD_START_NEW_SEASON);
    compare_state_league_new_season("bel_third_start_new_season", League(this), || o(this))
}
