// --- sx01c_1: Media::apply_story_effect_flags (0x736c00, media.cpp, 5,939 bytes), the whole routine in named pieces ---

/// The f32 constants of `apply_story_effect_flags` that have no name in `rdata_consts.rs` yet (cm0102.exe .rdata).
const STORY_F32_ONE_FIFTEENTH: usize = 0x96d888; // f32 = 1/15
const STORY_F32_ONE_150TH: usize = 0x96d884; // f32 = 1/150
const STORY_F32_30: usize = 0x96d880; // f32 = 30.0
/// `Media::queue_player_gratitude_news(player, club, kind, variant)` (0x732c80, media.cpp, thiscall, ret 0x10).
const FN_MEDIA_QUEUE_PLAYER_GRATITUDE_NEWS: usize = 0x732c80;
/// `Media::post_player_backing_request_news(player, club, kind, story)` (0x743370, media.cpp, thiscall, ret 0x10).
const FN_MEDIA_POST_PLAYER_BACKING_REQUEST_NEWS: usize = 0x743370;
/// `Media::post_player_criticism_reaction_news(player, club, topic, reply)` (0x743a50, media.cpp, thiscall, ret 0x10).
const FN_MEDIA_POST_PLAYER_CRITICISM_REACTION_NEWS: usize = 0x743a50;

/// What `apply_story_effect_flags` keeps in its frame while it walks the squad.
struct StoryEffects {
    this: usize,
    club: Club,
    kind: u32,
    flags: u32,
    weight: f32,
    player: Staff,
    /// the player's contract at the club
    contract: Contract,
    /// 0x13 when the story kind is 0x13 (a word, compared with 0, 0x13 and 0xa)
    story13: u16,
    /// the player may still apologise (cleared by a draw when he already answered a story)
    may_apologise: bool,
    /// set when the caller passed neither 0x80 nor 0x100 (the routine chose the reaction itself)
    chosen: bool,
    /// the player's form for the story (`exe::media_rate_player_form_for_story`)
    form: f32,
    /// what the manager expected of him; without a player, the bits of the response argument
    expected: f32,
}

impl StoryEffects {
    /// The form above the expectation by more than `margin`.
    fn above(&self, margin: f64) -> bool {
        (self.expected as f64 + margin) < self.form as f64
    }
    /// The form below the expectation by more than `margin`.
    fn below(&self, margin: f64) -> bool {
        (self.expected as f64 - margin) > self.form as f64
    }
}

/// The staff's confidence in the manager after a story: the club's `board_record::STAFF_CONFIDENCE` plus the change,
/// kept between 1 and 10000.
pub(crate) unsafe fn story_staff_confidence_after(club: Club, change: i16) -> i16 {
    read_i16(manager_manager_row(club) + board_record::STAFF_CONFIDENCE).wrapping_add(change).clamp(1, 10000)
}

/// The byte of the person's 79-byte state record that media stories raise (`staff_state::INJURY_COUNTER`).
unsafe fn story_unrest(p: Staff) -> StaffState {
    StaffState(staff_state_entry(p))
}

/// Raises the person's state byte by draw(n) (8 bits, wrapping).
unsafe fn story_raise_unrest(p: Staff, n: i32) {
    let s = story_unrest(p);
    s.set_injury_counter(s.injury_counter().wrapping_add(exe::utils_random(n) as u8));
}

/// Whether the player already answered a story in the media's log (`story_log_entry::PERSON_A` with
/// `story_log_entry::RESPONSE` 2).
unsafe fn story_already_answered(this: usize, player: Staff) -> bool {
    let log = read_usize(this + media_fields::STORY_LOG);
    (0..read_i32(this + media_fields::STORY_LOG_COUNT).max(0) as usize).any(|i| {
        let e = log + i * 0x17; // story_log_entry, 23 bytes
        read_u32(e + story_log_entry::PERSON_A) == read_u32(player.addr()) && read_u8(e + story_log_entry::RESPONSE) == 2
    })
}

/// The player's grievance of a strong or weak reaction: the contract flag 0x100 for a kind-0x13 story, otherwise the
/// event flags 0x400.
unsafe fn story_flag_grievance(s: &StoryEffects) {
    if s.story13 != 0 {
        exe::staff_contract_set_flags(s.contract, 0x100);
    } else {
        exe::staff_contract_apply_event_flags(s.contract, 0x400, 0);
    }
}

/// The player falls out with the manager: he dislikes him (`exe::staff_add_disliked_staff`) and his contract takes the
/// event flags 8.
unsafe fn story_fall_out(s: &StoryEffects, strong: u32) {
    exe::staff_add_disliked_staff(s.player, s.club.manager(), strong);
    exe::staff_contract_apply_event_flags(s.contract, 8, 0);
}

/// The player warms to the manager: he favours him (`exe::staff_add_favoured_staff`) and his contract loses the
/// grievances 8 and 0x2000.
unsafe fn story_make_up(s: &StoryEffects) {
    exe::staff_add_favoured_staff(s.player, s.club.manager(), 0);
    exe::staff_contract_clear_grievances(s.contract, 8);
    exe::staff_contract_clear_grievances(s.contract, 0x2000);
}

unsafe fn story_gratitude(s: &StoryEffects) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_MEDIA_QUEUE_PLAYER_GRATITUDE_NEWS);
    f(s.this, s.player.addr(), s.club.addr(), s.kind, 0)
}

unsafe fn story_backing_request(s: &StoryEffects, story: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_MEDIA_POST_PLAYER_BACKING_REQUEST_NEWS);
    f(s.this, s.player.addr(), s.club.addr(), story, s.kind)
}

unsafe fn story_criticism_reaction(s: &StoryEffects) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_MEDIA_POST_PLAYER_CRITICISM_REACTION_NEWS);
    f(s.this, s.player.addr(), s.club.addr(), s.kind as u16 as i16 as i32 as u32, 2)
}

/// The supporters' and board's confidence when the caller asks for it (flag 0x400): d = (form − expectation) × 225
/// (`ftol`); with flag 0x100 (backed) the supporters get d, d + 1000 for a player the club favours or d − 1000 for one it
/// dislikes, and the board d, d + 1000 × the chairman's weight for a player the chairman favours or d − 1000 × weight
/// for one he dislikes; with 0x80 (criticised) the signs turn (−d, −1000 − d, 1000 − d; the board −d, −1000 × weight
/// − d, 1000 × weight − d); with neither, h = |d/2| and the supporters get −h, −200 − h or 200 − h, the board −h,
/// −200 × weight − h or 200 × weight − h.
unsafe fn story_effects_confidence(s: &StoryEffects, chairman_weight: f64, response: u32) {
    covered!("sx01c_1 story effects confidence");
    let (club, player, kind) = (s.club, s.player, s.kind);
    let d = ftol((s.form as f64 - s.expected as f64) * read_f32(rdata::F32_225) as f64);
    let d16 = d as i16 as f64;
    let favoured = exe::club_favours_staff(player, club) != 0;
    let disliked = !favoured && exe::club_dislikes_staff(player, club) != 0;
    let board = if s.flags & 0x100 != 0 {
        let supporters = if favoured { d + 1000 } else if disliked { d - 1000 } else { d };
        player_story_supporters(s.this, club, player, kind, supporters, response);
        if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
            ftol(chairman_weight * read_f64(rdata::F64_1000) + d16)
        } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
            ftol(d16 - chairman_weight * read_f64(rdata::F64_1000))
        } else {
            d
        }
    } else if s.flags & 0x80 != 0 {
        let supporters = if favoured { -1000 - d } else if disliked { 1000 - d } else { d.wrapping_neg() };
        player_story_supporters(s.this, club, player, kind, supporters, response);
        if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
            ftol(chairman_weight * read_f64(rdata::F64_NEG_1000) - d16)
        } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
            ftol(chairman_weight * read_f64(rdata::F64_1000) - d16)
        } else {
            d.wrapping_neg()
        }
    } else {
        let h = (d as i16 as i32 / 2).abs();
        let supporters = if favoured { -200 - h } else if disliked { 200 - h } else { -h };
        player_story_supporters(s.this, club, player, kind, supporters, response);
        if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
            ftol(chairman_weight * read_f64(rdata::F64_NEG_200) - h as f64)
        } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
            ftol(chairman_weight * read_f64(rdata::F64_200) - h as f64)
        } else {
            -h
        }
    };
    player_story_board(s.this, club, player, kind, board, response);
}

/// The player's form and the manager's expectation (`form`, `expected`): the expectation is (`.pressure()` − 10) / 15 +
/// the contract's `.morale()` / 150, one less when the manager dislikes the player and one more when he favours him,
/// and one less when the player already answered a story (then a draw of his `.temperament()` below draw(6) stops
/// the apology).
unsafe fn story_effects_expectation(s: &mut StoryEffects, response: u32) {
    let (club, player) = (s.club, s.player);
    s.form = exe::media_rate_player_form_for_story(s.this, player, club, s.contract, s.kind, response, s.weight) as f32;
    s.expected = ((player.pressure() as i32 - 10) as f64 * read_f32(STORY_F32_ONE_FIFTEENTH) as f64
        + s.contract.morale() as f64 * read_f32(STORY_F32_ONE_150TH) as f64) as f32;
    if exe::staff_is_disliked_by(club.manager(), player) != 0 {
        s.expected = (s.expected as f64 - read_f32(rdata::F32_1) as f64) as f32;
    } else if exe::staff_is_favoured_by(club.manager(), player) != 0 {
        s.expected = (s.expected as f64 + read_f32(rdata::F32_1) as f64) as f32;
    }
    if story_already_answered(s.this, player) {
        s.expected = (s.expected as f64 - read_f32(rdata::F32_1) as f64) as f32;
        if exe::utils_random(player.temperament() as i32) < exe::utils_random(6) {
            s.may_apologise = false;
        }
    }
}

/// The squad member's morale by the flags (the first that is set): 1 raises it 5; 2 lowers it 5; 4 moves it draw(4) −
/// draw(4); 8: an ambitious member (draw(ambition) above draw(20)) without professionalism (draw below draw(20)) loses
/// 5, otherwise a professional one gains draw(5); 0x10: professional and ambitious, draw(5); 0x20: by the ambition +
/// the weight against draw(10) and the professionalism against the expectation + 30, −8 or draw(5); 0x200: the story's
/// player (when it is not the member) loses 8, the member's contract morale moves by the player's temperament × 3 −
/// 100, and on two zero draws the member dislikes the manager (event flags 8 on his contract). Then, with 0x40, a
/// member whose state byte is below 50 with a zero draw(professionalism) has it raised by draw(20).
unsafe fn story_effects_member_morale(s: &StoryEffects, m: Staff, member_contract: Contract) {
    let f = s.flags;
    let rnd = |n: i32| exe::utils_random(n);
    if f & 1 != 0 {
        adjust_morale(m.addr(), 5);
    } else if f & 2 != 0 {
        adjust_morale(m.addr(), -5);
    } else if f & 4 != 0 {
        let a = rnd(4);
        let b = rnd(4);
        exe::staff_adjust_morale(m, (a as u8).wrapping_sub(b as u8) as u32, 1); // the original pushes the byte difference
    } else if f & 8 != 0 {
        if rnd(m.ambition() as i32) > rnd(20) && rnd(m.professionalism() as i32) < rnd(20) {
            adjust_morale(m.addr(), -5);
        } else if rnd(m.professionalism() as i32) > rnd(20) {
            adjust_morale(m.addr(), rnd(5));
        }
    } else if f & 0x10 != 0 {
        if rnd(m.professionalism() as i32) > rnd(20) && rnd(m.ambition() as i32) > rnd(20) {
            adjust_morale(m.addr(), rnd(5));
        }
    } else if f & 0x20 != 0 {
        let threshold = || ftol(s.expected as f64 + read_f32(STORY_F32_30) as f64);
        if rnd(ftol(m.ambition() as f64 + s.weight as f64)) > rnd(10) && rnd(m.professionalism() as i32) < rnd(threshold()) {
            adjust_morale(m.addr(), -8);
        } else if rnd(m.professionalism() as i32) > rnd(threshold()) {
            adjust_morale(m.addr(), rnd(5));
        }
    } else if f & 0x200 != 0 && s.player != m {
        adjust_morale(s.player.addr(), -8);
        add_contract_morale(member_contract.addr(), s.player.temperament().wrapping_mul(3).wrapping_sub(100) as i32);
        if rnd(s.player.professionalism() as i32) == 0 && rnd(s.player.temperament() as i32) == 0 {
            exe::staff_add_disliked_staff(m, s.club.manager(), 0);
            exe::staff_contract_apply_event_flags(member_contract, 8, 0);
        }
    }
    if f & 0x40 != 0 && story_unrest(m).injury_counter() < 50 && rnd(m.professionalism() as i32) == 0 {
        story_raise_unrest(m, 20);
    }
}

/// Flag 0x80, the member is the story's player (criticised): he loses 5 of morale; in a kind-6 story a zero
/// draw(dirtiness) lowers his `.dirtiness()` (above 10 by 1 + draw(3), above 4 by 1); a zero draw(pressure) gives the
/// contract event 0x100000 on draw(pressure) below draw(15) (under 24) or draw(6); a player who has not answered a
/// story yet, hard-working and professional (draws above draw(20)), may gain consistency and work rate (up to 20) and
/// his state byte is cleared. Then by the form: well above the expectation (+0.5) the contract morale moves by the
/// temperament × 3 − 100, he falls out with the manager on a zero draw(professionalism) or else may answer the
/// criticism and takes the grievance 0x80 (contract flag for a kind-0x13 story, event 0x80000 otherwise), may ask for
/// backing, and for kinds 1, 0x1e and 3 a disloyal, unhappy foreigner may go absent (AWOL); well below (−0.5) he
/// apologises or answers the criticism; in between the contract morale moves by the temperament − 20, he may fall out
/// with the manager and ask for backing.
unsafe fn story_effects_player_criticised(s: &StoryEffects, member_contract: Contract) {
    covered!("sx01c_1 story effects player criticised");
    let (p, c, club) = (s.player, s.contract, s.club);
    let rnd = |n: i32| exe::utils_random(n);
    adjust_morale(p.addr(), -5);
    if s.kind as u16 == 6 && rnd(p.player().dirtiness() as i32) == 0 {
        let sheet = p.player();
        let d = sheet.dirtiness();
        if d > 10 {
            sheet.set_dirtiness(d.wrapping_add((0xffu8.wrapping_sub(rnd(3) as u8)) as i8));
        } else if d > 4 {
            sheet.set_dirtiness(d - 1);
        }
    }
    if rnd(p.pressure() as i32) == 0 {
        let hit = if (p.age() as i8) < 24 { rnd(p.pressure() as i32) < rnd(15) } else { rnd(p.pressure() as i32) < rnd(6) };
        if hit {
            exe::staff_contract_apply_event_flags(c, 0x100000, 0);
        }
    }
    if !story_already_answered(s.this, p) && rnd(p.player().work_rate() as i32) > rnd(20) && rnd(p.professionalism() as i32) > rnd(20) {
        let sheet = p.player();
        if (sheet.consistency() as i32) < rnd(30) {
            let v = sheet.consistency().wrapping_add(rnd(5) as i8);
            sheet.set_consistency(v.min(20));
        }
        let w = sheet.work_rate();
        if (w as i32) < rnd(30) {
            sheet.set_work_rate(w.wrapping_add(1).min(20));
        }
        story_unrest(p).set_injury_counter(0);
    }
    if s.above(read_f64(rdata::F64_0_5)) {
        covered!("sx01c_1 story effects player criticised above");
        add_contract_morale(c.addr(), p.temperament().wrapping_mul(3).wrapping_sub(100) as i32);
        if rnd(p.professionalism() as i32) == 0 {
            story_fall_out(s, 1);
        } else {
            if member_contract.grievances() == 0 && (p.player().player_morale() as i32) > rnd(30) + 10 && !s.chosen {
                story_criticism_reaction(s);
            }
            if s.story13 != 0 {
                exe::staff_contract_set_flags(c, 0x80);
            } else {
                exe::staff_contract_apply_event_flags(c, 0x80000, 0);
            }
        }
        if rnd(p.temperament() as i32) == 0 {
            let k = s.kind as u16;
            let strong = (k == 0x1e || k == 0x20) && (member_contract.grievances() & 0x40000 != 0 || rnd(p.pressure() as i32) == 0);
            if strong {
                story_backing_request(s, if s.chosen { 3 } else { 2 });
            } else if s.story13 == 0 && !s.chosen {
                story_backing_request(s, 0);
            }
            story_raise_unrest(p, 15);
        }
        let k = s.kind as u16;
        if (k == 1 || k == 0x1e || k == 3)
            && !s.chosen
            && exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) == 0
            && exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), p, 0) == 0
            && rnd(p.loyalty() as i32) == 0
            && rnd(p.pressure() as i32) == 0
            && exe::database_is_staff_foreign(p, club.nation(), 1) != 0
            && member_contract.morale() < -50
        {
            covered!("sx01c_1 story effects player goes awol");
            exe::awol_start_absence(read_usize(G_AWOL), p, 0, 0);
        }
    } else if s.below(read_f64(rdata::F64_0_5)) {
        covered!("sx01c_1 story effects player criticised below");
        if s.may_apologise && rnd(p.professionalism() as i32 + p.loyalty() as i32) > rnd(120) {
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(player_story_fns::QUEUE_PLAYER_APOLOGY_NEWS);
            f(s.this, p.addr(), club.addr(), s.kind);
        } else if member_contract.grievances() == 0 && (p.player().player_morale() as i32) > rnd(40) + 15 && s.story13 == 0 && !s.chosen {
            story_criticism_reaction(s);
        }
    } else {
        covered!("sx01c_1 story effects player criticised between");
        add_contract_morale(c.addr(), p.temperament().wrapping_sub(20) as i32);
        if rnd(p.loyalty() as i32 + p.professionalism() as i32) == 0 {
            exe::staff_add_disliked_staff(p, club.manager(), 0);
            if s.story13 != 0 {
                exe::staff_contract_set_flags(c, 0x80);
                return;
            }
            exe::staff_contract_apply_event_flags(c, 0x80000, 0);
        } else if s.story13 != 0 {
            return;
        }
        if rnd(p.temperament() as i32) == 0 && rnd(p.professionalism() as i32) == 0 {
            story_backing_request(s, s.chosen as u32);
            story_raise_unrest(p, 15);
        }
    }
}

/// Flag 0x80, a team-mate of the criticised player: one the player dislikes gains 5 of morale; with one he favours, by
/// the form (±1.5 around the expectation), the player may take the grievance (`story_flag_grievance`) and on zero draws
/// fall out with the manager; with any other, a respected player (leadership × 50 + reputation above 5250 and the
/// team-mate's) may take the grievance when the form is well above the expectation.
unsafe fn story_effects_team_mate_criticised(s: &StoryEffects, m: Staff) {
    let p = s.player;
    let rnd = |n: i32| exe::utils_random(n);
    let (above, below) = (s.above(read_f64(rdata::F64_1_5)), s.below(read_f64(rdata::F64_1_5)));
    if exe::staff_is_disliked_by(p, m) != 0 {
        adjust_morale(m.addr(), 5);
        return;
    }
    if exe::staff_is_favoured_by(p, m) == 0 {
        if above {
            let sheet = p.player();
            let standing = sheet.leadership() as i32 * 50 + sheet.current_reputation() as i32;
            if standing > 0x1482 && standing > m.player().current_reputation() as i32 && rnd(m.temperament() as i32) == 0 && rnd(m.ambition() as i32) != 0 {
                story_flag_grievance(s);
            }
        }
        return;
    }
    covered!("sx01c_1 story effects team mate criticised");
    let fall_out = if above {
        if rnd(m.loyalty() as i32) != 0 {
            story_flag_grievance(s);
        }
        rnd(m.temperament() as i32) == 0 && rnd(m.professionalism() as i32) == 0
    } else if below {
        if rnd(m.loyalty() as i32) > rnd(20) {
            story_flag_grievance(s);
        }
        rnd(m.temperament() as i32) == 0 && rnd(m.professionalism() as i32) == 0 && rnd(p.loyalty() as i32) == 0
    } else {
        if rnd(m.loyalty() as i32) > rnd(10) {
            story_flag_grievance(s);
        }
        rnd(m.temperament() as i32) == 0 && rnd(m.professionalism() as i32 + 5) == 0 && rnd(p.loyalty() as i32 + 10) != 0
    };
    if fall_out {
        story_fall_out(s, 0);
    }
}

/// Flag 0x100, the member is the story's player (backed): in a kind-0x13 story with the contract grievance 0x2000000,
/// a disloyal player (draw(loyalty) below 5) takes the event 0x8000000 unless he is professional (7+) and the injury
/// count reaches 4 (the original counts the player's own injury status once per filled squad slot). Otherwise he gains
/// 10 of morale; in a kind-6 story zero draws of his professionalism may lower his temperament and raise his
/// aggression and dirtiness (up to 20); the grievance 0x40000 may clear; his work rate may drop (and the state byte
/// rise) or his loyalty rise (and the state byte clear). By the form (±0.5): the state byte may rise, he may thank the
/// manager (`queue_player_gratitude_news`) and, ambitious and loyal, favour him (grievances 8 and 0x2000 cleared).
unsafe fn story_effects_player_backed(s: &StoryEffects, member_contract: Contract) {
    covered!("sx01c_1 story effects player backed");
    let (p, club) = (s.player, s.club);
    let rnd = |n: i32| exe::utils_random(n);
    if s.story13 != 0 && member_contract.grievances() & 0x2000000 != 0 {
        let mut injured: u8 = 0;
        for k in 0..50 {
            // defect of the original: the status asked is the story player's, not the squad member's
            if club.squad(k) != 0 && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
                injured = injured.wrapping_add(1);
            }
        }
        if (injured as i8) >= 4 && p.professionalism() >= 7 {
            return;
        }
        if rnd(p.loyalty() as i32) < 5 {
            exe::staff_contract_apply_event_flags(s.contract, 0x8000000, 0);
        }
        return;
    }
    adjust_morale(p.addr(), 10);
    if s.kind as u16 == 6 {
        if p.temperament() > 1 && rnd(p.professionalism() as i32) == 0 {
            p.set_temperament(p.temperament() - 1);
        }
        if p.player().aggression() < 20 && rnd(p.professionalism() as i32) == 0 {
            p.player().set_aggression(p.player().aggression() + 1);
        }
        if p.player().dirtiness() < 20 && rnd(p.professionalism() as i32) == 0 {
            p.player().set_dirtiness(p.player().dirtiness() + 1);
        }
    }
    if member_contract.grievances() & 0x40000 != 0 && rnd(p.pressure() as i32) > rnd(8) {
        exe::staff_contract_clear_grievances(member_contract, 0x40000);
    }
    if rnd(p.player().work_rate() as i32) == 0 && rnd(p.professionalism() as i32) < rnd(10) {
        if rnd(p.player().work_rate() as i32) != 0 {
            p.player().set_work_rate(p.player().work_rate() - 1);
        }
        story_raise_unrest(p, 15);
    } else if rnd(p.professionalism() as i32) > rnd(25) && rnd(p.player().work_rate() as i32) > rnd(40) {
        if story_unrest(p).injury_counter() != 0 {
            story_unrest(p).set_injury_counter(0);
        }
        let l = p.loyalty();
        if (l as i32) < rnd(30) {
            p.set_loyalty(l.wrapping_add(1).min(20));
        }
    }
    let raise_ok = |extra: i32| story_unrest(p).injury_counter() < 50 && rnd(p.professionalism() as i32 + extra) == 0;
    // the 0xa test never holds (defect of the original: story13 is 0 or 0x13), so the grievance 0x4000000 stays
    let thanks = |draw: i32, clear_0x4000000: bool| {
        if rnd(p.loyalty() as i32) > rnd(draw) {
            story_gratitude(s);
            if clear_0x4000000 && s.story13 == 0xa && member_contract.grievances() & 0x4000000 != 0 {
                exe::staff_contract_clear_grievances(member_contract, 0x4000000);
            }
        }
    };
    let make_up = |draw: i32| {
        if rnd(p.ambition() as i32) > rnd(draw) && rnd(p.loyalty() as i32) > rnd(draw) {
            story_make_up(s);
        }
    };
    if s.above(read_f64(rdata::F64_0_5)) {
        covered!("sx01c_1 story effects player backed above");
        if raise_ok(10) {
            story_raise_unrest(p, 20);
        }
        thanks(40, true);
        make_up(60);
    } else if s.below(read_f64(rdata::F64_0_5)) {
        if raise_ok(0) {
            story_raise_unrest(p, 30 - p.professionalism() as i32);
        }
        thanks(10, false);
        make_up(10);
    } else {
        if raise_ok(5) {
            story_raise_unrest(p, 20);
        }
        thanks(20, true);
        make_up(40);
    }
}

/// Flag 0x100, a team-mate of the backed player: for a kind-0x1e story zero draws make the player fall out with the
/// manager; a team-mate the player dislikes loses morale (5, 15 for 0x1e; 15 and 25 when the form is well below), one
/// he favours gains 5 (well above), 10 (well below) or loses 5 (0x1e).
unsafe fn story_effects_team_mate_backed(s: &StoryEffects, m: Staff) {
    let p = s.player;
    let rnd = |n: i32| exe::utils_random(n);
    let k1e = s.kind as u16 == 0x1e;
    let (disliked, favoured) = if s.above(read_f64(rdata::F64_1_5)) {
        if k1e && rnd(m.temperament() as i32) == 0 && rnd(m.professionalism() as i32 + 5) == 0 && rnd(p.loyalty() as i32 + 10) != 0 {
            story_fall_out(s, 0);
        }
        (if k1e { -15 } else { -5 }, if k1e { -5 } else { 5 })
    } else if s.below(read_f64(rdata::F64_1_5)) {
        if k1e && rnd(m.temperament() as i32 / 2) == 0 && rnd(m.professionalism() as i32 / 2) == 0 {
            story_fall_out(s, 0);
        }
        (if k1e { -25 } else { -15 }, if k1e { -5 } else { 10 })
    } else {
        if k1e && rnd(m.temperament() as i32) == 0 && rnd(m.professionalism() as i32) == 0 {
            story_fall_out(s, 0);
        }
        return;
    };
    if exe::staff_is_disliked_by(p, m) != 0 {
        adjust_morale(m.addr(), disliked);
    } else if exe::staff_is_favoured_by(p, m) != 0 {
        adjust_morale(m.addr(), favoured);
    }
}

/// Applies the effects of a media story's outcome to the club and the squad.
///
/// First the staff's confidence in the manager (`board_record::STAFF_CONFIDENCE` of the club's row in
/// MANAGER_MANAGER) moves by `confidence_change`, kept between 1 and 10000, and the chairman's weight is
/// (`.ambition()` − 10) × 0.05 + 1 (0.25 without a chairman). With a player: no contract at the club ends it; otherwise
/// his form (`exe::media_rate_player_form_for_story`) and the manager's expectation (`story_effects_expectation`); with
/// flag 0x400 the supporters' and board's confidence (`story_effects_confidence`); and when the caller passed neither
/// 0x80 nor 0x100 the routine marks the reaction as its own and treats the story as criticism (0x80) when the form beats
/// the expectation by more than 1.
///
/// Then each of the 50 squad members with a player sheet, an id below the staff count and a contract at the club, when
/// draw(contract `.happiness()` + 150) is not below draw(100): his morale by the flags
/// (`story_effects_member_morale`); with a player, flag 0x80 gives `story_effects_player_criticised` (the member is the
/// player) or `story_effects_team_mate_criticised`, and flag 0x100 `story_effects_player_backed` or
/// `story_effects_team_mate_backed`.
///
/// Without a player the expectation read by flag 0x20 is the bits of the response argument as a float (the original's
/// slot is never written). Defects of the original: the injured count of a backed kind-0x13 story asks the player's
/// own status once per filled slot; the thanks of a backed player test the story kind 0x13 against 0xa, never true.
/// Original: `Media::apply_story_effect_flags` (0x736c00, media.cpp, thiscall, ret 0x1c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn apply_story_effect_flags(this: usize, club: usize, kind: u32, flags: u32, weight: f32, response: u32, confidence_change: u32, player: usize) {
    covered!("sx01c_1 story effects");
    let club = Club(club);
    if !club.is_null() {
        let row = manager_manager_row(club);
        write_i16(row + board_record::STAFF_CONFIDENCE, story_staff_confidence_after(club, confidence_change as i16));
    }
    let chairman = club.chairman();
    let chairman_weight = if chairman.is_null() {
        0.25
    } else {
        (chairman.ambition() as i32 - 10) as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_1)
    };
    let mut s = StoryEffects {
        this,
        club,
        kind,
        flags,
        weight,
        player: Staff(player),
        contract: Contract(0),
        story13: if kind as u16 == 0x13 { 0x13 } else { 0 },
        may_apologise: true,
        chosen: false,
        form: 0.0,
        expected: f32::from_bits(response),
    };
    if !s.player.is_null() {
        s.contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), s.player, club));
        if s.contract.is_null() {
            return;
        }
        story_effects_expectation(&mut s, response);
        if s.flags & 0x400 != 0 {
            story_effects_confidence(&s, chairman_weight, response);
        }
        if s.flags & 0x80 == 0 && s.flags & 0x100 == 0 {
            s.chosen = true;
            if s.above(read_f64(rdata::F64_1)) {
                covered!("sx01c_1 story effects chosen criticism");
                s.flags |= 0x80;
            }
        }
    }
    for k in 0..50 {
        let m = Staff(club.squad(k));
        if m.is_null() || m.player().is_null() || read_i32(m.addr()) >= read_i32(G_STAFF_COUNT) {
            continue;
        }
        let mc = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), m, club));
        if mc.is_null() || exe::utils_random(mc.happiness() as i32 + 150) < exe::utils_random(100) {
            continue;
        }
        story_effects_member_morale(&s, m, mc);
        if s.player.is_null() {
            continue;
        }
        if s.flags & 0x80 != 0 {
            if m == s.player {
                story_effects_player_criticised(&s, mc);
            } else {
                story_effects_team_mate_criticised(&s, m);
            }
        } else if s.flags & 0x100 != 0 {
            if m == s.player {
                story_effects_player_backed(&s, mc);
            } else {
                story_effects_team_mate_backed(&s, m);
            }
        }
    }
}

/// Comparer for `apply_story_effect_flags`: the routine draws, posts news and changes confidences, relations and
/// morale, so it cannot run twice; only the original runs and the staff's confidence it leaves is checked against the
/// copy's first piece (`story_staff_confidence_after`). The rest is proven by the career and the mutations.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_apply_story_effect_flags(this: usize, club: usize, kind: u32, flags: u32, weight: f32, response: u32, confidence_change: u32, player: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, f32, u32, u32, usize)>(%%I0%%);
    if club == 0 {
        return o(this, club, kind, flags, weight, response, confidence_change, player);
    }
    let before = read_i16(manager_manager_row(Club(club)) + board_record::STAFF_CONFIDENCE);
    let expected = story_staff_confidence_after(Club(club), confidence_change as i16);
    o(this, club, kind, flags, weight, response, confidence_change, player);
    let after = read_i16(manager_manager_row(Club(club)) + board_record::STAFF_CONFIDENCE);
    SCOREBOARD.record(after == expected, || format!(
        "apply_story_effect_flags(club {:x}, kind {:#x}, flags {:#x}, change {}) staff confidence {} -> {} x original {}",
        club, kind, flags, confidence_change as i16, before, expected, after));
}
