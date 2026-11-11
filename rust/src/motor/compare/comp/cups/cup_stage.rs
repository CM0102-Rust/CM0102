use super::*;

// The comparers of the routines in ../comp/cups/cup_stage.rs.

/// Comparer for `construct_cup_stage`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_construct_cup_stage(this: usize, parent: usize, n: u32, teams: usize, rounds: u32, a5: u32, template: usize, year: u32, stage: u32, b2: u32,
    id: u32, b3: u32, arvore: u32, by_order: u32, a14: u32, seeds: usize) -> usize {
    let this = Cup(this);
    #[allow(clippy::type_complexity)]
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize>(IDX_CONSTRUCT_CUP_STAGE);
    let k = (n as u16 as i16).max(0) as usize;
    let plan: Vec<(usize, u8)> = (0..k).map(|i| {
        let s = if seeds != 0 { read_u8(seeds + i) } else if by_order != 0 { i as u8 } else { 0 };
        (read_usize(teams + i * 4), s)
    }).collect();
    let size = (rounds as u16 as i16 as i32 * 0x68).max(0) as usize;
    let template_before: Vec<u8> = core::slice::from_raw_parts(template as *const u8, size).to_vec();
    let r = o(this.addr(), parent, n, teams, rounds, a5, template, year, stage, b2, id, b3, arvore, by_order, a14, seeds);
    let l = this.team_table();
    let t = this.round_table();
    // the tree (called at the end) rewrites parts of the round table and of the list (in the 06/11 comparison the two
    // differed from what the constructor writes): checks the fixed fields, the built table and the set of clubs in the list
    let _ = &template_before;
    let ok = r == this.addr() && this.team_count() as u16 == n as u16 && this.round_count() as u16 == rounds as u16 && this.year() as u16 == year as u16
        && t != 0 && l != 0 && {
            let mut a: Vec<usize> = (0..plan.len()).map(|i| read_usize(l + i * 6)).collect();
            let mut b: Vec<usize> = plan.iter().map(|x| x.0).collect();
            a.sort_unstable();
            b.sort_unstable();
            a == b
        };
    SCOREBOARD.record(ok, || format!("construct_cup_stage({:#x}) n={} rounds={} year={}", this.addr(), n as u16 as i16, rounds as u16 as i16, year as u16));
    r
}

/// Comparer for `destroy_cup_stage`.
pub(crate) unsafe extern "thiscall" fn compare_destroy_cup_stage(this: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_DESTROY_CUP_STAGE)(this)
}
