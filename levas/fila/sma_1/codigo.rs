// ---------- batch %%LEVA%%: how well a staff member fits a job at a club (manager_manager.cpp) ----------

/// The `.rdata` doubles of `score_staff_for_job` that `rdata_consts.rs` does not name yet (value in cm0102.exe 3.9.68;
/// still read from the exe).
#[allow(dead_code)]
pub(crate) mod staff_job_score_rdata {
    pub(crate) const F64_250: usize = 0x968898; // f64 = 250.0
    pub(crate) const F64_0_0013333333333: usize = 0x96d250; // f64 = 0.0013333333333333333
    pub(crate) const F64_0_0044444444444: usize = 0x96d258; // f64 = 0.0044444444444444444
    pub(crate) const F64_0_475: usize = 0x96d270; // f64 = 0.475
}

/// The nine non-player attributes the score compares between the person and the viewer (the club's chairman), in the
/// order the original goes through them: attacking, coaching technique, directness, discipline, free roles, marking,
/// offside, pressing, youngsters.
const STAFF_JOB_SIMILARITY: [usize; 9] = [non_player::ATTACKING, non_player::COACHING_TECHNIQUE, non_player::DIRECTNESS, non_player::DISCIPLINE,
    non_player::FREE_ROLES, non_player::MARKING, non_player::OFFSIDE, non_player::PRESSING, non_player::YOUNGSTERS];

/// `v` × (`a` × k + 1), truncated (`ftol`): one attribute weight of the score (k read from the exe at `k`).
pub(crate) unsafe fn staff_job_weight(v: i32, a: i8, k: usize) -> i32 {
    ftol((a as i32 as f64 * read_f64(k) + read_f64(rdata::F64_1)) * v as i32 as f64)
}

/// The person's four attitudes (loyalty, determination, pressure, professionalism), each × ((value − 10) × k + 1),
/// truncated after each.
pub(crate) unsafe fn staff_job_attitudes(p: Staff, mut v: i32, k: usize) -> i32 {
    for a in [p.loyalty(), p.determination(), p.pressure(), p.professionalism()] {
        v = staff_job_weight(v, a.wrapping_sub(10), k);
    }
    v
}

/// The similarity with the viewer: `v` × 1.5 (truncated), then for each of `STAFF_JOB_SIMILARITY` × (1 − |difference|
/// × k) (`ks[i]`, an exe address), or, with `div` (the manager jobs), × (1 − |difference| / (F × div[i])), truncated
/// after each.
pub(crate) unsafe fn staff_job_similarity(v: i32, np: usize, viewer_np: usize, ks: &[usize; 9], div: Option<(f64, [usize; 9])>) -> i32 {
    let mut v = ftol(v as f64 * read_f64(rdata::F64_1_5));
    for (i, &a) in STAFF_JOB_SIMILARITY.iter().enumerate() {
        let d = ((read_i8(viewer_np + a) as i32) - (read_i8(np + a) as i32)).abs() as f64;
        let s = match div {
            Some((f, ds)) => read_f64(rdata::F64_1) - d / (f * read_f64(ds[i])),
            None => read_f64(rdata::F64_1) - d * read_f64(ks[i]),
        };
        v = ftol(v as f64 * s);
    }
    v
}

/// `max(v × k, v + 2000)` truncated: the bonus of a favourite (k = 1.1) or of a national team's own nation (k = 1.2).
pub(crate) unsafe fn staff_job_bonus(v: i32, k: usize) -> i32 {
    let a = v as f64 * read_f64(k);
    let b = v.wrapping_add(2000) as f64;
    ftol(if a > b { a } else { b })
}

/// How well `staff` fits job `job` at `club`, as seen by `viewer` (a staff member; by default the club's chairman):
/// −10000 for "never". Used by the manager market (`consider_sacking_manager`, `add_job_vacancy`,
/// `best_manager_candidate_score`, 0x69ca00) and by the shortlists of the staff search.
///
/// Without a non-player sheet (`.non_player()`): −10000. A viewer without a sheet is replaced by the club's chairman
/// (`.chairman()`), and without one either, no viewer. The person's role: `.job_for_club()`, else `.job_for_nation()`,
/// else 11. The score starts at the reputation as the viewer sees it (`exe::database_get_staff_reputation_for_viewer`,
/// non-player; without a viewer, a local staff record with id 0, the club's nation and the club as its club job, as the
/// original builds it). In role 12 or 5 at a club with reputation (`.reputation()`) above 4250: + current ability × 25,
/// and with world reputation above 3250 also + potential ability × 5. Already at the club (`.club_job()`): + 100 for a
/// human manager (id ≥ `G_STAFF_COUNT` − 16), otherwise + loyalty². A favourite of the club
/// (`exe::club_favours_staff`): max(× 1.1, + 2000); otherwise rejected by it (`exe::club_dislikes_staff`): × 0.25. The
/// same with the viewer (`exe::staff_is_favoured_by`, `exe::staff_is_disliked_by`).
///
/// By the job (other jobs and 11: −10000):
/// - 5 and 12 (manager jobs): a human manager skips straight to the common part. By the role: 12 × 0.95; 6 and 13 at
///   the same club or national team (`.national_job()`) × 0.75, otherwise × 0.6 and −10000 above reputation 5750; 7 ×
///   0.5 / × 0.3 and 5250; 8 × 0.6 / × 0.475 and 5250; 11 × 0.95 / × 0.75 and 5250; 15 × 0.6 / × 0.4 and 5750; 16 ×
///   0.75 and, outside the club, 5750; 9 × 0.2 and 3250; 10, 14 and the others × 0.001 and 5750; 5 nothing. Age
///   (`.age()`) above 59: × 0.875 at a national team above reputation 7000, × 0.9 above 7250, else × 0.95; below 38
///   with world reputation below 4250: × (age × 0.05 − 0.95). Below 45 at a club below reputation 7000: + min(potential
///   − current, 50)² × the viewer's patience (10 without a viewer) × 0.1. With a viewer, the similarity (difference
///   divided by F × 100, 250, 100, 150, 150, 300, 300, 200, 150, F = 1.25 − the viewer's interference × 0.025); the
///   weights coaching goalkeepers and coaching (× 0.000833), judgement, judging potential, man handling, motivating
///   (× 0.002), tactics (× 0.008 above reputation 7250, × 0.004 above 5750, else × 0.002); the attitudes × 0.005.
/// - 6, 7, 13, 14 (coaches): similarity × 0.00667, 0.00667, 0.01, 0.00444, 0.00667, 0.01, 0.01, 0.00667, 0.004;
///   weights × 0.002, 0.002, 0.00133, 0.00133, 0.000667, 0.00133, tactics × 0.002; attitudes × 0.005.
/// - 8, 15 (assistant jobs): similarity as the coaches but coaching technique × 0.01; weights × 0.005, 0.005, 0.00133,
///   0.00133, 0.00133, 0.000833, tactics × 0.004; attitudes × 0.02.
/// - 9 (scout): similarity × 0.002 each; judgement and judging potential × 0.005; attitudes × 0.002.
/// - 10 (physio): physiotherapy × 0.01; attitudes × 0.02.
///
/// The common part. A club at the person's nation, at the nation of the club job or of the national job (the first one
/// whose nation is the club's), else the person's nation. A foreigner (`exe::database_is_staff_foreign`, groups) at a
/// club above reputation 4750 whose club job is not in the club's country, coming from a weaker league
/// (`.league_standard()` of his nation or of his club's nation, the larger, below the club nation's) and not from a
/// club with reputation 6750 or more: −10000 outside roles 12 and 5, there × max(1 + difference × 0.025, 0.75).
/// National team (`exe::database_is_national_team`): its own nation max(× 1.2, + 2000); not foreign × (0.75 −
/// reputation × 0.000025); foreign × 0.35; above reputation 6250 × (world reputation × 0.000025 + 0.5) outside 12/5,
/// foreign × (× 0.00005 + 0.5), otherwise × (× 0.000025 + 0.75). A human manager there: by the reputation the viewer
/// sees, below 4250 the score is 0, below 6750 × 0.75; below 7250 and before 23/6/2003 × 0.75. Club above reputation
/// 7250 with a division and a nation of league standard above 10: + (the club job's reputation, or the world
/// reputation, − the club's) × 0.15 (truncated to a word); above 8250 with world reputation below 4250: −10000 for a
/// foreigner, someone without a club job or division, or from a division below the club's (`.reputation()` of
/// `.division()`); then, outside job 9, the reputation factor as at the national team. Other clubs: a foreigner outside
/// job 9 × (world reputation × 0.00005 + 0.5). Then × (language fluency × 0.05 + 0.5) (`exe::playerstats_get_language_fluency`
/// in the club nation's `.first_language()`, by contract). Last, the chosen nation's `.state_of_development()`: 1 as
/// is; 2 × 0.75 when the club's nation is 1; 3 × 0.25 when the club's nation is 1 or 2; other × 0.05; no nation × 0.25.
/// Defect of the original: without any of those nations the routine reads the development of a stack value (the
/// person's professionalism − 10, or garbage) as a nation; every staff member has a nation, so the copy takes "no
/// nation" there.
/// Original: `score_staff_for_job` (0x69d1b0, manager_manager.cpp, stdcall, ret 0x10; ecx unused).
pub(crate) unsafe extern "thiscall" fn score_staff_for_job(_this: usize, staff: usize, club: usize, viewer: usize, job: u32) -> i32 {
    use staff_job_score_rdata::*;
    const NEVER: i32 = -10000;
    let p = Staff(staff);
    let club = Club(club);
    let np = p.non_player().addr();
    if np == 0 {
        return NEVER;
    }
    let mut viewer = Staff(viewer);
    if viewer.is_null() || viewer.non_player().is_null() {
        viewer = club.chairman();
        if viewer.is_null() || viewer.non_player().is_null() {
            viewer = Staff(0);
        }
    }
    let age = p.age() as i8;
    let mut role = p.job_for_club();
    if role == 0 {
        role = p.job_for_nation();
        if role == 0 {
            role = 11;
        }
    }
    let manager_role = role == 12 || role == 5;
    let human = read_i32(p.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10);
    let mut v = if !viewer.is_null() {
        exe::database_get_staff_reputation_for_viewer(p, viewer, 1) as i32
    } else {
        // the local staff record of the original: the dates constructed, id 0, the club's nation, the club as club job
        let mut local = [0u8; staff::SIZE];
        let l = local.as_mut_ptr() as usize;
        for d in [staff::DATE_OF_BIRTH, staff::DATE_JOINED_NATION, 0x31, staff::DATE_JOINED_CLUB, staff::DATE_EXPIRES_CLUB] {
            exe::date_date_54a110(l + d);
        }
        write_u32(l + staff::NATION, club.nation().addr() as u32);
        write_u32(l + staff::CLUB_JOB, club.addr() as u32);
        exe::database_get_staff_reputation_for_viewer(p, l, 1) as i32
    };
    let npr = NonPlayer(np);
    if manager_role && club.reputation() > 0x109a {
        v += npr.current_ability() as i32 * 25;
        if npr.world_reputation() > 0xcb2 {
            v += read_i16(np + non_player::POTENTIAL_ABILITY) as i32 * 5;
        }
    }
    if p.club_job() == club {
        if human {
            v += 100;
        } else {
            let l = p.loyalty() as i32;
            v += l * l;
        }
    }
    if exe::club_favours_staff(p, club) != 0 {
        v = staff_job_bonus(v, rdata::F64_1_1);
    } else if exe::club_dislikes_staff(p, club) != 0 {
        v = ftol(v as f64 * read_f64(rdata::F64_0_25));
    }
    if exe::staff_is_favoured_by(p, viewer) != 0 {
        v = staff_job_bonus(v, rdata::F64_1_1);
    } else if exe::staff_is_disliked_by(p, viewer) != 0 {
        v = ftol(v as f64 * read_f64(rdata::F64_0_25));
    }
    let mul = |v: i32, k: usize| ftol(v as f64 * read_f64(k));
    let rep = club.reputation();
    let at_club = p.club_job() == club || p.national_job() == club;
    let vnp = if viewer.is_null() { 0 } else { viewer.non_player().addr() };
    use non_player as n;
    let job = job as u8;
    match job {
        5 | 12 => {
            if !human {
                covered!("sma_1 manager job");
                // the role: (factor at the club or national team, factor elsewhere, reputation limit elsewhere)
                let (here, away, limit): (usize, usize, i16) = match role {
                    5 => (0, 0, 0),
                    6 | 13 => (rdata::F64_0_75, rdata::F64_0_6, 0x1676),
                    7 => (rdata::F64_0_5, rdata::F64_0_3, 0x1482),
                    8 => (rdata::F64_0_6, F64_0_475, 0x1482),
                    11 => (rdata::F64_0_95, rdata::F64_0_75, 0x1482),
                    15 => (rdata::F64_0_6, rdata::F64_0_4, 0x1676),
                    12 => (rdata::F64_0_95, rdata::F64_0_95, i16::MAX),
                    16 => (rdata::F64_0_75, rdata::F64_0_75, 0x1676),
                    9 => (rdata::F64_0_2, rdata::F64_0_2, 0xcb2), // also at the club
                    _ => (rdata::F64_0_001, rdata::F64_0_001, 0x1676), // 10, 14 and the others, also at the club
                };
                if role != 5 {
                    let checked_here = matches!(role, 9 | 10 | 14) || !(5..=16).contains(&role);
                    if at_club && !checked_here {
                        v = mul(v, here);
                    } else {
                        v = mul(v, away);
                        if rep > limit {
                            return NEVER;
                        }
                    }
                }
                if age > 0x3b {
                    v = if exe::database_is_national_team(club) != 0 && rep > 0x1b58 {
                        mul(v, rdata::F64_0_875)
                    } else if rep > 0x1c52 {
                        mul(v, rdata::F64_0_9)
                    } else {
                        mul(v, rdata::F64_0_95)
                    };
                } else if age < 0x26 && npr.world_reputation() < 0x109a {
                    v = ftol((age as i32 as f64 * read_f64(rdata::F64_0_05) - read_f64(rdata::F64_0_95)) * v as f64);
                }
                if age < 0x2d && rep < 0x1b58 {
                    covered!("sma_1 young manager potential");
                    let gap = (read_i16(np + n::POTENTIAL_ABILITY) as i32 - npr.current_ability() as i32).min(50);
                    let patience = if vnp != 0 { read_i8(vnp + n::PATIENCE) as i32 } else { 10 };
                    v += ftol(game_pow(gap as f64, read_f64(rdata::F64_2)) * patience as f64 * read_f64(rdata::F64_0_1)); // sma1: the potential bonus
                }
                if vnp != 0 {
                    let f = read_f64(rdata::F64_1_25) - read_i8(vnp + n::INTERFERENCE) as i32 as f64 * read_f64(rdata::F64_0_025);
                    let ds = [rdata::F64_100, F64_250, rdata::F64_100, rdata::F64_150, rdata::F64_150, rdata::F64_300, rdata::F64_300, rdata::F64_200, rdata::F64_150];
                    v = staff_job_similarity(v, np, vnp, &ds, Some((f, ds)));
                }
                for (a, k) in [(n::COACHING_GKS, rdata::F64_0_0008333333333), (n::COACHING, rdata::F64_0_0008333333333), (n::JUDGEMENT, rdata::F64_0_002),
                    (n::JUDGING_POTENTIAL, rdata::F64_0_002), (n::MAN_HANDLING, rdata::F64_0_002), (n::MOTIVATING, rdata::F64_0_002)] {
                    v = staff_job_weight(v, read_i8(np + a), k);
                }
                let k = if rep > 0x1c52 { rdata::F64_0_008 } else if rep > 0x1676 { rdata::F64_0_004 } else { rdata::F64_0_002 };
                v = staff_job_weight(v, read_i8(np + n::TACTICS), k); // sma1: the manager's tactics weight
                v = staff_job_attitudes(p, v, rdata::F64_0_005);
            }
        }
        6 | 7 | 13 | 14 | 8 | 15 => {
            let assistant = job == 8 || job == 15;
            if vnp != 0 {
                let t = if assistant { rdata::F64_0_01 } else { rdata::F64_0_006666666667 };
                let ks = [rdata::F64_0_006666666667, t, rdata::F64_0_01, F64_0_0044444444444, rdata::F64_0_006666666667, rdata::F64_0_01, rdata::F64_0_01,
                    rdata::F64_0_006666666667, rdata::F64_0_004];
                v = staff_job_similarity(v, np, vnp, &ks, None);
            }
            let ws: [(usize, usize); 7] = if assistant {
                [(n::COACHING_GKS, rdata::F64_0_005), (n::COACHING, rdata::F64_0_005), (n::JUDGEMENT, F64_0_0013333333333), (n::JUDGING_POTENTIAL, F64_0_0013333333333),
                    (n::MAN_HANDLING, F64_0_0013333333333), (n::MOTIVATING, rdata::F64_0_0008333333333), (n::TACTICS, rdata::F64_0_004)]
            } else {
                [(n::COACHING_GKS, rdata::F64_0_002), (n::COACHING, rdata::F64_0_002), (n::JUDGEMENT, F64_0_0013333333333), (n::JUDGING_POTENTIAL, F64_0_0013333333333),
                    (n::MAN_HANDLING, rdata::F64_0_0006666666667), (n::MOTIVATING, F64_0_0013333333333), (n::TACTICS, rdata::F64_0_002)]
            };
            for (a, k) in ws {
                v = staff_job_weight(v, read_i8(np + a), k);
            }
            v = staff_job_attitudes(p, v, if assistant { rdata::F64_0_02 } else { rdata::F64_0_005 }); // sma1: the coaches' attitudes
        }
        9 => {
            if vnp != 0 {
                v = staff_job_similarity(v, np, vnp, &[rdata::F64_0_002; 9], None);
            }
            v = staff_job_weight(v, read_i8(np + n::JUDGEMENT), rdata::F64_0_005);
            v = staff_job_weight(v, read_i8(np + n::JUDGING_POTENTIAL), rdata::F64_0_005);
            v = staff_job_attitudes(p, v, rdata::F64_0_002);
        }
        10 => {
            v = staff_job_weight(v, read_i8(np + n::PHYSIOTHERAPY), rdata::F64_0_01);
            v = staff_job_attitudes(p, v, rdata::F64_0_02);
        }
        _ => return NEVER,
    }
    // the nation that sets the development factor at the end
    let cn = club.nation();
    let cj = p.club_job();
    let nj = p.national_job();
    let nation = if !cj.is_null() && cj.nation() == cn {
        cn.addr()
    } else if !nj.is_null() && nj.nation() == cn {
        cn.addr()
    } else {
        p.nation().addr()
    };
    // a foreigner from a weaker league
    if rep > 0x128e && !cn.is_null() && !p.nation().is_null() && exe::database_is_staff_foreign(p, cn, 1) != 0 && (cj.is_null() || cj.nation() != cn) {
        let mut own = p.nation().league_standard() as i8;
        let mut weaker = true;
        if !cj.is_null() {
            let jn = cj.nation();
            if !jn.is_null() && jn.league_standard() as i8 > own {
                own = jn.league_standard() as i8;
            }
            weaker = cj.reputation() < 0x1a5e;
        }
        let theirs = cn.league_standard() as i8;
        if weaker && own < theirs {
            covered!("sma_1 foreigner from a weaker league");
            let mut f = (own as i32 - theirs as i32) as f64 * read_f64(rdata::F64_0_025) + read_f64(rdata::F64_1);
            if f < read_f64(rdata::F64_0_75) {
                f = read_f32(rdata::F32_0_75) as f64;
            }
            if !manager_role {
                return NEVER;
            }
            v = ftol(v as f64 * f);
        }
    }
    let foreign = || exe::database_is_staff_foreign(p, cn, 1) != 0;
    // the reputation factor: (world reputation × k1 + k2) by the role and the nationality
    let rep_factor = |v: i32| {
        let w = npr.world_reputation() as i32 as f64;
        let f = if !manager_role {
            w * read_f64(rdata::F64_2_5ENEG_05) + read_f64(rdata::F64_0_5)
        } else if foreign() {
            w * read_f64(rdata::F64_5ENEG_05) + read_f64(rdata::F64_0_5)
        } else {
            w * read_f64(rdata::F64_2_5ENEG_05) + read_f64(rdata::F64_0_75)
        };
        ftol(f * v as f64)
    };
    if exe::database_is_national_team(club) != 0 {
        covered!("sma_1 national team");
        if p.nation() == cn {
            v = staff_job_bonus(v, rdata::F64_1_2); // sma1: the national team's own nation
        } else if !foreign() {
            v = ftol((read_f64(rdata::F64_0_75) - rep as i32 as f64 * read_f64(rdata::F64_2_5ENEG_05)) * v as f64);
        } else {
            v = mul(v, rdata::F64_0_35);
        }
        if rep > 0x186a {
            v = rep_factor(v);
        }
        if human {
            let r = exe::database_get_staff_reputation_for_viewer(p, viewer, 1);
            if r < 0x109a {
                v = 0;
            } else if r < 0x1a5e {
                v = mul(v, rdata::F64_0_75);
            }
            if r < 0x1c52 {
                let mut d = [0u16; 4];
                exe::date_date_549f70(&mut d, 0x17, 6, 0x7d3, u32::MAX);
                let today = read_u16(G_CURRENT_DATE_YEAR);
                if d[1] > today || (d[1] == today && d[0] as i16 > read_i16(G_CURRENT_DATE)) {
                    v = mul(v, rdata::F64_0_75);
                }
            }
        }
    } else if rep > 0x1c52 && !club.division().is_null() && !cn.is_null() && cn.league_standard() as i8 > 10 {
        covered!("sma_1 big club");
        let d = if !cj.is_null() { cj.reputation() as i32 - rep as i32 } else { npr.world_reputation() as i32 - rep as i32 };
        v += ftol(d as f64 * read_f64(rdata::F64_0_15)) as i16 as i32;
        if rep > 0x203a && npr.world_reputation() < 0x109a {
            if foreign() || cj.is_null() || cj.division().is_null() || cj.division().reputation() < club.division().reputation() {
                return NEVER;
            }
        }
        if job != 9 {
            v = rep_factor(v);
        }
    } else if foreign() && job != 9 {
        v = ftol((npr.world_reputation() as i32 as f64 * read_f64(rdata::F64_5ENEG_05) + read_f64(rdata::F64_0_5)) * v as f64);
    }
    if !cn.is_null() {
        let fl = exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p, club, cn.first_language() as u8 as u32, 1) as i8;
        v = ftol((fl as i32 as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_0_5)) * v as f64); // sma1: the language factor
    }
    if nation == 0 {
        return mul(v, rdata::F64_0_25);
    }
    let club_dev = || if cn.is_null() { None } else { Some(cn.state_of_development() as i8) };
    match Nation(nation).state_of_development() as i8 {
        1 => v,
        2 => if club_dev() == Some(1) { mul(v, rdata::F64_0_75) } else { v },
        3 => if matches!(club_dev(), Some(1) | Some(2)) { mul(v, rdata::F64_0_25) } else { v },
        _ => mul(v, rdata::F64_0_05),
    }
}

compare_value!(compare_score_staff_for_job, score_staff_for_job, %%I0%%, (staff: usize, club: usize, viewer: usize, job: u32) -> i32);
