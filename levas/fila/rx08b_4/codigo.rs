// ---------- rx08b_4: the board's handlers: a manager's ultimatum, his requests, a ban appeal and a job application (manager_manager.cpp) ----------

/// The signed +4 byte of a club's 6-byte state record (`G_CLUB_STATE_TABLE`) [inference: the club's direction; above 0
/// the board leans to the manager, below 0 against him].
pub(crate) unsafe fn club_state_mark(club: Club) -> i8 {
    read_i8(read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(6)) + 4)
}

/// The ultimatum record the board answers: +0 the manager's id, +4 the club's id, +8 what the manager demands (signed
/// byte: 1 a bigger stadium, 2 better training facilities, 3 transfer funds, any other a wage rise), +9 the answer (1
/// granted, 2 he goes, 3 refused, 4 refused, a softer blow).
const ULTIMATUM_DEMAND: usize = 8;
const ULTIMATUM_ANSWER: usize = 9;

/// `MANAGER_MANAGER::post_ultimatum_response_news` (0x6ad930; thiscall(this, manager, club, a, b, c, d), ret 0x18),
/// through the exe (the rx08_10 port runs behind its detour).
const ULTIMATUM_RESPONSE_NEWS: usize = 0x6ad930;

/// Posts the ultimatum news (`ULTIMATUM_RESPONSE_NEWS`).
unsafe fn ultimatum_news(this: usize, manager: Staff, club: Club, codes: [u32; 4]) {
    let post: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(ULTIMATUM_RESPONSE_NEWS);
    post(this, manager.addr(), club.addr(), codes[0], codes[1], codes[2], codes[3]);
}

/// The demands and the answers of an ultimatum.
const DEMAND_STADIUM: i8 = 1;
const DEMAND_TRAINING: i8 = 2;
const DEMAND_FUNDS: i8 = 3;
const ANSWER_GRANTED: u8 = 1;
const ANSWER_LET_GO: u8 = 2;
const ANSWER_REFUSED: u8 = 3;
const ANSWER_REFUSED_MILDLY: u8 = 4;

/// A person's non-player `non_player::CURRENT_REPUTATION` (signed word); the person must have the sheet.
unsafe fn ultimatum_reputation(person: Staff) -> i32 {
    read_i16(person.non_player().addr() + non_player::CURRENT_REPUTATION) as i32
}

/// The days since the manager joined his club (`exe::date_days_since` from `G_CURRENT_DATE`).
unsafe fn ultimatum_days_in_job(manager: Staff) -> i32 {
    exe::date_days_since(G_CURRENT_DATE, read_u32(manager.addr() + staff::DATE_JOINED_CLUB), read_u32(manager.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR))
}

/// What the board gives the manager before the demand itself (`lean`, in reputation points): −500 in his first 30
/// days, −250 up to 180, +250 after; +750 when his last ultimatum was under 15 days ago, +500 under 30, −250 under 180;
/// a chairman with patience 1 takes 500 off, under 6 takes 250 off, over 15 adds 250.
unsafe fn ultimatum_lean(club: Club, in_job: i32, since_last: i32) -> i32 {
    let mut lean: i32 = if in_job < 0x1e { -0x1f4 } else if in_job < 0xb4 { -0xfa } else { 0xfa };
    if since_last < 0xf {
        lean += 0x2ee;
    } else if since_last < 0x1e {
        lean += 0x1f4;
    } else if since_last < 0xb4 {
        lean -= 0xfa;
    }
    let chairman = club.chairman();
    if !chairman.is_null() && !chairman.non_player().is_null() {
        let patience = read_i8(chairman.non_player().addr() + non_player::PATIENCE);
        if patience == 1 {
            lean -= 0x1f4;
        } else if patience < 6 {
            lean -= 0xfa;
        } else if patience > 0xf {
            lean += 0xfa;
        }
    }
    lean
}

/// The board's squad investment above its spending (`board_record::SQUAD_INVESTMENT` > `board_record::SPENDING`).
unsafe fn ultimatum_invested(board: usize) -> bool {
    read_f64(board + board_record::SQUAD_INVESTMENT) > read_f64(board + board_record::SPENDING)
}

/// The usual closing test: with the squad investment not above the spending the manager goes; a club state above 2
/// refuses; otherwise he goes when the reputation of the club's manager (`club.manager()`) is at most `floor` or at
/// most the club's reputation − lean − `margin`, and is refused otherwise.
unsafe fn ultimatum_closing(club: Club, board: usize, mark: i8, lean: i32, floor: i32, margin: i32) -> u8 {
    if !ultimatum_invested(board) {
        return ANSWER_LET_GO;
    }
    if mark > 2 {
        return ANSWER_REFUSED;
    }
    let rep = ultimatum_reputation(club.manager());
    let club_rep = read_i16(club.addr() + club::REPUTATION) as i32;
    if rep <= floor || rep <= club_rep - lean - margin { ANSWER_LET_GO } else { ANSWER_REFUSED }
}

/// A wage rise: a chance from the chairman's business sense (10 − it) and the club's wealth band (−1: −5, 0: +5, 1:
/// +10, 2: +15, other: −15), byte arithmetic, scaled by the club state (above 4 × 1.5, above 0 × 0.75, else × 0.25).
/// The club's manager below 5000 reputation, or below the club's reputation − lean − 1500: he goes. With a rise
/// already granted (`board_record::WAGE_RISE_GRANTED`), or a wage ceiling above 90000, `ultimatum_closing` (floors
/// 5750 and 5750 − lean, margin 1250). Otherwise a draw of the chance against one of 50: above it the rise is granted
/// (the chairman's ambition − 5, at least 5; without a chairman sheet 5 + a draw of 10) and the wage ceiling raised by
/// it (`adjust_max_wage`); not above it, `ultimatum_closing` (floor 5750 − lean, margin 1250).
unsafe fn ultimatum_wages(club: Club, board: usize, finance: usize, lean: i32) -> u8 {
    let chairman = club.chairman();
    let has_chairman = !chairman.is_null() && !chairman.non_player().is_null();
    let mut chance: i8 = if has_chairman { 10i8.wrapping_sub(read_i8(chairman.non_player().addr() + non_player::BUSINESS)) } else { 0 };
    chance = chance.wrapping_add(match exe::finance_get_wealth_band(finance) as i8 {
        -1 => -5,
        0 => 5,
        1 => 10,
        2 => 0xf,
        _ => -0xf,
    });
    let mark = club_state_mark(club);
    let scale = if mark > 4 { read_f64(rdata::F64_1_5) } else if mark > 0 { read_f64(rdata::F64_0_75) } else { read_f64(rdata::F64_0_25) };
    let chance = ftol(chance as f64 * scale) as i8;
    let manager_rep = ultimatum_reputation(club.manager());
    let club_rep = read_i16(club.addr() + club::REPUTATION) as i32;
    if manager_rep < 0x1388 || manager_rep < club_rep - lean - 0x5dc {
        return ANSWER_LET_GO;
    }
    if read_u8(board + board_record::WAGE_RISE_GRANTED) != 0 {
        return ultimatum_closing(club, board, mark, lean, 0x1676, 0x4e2);
    }
    if read_i32(finance + finance::WAGE_CEILING) > 0x15f90 {
        return ultimatum_closing(club, board, mark, lean, 0x1676 - lean, 0x4e2);
    }
    if random(chance as i32) > random(0x32) {
        covered!("rx08b_4 wage rise granted");
        let rise = if has_chairman {
            let ambition = read_i8(chairman.addr() + staff::AMBITION);
            if ambition > 5 { ambition - 5 } else { 5 }
        } else {
            (random(10) as i8).wrapping_add(5)
        };
        write_i8(board + board_record::WAGE_RISE_GRANTED, rise);
        adjust_max_wage(finance, rise);
        return ANSWER_GRANTED;
    }
    ultimatum_closing(club, board, club_state_mark(club), lean, 0x1676 - lean, 0x4e2)
}

/// The board's first test for the facilities and the funds: the manager below `floor` reputation goes; so does one
/// whose board confidence (`board_record::CONFIDENCE`) is below `floor` while his reputation is below the club's
/// reputation + `offset` + 750.
unsafe fn ultimatum_first_test(club: Club, manager: Staff, board: usize, floor: i32, offset: i32) -> bool {
    let rep = ultimatum_reputation(manager);
    let club_rep = read_i16(club.addr() + club::REPUTATION) as i32;
    rep < floor || ((read_i16(board + board_record::CONFIDENCE) as i32) < floor && rep < club_rep + offset + 0x2ee)
}

/// Transfer funds: the funds available (`exe::finance_get_transfer_budget` (0, 0, 0, 1, 0), low dword, plus the
/// squad investment truncated) against what the club needs by its reputation R (above 7500: (5R − 25000) × 1000;
/// above 5250: (5R − 11250) × 200; else (5R − 22500) × 20), scaled by the club state (above 4 × 1.5, 0 or more × 0.75,
/// below × 0.25). `ultimatum_first_test` (floor 4250 − lean, offset −lean): he goes. More than needed:
/// `ultimatum_closing` (floor 5750, margin 750). Otherwise the board says yes unless the chairman's ambition draw is 0
/// (no chairman: a draw of 5 is 0), and grants by R (above 7500: (R − 6500) × 5000; above 4250: (R − 2250) × 1000;
/// else (R + 500) × 100); more than the bank balance: refused when the chairman's business draw is above 1 (no sheet: a
/// draw of 5 not 0). The grant goes to `board_record::EXTRA_FUNDS_GRANTED`. Before the chairman's draw the original
/// writes a refusal when funds were already granted, or when the closing test (floor 5750, margin 750) refuses, and
/// the yes always overwrites it: an earlier grant does not stop a second one (DEFEITOS_DO_ORIGINAL).
unsafe fn ultimatum_funds(club: Club, manager: Staff, board: usize, finance: usize, lean: i32) -> u8 {
    let budget = exe::finance_get_transfer_budget(finance, 0, 0, 0usize, 1, 0) as i32;
    let available = budget.wrapping_add(ftol(read_f64(board + board_record::SQUAD_INVESTMENT)));
    let club_rep = read_i16(club.addr() + club::REPUTATION) as i32;
    let needed = if club_rep > 0x1d4c {
        (club_rep * 5 - 0x61a8).wrapping_mul(1000)
    } else if club_rep > 0x1482 {
        (club_rep * 5 - 0x2bf2).wrapping_mul(200)
    } else {
        (club_rep * 5 - 0x57e4).wrapping_mul(20)
    };
    let mark = club_state_mark(club);
    let scale = if mark > 4 { read_f64(rdata::F64_1_5) } else if mark >= 0 { read_f64(rdata::F64_0_75) } else { read_f64(rdata::F64_0_25) };
    let needed = ftol(needed as f64 * scale);
    if ultimatum_first_test(club, manager, board, 0x109a - lean, -lean) {
        return ANSWER_LET_GO;
    }
    if available > needed {
        return ultimatum_closing(club, board, mark, lean, 0x1676, 0x2ee);
    }
    // the original writes a refusal here when funds were already granted, or when the investment is above the
    // spending and the closing test refuses; the answer below always replaces it (DEFEITOS_DO_ORIGINAL)
    let chairman = club.chairman();
    let refused = if !chairman.is_null() { random(read_i8(chairman.addr() + staff::AMBITION) as i32) == 0 } else { random(5) == 0 };
    if refused {
        return ANSWER_REFUSED;
    }
    covered!("rx08b_4 funds granted");
    let grant = if club_rep > 0x1d4c {
        (club_rep - 0x1964).wrapping_mul(5000)
    } else if club_rep > 0x109a {
        (club_rep - 0x8ca).wrapping_mul(1000)
    } else {
        (club_rep + 0x1f4).wrapping_mul(100)
    };
    if grant as i64 > read_i64(finance + finance::BALANCE) {
        let too_much = if !chairman.is_null() && !chairman.non_player().is_null() {
            random(read_i8(chairman.non_player().addr() + non_player::BUSINESS) as i32) > 1
        } else {
            random(5) != 0
        };
        if too_much {
            return ANSWER_REFUSED;
        }
    }
    write_i32(board + board_record::EXTRA_FUNDS_GRANTED, grant);
    ANSWER_GRANTED
}

/// The last tests of the facilities demands: with the club state at most 2, the manager at most `floor` or at most
/// the club's reputation + `offset` goes; otherwise refused when the board confidence beats 2500 + a draw of `draw`,
/// and he goes when it does not.
unsafe fn ultimatum_facilities_end(club: Club, manager: Staff, board: usize, (floor, offset): (i32, i32), draw: i32) -> u8 {
    if club_state_mark(club) <= 2 {
        let rep = ultimatum_reputation(manager);
        if rep <= floor || rep <= read_i16(club.addr() + club::REPUTATION) as i32 + offset {
            return ANSWER_LET_GO;
        }
    }
    let bar = (random(draw) + 0x9c4) as i16;
    if read_i16(board + board_record::CONFIDENCE) > bar { ANSWER_REFUSED } else { ANSWER_LET_GO }
}

/// A bigger stadium: `ultimatum_first_test` (floor 4250, offset +lean). The capacity a club of reputation R needs by its
/// league's strength (`nation::LEAGUE_STANDARD`: above 18, 7R + 5000; above 15, 5R + 2500; above 11, 3R + 10000; above
/// 8, 2R + 2500; else 2R + 1250; half, truncated, for a part-time club): a smaller stadium (`stadium::CAPACITY`) at a
/// club above 5750 with the club state not below 0 and the board confidence above 7250 is granted. Then
/// `ultimatum_facilities_end` (floor 5250, offset lean + 750, draw 7500).
unsafe fn ultimatum_stadium(club: Club, manager: Staff, board: usize, lean: i32) -> u8 {
    if ultimatum_first_test(club, manager, board, 0x109a, lean) {
        return ANSWER_LET_GO;
    }
    let r = read_i16(club.addr() + club::REPUTATION) as i32;
    let standard = read_i8(club.nation().addr() + nation::LEAGUE_STANDARD);
    let mut needed = if standard > 0x12 {
        r * 7 + 0x1388
    } else if standard > 0xf {
        r * 5 + 0x9c4
    } else if standard > 0xb {
        r * 3 + 0x2710
    } else if standard > 8 {
        r * 2 + 0x9c4
    } else {
        r * 2 + 0x4e2
    };
    if read_u8(club.addr() + club::PROFESSIONAL_STATUS) == 3 {
        needed = ftol(needed as f64 * read_f64(rdata::F64_0_5));
    }
    let capacity = read_i32(read_usize(club.addr() + club::STADIUM) + stadium::CAPACITY);
    if capacity < needed && r > 0x1676 && club_state_mark(club) >= 0 && read_i16(board + board_record::CONFIDENCE) > 0x1c52 {
        covered!("rx08b_4 stadium granted");
        return ANSWER_GRANTED;
    }
    ultimatum_facilities_end(club, manager, board, (0x1482, lean + 0x2ee), 0x1d4c)
}

/// Better training facilities: `ultimatum_first_test` (floor 4250 − lean, offset −lean). The facilities a club of
/// reputation R needs by its league's strength (above 14: 20 above 6250, 15 above 4250, 10 above 3250, else 7; above 8:
/// 15, 10 above 4250, else 4; else 10, 7 above 4250, else 2; half, truncated, for a part-time club): poorer facilities
/// (`club::TRAINING`) at a club above 5750 with the club state not below 0 and the board confidence above 7250 − lean
/// are granted. Then `ultimatum_facilities_end` (floor 5250 − lean, offset −lean − 750, draw 5250).
unsafe fn ultimatum_training(club: Club, manager: Staff, board: usize, lean: i32) -> u8 {
    if ultimatum_first_test(club, manager, board, 0x109a - lean, -lean) {
        return ANSWER_LET_GO;
    }
    let r = read_i16(club.addr() + club::REPUTATION);
    let standard = read_i8(club.nation().addr() + nation::LEAGUE_STANDARD);
    let mut needed: i16 = if standard > 0xe {
        if r > 0x186a { 0x14 } else if r > 0x109a { 0xf } else if r > 0xcb2 { 10 } else { 7 }
    } else if standard > 8 {
        if r > 0x186a { 0xf } else if r > 0x109a { 10 } else { 4 }
    } else if r > 0x186a {
        10
    } else if r > 0x109a {
        7
    } else {
        2
    };
    if read_u8(club.addr() + club::PROFESSIONAL_STATUS) == 3 {
        needed = ftol(needed as f64 * read_f64(rdata::F64_0_5)) as i16;
    }
    let training = read_i8(club.addr() + club::TRAINING) as i16;
    if training < needed && r > 0x1676 && club_state_mark(club) >= 0 && (read_i16(board + board_record::CONFIDENCE) as i32) > 0x1c52 - lean {
        covered!("rx08b_4 training granted");
        return ANSWER_GRANTED;
    }
    ultimatum_facilities_end(club, manager, board, (0x1482 - lean, -lean - 0x2ee), 0x1482)
}

/// rule: the board answers a manager's ultimatum: a second one within 180 days of the last (or within 90 of taking
/// the job) is refused (a manager of reputation 5250 or more after the first 180 days of the game: refused outright,
/// otherwise a milder refusal); otherwise each demand has its own test (wages, a bigger stadium, better training
/// facilities, transfer funds) weighed by his reputation, the club's, how long he has been there, the board's
/// confidence, the chairman, the club's money and direction; the answer is granted, refused, or "he goes"
///
/// `this.decide_ultimatum_response(record, club, manager)`: no record: the manager_manager.cpp box of line 0x5a1c; no
/// club or manager: the boxes of lines 0x5a22 and 0x5a29 and the answer 3; no finances
/// (`exe::financetable_get_club_finance` of `G_FINANCE_TABLE`): 3. The board row (`board_row`): the last ultimatum date
/// (`board_record::LAST_ULTIMATUM_DATE`) is read and set to today (`exe::date_operator_3d`). `ultimatum_lean` by the
/// days in the job and since the last one. Since the last above 180, or above 90 with fewer than 90 days in the job:
/// 3 when the manager's reputation is 5250 or more and the game is 180 days old (`G_GAME_START_DATE`), 4 otherwise.
/// Otherwise by the demand (`ULTIMATUM_DEMAND`): `ultimatum_stadium`, `ultimatum_training`, `ultimatum_funds` or
/// `ultimatum_wages`; the answer goes to `ULTIMATUM_ANSWER`.
/// Original: `MANAGER_MANAGER::decide_ultimatum_response` (0x6aca50, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn decide_ultimatum_response(this: usize, record: usize, club: usize, manager: usize) {
    use appoint_fns::MANAGER_MANAGER_CPP as FILE;
    if record == 0 {
        show_error_at(FILE, 0x5a1c);
        return;
    }
    if club == 0 || manager == 0 {
        show_error_at(FILE, if club == 0 { 0x5a22 } else { 0x5a29 });
        write_u8(record + ULTIMATUM_ANSWER, ANSWER_REFUSED);
        return;
    }
    let (club, manager) = (Club(club), Staff(manager));
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if finance == 0 {
        write_u8(record + ULTIMATUM_ANSWER, ANSWER_REFUSED);
        return;
    }
    covered!("rx08b_4 ultimatum answered");
    let board = board_row(this, club);
    let (last, last_leap) = (read_u32(board + board_record::LAST_ULTIMATUM_DATE), read_u32(board + board_record::LAST_ULTIMATUM_DATE + 4));
    let mut today = [0u32; 2];
    exe::date_operator_3d(board + board_record::LAST_ULTIMATUM_DATE, today.as_mut_ptr() as usize, G_CURRENT_DATE);
    let since_last = exe::date_days_since(G_CURRENT_DATE, last, last_leap);
    let lean = ultimatum_lean(club, ultimatum_days_in_job(manager), since_last);
    let answer = if since_last > 0xb4 || (since_last > 0x5a && ultimatum_days_in_job(manager) < 0x5a) {
        let game_days = exe::date_days_since(G_CURRENT_DATE, read_u32(G_GAME_START_DATE), read_u32(G_GAME_START_DATE_FLAGS));
        if ultimatum_reputation(manager) >= 0x1482 && game_days >= 0xb4 { ANSWER_REFUSED } else { ANSWER_REFUSED_MILDLY } // rx08b_4: too soon after the last one
    } else {
        match read_i8(record + ULTIMATUM_DEMAND) {
            DEMAND_STADIUM => ultimatum_stadium(club, manager, board, lean),
            DEMAND_TRAINING => ultimatum_training(club, manager, board, lean),
            DEMAND_FUNDS => ultimatum_funds(club, manager, board, finance, lean),
            _ => ultimatum_wages(club, board, finance, lean),
        }
    };
    write_u8(record + ULTIMATUM_ANSWER, answer);
}

/// rule: a manager's ultimatum to his board: a rumour first when he is well known (reputation above 6750, or above 2000 +
/// a draw of 6750), then the board's answer as news; if he goes, his departure is queued (the reason: his reputation,
/// his short time there, or whether the ultimatum was public) and the chairman may take against him; a refusal costs
/// him 1250 of board confidence, a milder one 500
///
/// `this.handle_manager_ultimatum(manager, club, demand)`: nothing without the manager, his non-player sheet, the club,
/// its division (`club::DIVISION`) or its stadium. The ultimatum record (manager id, club id, the demand, the answer;
/// the original also builds tomorrow's date in front of it and never reads it). With the rumour,
/// `ultimatum_news(demand, 1, x, 0)` where x is 2 (the board backs him) when the club
/// state is above 0, the supporters' confidence (`board_record::SUPPORTERS_CONFIDENCE`) is above 5250 after 1000 days in
/// the job or above 6250, or he has been there under 180 days with a reputation above 6500, and 1 otherwise. Then
/// `decide_ultimatum_response` and `ultimatum_news(demand, 3, answer, rumour)`. Answer 2:
/// the departure item (1, the reason, the club id) to `manager_manager_fields::DEPARTURE_LIST` (`exe::llist_add_tail`,
/// 6 bytes) with the reason 0xc (his reputation above 5250 and above 7250 or above the club's − 750), 0xd (under 240
/// days in the job), 0xe (public) or 0xf; `board_record::FLAGS` | 2; a chairman with preferences and a sheet takes
/// against him (`exe::staff_add_disliked_staff`, forced) when a draw of his patience is below a draw of 21. Answers 3
/// and 4: `board_record::CONFIDENCE` − 1250 or − 500, then kept between 1 and 10000.
/// Original: `MANAGER_MANAGER::handle_manager_ultimatum` (0x6ad610, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn handle_manager_ultimatum(this: usize, manager: usize, club: usize, demand: u32) {
    let (manager, club) = (Staff(manager), Club(club));
    if manager.is_null() || manager.non_player().is_null() || club.is_null() || read_u32(club.addr() + club::DIVISION) == 0 || read_u32(club.addr() + club::STADIUM) == 0 {
        return;
    }
    covered!("rx08b_4 ultimatum handled");
    let mut record = [0u8; 10];
    let r = record.as_mut_ptr() as usize;
    write_u32(r, read_u32(manager.addr()));
    write_u32(r + 4, read_u32(club.addr()));
    write_u8(r + ULTIMATUM_DEMAND, demand as u8);
    let reputation = ultimatum_reputation(manager);
    let public = reputation > 0x1a5e || reputation > random(0x1a5e) + 0x7d0; // rx08b_4: the rumour
    let board = board_row(this, club);
    if public {
        let supporters = read_i16(board + board_record::SUPPORTERS_CONFIDENCE);
        let backed = club_state_mark(club) > 0
            || (supporters > 0x1482 && ultimatum_days_in_job(manager) > 0x3e8)
            || read_i16(board + board_record::SUPPORTERS_CONFIDENCE) > 0x186a
            || (ultimatum_days_in_job(manager) < 0xb4 && ultimatum_reputation(manager) > 0x1964);
        ultimatum_news(this, manager, club, [demand, 1, if backed { 2 } else { 1 }, 0]);
    }
    write_u8(r + ULTIMATUM_ANSWER, 0);
    decide_ultimatum_response(this, r, club.addr(), manager.addr());
    let answer = read_u8(r + ULTIMATUM_ANSWER);
    ultimatum_news(this, manager, club, [demand, 3, answer as u32, public as u32]);
    match answer {
        ANSWER_LET_GO => {
            let rep = ultimatum_reputation(manager);
            let reason: u8 = if rep > 0x1482 && (rep > 0x1c52 || rep > read_i16(club.addr() + club::REPUTATION) as i32 - 0x2ee) {
                0xc
            } else if ultimatum_days_in_job(manager) < 0xf0 {
                0xd
            } else if public {
                0xe
            } else {
                0xf
            };
            let mut item = [1u8, reason, 0, 0, 0, 0];
            item[2..].copy_from_slice(&read_u32(club.addr()).to_le_bytes());
            exe::llist_add_tail(read_usize(this + manager_manager_fields::DEPARTURE_LIST), item.as_ptr() as usize, 6);
            write_u8(board + board_record::FLAGS, read_u8(board + board_record::FLAGS) | 2);
            let chairman = club.chairman();
            if !chairman.is_null() && chairman.staff_preferences() != 0 && !chairman.non_player().is_null() {
                let patience = random(read_i8(chairman.non_player().addr() + non_player::PATIENCE) as i32);
                if patience < random(0x15) {
                    exe::staff_add_disliked_staff(chairman, manager, 1);
                }
            }
        }
        ANSWER_REFUSED | ANSWER_REFUSED_MILDLY => {
            let cut: i16 = if answer == ANSWER_REFUSED { -0x4e2 } else { -0x1f4 };
            let confidence = read_i16(board + board_record::CONFIDENCE).wrapping_add(cut);
            write_i16(board + board_record::CONFIDENCE, confidence.clamp(1, 0x2710));
        }
        _ => {}
    }
}

/// Comparer for `decide_ultimatum_response`: side by side on the answer, the club's board row, its finances' first
/// 0x20 bytes (the wage ceiling) and the random generator (the copy's draws not counted); the boxes run only the original.
pub(crate) unsafe extern "thiscall" fn compare_decide_ultimatum_response(this: usize, record: usize, club: usize, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I0%%);
    let finance = if club != 0 { exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club) } else { 0 };
    if record == 0 || club == 0 || manager == 0 || finance == 0 {
        return o(this, record, club, manager);
    }
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.extend([(record + ULTIMATUM_ANSWER, 1), (board_row(this, Club(club)), 0x49), (finance, 0x20)]);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, record, club, manager),
        || crate::rng::without_counting(|| decide_ultimatum_response(this, record, club, manager)));
    SCOREBOARD.record(same, || format!("decide_ultimatum_response (board ultimatum, demand {}): {:?}", read_i8(record + ULTIMATUM_DEMAND), first_differences(&mo, &mr)));
}

/// Comparer for `handle_manager_ultimatum`: it posts news and queues the departure (it cannot run twice), so only the
/// original runs; the board's answer is compared by its own comparer.
pub(crate) unsafe extern "thiscall" fn compare_handle_manager_ultimatum(this: usize, manager: usize, club: usize, demand: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I1%%)(this, manager, club, demand);
}

/// `MANAGER_MANAGER::post_board_request_reply_news` (0x6a9350; thiscall(this, club, request, reply), ret 0xc), through
/// the exe (the sx14b_4 port runs behind its detour).
const BOARD_REQUEST_REPLY_NEWS: usize = 0x6a9350;

/// The board's replies to a request: granted, no money, asked again, refused, no confidence, already granted, no own
/// stadium, the squad cost too much, too late to ask, too many requests, the manager is not respected, the last request.
const REPLY_GRANTED: u32 = 0;
const REPLY_NO_MONEY: u32 = 2;
const REPLY_ASKED_AGAIN: u32 = 3;
const REPLY_REFUSED: u32 = 4;
const REPLY_NO_CONFIDENCE: u32 = 5;
const REPLY_ALREADY_GRANTED: u32 = 6;
const REPLY_NO_OWN_STADIUM: u32 = 7;
const REPLY_SQUAD_TOO_COSTLY: u32 = 8;
const REPLY_TOO_LATE: u32 = 9;
const REPLY_TOO_MANY: u32 = 0xa;
const REPLY_NOT_RESPECTED: u32 = 0xb;
const REPLY_LAST_ONE: u32 = 0xc;

/// The requests: a bigger stadium, better training facilities, transfer funds, a wage rise, more time, and request 6
/// (money for the club [inference]).
const REQUEST_STADIUM: i8 = 1;
const REQUEST_TRAINING: i8 = 2;
const REQUEST_FUNDS: i8 = 3;
const REQUEST_WAGES: i8 = 4;
const REQUEST_MORE_TIME: i8 = 5;
const REQUEST_MONEY: i8 = 6;

/// Posts the board's reply to the manager's request (`BOARD_REQUEST_REPLY_NEWS`).
unsafe fn board_request_reply(this: usize, club: Club, request: u32, reply: u32) {
    let post: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(BOARD_REQUEST_REPLY_NEWS);
    post(this, club.addr(), request, reply);
}

/// Whether today is inside the season of the club's nation: the key nation index (`exe::key_nation_index_of_nation`;
/// 0 when the club has no nation), from 0 to 34, gives the season's two dates (`G_LEAGUE_WORLD_CUP_START_DATE`, 0x48
/// bytes a nation); today after the first and before the second.
unsafe fn board_request_in_season(club: Club) -> bool {
    let nation = club.nation().addr();
    let k = if nation != 0 { exe::key_nation_index_of_nation(nation) } else { 0 };
    if !(0..0x23).contains(&k) {
        return false;
    }
    let t = G_LEAGUE_WORLD_CUP_START_DATE + k as usize * 0x48;
    let (day, year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    let after = (day > read_i16(t) && year == read_u16(t + 2)) || year > read_u16(t + 2);
    let before = (day < read_i16(t + 0xf) && year == read_u16(t + 0x11)) || year < read_u16(t + 0x11);
    after && before
}

/// Scales an amount by how the club did against the board's target, in season: the target position
/// (`board_record::TARGET_POSITION`, a byte) minus last season's final position (`comp_stats_club::FINAL_POSITION`)
/// above 4: × 1.5; above 0: as it is; otherwise × 0.25 (truncated). Out of season: × 0.75.
unsafe fn board_request_scale(amount: i32, in_season: bool, board: usize, stats: usize) -> i32 {
    if !in_season {
        return ftol(amount as f64 * read_f64(rdata::F64_0_75));
    }
    let lead = read_u8(board + board_record::TARGET_POSITION).wrapping_sub(read_u8(stats + comp_stats_club::FINAL_POSITION)) as i8;
    if lead > 4 {
        ftol(amount as f64 * read_f64(rdata::F64_1_5))
    } else if lead > 0 {
        amount
    } else {
        ftol(amount as f64 * read_f64(rdata::F64_0_25))
    }
}

/// The club's 64-bit bank balance (`finance::BALANCE`).
unsafe fn board_request_balance(finance: usize) -> i64 {
    read_i64(finance + finance::BALANCE)
}

/// A bigger stadium: already approved (`board_record::FLAGS` 0x40), 6; no stadium or not its own (`club::OWN_STADIUM`),
/// 7; above 75000 seats, refused; the money by the capacity (above 25000: 7,500,000; above 15000: 5,000,000; else
/// 2,500,000), less in the bank: no money. A crowd (`club::ATTENDANCE`) not above 85% of the capacity: refused; then
/// a chairman's ambition draw (no chairman: a draw of 5) at 0 refuses, otherwise approved (flag 0x40).
unsafe fn board_request_stadium(this: usize, club: Club, request: u32, board: usize, finance: usize) {
    if read_u8(board + board_record::FLAGS) & 0x40 != 0 {
        return board_request_reply(this, club, request, REPLY_ALREADY_GRANTED);
    }
    let stadium = read_usize(club.addr() + club::STADIUM);
    if stadium == 0 || read_u8(club.addr() + club::OWN_STADIUM) == 0 {
        return board_request_reply(this, club, request, REPLY_NO_OWN_STADIUM);
    }
    let capacity = read_i32(stadium + stadium::CAPACITY);
    if capacity > 0x124f8 {
        return board_request_reply(this, club, request, REPLY_REFUSED);
    }
    let cost: i64 = if capacity > 0x61a8 { 0x7270e0 } else if capacity > 0x3a98 { 0x4c4b40 } else { 0x2625a0 };
    if board_request_balance(finance) < cost {
        return board_request_reply(this, club, request, REPLY_NO_MONEY);
    }
    let full = capacity as f64 * read_f64(rdata::F64_0_85) < read_i32(club.addr() + club::ATTENDANCE) as f64;
    let chairman = club.chairman();
    let yes = full && if !chairman.is_null() { random(read_i8(chairman.addr() + staff::AMBITION) as i32) != 0 } else { random(5) != 0 };
    if !yes {
        return board_request_reply(this, club, request, REPLY_REFUSED);
    }
    covered!("rx08b_4 stadium approved");
    board_request_reply(this, club, request, REPLY_GRANTED);
    write_u8(board + board_record::FLAGS, read_u8(board + board_record::FLAGS) | 0x40);
}

/// Better training facilities (`club::TRAINING` = t): already approved (flag 0x80), 6; less than (t + 2) × 125000 in the
/// bank: no money. Approved for a club above 7250 reputation under 20, above 4750 under 15, above 3250 under 10, or
/// with more than 1,250,000 in the bank under 6; otherwise a draw of 20 against the chairman's ambition − t + 10 (no
/// chairman: 20 − t): above it refused, otherwise approved (flag 0x80).
unsafe fn board_request_training(this: usize, club: Club, request: u32, board: usize, finance: usize) {
    if read_u8(board + board_record::FLAGS) & 0x80 != 0 {
        return board_request_reply(this, club, request, REPLY_ALREADY_GRANTED);
    }
    let training = read_i8(club.addr() + club::TRAINING) as i32;
    let balance = board_request_balance(finance);
    if balance < ((training + 2).wrapping_mul(125000)) as i64 {
        return board_request_reply(this, club, request, REPLY_NO_MONEY);
    }
    let rep = read_i16(club.addr() + club::REPUTATION);
    let deserved = (rep > 0x1c52 && training < 0x14) || (rep > 0x128e && training < 0xf) || (rep > 0xcb2 && training < 10) || (balance > 0x1312d0 && training < 6);
    let yes = deserved || {
        let chairman = club.chairman();
        let r = random(0x14);
        let bar = if !chairman.is_null() { read_i8(chairman.addr() + staff::AMBITION) as i32 - training + 10 } else { 0x14 - training };
        bar >= r
    };
    if !yes {
        return board_request_reply(this, club, request, REPLY_REFUSED);
    }
    covered!("rx08b_4 training approved");
    board_request_reply(this, club, request, REPLY_GRANTED);
    write_u8(board + board_record::FLAGS, read_u8(board + board_record::FLAGS) | 0x80);
}

/// Transfer funds: already granted (`board_record::EXTRA_FUNDS_GRANTED`), 6; under 2,500,000 in the bank, no money; a
/// squad investment above the spending, 8; a chairman's ambition draw (no chairman: a draw of 5) at 0, refused. The
/// funds available (`exe::finance_get_transfer_budget` (0, 0, 0, 1, 0), low dword, plus the squad investment
/// truncated) against what the club needs by its reputation R (above 7500: (R − 5000) × 5000; above 5250: (R − 2250)
/// × 1000; else (R − 4500) × 100), scaled (`board_request_scale`): more available, refused. The grant by R (above 7500:
/// (R − 6500) × 5000; above 4250: (R − 2250) × 1000; else (R + 500) × 100); more than the bank balance: no money unless
/// the chairman's business draw (no sheet: a draw of 10) is 0. Granted, kept in `EXTRA_FUNDS_GRANTED`.
unsafe fn board_request_funds(this: usize, club: Club, request: u32, (board, stats, finance): (usize, usize, usize), in_season: bool) {
    if read_u32(board + board_record::EXTRA_FUNDS_GRANTED) != 0 {
        return board_request_reply(this, club, request, REPLY_ALREADY_GRANTED);
    }
    if board_request_balance(finance) < 0x2625a0 {
        return board_request_reply(this, club, request, REPLY_NO_MONEY);
    }
    if read_f64(board + board_record::SQUAD_INVESTMENT) > read_f64(board + board_record::SPENDING) {
        return board_request_reply(this, club, request, REPLY_SQUAD_TOO_COSTLY);
    }
    let chairman = club.chairman();
    let draw = if !chairman.is_null() { random(read_i8(chairman.addr() + staff::AMBITION) as i32) } else { random(5) };
    if draw == 0 {
        return board_request_reply(this, club, request, REPLY_REFUSED);
    }
    let budget = exe::finance_get_transfer_budget(finance, 0, 0, 0usize, 1, 0) as i32;
    let available = budget.wrapping_add(ftol(read_f64(board + board_record::SQUAD_INVESTMENT)));
    let r = read_i16(club.addr() + club::REPUTATION) as i32;
    let needed = if r > 0x1d4c {
        (r - 0x1388).wrapping_mul(5000)
    } else if r > 0x1482 {
        (r - 0x8ca).wrapping_mul(1000)
    } else {
        (r - 0x1194).wrapping_mul(100)
    };
    if available > board_request_scale(needed, in_season, board, stats) {
        return board_request_reply(this, club, request, REPLY_REFUSED);
    }
    let grant = if r > 0x1d4c {
        (r - 0x1964).wrapping_mul(5000)
    } else if r > 0x109a {
        (r - 0x8ca).wrapping_mul(1000)
    } else {
        (r + 0x1f4).wrapping_mul(100)
    };
    if grant as i64 > board_request_balance(finance) {
        let np = if chairman.is_null() { 0 } else { chairman.non_player().addr() };
        let draw = if np != 0 { random(read_i8(np + non_player::BUSINESS) as i32) } else { random(10) };
        if draw != 0 {
            return board_request_reply(this, club, request, REPLY_NO_MONEY);
        }
    }
    covered!("rx08b_4 funds approved");
    write_i32(board + board_record::EXTRA_FUNDS_GRANTED, grant);
    board_request_reply(this, club, request, REPLY_GRANTED);
}

/// A wage rise: already granted (`board_record::WAGE_RISE_GRANTED`), 6; a wage ceiling above 60000, refused. A chance:
/// 10 − the chairman's business sense (0 without one), plus the wealth band's (−1: −5, 0: +5, 1: +10, 2: +15, other:
/// −15), byte arithmetic, scaled (`board_request_scale`, truncated to a byte); a draw of it not above a draw of 100:
/// refused. Otherwise the rise (the chairman's ambition − 5, at least 5; without a chairman sheet 5 + a draw of 10)
/// raises the wage ceiling (`adjust_max_wage`), granted.
unsafe fn board_request_wages(this: usize, club: Club, request: u32, (board, stats, finance): (usize, usize, usize), in_season: bool) {
    if read_u8(board + board_record::WAGE_RISE_GRANTED) != 0 {
        return board_request_reply(this, club, request, REPLY_ALREADY_GRANTED);
    }
    if read_i32(finance + finance::WAGE_CEILING) > 0xea60 {
        return board_request_reply(this, club, request, REPLY_REFUSED);
    }
    let chairman = club.chairman();
    let has_chairman = !chairman.is_null() && !chairman.non_player().is_null();
    let mut chance: i8 = if has_chairman { 10i8.wrapping_sub(read_i8(chairman.non_player().addr() + non_player::BUSINESS)) } else { 0 };
    chance = chance.wrapping_add(match exe::finance_get_wealth_band(finance) as i8 {
        -1 => -5,
        0 => 5,
        1 => 10,
        2 => 0xf,
        _ => -0xf,
    });
    let chance = board_request_scale(chance as i32, in_season, board, stats) as i8;
    if random(chance as i32) <= random(100) {
        return board_request_reply(this, club, request, REPLY_REFUSED);
    }
    covered!("rx08b_4 wage rise approved");
    let rise = if has_chairman {
        let ambition = read_i8(chairman.addr() + staff::AMBITION);
        if ambition > 5 { ambition - 5 } else { 5 }
    } else {
        (random(10) as i8).wrapping_add(5)
    };
    write_i8(board + board_record::WAGE_RISE_GRANTED, rise);
    adjust_max_wage(finance, read_i8(board + board_record::WAGE_RISE_GRANTED));
    board_request_reply(this, club, request, REPLY_GRANTED);
}

/// More time: already given (`board_record::REQUEST_STATE` bit 0), 6; a board confidence above 4275, refused. The
/// manager's days in the job less 10 × the chairman's patience (100 without a chairman sheet): 180 or more, too late.
/// Otherwise the bit is set and the confidence goes up 1250, to at least 4275; granted.
unsafe fn board_request_more_time(this: usize, club: Club, request: u32, board: usize) {
    if read_u8(board + board_record::REQUEST_STATE) & 1 != 0 {
        return board_request_reply(this, club, request, REPLY_ALREADY_GRANTED);
    }
    if read_i16(board + board_record::CONFIDENCE) > 0x10b3 {
        return board_request_reply(this, club, request, REPLY_REFUSED);
    }
    let manager = club.manager();
    let days = exe::date_days_since(G_CURRENT_DATE, read_u32(manager.addr() + staff::DATE_JOINED_CLUB), read_u32(manager.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR));
    let chairman = club.chairman();
    let np = if chairman.is_null() { 0 } else { chairman.non_player().addr() };
    let days = if np != 0 { days - 10 * read_i8(np + non_player::PATIENCE) as i32 } else { days - 0x64 };
    if days >= 0xb4 {
        return board_request_reply(this, club, request, REPLY_TOO_LATE);
    }
    covered!("rx08b_4 more time given");
    write_u8(board + board_record::REQUEST_STATE, read_u8(board + board_record::REQUEST_STATE) | 1);
    let confidence = read_i16(board + board_record::CONFIDENCE).wrapping_add(0x4e2);
    write_i16(board + board_record::CONFIDENCE, confidence.max(0x10b3));
    board_request_reply(this, club, request, REPLY_GRANTED);
}

/// rule: the board answers the manager's request (a bigger stadium, better training facilities, transfer funds, a
/// wage rise, more time, money): too many requests (more than the chairman's patience / 5 + 2, 3 without a chairman)
/// are turned down and cost 250 of confidence, a board confidence below 3500 or the same request twice in a row is
/// turned down; a manager below 5000 reputation (4250 for the training) or 1500 below the club's is not listened to;
/// each request then has its own test of money, need and the chairman's mood
///
/// `this.handle_board_request(club, request)`: nothing without the club, its division, its manager or his sheet, or for
/// a national team (`exe::database_is_national_team`). The requests count (`board_record::REQUESTS_MADE`) goes up;
/// the limit is the chairman's patience / 5 (signed) + 2 (3 without a chairman sheet). Except for request 5: at the
/// limit, reply 0xc; above it, reply 0xa and the confidence − 250 (no clamp); confidence below 3500, reply 5; the same
/// request as the last one (`board_record::REQUEST_STATE` bits 1..4, signed) with flag 0x20, reply 3. The request is
/// kept as the last one. In season (`board_request_in_season`). The club's competition statistics
/// (`exe::comp_stats_find_club_entry` of its division's `league::COMP_STATS`) and finances
/// (`exe::financetable_get_club_finance`); without either, nothing. Requests 1, 3, 4 and 6 (and 2 with 4250): the
/// club's manager below 5000 reputation, or below the club's reputation − 1500: reply 0xb. Then
/// `board_request_stadium`, `board_request_training`, `board_request_funds`, `board_request_wages`,
/// `board_request_more_time`; request 6 replies "no money" under 500,000 in the bank and nothing at all otherwise
/// (DEFEITOS_DO_ORIGINAL). Other requests: nothing.
/// Original: `MANAGER_MANAGER::handle_board_request` (0x6a8810, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn handle_board_request(this: usize, club: usize, request: u32) {
    let club = Club(club);
    if club.is_null() || read_u32(club.addr() + club::DIVISION) == 0 || club.manager().is_null() || club.manager().non_player().is_null()
        || exe::database_is_national_team(club) != 0 {
        return;
    }
    covered!("rx08b_4 board request");
    let board = board_row(this, club);
    write_u8(board + board_record::REQUESTS_MADE, read_u8(board + board_record::REQUESTS_MADE).wrapping_add(1));
    let chairman = club.chairman();
    let limit: i8 = if !chairman.is_null() && !chairman.non_player().is_null() { read_i8(chairman.non_player().addr() + non_player::PATIENCE) / 5 + 2 } else { 3 };
    let made = read_i8(board + board_record::REQUESTS_MADE);
    let kind = request as u8 as i8;
    if kind != REQUEST_MORE_TIME {
        if made == limit {
            return board_request_reply(this, club, request, REPLY_LAST_ONE);
        }
        if made > limit {
            board_request_reply(this, club, request, REPLY_TOO_MANY);
            write_i16(board + board_record::CONFIDENCE, read_i16(board + board_record::CONFIDENCE).wrapping_sub(0xfa)); // rx08b_4: too many requests
            return;
        }
        if read_i16(board + board_record::CONFIDENCE) < 0xdac {
            return board_request_reply(this, club, request, REPLY_NO_CONFIDENCE);
        }
    }
    let state = read_u8(board + board_record::REQUEST_STATE);
    if ((state << 3) as i8) >> 4 == kind && read_u8(board + board_record::FLAGS) & 0x20 != 0 {
        return board_request_reply(this, club, request, REPLY_ASKED_AGAIN);
    }
    write_u8(board + board_record::REQUEST_STATE, (state & 0xe1) | ((request as u8 & 0xf) << 1));
    let in_season = board_request_in_season(club);
    let stats = exe::comp_stats_find_club_entry(read_usize(board_week_comp_object(club.division().addr()) + league::COMP_STATS), club);
    if stats == 0 {
        return;
    }
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if finance == 0 {
        return;
    }
    let respected = |floor: i16| {
        let rep = read_i16(club.manager().non_player().addr() + non_player::CURRENT_REPUTATION);
        rep >= floor && rep as i32 >= read_i16(club.addr() + club::REPUTATION) as i32 - 0x5dc
    };
    match kind {
        REQUEST_STADIUM | REQUEST_FUNDS | REQUEST_WAGES | REQUEST_MONEY if !respected(0x1388) => board_request_reply(this, club, request, REPLY_NOT_RESPECTED),
        REQUEST_TRAINING if !respected(0x109a) => board_request_reply(this, club, request, REPLY_NOT_RESPECTED),
        REQUEST_STADIUM => board_request_stadium(this, club, request, board, finance),
        REQUEST_TRAINING => board_request_training(this, club, request, board, finance),
        REQUEST_FUNDS => board_request_funds(this, club, request, (board, stats, finance), in_season),
        REQUEST_WAGES => board_request_wages(this, club, request, (board, stats, finance), in_season),
        REQUEST_MORE_TIME => board_request_more_time(this, club, request, board),
        REQUEST_MONEY => {
            if board_request_balance(finance) < 0x7a120 {
                board_request_reply(this, club, request, REPLY_NO_MONEY);
            }
        }
        _ => {}
    }
}

/// Comparer for `handle_board_request`: it posts the board's reply (it cannot run twice), so only the original runs;
/// the copy is proven by playing comparing (the reply news and the board's flags).
pub(crate) unsafe extern "thiscall" fn compare_handle_board_request(this: usize, club: usize, request: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%)(this, club, request);
}

/// `MANAGER_MANAGER` 0x69c7e0 (thiscall(this, person, his club, other club), ret 0xc): the applicant's own board
/// learns he applied elsewhere (the memo; the sx14b_4 port runs behind its detour).
const JOB_APPLICATION_MEMO: usize = 0x69c7e0;

/// The job application news item and the club history entry of a rejected application.
const JOB_APPLICATION_NEWS: u32 = 0x1773;
const HISTORY_REJECTED_APPLICATION: u8 = 0x96;

/// The vacancy item (`manager_manager_fields::VACANCY_LIST`, 0x26 bytes): +6 the candidate chosen by the club, +0xa
/// the best applicant's id, `job_vacancy::CLUB_ID`, +0x12 the best applicant's score.
const VACANCY_SIZE: usize = 0x26;
const VACANCY_CHOSEN: usize = 6;
const VACANCY_APPLICANT: usize = 0xa;
const VACANCY_SCORE: usize = 0x12;

/// The club's vacancy in the list: the cursor back to the head, then up to the count (signed word) of nodes
/// (`exe::llist_get_next`); a missing node shows the manager_manager.cpp box at `row` and gives `Err`.
unsafe fn find_vacancy(this: usize, club: Club, row: u32) -> Result<usize, ()> {
    let list = read_usize(this + manager_manager_fields::VACANCY_LIST);
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    for _ in 0..read_i16(list + llist::COUNT).max(0) {
        let item = exe::llist_get_next(read_usize(this + manager_manager_fields::VACANCY_LIST));
        if item == 0 {
            show_error_at(appoint_fns::MANAGER_MANAGER_CPP, row);
            return Err(());
        }
        if read_u32(item + job_vacancy::CLUB_ID) == read_u32(club.addr()) {
            return Ok(item);
        }
    }
    Ok(0)
}

/// Whether the club turned this applicant down in the last 180 days: its history (`exe::staff_records_get_club_history`
/// of `G_STAFF_RECORDS`, a list the caller frees), walked from the end (`exe::llist_get_prev`), with an entry of type
/// 0x96 for him dated within 180 days. The list is destroyed (`LLIST::~LLIST`, operator delete).
unsafe fn recently_rejected(club: Club, applicant: Staff) -> bool {
    let history = exe::staff_records_get_club_history(read_usize(G_STAFF_RECORDS), club, 0);
    if history == 0 {
        return false;
    }
    write_u32(history + llist::CURSOR, read_u32(history + llist::TAIL));
    let mut found = false;
    loop {
        let e = exe::llist_get_prev(history);
        if e == 0 {
            break;
        }
        if exe::date_days_since(G_CURRENT_DATE, read_u32(e), read_u32(e + 4)) < 0xb4 && read_u8(e + 8) == HISTORY_REJECTED_APPLICATION
            && read_u32(e + 9) == read_u32(applicant.addr()) {
            found = true;
            break;
        }
    }
    exe::drop_llist(history);
    exe::crt_operator_delete(history);
    found
}

/// Sends the application news (0x1773: the club's id, `rejected`) to the applicant (`exe::news_deliver_to_manager`).
unsafe fn job_application_news(club: Club, applicant: Staff, rejected: u32) {
    let mut news = [0u32; 0x3a]; // the local news item (0xe0 bytes in the original)
    let n = news.as_mut_ptr() as usize;
    exe::new_newsitem(n, JOB_APPLICATION_NEWS, 0);
    exe::newsitem_set_slot(n, 0, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 1, rejected);
    exe::news_deliver_to_manager(read_usize(G_NEWS), n, applicant, 0);
}

/// The applicant's own board when he applies elsewhere and is found out: a draw of 4 at 0 leaves it; a national team
/// as the other club leaves it too. In his first 180 days his loyalty drops by one (when above 1) and a draw of a
/// third of it (signed) at 0 sends the memo (`JOB_APPLICATION_MEMO`); later a draw of loyalty + 5 at 0 sends it. Then
/// the board's confidences (`board_record::CONFIDENCE`, `SUPPORTERS_CONFIDENCE`, `STAFF_CONFIDENCE`, `CONFIDENCE_4`)
/// fall: to 0.9, 0.75, 0.9 and 0.9 when his club's reputation is above the other's, otherwise to 0.975, 0.95, 0.975
/// and 0.975 (truncated); the application date (`board_record::JOB_APPLICATION_DATE`) becomes today.
unsafe fn job_application_board(this: usize, club: Club, applicant: Staff) {
    let own = applicant.club_job();
    if own.is_null() || random(4) == 0 || exe::database_is_national_team(club) != 0 {
        return;
    }
    let memo = || {
        let post: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(JOB_APPLICATION_MEMO);
        post(this, applicant.addr(), applicant.club_job().addr(), club.addr());
    };
    let in_job = exe::date_days_since(G_CURRENT_DATE, read_u32(applicant.addr() + staff::DATE_JOINED_CLUB), read_u32(applicant.addr() + staff::DATE_JOINED_CLUB_LEAP_YEAR));
    if in_job < 0xb4 {
        let loyalty = read_i8(applicant.addr() + staff::LOYALTY);
        if loyalty > 1 {
            write_i8(applicant.addr() + staff::LOYALTY, loyalty - 1);
        }
        if random(read_i8(applicant.addr() + staff::LOYALTY) as i32 / 3) == 0 {
            memo();
        }
    } else if random(read_i8(applicant.addr() + staff::LOYALTY) as i32 + 5) == 0 {
        memo();
    }
    let own = applicant.club_job();
    let board = board_row(this, own);
    let (high, low) = (read_f64(rdata::F64_0_9), read_f64(rdata::F64_0_975));
    let factors = if read_i16(own.addr() + club::REPUTATION) > read_i16(club.addr() + club::REPUTATION) {
        [high, read_f64(rdata::F64_0_75), high, high]
    } else {
        [low, read_f64(rdata::F64_0_95), low, low]
    };
    let fields = [board_record::CONFIDENCE, board_record::SUPPORTERS_CONFIDENCE, board_record::STAFF_CONFIDENCE, board_record::CONFIDENCE_4];
    for (field, k) in fields.into_iter().zip(factors) {
        write_i16(board + field, ftol(read_i16(board + field) as f64 * k) as i16); // rx08b_4: the board loses confidence
    }
    let mut today = [0u32; 2];
    exe::date_operator_3d(board + board_record::JOB_APPLICATION_DATE, today.as_mut_ptr() as usize, G_CURRENT_DATE);
}

/// rule: a manager applies for a job: his own board may find out (most of the time) and lose confidence in him, and
/// tell him so; a human manager's reputation gets a floor; his score for the job is worked out, the club opens the
/// vacancy if it had none, and he is told whether the club turned him down in the last six months; a better score than
/// the best so far makes him the leading applicant
///
/// `this.apply_for_job(club, applicant)`: `job_application_board`. A human manager (id among the last 16 of
/// `G_STAFF_COUNT`): his non-player `HOME_REPUTATION`, `CURRENT_REPUTATION` and `WORLD_REPUTATION` at least 1500, 1250
/// and 250. His score: `exe::score_staff_for_job(applicant, club, 0, 5)`, −250000 without a non-player sheet. The
/// club's vacancy (`find_vacancy`, box 0x307e); none: `exe::manager_manager_add_job_vacancy(club, 0)` and again (box
/// 0x3099), still none: the box of line 0x30a8. The vacancy is copied. Already the leading applicant: news 0x1773 with
/// 1 when the club turned him down in the last 180 days (`recently_rejected`) and nothing more, otherwise with 0; not
/// the leader: news with 0. Then, with a score above the copy's best: when the chosen candidate's id (+6) is not
/// "the applicant's id is 0" (the original's test, DEFEITOS_DO_ORIGINAL), the vacancy is replaced by the copy with him
/// and his score (`exe::llist_remove`, `exe::llist_add_tail`). The original also copies four uninitialised stack bytes
/// inside the local copy before filling it (dead stores).
/// Original: `MANAGER_MANAGER::apply_for_job` (0x69ca00, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn apply_for_job(this: usize, club: usize, applicant: usize) {
    let (club, applicant) = (Club(club), Staff(applicant));
    job_application_board(this, club, applicant);
    covered!("rx08b_4 job application");
    let np = applicant.non_player().addr();
    if read_i32(applicant.addr()) >= read_i32(G_STAFF_COUNT) - 0x10 {
        for (field, floor) in [(non_player::HOME_REPUTATION, 0x5dc), (non_player::CURRENT_REPUTATION, 0x4e2), (non_player::WORLD_REPUTATION, 0xfa)] {
            if read_i16(np + field) < floor {
                write_i16(np + field, floor);
            }
        }
    }
    let score = if applicant.non_player().is_null() { -250000 } else { exe::score_staff_for_job(this, applicant, club, 0, 5) };
    let item = match find_vacancy(this, club, 0x307e) {
        Ok(0) => {
            exe::manager_manager_add_job_vacancy(this, club, 0usize);
            match find_vacancy(this, club, 0x3099) {
                Ok(0) => return show_error_at(appoint_fns::MANAGER_MANAGER_CPP, 0x30a8),
                Ok(item) => item,
                Err(()) => return,
            }
        }
        Ok(item) => item,
        Err(()) => return,
    };
    let mut copy = [0u8; VACANCY_SIZE];
    core::ptr::copy_nonoverlapping(item as *const u8, copy.as_mut_ptr(), VACANCY_SIZE);
    if read_u32(item + VACANCY_APPLICANT) == read_u32(applicant.addr()) && recently_rejected(club, applicant) {
        return job_application_news(club, applicant, 1);
    }
    job_application_news(club, applicant, 0);
    let best = i32::from_le_bytes(copy[VACANCY_SCORE..VACANCY_SCORE + 4].try_into().unwrap());
    let id = read_u32(applicant.addr());
    if score > best && read_u32(item + VACANCY_CHOSEN) != (id == 0) as u32 {
        covered!("rx08b_4 leading applicant");
        exe::llist_remove(read_usize(this + manager_manager_fields::VACANCY_LIST), item);
        copy[VACANCY_APPLICANT..VACANCY_APPLICANT + 4].copy_from_slice(&id.to_le_bytes());
        copy[VACANCY_SCORE..VACANCY_SCORE + 4].copy_from_slice(&score.to_le_bytes());
        exe::llist_add_tail(read_usize(this + manager_manager_fields::VACANCY_LIST), copy.as_ptr() as usize, VACANCY_SIZE as u32);
    }
}

/// Comparer for `apply_for_job`: it sends news and changes the vacancy list (it cannot run twice), so only the original
/// runs; the copy is proven by playing comparing (the application news and the board's reaction).
pub(crate) unsafe extern "thiscall" fn compare_apply_for_job(this: usize, club: usize, applicant: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I3%%)(this, club, applicant);
}


/// `OfficialsManager::get_last_match_rating` (0x79d3b0; thiscall(officials, official), ret 4; the rating in `al`).
const BAN_APPEAL_OFFICIAL_RATING: usize = 0x79d3b0;

/// Size of an official record in `G_OFFICIAL_TABLE`.
const BAN_APPEAL_OFFICIAL_SIZE: usize = 0x2b;

/// The appeal item's kind byte (+0x18) that no appeal can win, and the two kinds that are always refused at the end.
const BAN_APPEAL_KIND_NO_CUT: u8 = 3;
const BAN_APPEAL_KIND_REFUSED: [u8; 2] = [0, 4];

/// The appeal results: refused, upheld (the whole ban goes), cut (`ban_appeal::MATCHES_CUT` or `DAYS_CUT`).
const BAN_APPEAL_REFUSED: u8 = 0;
const BAN_APPEAL_UPHELD: u8 = 1;
const BAN_APPEAL_CUT: u8 = 2;

/// The appeal item queued in `manager_manager_fields::BAN_APPEAL_LIST` (0x19 bytes): the person's id, the result,
/// the hearing date (today + 2), the four rules, the revoke flag, the ban (matches and days), the cuts and the kind.
struct BanAppealItem {
    bytes: [u8; 0x19],
}

impl BanAppealItem {
    /// The item as the arguments give it; the cuts stay 0 until set (the original leaves them as stack leftovers on
    /// the paths that do not set them; DEFEITOS_DO_ORIGINAL).
    unsafe fn new(person: Staff, rules: [u32; 4], revoke: u32, matches: u32, days: u32, kind: u32) -> Self {
        let mut bytes = [0u8; 0x19];
        bytes[..4].copy_from_slice(&read_u32(person.addr()).to_le_bytes());
        bytes[4] = BAN_APPEAL_UPHELD;
        for (k, r) in rules.iter().enumerate() {
            bytes[ban_appeal::RULES + k] = *r as u8;
        }
        bytes[ban_appeal::REVOKE_BY_LENGTH] = revoke as u8;
        bytes[ban_appeal::BAN_MATCHES] = matches as u8;
        bytes[ban_appeal::BAN_DAYS..ban_appeal::BAN_DAYS + 2].copy_from_slice(&(days as u16).to_le_bytes());
        bytes[0x18] = kind as u8; // the appeal's kind
        BanAppealItem { bytes }
    }

    fn set_result(&mut self, result: u8) {
        self.bytes[ban_appeal::RESULT] = result;
    }

    fn result(&self) -> u8 {
        self.bytes[ban_appeal::RESULT]
    }

    /// No cut (both −1).
    fn no_cut(&mut self) {
        self.bytes[ban_appeal::MATCHES_CUT] = 0xff;
        self.bytes[ban_appeal::DAYS_CUT..ban_appeal::DAYS_CUT + 2].copy_from_slice(&[0xff, 0xff]);
    }

    /// Refused with no cut.
    fn refuse(&mut self) {
        self.set_result(BAN_APPEAL_REFUSED);
        self.no_cut();
    }

    /// The hearing date two days ahead (`exe::date_add_days` on `G_CURRENT_DATE`), then the item goes to the end of
    /// the appeals list (`exe::llist_add_tail`).
    unsafe fn queue(&mut self, this: usize) {
        let mut later = [0u32; 2];
        let date = exe::date_add_days(G_CURRENT_DATE, later.as_mut_ptr() as usize, 2);
        for k in 0..8 {
            self.bytes[ban_appeal::DATE + k] = read_u8(date + k);
        }
        exe::llist_add_tail(read_usize(this + manager_manager_fields::BAN_APPEAL_LIST), self.bytes.as_ptr() as usize, 0x19);
    }
}

/// The club's ban appeals so far: the top five bits of `board_record::DISCIPLINE_COUNTERS` (signed, arithmetic shift).
unsafe fn ban_appeals_made(board: usize) -> i8 {
    read_i8(board + board_record::DISCIPLINE_COUNTERS) >> 3
}

/// The FA's patience with the club before appeals stop working: 2, 3 above 2000 reputation, 5 above 4250, 7 above
/// 6750; plus 3 for a manager above 6750 reputation, 1 above 4250.
unsafe fn ban_appeal_patience(club: Club, manager: Staff) -> i8 {
    let rep = read_i16(club.addr() + club::REPUTATION);
    let mut patience: i8 = if rep > 0x1a5e { 7 } else if rep > 0x109a { 5 } else if rep > 0x7d0 { 3 } else { 2 };
    let manager_rep = read_i16(manager.non_player().addr() + non_player::CURRENT_REPUTATION);
    if manager_rep > 0x1a5e {
        patience += 3;
    } else if manager_rep > 0x109a {
        patience += 1;
    }
    patience
}

/// The FA complaint stories a club with many appeals can draw (`queue_manager_fa_complaint_story` on `G_MEDIA`):
/// more than 3 appeals and a club state below −3, a draw of 5 at 0: kind 1 with the ban (matches and days dwords);
/// otherwise more than 5 appeals, a draw of 5 at 0: kind 3; otherwise a club state of 0 or more, a draw of 5 at 0:
/// kind 2. Returns whether a story went out.
unsafe fn ban_appeal_complaints(board: usize, club: Club, person: Staff, matches: u32, days: u32) -> bool {
    let appeals = read_u8(board + board_record::DISCIPLINE_COUNTERS) & 0xf8;
    if appeals as i8 <= 0x18 {
        return false;
    }
    let story = |kind: u32, a: u32, b: u32| queue_manager_fa_complaint_story(read_usize(G_MEDIA), club.addr(), person.addr(), kind, a, b, u32::MAX);
    if club_state_mark(club) < -3 && random(5) == 0 {
        story(1, matches, days);
    } else if (read_u8(board + board_record::DISCIPLINE_COUNTERS) & 0xf8) as i8 > 0x28 && random(5) == 0 {
        story(3, u32::MAX, u32::MAX);
    } else if club_state_mark(club) > -1 && random(5) == 0 {
        story(2, u32::MAX, u32::MAX);
    } else {
        return false;
    }
    true
}

/// The second chance of an appeal the first test refused: the club's reputation against a draw. The bar adds
/// 10 × (aggression² − professionalism² + dirtiness²) (16-bit arithmetic), 6000 less when the appeal is not contested.
/// A club outside Argentina, France and Croatia (`club_nation_not_argentina_france_croatia`) appealing a ban of more
/// than 60 days: reputation above 3250 + the bar + a draw of 10000 cuts the days (a draw of 5: 40, 30, 20, else 10).
/// Otherwise outside Argentina (`club_nation_is_not_argentina`): a ban of more than 3 matches with reputation above
/// 3250 + the bar + a draw of 12500 cuts the matches (France 1; Greece 1 or 2; others 1 up to 5 matches, 1 or 2
/// above); failing that, a ban of more than 2 matches with fewer appeals than the patience and reputation above 4250 +
/// the bar + a draw of 15000 cuts 1 match.
unsafe fn ban_appeal_second_chance(item: &mut BanAppealItem, (board, club, person): (usize, Club, Staff), contest: u32, (matches, days): (i8, i16), patience: i8) {
    let sheet = person.player().addr();
    let a = read_i8(sheet + player_sheet::AGGRESSION) as i16;
    let p = read_i8(person.addr() + staff::PROFESSIONALISM) as i16;
    let d = read_i8(sheet + player_sheet::DIRTINESS) as i16;
    let mut bar = a.wrapping_mul(a).wrapping_sub(p.wrapping_mul(p)).wrapping_add(d.wrapping_mul(d)).wrapping_mul(10);
    if contest as u8 == 0 {
        bar = bar.wrapping_sub(0x1770);
    }
    let rep = read_i16(club.addr() + club::REPUTATION);
    let beats = |draw: i32, base: i16| rep > (random(draw) as i16).wrapping_add(bar).wrapping_add(base);
    if club_nation_not_argentina_france_croatia(club.addr()) as u8 != 0 && days > 0x3c {
        if beats(0x2710, 0xcb2) {
            covered!("rx08b_5 appeal cuts days");
            item.set_result(BAN_APPEAL_CUT);
            let cut: u16 = match random(5) {
                0 => 0x28,
                1 => 0x1e,
                2 => 0x14,
                _ => 10,
            };
            item.bytes[ban_appeal::DAYS_CUT..ban_appeal::DAYS_CUT + 2].copy_from_slice(&cut.to_le_bytes());
        }
        return;
    }
    if club_nation_is_not_argentina(club.addr()) as u8 == 0 {
        return;
    }
    if matches > 3 && beats(0x30d4, 0xcb2) {
        covered!("rx08b_5 appeal cuts matches");
        item.set_result(BAN_APPEAL_CUT);
        let nation = read_u32(club.nation().addr());
        item.bytes[ban_appeal::MATCHES_CUT] = if nation == read_u32(KEY_NATION_FRANCE) {
            1
        } else if nation == read_u32(KEY_NATION_GREECE) || matches > 5 {
            (random(2) as u8).wrapping_add(1)
        } else {
            1
        };
        return;
    }
    if matches > 2 && ban_appeals_made(board) < patience && beats(0x3a98, 0x109a) {
        item.set_result(BAN_APPEAL_CUT);
        item.bytes[ban_appeal::MATCHES_CUT] = 1;
    }
}

/// rule: the FA hears a ban appeal two days later: refused outright when the referee of the match had a good game
/// and the ban is contested, or the player has no club or manager, or the club has appealed more often than the FA's
/// patience and fails a reputation draw; otherwise upheld by a chance from the club's and manager's reputation against
/// the player's conduct (1 in 3 refused above 6250, 19 in 20 above 3250, 49 in 50 below), never for clubs of Greece
/// or Australia, with a second chance to cut the ban; clubs that appeal a lot draw FA complaint stories
///
/// `this.submit_ban_appeal(person, contest, rule × 4, revoke, matches, days, kind, official)` → the result (`al`). No
/// person or player sheet: 0, nothing queued. The appeal item (`BanAppealItem`, result 1). An official (index above
/// −1, `G_OFFICIAL_TABLE`) whose last match rating (`OfficialsManager` on `G_OFFICIALS_MANAGER`, when there is one) is
/// above 6: a contested appeal is queued refused and answers 0; otherwise a contested one starts refused. No club
/// (`staff::CLUB_JOB`), no club manager or no manager sheet: queued refused, 0. `ban_appeal_patience`;
/// `ban_appeal_complaints`. With at least that many appeals already and a contested one, a draw of 10000 above the
/// club's reputation: queued refused, 0. Kind 3: refused with no cut. Without a complaint story, a draw of 1000 at 0
/// posts the kind 0 story. The appeals count goes up by one (bits 3..7 of `board_record::DISCIPLINE_COUNTERS`). The
/// chance: (club reputation + manager reputation) / 2 − 125 × conduct (dirtiness + aggression − professionalism,
/// × 1.5 with a world reputation above 5250); above 6250 a draw of 3 at 0 refuses, above 3250 one of 20 not at 0
/// refuses, otherwise one of 50 not at 0 refuses; a club of Greece or Australia refuses. Kind 3 or still upheld: no
/// cut; otherwise `ban_appeal_second_chance`. Kinds 0 and 4, or a complaint story, end refused with no cut. The item
/// is queued (`BanAppealItem::queue`) and the result answered.
/// Original: `MANAGER_MANAGER::submit_ban_appeal` (0x6ab800, manager_manager.cpp, thiscall, ret 0x2c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn submit_ban_appeal(this: usize, person: usize, contest: u32, rule0: u32, rule1: u32, rule2: u32, rule3: u32,
    revoke: u32, matches: u32, days: u32, kind: u32, official: i32) -> u8 {
    let person = Staff(person);
    if person.is_null() || person.player().is_null() {
        return 0;
    }
    covered!("rx08b_5 ban appeal");
    let mut item = BanAppealItem::new(person, [rule0, rule1, rule2, rule3], revoke, matches, days, kind);
    let contested = contest as u8 != 0;
    let official = if official > -1 { read_usize(G_OFFICIAL_TABLE).wrapping_add((official as usize).wrapping_mul(BAN_APPEAL_OFFICIAL_SIZE)) } else { 0 };
    let officials = read_usize(G_OFFICIALS_MANAGER);
    let rating: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(BAN_APPEAL_OFFICIAL_RATING);
    if officials != 0 && rating(officials, official) as i8 > 6 {
        if contested {
            item.refuse();
            item.queue(this);
            return 0;
        }
    } else if contested {
        item.set_result(BAN_APPEAL_REFUSED);
    }
    let club = person.club_job();
    let manager = if club.is_null() { Staff(0) } else { club.manager() };
    if club.is_null() || manager.is_null() || manager.non_player().is_null() {
        item.refuse();
        item.queue(this);
        return 0;
    }
    let patience = ban_appeal_patience(club, manager);
    let board = board_row(this, club);
    let complained = ban_appeal_complaints(board, club, person, matches, days);
    if ban_appeals_made(board) >= patience && contested && random(0x2710) > read_i16(club.addr() + club::REPUTATION) as i32 {
        item.refuse();
        item.queue(this);
        return 0;
    }
    if kind as u8 == BAN_APPEAL_KIND_NO_CUT {
        item.refuse();
    }
    if !complained && random(1000) == 0 {
        queue_manager_fa_complaint_story(read_usize(G_MEDIA), club.addr(), person.addr(), 0, u32::MAX, u32::MAX, u32::MAX);
    }
    let counters = read_u8(board + board_record::DISCIPLINE_COUNTERS);
    if ((counters as i8) >> 3) < 0x20 {
        write_u8(board + board_record::DISCIPLINE_COUNTERS, (counters & 0xf8).wrapping_add(8) ^ (counters & 7));
    }
    let strength = ftol((read_i16(club.addr() + club::REPUTATION) as i32 + read_i16(manager.non_player().addr() + non_player::CURRENT_REPUTATION) as i32) as f64
        * read_f64(rdata::F64_0_5));
    let sheet = person.player().addr();
    let mut conduct = read_i8(sheet + player_sheet::DIRTINESS) as i32 + read_i8(sheet + player_sheet::AGGRESSION) as i32 - read_i8(person.addr() + staff::PROFESSIONALISM) as i32;
    if read_i16(sheet + player_sheet::WORLD_REPUTATION) > 0x1482 {
        conduct = ftol(conduct as f64 * read_f64(rdata::F64_1_5));
    }
    let chance = strength.wrapping_sub(conduct.wrapping_mul(125)); // rx08b_5: the appeal's chance
    let refused = if chance > 0x186a { random(3) == 0 } else if chance > 0xcb2 { random(0x14) != 0 } else { random(0x32) != 0 };
    if refused {
        item.set_result(BAN_APPEAL_REFUSED);
    }
    let nation = read_u32(club.nation().addr());
    if nation == read_u32(KEY_NATION_GREECE) || nation == read_u32(KEY_NATION_AUSTRALIA) {
        item.set_result(BAN_APPEAL_REFUSED);
    }
    if kind as u8 == BAN_APPEAL_KIND_NO_CUT || item.result() != BAN_APPEAL_REFUSED {
        item.no_cut();
    } else {
        ban_appeal_second_chance(&mut item, (board, club, person), contest, (matches as i8, days as i16), patience);
    }
    if BAN_APPEAL_KIND_REFUSED.contains(&(kind as u8)) || complained {
        item.refuse();
    }
    item.queue(this);
    item.result()
}

/// Comparer for `submit_ban_appeal`: it queues the appeal and posts stories (it cannot run twice), so only the
/// original runs; the copy is proven by playing comparing (the appeal's answer arrives two days later).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_submit_ban_appeal(this: usize, person: usize, contest: u32, rule0: u32, rule1: u32, rule2: u32, rule3: u32,
    revoke: u32, matches: u32, days: u32, kind: u32, official: i32) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, i32) -> u8>(%%I4%%)(this, person, contest, rule0, rule1, rule2,
        rule3, revoke, matches, days, kind, official)
}
