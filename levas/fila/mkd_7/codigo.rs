// ---------- batch %%LEVA%%: a person's will to take a job at a club (shortlist_manager.cpp) ----------

/// The `.rdata` constants of the will to take a job that `rdata_consts.rs` does not name yet (cm0102.exe 3.9.68). The
/// value is still read from the exe.
#[allow(dead_code)]
pub(crate) mod job_interest_rdata {
    pub(crate) const F64_900: usize = 0x96f958; // f64 = 900.0
    pub(crate) const F64_1_45: usize = 0x96f940; // f64 = 1.45
    pub(crate) const F64_1_176470588: usize = 0x96f950; // f64 = 1.1764705882352942 (1 / 0.85)
    pub(crate) const F64_2_222222222: usize = 0x96f948; // f64 = 2.2222222222222223 (1 / 0.45)
    pub(crate) const F64_0_0002222222222: usize = 0x96f928; // f64 = 0.00022222222222222223 (1 / 4500)
    /// the three names the will is 0 for when they have a club job: "Dario Gradi", "Pat Nevin", "Kevin Keegan"
    pub(crate) const NAMES: [usize; 3] = [0xa82620, 0xa82614, 0xa82604];
}

/// The square of the state of development (1..3, otherwise 3) of a nation (3 without one), as a byte.
unsafe fn job_interest_development_squared(n: Nation) -> i8 {
    let d = if n.is_null() { 3 } else { n.state_of_development() as i8 };
    let d = if !(1..=3).contains(&d) { 3 } else { d };
    d.wrapping_mul(d)
}

/// The reputation of a club's competition: the club's continental competition (`club::CONTINENTAL_COMP_ID`, or the one
/// in `G_CLUB_STATE_TABLE` when it is −1 or, with `intertoto`, the Intertoto Cup of a club of reputation 5000), 600 ×
/// its reputation in 16 bits; without one, 0.8 × the division's value, at most 7000. Returns (value, record).
unsafe fn job_interest_by_competition(club: Club, by_division: i16, intertoto: bool) -> (i16, usize) {
    let mut idx = club.continental_comp_id() as i32;
    let mut has = true;
    if idx <= -1 || (intertoto && idx == read_i32(KEY_CLUB_COMP_INTER_TOTO_CUP) && club.reputation() >= 0x1388) {
        idx = read_i32(read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_u32(club.addr()) as usize).wrapping_mul(6)));
        if idx <= -1 {
            has = false;
        }
    }
    if has {
        let reg = read_usize(G_CLUB_COMP_TABLE).wrapping_add((idx as usize).wrapping_mul(0x6b));
        if reg != 0 {
            return (read_i16(reg + competition::REPUTATION).wrapping_mul(600), reg);
        }
    }
    (job_interest_by_division_cap(by_division), 0)
}

/// 0.8 × the division's value, at most 7000 (truncated).
unsafe fn job_interest_by_division_cap(by_division: i16) -> i16 {
    let mut x = by_division as f64 * read_f64(rdata::F64_0_8);
    if !(x < read_f64(rdata::F64_7000)) {
        x = read_f64(rdata::F64_7000);
    }
    ftol(x) as i16
}

/// The manager's reputation `r` as the board sees it: v = 0.67 × the club's row +6 + 0.33 × +8 in the manager-manager
/// table (0 without a club); below 5000, the greater of r / t and r − 1000 t with t = 5 − 0.0008 v; otherwise the lesser
/// of r × t and r + 1000 t with t = 0.0001 v + 0.5.
unsafe fn job_interest_by_board(club: Club, r: i16) -> i32 {
    let (w6, w8) = if club.is_null() {
        (0i16, 0i16)
    } else {
        let t = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_u32(club.addr()) as usize).wrapping_mul(0x49));
        (read_i16(t + 6), read_i16(t + 8))
    };
    let v = ftol(w6 as f64 * read_f64(rdata::F64_0_67) + w8 as f64 * read_f64(rdata::F64_0_33)) as i16;
    let d = r as f64;
    if v < 0x1388 {
        let t = read_f64(rdata::F64_5) - v as f64 * read_f64(rdata::F64_0_0008);
        let q1 = d / t;
        let m = d - t * read_f64(rdata::F64_1000);
        ftol(if q1 > m { q1 } else { m })
    } else {
        let t = v as f64 * read_f64(rdata::F64_0_0001) + read_f64(rdata::F64_0_5);
        let q1 = d * t;
        let m = t * read_f64(rdata::F64_1000) + d;
        ftol(if q1 < m { q1 } else { m })
    }
}

/// The language factor: max(F × 0.1, (e + 5000)² × 1/15000 / (12000 − 200 F)) + 0.2, with F the person's fluency in the
/// club nation's first language (`exe::playerstats_get_language_fluency`; 10 without a player sheet).
unsafe fn job_interest_language_factor(p: Staff, club: Club, e: i16) -> f64 {
    let q = (e as i32 + 0x1388) as f64;
    let l = q * read_f64(rdata::F64_6_666666667ENEG_05) * q;
    let f = if !p.player().is_null() {
        exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p, club, (club.nation().first_language() as u8) as u32, 1) as i8 as f64
    } else {
        read_f64(rdata::F64_10)
    };
    let a = f * read_f64(rdata::F64_0_1);
    let b = l / (read_f64(rdata::F64_12000) - f * read_f64(rdata::F64_200));
    max_f64(a, b) + read_f64(rdata::F64_0_2)
}

/// A club's weighted reputation on the person's scale (`e` = ftol(1.5 × r − 5000), the int-apps and status cuts
/// included), from the stadium, the division, the competition, the manager, the finances and the shortlists.
/// `current`: the club the person is at (no board view of a manager seen by the person, other wealth bands, ×1.35 for a
/// national side, the Intertoto rule). Returns (e, by division, competition record).
#[allow(clippy::too_many_arguments)]
unsafe fn job_interest_club_scale(this: usize, p: Staff, club: Club, job: i8, w: (f32, f32, f32, f32, f32), current: bool) -> (i16, i16, usize) {
    let (w60, w84, l74, w88, den) = w;
    let stad = reputation_by_stadium(club) as i16;
    let div = reputation_by_division(club);
    let (comp, rec) = job_interest_by_competition(club, div, current);
    let m = club.manager();
    let mgr: i16 = if current {
        if m.is_null() || m.non_player().is_null() {
            ftol(div as f64 * read_f64(rdata::F64_0_75)) as i16
        } else {
            let r = m.non_player().current_reputation();
            if read_usize(G_MANAGER_MANAGER) == 0 || job == 5 {
                r
            } else {
                job_interest_by_board(club, r) as i16
            }
        }
    } else if m.is_null() || m.non_player().is_null() {
        0
    } else {
        let r = exe::database_get_staff_reputation_for_viewer(m, p, 1);
        if read_usize(G_MANAGER_MANAGER) == 0 || job == 5 {
            r
        } else {
            job_interest_by_board(club, r) as i16
        }
    };
    // the reputation by the finances' wealth band
    let mut si: i16 = club.reputation();
    if exe::database_is_national_team(club) == 0 {
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        if fin != 0 {
            let r = si as f64;
            match exe::finance_get_wealth_band(fin) as i8 {
                -2 => si = ftol(r * read_f64(if current { rdata::F64_0_75 } else { rdata::F64_0_7 })) as i16,
                -1 => si = ftol(r * read_f64(if current { rdata::F64_0_9 } else { rdata::F64_0_85 })) as i16,
                1 => {
                    si = ftol(r * read_f64(rdata::F64_1_25)) as i16;
                    let min = if current { 0x128e } else { 0x109a };
                    if si < min {
                        si = min;
                    }
                }
                2 => {
                    si = ftol(r * read_f64(rdata::F64_1_5)) as i16;
                    if si < 0x1676 {
                        si = 0x1676;
                    }
                }
                _ => {}
            }
        }
    }
    let ax: i16 = if exe::database_is_national_team(club) != 0 && exe::club_is_national_team(club) == 0 {
        ftol(si as f64 * read_f64(if current { rdata::F64_1_35 } else { job_interest_rdata::F64_1_45 })) as i16
    } else {
        let a = read_i16(read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add((read_u32(club.addr()) as usize).wrapping_mul(9)) + 5);
        ftol(a as f64 * read_f64(rdata::F64_0_4) + si as f64 * read_f64(rdata::F64_0_6)) as i16
    };
    let s = (((comp as f64 * w60 as f64 + div as f64 * w84 as f64) + mgr as f64 * l74 as f64) + stad as f64 * w88 as f64) + ax as i32 as f64;
    let mut e = ftol(s / den as f64) as i16;
    if !current && exe::database_is_national_team(club) != 0 && club.nation() == p.nation() {
        e = ftol(e as f64 * read_f64(job_interest_rdata::F64_1_45)) as i16;
    }
    if exe::database_is_national_team(club) == 0 && club.professional_status() != 1 {
        // the reputation is cut for a non-professional club: min(e / 2, e − 5 × (int apps² + 400)), status 3 again
        let ia = p.int_apps() as i32;
        let lim = (ia * ia + 400) * 5;
        let a = e as i32;
        let (h, x) = (a / 2, a - lim);
        e = (if h < x { h } else { x }) as i16;
        if club.professional_status() == 3 {
            e = half_or_minus_2000(e);
        }
    }
    (ftol(e as f64 * read_f64(rdata::F64_1_5) - read_f64(rdata::F64_5000)) as i16, div, rec)
}

/// Compares the C string `a` with the one at `b` in the exe.
unsafe fn job_interest_name_is(a: &[u8], b: usize) -> bool {
    let mut k = 0usize;
    loop {
        let x = a[k];
        let y = read_u8(b + k);
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        k += 1;
    }
}

/// Returns in st0 a person's will to take a job (`job`) at a club and marks the reason bits, as
/// `staff_interest_in_joining_club` (0x846920) does for a player.
///
/// `staff_interest_in_job_at_club(person, club, check, job, reasons)` (the shortlist manager in ecx): 0 without a staff
/// sheet, without the club's or the person's nation, for Dario Gradi, Pat Nevin and Kevin Keegan when they have a club
/// job, for a media pundit (job 17), for the last 16 records (outside job 5), for a national manager (job 5 abroad), for a
/// staff member of reputation above 4250 or ability above 100 wanting a manager's job (5 or 12) elsewhere, for a physio
/// (10) asked for another job, for a player with a club job asked by another club, for a manager going to a club
/// of lower division or reputation (more than 750 below) and for a club below 2750 of his own nation or that finds him
/// foreign. Otherwise, the base: 3.5 at his own club; 2.25 for a manager's job, otherwise 1.5, plus (days out of a club
/// since the game began)² / (reputation + 900), at most 12; ×0.33 / ×2 / ×0.5..0.85 / ×3 by disliking and favouring
/// the clubs, ×0.5..0.85 − loyalty against a rival (Celtic and Rangers, Italy, the same division), ×0.05 / ×0.1 between
/// Croatia and Yugoslavia, ×2 / ×0.33 / ×2 / ×0.5 by the managers' liking; 0 with a future transfer; the contract's
/// morale; by the current job (×0.01 to ×10) and the job offered (×0.1 to ×10, 0 for job 0); ×1.4 for a manager the
/// board has little confidence in; the language factor; ×0.85..0.01 or ×0.3..0.001 for a foreigner by job; the time at
/// the club (0 below (loyalty + 8) × 15 days); the two clubs' weighted reputations (stadium, division, competition,
/// manager, wealth band, shortlists) and their difference by ambition; the development ratio of the two nations and
/// the continents; the reputation against his own. With `check` and a will of 1.75 or more: the default wage
/// (`exe::staff_contract_default_staff_contract`) × 3.5 / will against the club's staff wage cap
/// (`exe::finance_get_max_staff_wage`) and against his current wage.
/// Original: `staff_interest_in_job_at_club` (0x844780, shortlist_manager.cpp, thiscall, ret 0x14).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn staff_interest_in_job_at_club(this: usize, person: usize, club: usize, check: u32, job_arg: u32, reasons: *mut u32) -> f64 {
    let p = Staff(person);
    let c = Club(club);
    let job = job_arg as u8 as i8;
    let zero = read_f64(rdata::F64_0);
    let fl = move |b: u32| {
        if !reasons.is_null() {
            *reasons |= b;
        }
    };
    if !reasons.is_null() {
        *reasons = 0;
    }
    if p.non_player().is_null() || c.nation().is_null() || p.nation().is_null() {
        return zero;
    }
    // three managers never move from a club job
    let mut buf = [0u8; 0x1f4];
    for name in job_interest_rdata::NAMES {
        exe::database_format_staff_name(p, buf.as_mut_ptr() as usize, 0x1f4, 1);
        if job_interest_name_is(&buf, name) {
            if !p.club_job().is_null() {
                return zero;
            }
            break;
        }
    }
    let jfc = p.job_for_club() as i8;
    let jfn = p.job_for_nation() as i8;
    if jfc == 0x11 || jfn == 0x11 {
        return zero;
    }
    if job != 5 && read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) - 0x10 {
        return zero;
    }
    let nj = p.national_job();
    if !nj.is_null() && jfn == 5 {
        return zero;
    }
    let manager_now = jfc == 5 || jfc == 0xc || jfn == 5 || jfn == 0xc;
    if job != 0 {
        if job != 5 && manager_now && (p.non_player().current_reputation() > 0x109a || p.non_player().current_ability() > 100) {
            return zero;
        }
        if job == 10 && jfc != 10 && jfc != 0 {
            return zero;
        }
        if jfn == 5 && !nj.is_null() {
            return zero;
        }
        if job != 0xf && job != 0xc && job != 5 && !p.player().is_null() && jfc == 0xb {
            let cj = p.club_job();
            if !cj.is_null() && c != cj {
                return zero;
            }
        }
        if job != 10 && jfc == 10 {
            return zero;
        }
    }
    if !c.is_null() {
        if job != 5 && manager_now {
            let cj = p.club_job();
            if !cj.is_null() {
                let (dn, dc) = (c.division(), cj.division());
                if !dn.is_null() && !dc.is_null() && dc.reputation() >= dn.reputation() {
                    return zero;
                }
                if cj.reputation() as i32 > c.reputation() as i32 - 0x2ee {
                    return zero;
                }
            }
        }
        if c.reputation() < 0xabe {
            let n = c.nation();
            if !n.is_null() {
                let cj = p.club_job();
                if !cj.is_null() && cj.nation() == n {
                    return zero;
                }
                if exe::database_is_staff_foreign(p, n, 1) != 0 {
                    return zero;
                }
            }
        }
    }
    // the weights of the reputations (ambition and professionalism)
    let l74: f32 = if job == 5 || job == 0xc { 0.0 } else { 1.0 };
    let amb = p.ambition() as i32 as f64;
    let w88 = ((read_f64(rdata::F64_1_4) - amb * read_f64(rdata::F64_0_025)) - p.professionalism() as i32 as f64 * read_f64(rdata::F64_0_025)) as f32;
    let w84 = (amb * read_f64(rdata::F64_0_02) + read_f64(rdata::F64_0_8)) as f32;
    let b = amb * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_0_5);
    let w60 = b as f32;
    let d = (((b + w84 as f64) + w88 as f64) + l74 as f64) + read_f32(rdata::F32_1) as f64;
    let den: f32 = if d > zero { d as f32 } else { 0.5 };
    let w = (w60, w84, l74, w88, den);
    // the club he is at (or the national side), and his job there
    let mut cj = p.club_job();
    let mut jb = jfc;
    if (cj.is_null() || exe::database_is_national_team(c) != 0) && !nj.is_null() && jfn != 0xb {
        cj = nj;
        jb = jfn;
    }
    if !cj.is_null() && cj.nation().is_null() {
        return read_f64(rdata::F64_3_5);
    }
    let k = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, cj));
    let mut score: f64;
    covered!("mkd_7 job interest");
    if c == cj {
        score = 3.5;
    } else {
        if !k.is_null()
            && !cj.is_null()
            && !cj.nation().is_null()
            && !c.nation().is_null()
            && c.nation().first_language() == cj.nation().first_language()
            && k.grievances() & 0x20 != 0
        {
            return zero;
        }
        score = if job == 5 || job == 0xc { 2.25 } else { 1.5 }; // mkd7: the base for another club
        if cj.is_null() {
            // out of a club: (days since the later of the game's start and joining)² / (reputation + 900), at most 12
            let (gd, gy) = (read_i16(G_GAME_START_DATE), read_u16(G_GAME_START_DATE + 2));
            let (td, ty) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
            if !(gy > ty || (gy == ty && gd > td)) {
                let mut l = [0u8; 8];
                core::ptr::copy_nonoverlapping((p.addr() + staff::DATE_JOINED_CLUB) as *const u8, l.as_mut_ptr(), 8);
                let (ld, ly) = (i16::from_le_bytes([l[0], l[1]]), u16::from_le_bytes([l[2], l[3]]));
                if ly < gy || (ly == gy && ld < gd) {
                    let mut tmp = [0u8; 8];
                    exe::date_operator_3d(l.as_mut_ptr() as usize, tmp.as_mut_ptr() as usize, G_GAME_START_DATE);
                }
                let (ld, ly) = (i16::from_le_bytes([l[0], l[1]]), u16::from_le_bytes([l[2], l[3]]));
                let mut dd = (read_u16(G_CURRENT_DATE_YEAR) as i32 - ly as i32) * 365 - ld as i32 + read_i16(G_CURRENT_DATE) as i32;
                if dd > 0x1f4 {
                    dd = 0x1f4;
                }
                let mut x = dd.wrapping_mul(dd) as f64 / (p.non_player().current_reputation() as f64 + read_f64(job_interest_rdata::F64_900));
                if read_f64(rdata::F64_12) < x {
                    x = read_f64(rdata::F64_12);
                }
                score = x + score;
            }
        }
    }
    // liking and disliking the clubs
    if exe::staff_dislikes_club(p, c) != 0 {
        score *= read_f64(rdata::F64_0_33);
        fl(2);
    }
    if exe::staff_dislikes_club(p, cj) != 0 {
        score = score + score;
        fl(4);
    }
    if exe::staff_favours_club(p, cj) != 0 {
        score *= read_f64(if cj.professional_status() != 1 && c.professional_status() == 1 { rdata::F64_0_85 } else { rdata::F64_0_5 });
        if !reasons.is_null() && !k.is_null() && k.leaving_on_bosman() == 0 {
            *reasons |= 8;
        }
    }
    if exe::staff_favours_club(p, c) != 0 {
        fl(0x10);
        score *= read_f64(rdata::F64_3);
    }
    // rivals
    if !cj.is_null() {
        let rival = read_usize(cj.addr() + club::RIVAL1) == c.addr() || read_usize(cj.addr() + club::RIVAL2) == c.addr() || read_usize(cj.addr() + club::RIVAL3) == c.addr();
        if c != cj && (rival || exe::comp_util_get_league_clash_type(c, cj) != 0) {
            covered!("mkd_7 rival");
            let loy = p.loyalty() as i32 as f64;
            let (e1, e2) = (read_u32(G_KEY_CLUB_GLASGOW_CELTIC), read_u32(G_KEY_CLUB_GLASGOW_RANGERS));
            let (ic, ij) = (read_u32(c.addr()), read_u32(cj.addr()));
            let m = if (ic == e1 || ic == e2) && (ij == e1 || ij == e2) {
                read_f64(rdata::F64_0_5) - loy * read_f64(rdata::F64_0_025)
            } else if read_u32(c.nation().addr()) == read_u32(KEY_NATION_ITALY) {
                read_f64(rdata::F64_1) - loy * read_f64(rdata::F64_0_01666666667)
            } else if rival && read_usize(c.addr() + club::DIVISION) == read_usize(cj.addr() + club::DIVISION) {
                if (c.reputation() as i32 - cj.reputation() as i32).abs() < 0x4e2 {
                    read_f64(rdata::F64_0_75) - loy * read_f64(rdata::F64_0_025)
                } else {
                    read_f64(rdata::F64_0_8) - loy * read_f64(rdata::F64_0_01666666667)
                }
            } else {
                read_f64(rdata::F64_0_85) - loy * read_f64(rdata::F64_0_025)
            };
            score *= m;
            if !reasons.is_null() && m < read_f64(rdata::F64_0_7) {
                *reasons |= 0x20;
            }
        } else {
            let (yu, cr) = (read_u32(KEY_NATION_YUGOSLAVIA), read_u32(KEY_NATION_CROATIA));
            let nc = read_u32(c.nation().addr());
            let nj_ = read_u32(cj.nation().addr());
            let np_ = read_u32(p.nation().addr());
            if nc == yu {
                if nj_ == cr && np_ == cr {
                    score *= read_f64(rdata::F64_0_05);
                    fl(2);
                }
            } else if nc == cr && nj_ == yu && np_ == yu {
                score *= read_f64(rdata::F64_0_1);
                fl(2);
            }
        }
    }
    // the managers' liking
    if job != 5 && job != 0xc {
        let m = c.manager();
        if exe::staff_is_favoured_by(m, p) != 0 {
            score = score + score;
            fl(0x40);
        }
        if exe::staff_is_disliked_by(m, p) != 0 {
            score *= read_f64(rdata::F64_0_33);
            fl(0x80);
        }
    }
    if !cj.is_null() {
        let m = cj.manager();
        if exe::staff_is_disliked_by(m, p) != 0 {
            score = score + score;
            fl(0x100);
        }
        if exe::staff_is_favoured_by(m, p) != 0 {
            score *= read_f64(rdata::F64_0_5);
            fl(0x200);
        }
    }
    if !p.player().is_null() && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) != 0 {
        fl(0x10000000);
        return zero;
    }
    // the morale in the contract
    if !k.is_null() {
        let m44 = k.morale() as i32 as f64;
        if c == cj {
            score = (m44 * read_f64(rdata::F64_0_0025) + read_f64(rdata::F64_1)) * score;
        } else {
            let v = read_f64(rdata::F64_1) - m44 * read_f64(rdata::F64_0_0025);
            score = v * score;
            if !reasons.is_null() && k.transfer_status() & 8 == 0 {
                if v > read_f64(rdata::F64_1_5) {
                    *reasons |= 0x400;
                } else if v < read_f64(rdata::F64_0_5) {
                    *reasons |= 0x800;
                }
            }
        }
    }
    if c == cj {
        jb = job;
    }
    let jb_manager = jb == 5 || jb == 0xc;
    let job_manager = job == 5 || job == 0xc;
    let mut devsq_new: i8 = 0;
    let mut e_new: i16 = 0;
    let mut div_new: i16 = 0;
    let mut rec_new = 0usize; // [esp+0x78]
    let mut l7c = 1.0f64; // [esp+0x7c]: how long he has been at the club (0.0025 × days² / 20 / (loyalty + 6), at most 1)
    if c != cj {
        // by the job he has now
        match jb {
            1 => score *= read_f64(rdata::F64_0_01),
            2..=4 => {
                if job == 5 && c.reputation() >= 0x1770 {
                    score *= read_f64(rdata::F64_0_5);
                } else {
                    score *= read_f64(rdata::F64_0_01);
                }
            }
            5 | 0xc => {
                if !cj.is_null() {
                    score *= read_f64(rdata::F64_0_45);
                    fl(0x4000000);
                } else {
                    score *= read_f64(rdata::F64_0_65);
                }
            }
            6 | 0xd => {
                score *= read_f64(rdata::F64_1_5);
                if job_manager {
                    fl(0x8000000);
                }
            }
            7 | 0xe => {
                score *= read_f64(rdata::F64_2_2);
                if job_manager {
                    fl(0x8000000);
                }
            }
            8 | 0xf => {
                score = (amb * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_0_8)) * score;
                if job_manager {
                    fl(0x8000000);
                }
            }
            9 | 10 => {
                if matches!(job, 5 | 0xc | 6 | 0xd | 0xe | 0xf | 7 | 8) {
                    fl(0x8000000);
                }
            }
            0x10 => {
                let a = p.ambition();
                score = (read_f64(rdata::F64_10) / (if a > 1 { a as i32 } else { 1 }) as f64) * score;
            }
            _ => {
                score *= read_f64(rdata::F64_10); // mkd7: a player taking a staff job
                fl(0x8000000);
            }
        }
        // by the job offered
        match job {
            0 => return zero,
            1 => score *= read_f64(rdata::F64_10),
            2..=4 => score *= read_f64(job_interest_rdata::F64_1_176470588),
            5 | 0xc => {
                score *= read_f64(job_interest_rdata::F64_2_222222222);
                if jb_manager {
                    score *= read_f64(rdata::F64_1_5);
                }
            }
            6 | 0xd => {
                score *= read_f64(rdata::F64_0_5);
                if jb_manager {
                    score *= read_f64(rdata::F64_0_4);
                }
            }
            7 | 8 | 0xe | 0xf => {
                score *= read_f64(rdata::F64_0_4);
                if jb_manager {
                    score *= read_f64(rdata::F64_0_4);
                }
            }
            9 | 10 => {
                score *= read_f64(if jb == 9 || jb == 10 { rdata::F64_0_4 } else { rdata::F64_0_25 });
                if !p.player().is_null() {
                    score *= read_f64(rdata::F64_0_25);
                }
                if jb_manager {
                    score *= read_f64(rdata::F64_0_4);
                }
            }
            _ => {
                score *= read_f64(rdata::F64_0_1);
                if jb_manager {
                    score *= read_f64(rdata::F64_0_4);
                }
            }
        }
        // a manager the board has little confidence in
        if !cj.is_null() && jb_manager && read_usize(G_MANAGER_MANAGER) != 0 {
            let t = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_u32(cj.addr()) as usize).wrapping_mul(0x49));
            let v = read_i16(t + 8) as i32 + (p.loyalty() as i32 + p.determination() as i32) * 500 + read_i16(t + 6) as i32;
            if v < 0x1f40 {
                score *= read_f64(rdata::F64_1_4);
            }
        }
        // the new club's reputation on his scale, and the language
        devsq_new = job_interest_development_squared(c.nation());
        let (e, dv, rec) = job_interest_club_scale(this, p, c, job, w, false);
        e_new = e;
        div_new = dv;
        rec_new = rec;
        if !c.nation().is_null() {
            let m = job_interest_language_factor(p, c, e_new);
            score = m * score;
            if !reasons.is_null() && m < read_f64(rdata::F64_0_6) {
                *reasons |= 0x10000;
            }
        }
        // a foreigner: by the job offered (milder when not a foreigner to the club's manager)
        let applies = if !cj.is_null() {
            let n = c.nation();
            !(!n.is_null() && exe::database_is_staff_foreign(p, n, 1) == 0) && cj.nation() != c.nation()
        } else {
            true
        };
        if applies {
            let m = c.manager();
            let mild = !job_manager && !m.is_null() && !m.nation().is_null() && exe::database_is_staff_foreign(p, m.nation(), 1) == 0;
            let f = if mild {
                match job {
                    2..=4 => rdata::F64_0_85,
                    6 | 0xd => rdata::F64_0_75,
                    7 | 8 | 0xe | 0xf => rdata::F64_0_65,
                    9 | 10 => rdata::F64_0_5,
                    _ => rdata::F64_0_01,
                }
            } else {
                covered!("mkd_7 foreigner");
                match job {
                    2..=4 => rdata::F64_0_25,
                    5 | 0xc => rdata::F64_0_3,
                    6 | 0xd => rdata::F64_0_225,
                    7..=10 | 0xe | 0xf => rdata::F64_0_2,
                    _ => rdata::F64_0_001,
                }
            };
            score *= read_f64(f);
        }
        if cj.is_null() {
            score *= read_f64(rdata::F64_2_5);
        } else {
            // the time at his club: 0 below (loyalty + 8) × 15 days
            let dd = (read_u16(G_CURRENT_DATE_YEAR) as i32 - p.date_joined_club_year() as i32) * 365 - read_i16(p.addr() + staff::DATE_JOINED_CLUB) as i32
                + read_i16(G_CURRENT_DATE) as i32;
            let loy = p.loyalty() as i32;
            if dd < (loy + 8) * 0xf {
                covered!("mkd_7 too soon at his club");
                return zero;
            }
            let q = dd.wrapping_mul(dd) / 20;
            let mut r = q / (if loy + 6 < 1 { 1 } else { loy + 6 });
            if r < 0 {
                return zero;
            }
            if r > 0x190 {
                r = 0x190;
            } else if r == 0 {
                return zero;
            }
            l7c = r as f64 * read_f64(rdata::F64_0_0025);
        }
    }
    // his current club's reputation (or his own) on the same scale
    let devsq_cur: i8;
    let e_cur: i16;
    let mut div_cur: i16 = 0;
    let mut rec_cur = 0usize; // [esp+0x70]
    if cj.is_null() {
        devsq_cur = job_interest_development_squared(p.nation());
        let np = p.non_player();
        let avg = ((np.world_reputation() as i32 + np.current_reputation() as i32 + np.home_reputation() as i32) / 3) as i16;
        e_cur = ftol(avg as f64 * read_f64(rdata::F64_1_5) - read_f64(rdata::F64_5000)) as i16;
    } else {
        devsq_cur = job_interest_development_squared(cj.nation());
        let (e, dv, rec) = job_interest_club_scale(this, p, cj, job, w, true);
        e_cur = e;
        div_cur = dv;
        rec_cur = rec;
        if c != cj {
            score /= job_interest_language_factor(p, cj, e_cur);
        }
    }
    if c != cj {
        let d = c.division();
        if d.is_null() || read_u32(d.addr()) == read_u32(KEY_CLUB_COMP_A_LOWER_DIVISION) {
            score *= read_f64(rdata::F64_0_1);
        }
        let np = p.non_player();
        let avg = (np.world_reputation() as i32 + np.current_reputation() as i32 + np.home_reputation() as i32) / 3;
        let g = (c.reputation() as i32 - avg) as f64 * read_f64(rdata::F64_0_000125) + read_f64(rdata::F64_1);
        // the difference of the two scales, by ambition (up) or by the job (down)
        let diff = (e_new as i32 - e_cur as i32) as f64;
        let mut y = if e_new > e_cur {
            diff * (amb * read_f64(rdata::F64_0_08771929825) + read_f64(rdata::F64_0_75))
        } else if !job_manager {
            diff * read_f64(rdata::F64_1_5)
        } else if !(score > read_f64(rdata::F64_10)) {
            diff
        } else {
            let mut k = read_f64(rdata::F64_1_1) - score * read_f64(rdata::F64_0_01);
            if k < read_f64(rdata::F64_0_1) {
                k = read_f64(rdata::F64_0_1);
            } else if k > read_f64(rdata::F64_1) {
                k = read_f64(rdata::F64_1);
            }
            k * diff
        };
        if y > zero && l7c < read_f64(rdata::F64_1) {
            let mut z = y * read_f64(rdata::F64_0_02);
            if read_f64(rdata::F64_100) < z {
                z = read_f64(rdata::F64_100);
            }
            let f = ((read_f64(rdata::F64_150) - z) * l7c + z) * read_f64(rdata::F64_0_006666666667);
            score *= f;
            if !reasons.is_null() && f < read_f64(rdata::F64_0_6) {
                *reasons |= 0x100000;
            }
        }
        if !jb_manager && job_manager && y < zero {
            let k = if jb == 8 || jb == 0xf || jb == 6 {
                let k = read_f64(rdata::F64_1) - score * read_f64(rdata::F64_0_04);
                if k < read_f64(rdata::F64_0_4) { read_f64(rdata::F64_0_4) } else { k }
            } else {
                let k = read_f64(rdata::F64_1) - score * read_f64(rdata::F64_0_1);
                if k < read_f64(rdata::F64_0_2) { read_f64(rdata::F64_0_2) } else { k }
            };
            y *= k;
        }
        let mut t = y * read_f64(job_interest_rdata::F64_0_0002222222222) + read_f64(rdata::F64_1); // mkd7: the scales' difference
        if t < read_f64(rdata::F64_0_001) {
            t = 0.001;
        }
        score *= t;
        if !reasons.is_null() {
            let bosman_ok = k.is_null() || k.leaving_on_bosman() == 0;
            let ambitious = w60 as f64 > read_f64(rdata::F64_1_2);
            if t < read_f64(rdata::F64_0_6) {
                if ambitious && rec_cur != 0 && rec_new == 0 && score > read_f64(rdata::F64_1_25) && jb_manager && bosman_ok {
                    *reasons |= 0x400000;
                } else {
                    *reasons |= 0x800000;
                }
            } else if t > read_f64(rdata::F64_1_5) && ambitious && rec_cur == 0 && rec_new != 0 && job_manager && bosman_ok {
                *reasons |= 0x200000;
            }
        }
        // the development of the two nations and the continents
        if !cj.is_null() {
            if devsq_cur == devsq_new {
                let pc = p.nation().continent();
                if pc != 0 && read_u32(pc) == read_u32(KEY_CONTINENT_EUROPE) {
                    let cc = c.nation().continent();
                    if cc != 0 && read_u32(cc) != read_u32(KEY_CONTINENT_EUROPE) && cc != cj.nation().continent() {
                        score *= read_f64(rdata::F64_0_4);
                        fl(0x2000000);
                    }
                }
            } else {
                let dc = if div_cur > 0x1f4 { div_cur as i32 } else { 0x1f4 };
                let mut r = (devsq_cur as i32 as f64 * div_new as f64) / (dc as f64 * devsq_new as i32 as f64);
                if !(read_f64(rdata::F64_0_01) <= r) {
                    r = read_f64(rdata::F64_0_01);
                } else if read_f64(rdata::F64_10) < r {
                    r = read_f64(rdata::F64_10);
                }
                score = r * score;
                if c.nation().continent() != cj.nation().continent() && r < read_f64(rdata::F64_1) {
                    score *= read_f64(rdata::F64_0_2);
                    fl(0x2000000);
                }
            }
        }
        score *= g;
        if !reasons.is_null() && g < read_f64(rdata::F64_0_6) {
            *reasons |= 0x800000;
        }
    }
    // the wage: the default wage × 3.5 / will against the club's staff cap and his current wage
    if check != 0 && !(score < read_f64(rdata::F64_1_75)) {
        covered!("mkd_7 wage check");
        let mut local = [0u8; 0x50];
        exe::new_staff_contract(local.as_mut_ptr() as usize);
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c);
        if fin == 0 {
            return zero;
        }
        let wage = exe::staff_contract_default_staff_contract(local.as_mut_ptr() as usize, p, c, 0, u32::MAX, job_arg, 0);
        let m = if read_f64(rdata::F64_0_1) <= score { score } else { read_f64(rdata::F64_0_1) };
        let mut q = read_f64(rdata::F64_3_5) / m;
        if q < read_f64(rdata::F64_0_9) {
            q = 0.9;
        }
        let asked = q * wage;
        let mw = exe::finance_get_max_staff_wage(fin, job) as f64;
        let (s2, wanted) = if !(asked <= mw) && !(asked <= zero) { ((mw / asked) * score, mw) } else { (score, asked) };
        if k.is_null() {
            return s2;
        }
        let mut cw = k.wage() as f64;
        if cw < read_f64(rdata::F64_10) {
            cw = read_f64(rdata::F64_10);
        }
        let ratio = wanted / cw;
        if !(read_f64(rdata::F64_0_1) <= ratio) {
            return s2;
        }
        if read_f64(rdata::F64_10) < ratio {
            return (read_f64(rdata::F64_10) / q) * s2;
        }
        if ratio < read_f64(rdata::F64_1) {
            return s2;
        }
        return (ratio / q) * s2;
    }
    score
}

/// Comparer for `staff_interest_in_job_at_club`: original and copy side by side over the reasons,
/// `G_CONTRACT_LOOKUP_RESULT` (written by the default wage) and the generator's state.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_staff_interest_in_job_at_club(this: usize, person: usize, club: usize, check: u32, job: u32, reasons: *mut u32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut u32) -> f64>(%%I0%%);
    let est = crate::rng::STATE_REGIONS;
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize), est[0], est[1]];
    if !reasons.is_null() {
        regions.push((reasons as usize, 4));
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&regions,
        || o(this, person, club, check, job, reasons),
        || crate::rng::without_counting(|| staff_interest_in_job_at_club(this, person, club, check, job, reasons)));
    SCOREBOARD.record(same && r.to_bits() == n.to_bits(), || format!("staff_interest_in_job_at_club(person {:#x}, club {:#x}, {}, job {}) original={} rust={} {:x?} x {:x?}",
        person, club, check, job as u8, r, n, mo, mr));
    r
}
