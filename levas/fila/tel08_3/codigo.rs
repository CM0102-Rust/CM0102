// ---------- batch %%LEVA%%: the staff profile's Transfer tab (staff_screens.cpp, part 3) ----------

/// The "  " prefix the wrapped lines of the Transfer tab pass to `add_area_text_by_ref`.
const TRANSFER_TAB_INDENT: usize = 0xa78880;
/// The game's empty text, the argument of the "<%s - COMMENT ...>" formats.
const TRANSFER_TAB_COMMENT: usize = 0xad9c64;
/// The staff_screens.cpp path of the error box.
const TRANSFER_TAB_SOURCE: usize = 0xa84aac;
/// Byte +4 of a club's row in `G_CLUB_STATE_TABLE` (6 bytes per club id): the league form against the expectations,
/// signed (> 2 above, < 0 below) [inference].
const TRANSFER_TAB_CLUB_FORM: usize = 4;
/// Byte +5 of a club's row in `G_CLUB_STATE_TABLE`: nonzero when the dressing room is unsettled [inference].
const TRANSFER_TAB_CLUB_UNREST: usize = 5;

/// What the Transfer tab carries from one piece to the next: the area and the row being written, the first row of the
/// current section (the empty label cells go below it), the person, his contract and its club, the viewer's club, the
/// future transfer the Offers section asked for, the list of interested clubs and the five bytes of the club's league
/// statistics entry (matches played, current and final position, losing and winning run).
struct TransferTab {
    area: u32,
    row: i32,
    first: i32,
    p: Staff,
    contract: Contract,
    club: Club,
    viewer: Club,
    future: usize,
    list: usize,
    matches_played: i8,
    position: i8,
    final_position: i8,
    losing_run: i8,
    winning_run: i8,
}

impl TransferTab {
    /// A cell of the current row in column `col`.
    unsafe fn cell(&self, col: u32, colour: usize, text: usize) {
        staff_tab_cell(self.area, col, self.row, 2, 0, 1, 2, colour, text);
    }
    /// The label (column 0, pale grey) of the current row.
    unsafe fn label(&self, text: usize) {
        self.cell(0, G_GUI_COLOUR_PALE_GREY, text);
    }
    /// The value (column 1, yellow) of the current row.
    unsafe fn value(&self, text: usize) {
        self.cell(1, G_GUI_COLOUR_YELLOW, text);
    }
    /// The screens' buffer as a yellow button of the current row in column 1.
    unsafe fn button(&self, id: u32, value: usize) {
        staff_tab_button(self.area, 1, self.row, 2, 0, 1, 2, G_GUI_COLOUR_YELLOW, JOB_TEXT, id, value as u32);
    }
    /// The empty label of a row below the first of the section.
    unsafe fn pad(&self) {
        if self.row > self.first {
            self.label(0);
        }
    }
    /// The screens' buffer as a wrapped text of the current row (`add_area_text_by_ref`, 0x804160: area, column 1,
    /// the row by address, the style, the colour word, the text, width 0x209 and the "  " indent).
    unsafe fn wrapped(&mut self, colour: usize) {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, *mut i32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(0x804160usize);
        f(G_SCREEN_MANAGER, self.area, 1, &mut self.row, 2, 0, 1, 2, job_colour(colour), JOB_TEXT as u32, 0x209, TRANSFER_TAB_INDENT as u32);
    }
    /// A line of the Future section: the wrapped text in `colour`, the empty label and the next row.
    unsafe fn line_in(&mut self, colour: usize) {
        self.wrapped(colour);
        self.pad();
        self.row += 1;
    }
    /// A yellow line of the Future section.
    unsafe fn line(&mut self) {
        self.line_in(G_GUI_COLOUR_YELLOW);
    }
    /// The translated text `s` as a yellow line of the Future section.
    unsafe fn say(&mut self, s: usize) {
        staff_tab_lang()(JOB_TEXT, s);
        self.line();
    }
    /// Days from the date at `date` (a TCMDate) to today (`Date::days_since` on today).
    unsafe fn days_since(&self, date: usize) -> i32 {
        exe::date_days_since(G_CURRENT_DATE, read_u32(date), read_u32(date + 4))
    }
    /// Days since the person joined his club.
    unsafe fn days_joined(&self) -> i32 {
        self.days_since(self.p.addr() + staff::DATE_JOINED_CLUB)
    }
    /// Days since the contract started.
    unsafe fn days_started(&self) -> i32 {
        self.days_since(self.contract.addr() + contract::DATE_STARTED)
    }
    /// Days from today to the contract's expiry (`Date::days_since` on the expiry date).
    unsafe fn days_to_expiry(&self) -> i32 {
        exe::date_days_since(self.contract.addr() + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP))
    }
    /// Whether the person has a future transfer (asked again, without keeping it).
    unsafe fn future_now(&self) -> bool {
        exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), self.p, 0) != 0
    }
    /// Byte `k` of the club's row in `G_CLUB_STATE_TABLE`.
    unsafe fn club_state(&self, k: usize) -> i8 {
        read_i8(read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_i32(self.club.addr()) as isize * 6) as usize) + k)
    }
    /// The squad status bits of the contract (the high nibble of `contract::SQUAD_STATUS`, in place).
    unsafe fn squad_bits(&self) -> u8 {
        self.contract.squad_status() & 0xf0
    }
    /// The squad status of the contract (`staff_contract_get_squad_status`, 0x8857d0: the high nibble).
    unsafe fn squad(&self) -> u8 {
        self.contract.squad_status() >> 4
    }
    /// `STAFF_CONTRACT::get_flags`.
    unsafe fn flags(&self) -> u32 {
        exe::staff_contract_get_flags(self.contract)
    }
    /// The squad status the person deserves (`contract_manager_get_deserved_squad_status`).
    unsafe fn deserved(&self) -> u8 {
        exe::contract_manager_get_deserved_squad_status(read_usize(G_CONTRACT_MANAGER), self.p, self.contract)
    }
}

/// The person's name into `out` (`database_format_staff_name`, kind `kind`).
unsafe fn transfer_tab_name(s: impl Addr, out: &mut [u8], size: u32, kind: u32) -> usize {
    let b = out.as_mut_ptr() as usize;
    exe::database_format_staff_name(s, b, size, kind);
    b
}

/// Whether the current human controls the club of the person's contract: the contract asked twice (each lookup left in
/// `G_CONTRACT_LOOKUP_RESULT`), false without a contract.
unsafe fn transfer_tab_controls_contract_club(p: Staff) -> bool {
    let (c, club) = contract_tab_lookup(p);
    if c.is_null() || club == 0 {
        return false;
    }
    let (_, club) = contract_tab_lookup(p);
    exe::human_manager_current_controls_club(club, 0, 1) != 0
}

/// The board confidence in the club's manager (`manager_manager_get_board_confidence`, 0x885800: the word +6 of the
/// club's 0x49-byte board record; 0 for a null club).
unsafe fn transfer_tab_board_confidence(club: Club) -> i16 {
    if club.is_null() {
        return 0;
    }
    read_i16(read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add(read_i32(club.addr()) as usize * 0x49) + board_record::CONFIDENCE)
}

/// A money amount as the game shows it (`Cash::format_display_amount`, 0x450aa0, on a local `Cash`) into `out`.
unsafe fn transfer_tab_cash(amount: i32, out: &mut [u8]) -> usize {
    let f: unsafe extern "thiscall" fn(*const i32, usize, u32, u32) -> u32 = core::mem::transmute(0x450aa0usize);
    let b = out.as_mut_ptr() as usize;
    f(&amount, b, 0xc8, 6);
    b
}

/// The top of the Transfer tab: Full Name (when the person has a common name or the human gave him a nickname: the
/// full name, kind 7), Availability (transfer listed by the club, by request, listed for loan or unknown), Value (for
/// a player job without a trial contract: `staff::VALUE` as money, a button 5 when the human controls the person's
/// club or his contract's club), Second Nationality (when set and different: "2nd Nationality" with English names,
/// the nationality as a button 0x7d5 to the national team) and the fluent (fluency ≥ 8) and basic (4..7) languages,
/// joined with ", " and " and ", the basic row only when there is one. Leaves the contract (first lookup, not stored)
/// in the state.
unsafe fn transfer_tab_details(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    let p = st.p;
    let mut buf = [0u8; 0x578];
    let b = buf.as_mut_ptr() as usize;

    let common = read_usize(p.addr() + staff::COMMON_NAME);
    if (common != 0 && read_u8(common) != 0) || exe::human_manager_has_nickname(read_usize(G_HUMAN_MANAGER), p, 0) != 0 {
        covered!("tel08_3 transfer tab full name");
        lang(JOB_TEXT, 0xa8aeccusize); // "  Full Name"
        st.label(JOB_TEXT);
        exe::database_format_staff_name(p, b, 0xc8, 7);
        format(JOB_TEXT, 0x987ff0usize, b); // "  %s"
        st.value(JOB_TEXT);
        st.row += 1;
    }

    lang(JOB_TEXT, 0xa8aebcusize); // "  Availability"
    st.label(JOB_TEXT);
    if exe::staff_is_transfer_listed(p, 0usize) != 0 {
        lang(JOB_TEXT, 0xa8aea0usize); // "  Transfer listed by club"
    } else if exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0 {
        lang(JOB_TEXT, 0xa8ae80usize); // "  Transfer listed by request"
    } else if exe::staff_is_loan_listed(p, 0usize) != 0 {
        lang(JOB_TEXT, 0xa8ae6cusize); // "  Listed for loan"
    } else {
        lang(JOB_TEXT, 0xa8ae3cusize, TRANSFER_TAB_COMMENT); // "  Unknown<%s - COMMENT - unknown availabilty>" tel08_3 unknown availability
    }
    st.value(JOB_TEXT);
    st.row += 1;

    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
    st.contract = contract;
    if exe::database_is_player_job(p.job_for_club() as i8) != 0 && {
        let (c, _) = contract_tab_lookup(p);
        c.is_null() || c.contract_type() & 0x3f != 5
    } {
        covered!("tel08_3 transfer tab value");
        lang(JOB_TEXT, 0xa8ae34usize); // "  Value"
        st.label(JOB_TEXT);
        let value = read_u32(p.addr() + staff::VALUE);
        let amount = exe::cash_get_display_amount(&value, 6);
        exe::cash_format_money(b, 0xc8, amount, 6);
        format(JOB_TEXT, 0x987ff0usize, b); // "  %s"
        let club = p.club_job();
        if (!club.is_null() && exe::human_manager_current_controls_club(club, 0, 1) != 0)
            || (!contract.is_null() && exe::human_manager_current_controls_club(club_at(contract.club() as isize), 0, 1) != 0)
        {
            st.button(5, 0);
        } else {
            st.value(JOB_TEXT);
        }
        st.row += 1;
    }

    let second = read_usize(p.addr() + staff::SECOND_NATION);
    if second != 0 && second != read_usize(p.addr() + staff::NATION) {
        covered!("tel08_3 transfer tab second nationality");
        if read_u32(G_DATABASE_NAMES_LANGUAGE) == 0 {
            staff_tab_strcpy(JOB_TEXT, 0xa8ae20); // "  2nd Nationality"
        } else {
            lang(JOB_TEXT, 0xa8ae08usize); // "  Second Nationality"
        }
        st.label(JOB_TEXT);
        format(JOB_TEXT, 0x987ff0usize, second + nation::NATIONALITY); // "  %s"
        st.button(0x7d5, exe::database_get_national_team(second));
        st.row += 1;
    }

    lang(JOB_TEXT, 0xa8adf4usize); // "  Fluent Languages"
    st.label(JOB_TEXT);
    let mut list = [0u8; 0x3e8];
    let l = list.as_mut_ptr() as usize;
    lang(l, 0xa8add0usize, TRANSFER_TAB_COMMENT); // "  None<%s - COMMENT - no languages>"
    transfer_tab_languages(p, b, l, 8, 0x7f);
    st.value(l);
    st.row += 1;
    list[0] = 0;
    transfer_tab_languages(p, b, l, 4, 7);
    if list[0] != 0 {
        covered!("tel08_3 transfer tab basic languages");
        lang(JOB_TEXT, 0xa8ad70usize); // "  Basic Languages"
        st.label(JOB_TEXT);
        st.value(l);
        st.row += 1;
    }
}

/// The languages (1..64) the person speaks with a fluency from `low` to `high` (`PlayerStats::get_language_fluency`),
/// joined into `list` ("  a", "a, b", "a, b and c") through `tmp`: a first pass counts, the second writes.
unsafe fn transfer_tab_languages(p: Staff, tmp: usize, list: usize, low: i8, high: i8) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    let mut total = 0;
    for pass in 1..=2 {
        let mut n = 0;
        for language in 1..=0x40u32 {
            let f = exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p, 0usize, language, 1) as i8;
            if f < low || f > high {
                continue;
            }
            n += 1;
            if pass != 2 {
                total = n;
                continue;
            }
            let name = database_get_language_name(language);
            if n == 1 {
                format(tmp, 0x987ff0usize, name); // "  %s"
            } else if n < total {
                lang(tmp, 0xa8adacusize, list, name); // "<%s - languages>, <%s - language>"
            } else {
                lang(tmp, 0xa8ad84usize, list, name); // "<%s - languages> and <%s - language>"
            }
            staff_tab_strcpy(list, tmp);
        }
    }
}

/// The Offers section: "None" (no bids) when the person is not under offer; otherwise the future transfer ("<club> -
/// transfer arranged for <date>", the club linked) or each offer of the list in screen parameter 3 that the viewer may
/// see, then a trial offer of the viewer's club, and "None" (no offers) when nothing was written. Asks for the future
/// transfer into the state.
unsafe fn transfer_tab_offers(st: &mut TransferTab, offers: usize) {
    let lang = staff_tab_lang();
    let p = st.p;
    let tm = read_usize(G_TRANSFER_MANAGER);
    let mut buf = [0u8; 0x200];
    let b = buf.as_mut_ptr() as usize;
    lang(JOB_TEXT, 0xa8ad64usize); // "  Offers"
    st.label(JOB_TEXT);
    let viewer = st.viewer;
    st.first = st.row;
    let shortlisted = !viewer.is_null() && exe::scout_manager_is_player_shortlisted(read_usize(G_SCOUT_MANAGER), p, read_usize(viewer.addr() + club::MANAGER)) != 0;
    if exe::transfer_manager_is_player_under_offer(tm, p, 1) == 0 {
        lang(JOB_TEXT, 0xa8ad44usize, TRANSFER_TAB_COMMENT); // "  None<%s - COMMENT - no bids>"
        st.value(JOB_TEXT);
        st.row += 1;
        return;
    }
    if exe::transfer_manager_get_future_transfer(tm, p, &mut st.future as *mut usize as u32) != 0 {
        covered!("tel08_3 transfer tab future transfer");
        let slot = exe::transfer_manager_get_offer_slot(tm, read_i32(st.future + future_transfer::OFFER_SLOT));
        if slot == 0 {
            tel26_error_box(TRANSFER_TAB_SOURCE, 0x1902);
        } else {
            exe::date_get_long_date_string(st.future, b, 0xc8, 0xff);
            let offer = read_usize(slot);
            let club = if read_i32(p.addr()) == read_i32(offer + offer::TARGET_STAFF_ID) {
                club_at(read_i32(offer + offer::BUYER_CLUB_ID) as isize)
            } else {
                let (_, club) = contract_tab_lookup(staff_at(read_i32(offer + offer::TARGET_STAFF_ID) as isize));
                Club(club)
            };
            let g = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32;
            // "  {}<%s - Club Name(e.g.Chelsea)>{} - transfer arranged for {}<%s - Date(e.g.10th November 1998)>."
            lang(JOB_TEXT, 0xa8ace0usize, g, g, 100u32, club.addr() + club::SHORT_NAME, b);
            st.button(0x7d5, club.addr());
            st.row += 1;
        }
    } else if offers != 0 && read_u16(offers) != 0 {
        transfer_tab_offer_list(st, offers, shortlisted, b);
    }
    if !viewer.is_null() {
        let o = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(p.addr()), read_u32(viewer.addr()));
        if o != 0 && read_u8(o + contract_offer::CONTRACT_TYPE) & 0x3f == 5 {
            covered!("tel08_3 transfer tab trial offer");
            let g = read_i8(viewer.addr() + club::SHORT_GENDER_NAME) as i32;
            lang(JOB_TEXT, 0xa8ab84usize, g, g, viewer.addr() + club::SHORT_NAME); // "  {}<%s - Club(e.g.Arsenal)>{} - trial offer"
            st.button(0x7d5, viewer.addr());
            st.row += 1;
        }
    }
    if st.row == st.first {
        lang(JOB_TEXT, 0xa8ab60usize, TRANSFER_TAB_COMMENT); // "  None<%s - COMMENT - no offers>"
        st.value(JOB_TEXT);
        st.row += 1;
    }
}

/// The offers of the list `list` (each node holds an offer slot index): an offer is shown when the human controls the
/// person's contract club or the bidding club, the person knows of it (`offer::PLAYER_AWARE`) or the viewer has him
/// shortlisted. The detailed text (the bid's total value as money, with "part-exchange", or the part-exchange, loan
/// or free transfer kind) goes to whoever controls one of the two clubs or when the person knows; otherwise only the
/// bidding club. The row is a button 0x13 (the human's club, slot), 0xe (the bidding human, slot) or 0x7d5 (the
/// bidding club); loan and free transfer offers always link the club.
unsafe fn transfer_tab_offer_list(st: &mut TransferTab, list: usize, shortlisted: bool, b: usize) {
    let lang = staff_tab_lang();
    let p = st.p;
    let tm = read_usize(G_TRANSFER_MANAGER);
    write_u32(list + 0xa, read_u32(list + 2)); // the list's cursor back to its head
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        let slot = exe::transfer_manager_get_offer_slot(tm, read_i32(node));
        let offer = read_usize(slot);
        let bidder = club_at(read_i32(offer + offer::BUYER_CLUB_ID) as isize);
        let controls = |c: Club| exe::human_manager_current_controls_club(c, 0, 1) != 0;
        if transfer_tab_controls_contract_club(p) || controls(bidder) || read_u8(offer + offer::PLAYER_AWARE) != 0 || shortlisted {
            covered!("tel08_3 transfer tab offer");
            if st.row > st.first {
                st.label(0);
            }
            let value = exe::transfer_offer_get_total_value(offer, 1, 1, 0, 1);
            let v = value as u32;
            let amount = exe::cash_get_display_amount(&v, 5);
            exe::cash_format_money(b, 0xc8, amount, 5);
            let mut kind_only = false;
            let g = read_i8(bidder.addr() + club::SHORT_GENDER_NAME) as i32;
            let name = bidder.addr() + club::SHORT_NAME;
            if transfer_tab_controls_contract_club(p) || controls(bidder) || read_u8(offer + offer::PLAYER_AWARE) != 0 {
                let exchange = read_usize(offer + offer::EXCHANGE_PLAYERS);
                if value != 0 && exchange != 0 {
                    lang(JOB_TEXT, 0xa8ac78usize, g, g, name, b); // "  {}<%s - Club>{} - <%s - Cash> part-exchange"
                } else if read_u8(offer + offer::PRE_CONTRACT) == 0 && value != 0 {
                    lang(JOB_TEXT, 0xa8ac40usize, g, g, name, b); // "  {}<%s - Club>{} - <%s - Cash>"
                } else if exchange != 0 {
                    lang(JOB_TEXT, 0xa8ac10usize, g, g, name); // "  {}<%s - Club(e.g.Arsenal)>{} - part-exchange"
                } else if read_u8(offer + offer::OFFER_TYPE) == 1 {
                    lang(JOB_TEXT, 0xa8abe4usize, g, g, name); // "  {}<%s - Club(e.g.Arsenal)>{} - loan offer"
                    kind_only = true;
                } else {
                    lang(JOB_TEXT, 0xa8abb4usize, g, g, name); // "  {}<%s - Club(e.g.Arsenal)>{} - free transfer"
                    kind_only = true;
                }
            } else {
                lang(JOB_TEXT, 0xa8acc0usize, g, g, name); // "  {}<%s - Club(e.g.Arsenal)>{}"
            }
            if transfer_tab_controls_contract_club(p) && !kind_only {
                st.button(0x13, slot);
            } else if controls(bidder) && !kind_only {
                st.button(0xe, slot);
            } else {
                st.button(0x7d5, bidder.addr());
            }
            st.row += 1;
        }
        node = exe::llist_get_next(list);
    }
}

/// The Interested section: the clubs the shortlist manager finds interested (`collect_interested_clubs` into the
/// state's list), each a button 0x7d5 with its short name, or "None" (no clubs interested).
unsafe fn transfer_tab_interested(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let format = staff_tab_format();
    lang(JOB_TEXT, 0xa8ab30usize, TRANSFER_TAB_COMMENT); // "  Interested<%s - COMMENT - clubs interested>"
    st.label(JOB_TEXT);
    let shortlists = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
    exe::shortlist_manager_collect_interested_clubs(shortlists, st.p, st.list, 1, 1, 0);
    if read_u16(st.list) == 0 {
        lang(JOB_TEXT, 0xa8ab04usize, TRANSFER_TAB_COMMENT); // "  None<%s - COMMENT - no clubs interested>"
        st.value(JOB_TEXT);
        st.row += 1;
        return;
    }
    covered!("tel08_3 transfer tab interested");
    write_u32(st.list + 0xa, read_u32(st.list + 2));
    st.first = st.row;
    let mut node = exe::llist_get_next(st.list);
    while node != 0 {
        st.pad();
        let club = club_at(read_i32(node + 4) as isize);
        format(JOB_TEXT, 0x987ff0usize, club.addr() + club::SHORT_NAME); // "  %s"
        st.button(0x7d5, club.addr());
        st.row += 1;
        node = exe::llist_get_next(st.list);
    }
}

/// The Future section: the contract offer of the viewer's club being considered (a grey line), "Retired from
/// international football", then either the retirement (retired, no future at under 30, retiring from the playing
/// staff or at the end of the season) or, without a contract, "-", or the person's mood comments
/// (`transfer_tab_mood`) and "Unknown" when nothing was written.
unsafe fn transfer_tab_future(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    lang(JOB_TEXT, 0xa8aaf8usize); // "  Future"
    st.label(JOB_TEXT);
    st.first = st.row;
    let p = st.p;
    let viewer = st.viewer;
    if !viewer.is_null() {
        let (_, club) = contract_tab_lookup(p);
        if viewer.addr() == club {
            let o = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(p.addr()), read_u32(viewer.addr()));
            if o != 0 && read_u8(o + contract_offer::CONTRACT_TYPE) & 0x3f == 2 && read_u8(o + contract_offer::STATE) == 0 {
                covered!("tel08_3 transfer tab considering offer");
                let g = read_i8(viewer.addr() + club::SHORT_GENDER_NAME) as i32;
                // "  Currently considering new contract offer from {}<%s - Club Name(e.g.Falkirk)>{}"
                lang(JOB_TEXT, 0xa8aaa4usize, g, g, viewer.addr() + club::NAME);
                st.line_in(G_GUI_COLOUR_PALE_GREY);
            }
        }
    }
    let dislikes_national: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x776380usize);
    if dislikes_national(read_usize(G_NATIONAL_TEAMS), p.addr()) != 0 && !p.player().is_null() {
        st.say(0xa8aa7c); // "  Retired from international football"
    }
    let regen = read_usize(G_PLAYER_REGEN);
    if exe::player_regen_get_retirement_status(regen, p) == 2 {
        st.say(0xa8aa54); // "  Retired from professional football"
        return;
    }
    if exe::player_regen_get_retirement_status(regen, p) == 1 {
        covered!("tel08_3 transfer tab retiring");
        let s = if (p.age() as i8) < 0x1e {
            0xa8aa1c // "  Believes he has no future in professional football"
        } else if !p.player().is_null() && !p.non_player().is_null() {
            0xa8a9e0 // "  Retiring from the playing staff at the end of the season"
        } else {
            0xa8a9bc // "  Retiring at the end of the season"
        };
        st.say(s);
        return;
    }
    if st.contract.is_null() {
        st.say(0x98e8c8); // "  -"
        return;
    }
    covered!("tel08_3 transfer tab mood");
    transfer_tab_mood(st);
    let club = st.club;
    let pl = p.player();
    if !club.is_null()
        && !pl.is_null()
        && st.future == 0
        && p.ambition() > 0xf
        && p.loyalty() < 6
        && p.professionalism() < 6
        && club.reputation() < 0x1676
        && pl.current_reputation() > 0xcb2
        && pl.current_reputation() as i32 > club.reputation() as i32 + 0x2ee
    {
        // "  Sure he will soon move to a bigger club<%s - COMMENT - Boasting about moving to a bigger club>"
        lang(JOB_TEXT, 0xa86a8cusize, TRANSFER_TAB_COMMENT);
        st.line();
    }
    if st.row == st.first {
        lang(JOB_TEXT, 0xa86a68usize, TRANSFER_TAB_COMMENT); // "  Unknown<%s - COMMENT - Unknown>"
        st.line();
    }
}

/// The person's mood comments of the Future section, in the original's order (each piece below). Leaves the
/// contract's club (looked up again) and the club's league statistics entry (matches played; current and final
/// position, losing and winning run when more than 8 matches were played) in the state.
unsafe fn transfer_tab_mood(st: &mut TransferTab) {
    let (_, club) = contract_tab_lookup(st.p);
    let club = Club(club);
    st.club = club;
    let division = read_usize(club.addr() + club::DIVISION);
    if division != 0 {
        let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_i32(division) as usize * 4));
        if comp != 0 && read_usize(comp + 8) != 0 {
            let e = exe::comp_stats_find_club_entry(read_usize(comp + 8), club);
            if e != 0 {
                st.matches_played = read_i8(e + comp_stats_club::MATCHES_PLAYED);
                if read_u8(e + comp_stats_club::MATCHES_PLAYED) > 8 {
                    st.position = read_i8(e + comp_stats_club::CURRENT_POSITION);
                    st.final_position = read_i8(e + comp_stats_club::FINAL_POSITION);
                    st.losing_run = read_i8(e + comp_stats_club::LOSING_RUN);
                    st.winning_run = read_i8(e + comp_stats_club::WINNING_RUN);
                }
            }
        }
    }
    if transfer_tab_mood_interest(st) {
        transfer_tab_mood_sales(st);
    }
    transfer_tab_mood_squad(st);
    transfer_tab_mood_league(st);
    transfer_tab_mood_listing(st);
    transfer_tab_mood_pay(st);
    transfer_tab_mood_dislikes(st);
    transfer_tab_mood_manager(st);
    transfer_tab_mood_contract(st);
    transfer_tab_mood_grievances(st);
    transfer_tab_mood_selection(st);
    transfer_tab_mood_personal(st);
    transfer_tab_mood_outlook(st);
}

/// Mood, part 1: the university course (the easter egg of "Andrew Mainwaring", potential over 100, 18 to 21), the
/// 'Sporting Just Cause' dispute with the club, the interest shown in him (a player not listed, without a future
/// transfer, with interested clubs and more than 150 days at the club: the visiting scouts, pressure or flattery by
/// the contract grievances, the club's reputation, ambition, loyalty and pressure) and the favoured team-mate (no
/// loan, not listed, no future transfer, none of the grievances 0x10181). Returns false (the original's jump past the
/// sales and the first new contract check) when the contract has no club.
unsafe fn transfer_tab_mood_interest(st: &mut TransferTab) -> bool {
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let pl = p.player();
    let age = p.age() as i8;
    if exe::staff_name_equals(p, 0xa8a9a8usize) != 0 // "Andrew Mainwaring"
        && !club.is_null()
        && !pl.is_null()
        && pl.potential_ability() > 100
        && age > 0x11
        && age < 0x16
    {
        st.say(0xa8a96c); // "  Wants to finish his university course before moving clubs"
    }
    let dispute = read_usize(G_DISPUTE);
    if dispute != 0 {
        if club.is_null() {
            return false;
        }
        let in_dispute: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x569e30usize);
        let dispute_club: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x56a030usize);
        if in_dispute(dispute, p.addr(), 0) != 0 && dispute_club(read_usize(G_DISPUTE), p.addr()) == read_i32(club.addr()) {
            st.say(0xa8a920); // "  Is attempting to leave the club under the 'Sporting Just Cause' clause"
        }
    }
    if club.is_null() {
        return false;
    }
    if !pl.is_null() && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0 && !st.future_now() && read_u16(st.list) != 0 && st.days_joined() > 0x96 {
        let g = c.grievances();
        let small = g & 0x14 != 0 || club.reputation() < 0x109a;
        let s = if g & 0x101 != 0 {
            Some(0xa8a8f4) // "  Is hoping to impress the visiting scouts"
        } else if small && p.ambition() > 0xf && p.loyalty() < 6 {
            Some(0xa8a8f4)
        } else if small {
            if p.pressure() < 6 {
                Some(0xa8a8bc) // "  Feels pressured by the interest being shown in him"
            } else if club.reputation() < 0x109a && p.ambition() > 0xf && p.pressure() > 0xf {
                Some(0xa8a88c) // "  Flattered by the interest being shown in him"
            } else {
                None
            }
        } else {
            None
        };
        if let Some(s) = s {
            st.say(s);
        }
    }
    if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) == 0
        && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0
        && !st.future_now()
        && c.grievances() & 0x10181 == 0
    {
        let fav = Staff(staff_pick_favoured_team_mate(p.addr(), club.addr()));
        if !fav.is_null() && !fav.player().is_null() {
            transfer_tab_favoured_team_mate(st, fav);
        }
    }
    true
}

/// The favoured team-mate line (a button 0x7d7 to him, then the same text wrapped): a fellow national (with the
/// grievances 0x20020), an old hand (over 34 and less than 20 happiness at the club: a good influence when
/// professional and a leader, else experience), a youngster (under 21: listed, capped, rated against the club, or a
/// hot prospect), or by the listing, his contract, the positions, his role, age and reputation, leadership and
/// happiness (retain, renew, learn from, a more important role, honoured, hang onto, inspired leader, essential, a lot
/// to offer).
unsafe fn transfer_tab_favoured_team_mate(st: &mut TransferTab, fav: Staff) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let fc = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), fav, club));
    let mut buf = [0u8; 0x1f4];
    let n = transfer_tab_name(fav, &mut buf, 0x1f4, 1);
    let nation = read_usize(p.addr() + staff::NATION);
    let fage = fav.age() as i8;
    let fp = fav.player();
    if nation != 0 && nation == read_usize(fav.addr() + staff::NATION) && c.grievances() & 0x20020 != 0 {
        // "  Is finding the presence of fellow {}<%s - Nationality>{} player <%s - Admired Staff Name>{} helpful"
        lang(JOB_TEXT, 0xa8a804usize, 100u32, 100u32, 100u32, nation + nation::NATIONALITY, n);
    } else if fage > 0x22 && fc.happiness() < 0x14 {
        if fav.professionalism() > 0xf && fp.leadership() > 0xf {
            lang(JOB_TEXT, 0xa8a7a0usize, n); // "  Thinks <%s> is a good influence on youngsters at the club"
        } else {
            lang(JOB_TEXT, 0xa8a744usize, 100u32, n); // "  Thinks <%s>{s} experience is useful to the team"
        }
    } else if fage < 0x15 {
        if exe::staff_is_transfer_listed_or_requested(fav, 0usize) != 0 {
            lang(JOB_TEXT, 0xa8a6dcusize, n); // "  Thinks <%s> should be given the chance to develop as a player"
        } else if fav.int_apps() != 0
            || (!fp.is_null() && fp.current_reputation() > 0xea6 && fp.current_reputation() as i32 > club.reputation() as i32 - 0x2ee)
        {
            lang(JOB_TEXT, 0xa8a64cusize, n); // "  Thinks <%s> is a talented player"
        } else {
            lang(JOB_TEXT, 0xa8a694usize, n); // "  Thinks <%s> is a hot prospect"
        }
    } else if exe::staff_is_transfer_listed_or_requested(fav, 0usize) != 0 {
        lang(JOB_TEXT, 0xa8a5ecusize, 100u32, n); // "  Thinks the club should retain the services of <%s>{}"
    } else if fc.is_null() {
        lang(JOB_TEXT, 0xa8a578usize, 100u32, n); // "  Thinks the club should try everything possible to renew <%s>{s} contract"
    } else {
        let s = transfer_tab_favoured_opinion(st, fav, fc);
        lang(JOB_TEXT, s, n);
    }
    st.button(0x7d7, fav.addr());
    st.line();
}

/// The favoured team-mate text of a team-mate under contract at the club (see `transfer_tab_favoured_team_mate`).
unsafe fn transfer_tab_favoured_opinion(st: &TransferTab, fav: Staff, fc: Contract) -> usize {
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let fage = fav.age() as i8;
    let fp = fav.player();
    if (p.age() as i8) < 0x18 && p.pressure() > 10 && exe::staff_get_natural_position_mask(p) == exe::staff_get_natural_position_mask(fav) {
        return 0xa8a528; // "  Thinks he can learn a lot from <%s>"
    }
    if fc.unhappy_flag() > 2 && fc.happiness() < 0x32 {
        return 0xa8a4c0; // "  Thinks that <%s> should play a more important role at the club"
    }
    if (fage > 0x1c || (fage > 0x18 && fav.int_apps() > 10))
        && fp.current_reputation() > 0x1c52
        && (p.age() as i8) < 0x19
        && fc.happiness() > 0
        && fc.matches_left_out() as i8 > 0
        && c.happiness() < 0
        && p.player().current_reputation() < 0x128e
    {
        return if c.matches_left_out() != 0 {
            0xa8a470 // "  Honoured to be playing alongside <%s>"
        } else if p.playing_squad() == fav.playing_squad() {
            0xa8a420 // "  Honoured to be training alongside <%s>"
        } else {
            0xa8a3cc // "  Honoured to be at the same club as <%s>"
        };
    }
    if fage < 0x1d && fp.current_reputation() > 0x1c52 && club.reputation() < 0x1a5e {
        return 0xa8a384; // "  Hopes the club can hang onto <%s>"
    }
    let pp = p.player();
    if !pp.is_null()
        && fage > 0x1a
        && fc.happiness() > 0
        && fp.leadership() > 0xf
        && fp.current_reputation() > 0xea6
        && (fp.current_reputation() > club.reputation().wrapping_sub(0x4e2) || fp.world_reputation() > 0x1676)
        && pp.leadership() < 0xe
    {
        return 0xa8a33c; // "  Thinks <%s> is an inspired leader"
    }
    if fc.happiness() > 0x14 {
        0xa8a2ec // "  Thinks <%s> is an essential squad member"
    } else {
        0xa8a29c // "  Thinks <%s> has a lot to offer the club"
    }
}

/// Mood, part 2: the sale of a promising youngster (grievance 0x40000000, named when the opinion link holds him) and
/// of a star player (grievance 0x20000000; by the club's form, ambition and loyalty).
unsafe fn transfer_tab_mood_sales(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let c = st.contract;
    if c.grievances() & 0x40000000 != 0 {
        let link = staff_get_opinion_link_staff(p.addr(), 0x40000000);
        if link != 0 {
            let mut buf = [0u8; 0x1f4];
            let n = transfer_tab_name(link, &mut buf, 0x1f4, 5);
            if p.pressure() < 6 {
                lang(JOB_TEXT, 0xa8a240usize, n); // "  Thinks selling youngster <%s> was a very short-sighted move"
            } else {
                lang(JOB_TEXT, 0xa8a1e4usize, n); // "  Is disappointed that the club sold promising youngster <%s>"
            }
        } else if p.pressure() < 6 {
            lang(JOB_TEXT, 0xa8a19cusize); // "  Is upset by the sale of one of the club's most promising youngsters"
        } else {
            lang(JOB_TEXT, 0xa8a150usize); // "  Is disappointed that the club sold one of its most promising youngsters"
        }
        st.line();
    }
    if c.grievances() & 0x20000000 != 0 {
        let link = staff_get_opinion_link_staff(p.addr(), 0x20000000);
        let mut buf = [0u8; 0x1f4];
        let n = if link != 0 { transfer_tab_name(link, &mut buf, 0x1f4, 5) } else { 0 };
        let ambitious = p.ambition() > 0xf && p.loyalty() < 6;
        let s = if st.club_state(TRANSFER_TAB_CLUB_FORM) > -2 {
            match (link != 0, ambitious) {
                (true, true) => 0xa8a0ec,   // "  Thinks the club's lack of ambition is shown by the sale of <%s>"
                (true, false) => 0xa8a0a8,  // "  Is unhappy at the sale of <%s>"
                (false, true) => 0xa89f9c,  // "  Thinks the club's lack of ambition is shown by the sale of its star players"
                (false, false) => 0xa89f60, // "  Is unhappy at the sale of one of the club's star players"
            }
        } else {
            match (link != 0, p.loyalty() > 0xf) {
                (true, true) => 0xa8a064,   // "  Disappointed by the sale of <%s>"
                (true, false) => 0xa89fec,  // "  Believes the club will never turn things around while they sell players like <%s>"
                (false, true) => 0xa89f20,  // "  Hopes the club can hold onto its star players in the future"
                (false, false) => 0xa89ec8, // "  Believes the club will never turn things around while they sell their best players"
            }
        };
        if link != 0 {
            lang(JOB_TEXT, s, n);
        } else {
            lang(JOB_TEXT, s);
        }
        st.line();
    }
}

/// Mood, part 3: the new contract (pleased after 300 days at the club, flags and grievances clear and loyal; or very
/// happy after 180 days, no grievances, loyal and professional; both within 30 days of the start), and for a player
/// the competition for a place (flag 0x200), a rival signed (flag 0x80000000: his position, named when linked), the
/// rumoured replacement (flag 0x800), the lack of harmony (flag 4 after 30 days), else the good dressing room
/// atmosphere (no grievances, not listed, a settled club, professional), and the squad depth (flag 0x400: by the
/// club's form and transfer window, or its reputation).
unsafe fn transfer_tab_mood_squad(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let pl = p.player();
    if !club.is_null() && st.days_joined() > 0x12c && st.days_started() < 0x1e && st.flags() == 0 && c.grievances() == 0 && p.loyalty() > 0xf {
        st.say(0xa89e9c); // "  Is pleased to have signed a new contract"
    } else if c.grievances() == 0 && st.days_joined() > 0xb4 && st.days_started() < 0x1e && p.loyalty() > 10 && p.professionalism() > 10 {
        st.say(0xa89e74); // "  Is very happy with his new contract"
    }
    let mut harmony = false;
    if !pl.is_null() {
        if st.flags() & 0x200 != 0 {
            st.say(0xa89e34); // "  Feels there is too much competition for a first team place"
        }
        if st.flags() & 0x80000000 != 0 {
            let mask = exe::staff_get_natural_position_mask(p);
            let mut pos = [0u8; 0x200];
            let pb = pos.as_mut_ptr() as usize;
            let g = position_mask_get_text(mask, pb, 0) as i8 as i32;
            let link = staff_get_opinion_link_staff(p.addr(), 0x80000000);
            if link != 0 {
                let mut buf = [0u8; 0x1f4];
                let n = transfer_tab_name(link, &mut buf, 0x1f4, 5);
                // "  Is upset that his manager has signed rival {}<%s - Position>{} <%s - Rival Name>{}" / "  Is unhappy that ..."
                lang(JOB_TEXT, if p.pressure() < 6 { 0xa89ddcusize } else { 0xa89d70 }, g, g, 100u32, pb, n);
            } else {
                // "  Is upset that his manager has signed a rival {}<%s>{}" / "  Is unhappy that the manager has signed another {}<%s>{}"
                lang(JOB_TEXT, if p.pressure() < 6 { 0xa89d2cusize } else { 0xa89cd0 }, g, g, pb);
            }
            st.line();
        }
        if st.flags() & 0x800 != 0 {
            st.say(0xa89c8c); // "  Worried by rumours that the club is signing a replacement for him"
        }
        if st.days_joined() > 0x1e && st.flags() & 4 != 0 {
            st.say(if p.pressure() < 6 {
                0xa89c5c // "  Worried by the lack of harmony at the club"
            } else if p.pressure() > 0xf {
                0xa89c28 // "  Disappointed by the lack of harmony at the club"
            } else {
                0xa89bf8 // "  Dismayed by the lack of harmony at the club"
            });
            harmony = true;
        }
    }
    if club.is_null() {
        return;
    }
    if !harmony
        && !pl.is_null()
        && c.grievances() == 0
        && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0
        && st.club_state(TRANSFER_TAB_CLUB_UNREST) == 0
        && p.professionalism() > 0xe
    {
        st.say(0xa89bbc); // "  Thinks that the club has a good dressing room atmosphere"
    }
    if st.flags() & 0x400 != 0 {
        covered!("tel08_3 transfer tab squad depth");
        let nation = club.nation();
        let mut k: i8 = 0;
        let mut in_window = false;
        if !nation.is_null() {
            k = exe::key_nation_index_of_nation(nation);
        }
        if nation.is_null() || (0..0x23).contains(&k) {
            let t = G_LEAGUE_WORLD_CUP_START_DATE.wrapping_add((k as isize * 0x48) as usize);
            let day = read_i16(G_CURRENT_DATE);
            let year = read_u16(G_CURRENT_DATE + 2);
            let after = (day > read_i16(t) && year == read_u16(t + 2)) || year > read_u16(t + 2);
            let before = (day < read_i16(t + 0xf) && year == read_u16(t + 0x11)) || year < read_u16(t + 0x11);
            in_window = after && before;
        }
        let s = if st.club_state(TRANSFER_TAB_CLUB_FORM) > 2 {
            if in_window {
                0xa89b50 // "  Feels the club needs more squad depth to sustain its league form"
            } else {
                0xa89b94 // "  Feels the club needs more squad depth"
            }
        } else if club.reputation() > 0x1676 {
            0xa89b24 // "  Believes the club needs more squad depth"
        } else {
            0xa89ae4 // "  Is worried by the lack of competition for places in the team"
        };
        lang(JOB_TEXT, s);
        st.line();
    }
}

/// The league position comment of `transfer_tab_mood_league` (flag 0x10 of a player at an underachieving club).
unsafe fn transfer_tab_league_position(st: &TransferTab) -> usize {
    let p = st.p;
    let c = st.contract;
    let pl = p.player();
    let top_or_first = matches!(st.squad_bits(), 0x10 | 0x20 | 0x30);
    if p.pressure() > 10 && p.loyalty() < 6 && pl.current_reputation() > 0x1676 && st.club.reputation() < 0x186a && top_or_first {
        return 0xa89ac0; // "  Feels he is too good for the club"
    }
    if p.pressure() < 6 && p.loyalty() < 6 && pl.current_reputation() > 0x1676 {
        return 0xa89a74; // "  Worried that the club's poor league position will affect his reputation"
    }
    let near_top = st.position < 8 && st.final_position < 4;
    if near_top && p.ambition() > 10 {
        return 0xa89a44; // "  Thinks the club should be topping the table"
    }
    if st.losing_run > 2 && p.loyalty() > 0xf {
        return if near_top {
            0xa89a44
        } else {
            0xa89a0c // "  Is ashamed of the team's recent dismal league form"
        };
    }
    if p.loyalty() < 6 && (st.position > 8 || st.final_position > 3) && top_or_first {
        return 0xa899d8; // "  Disappointed by the club's poor league position"
    }
    if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 2
        && st.winning_run == 0
        && p.loyalty() > 0xf
        && p.pressure() > 10
        && c.happiness() > 10
        && exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), pl) < 0x1e
        && matches!(st.squad_bits(), 0x10 | 0x20)
    {
        return 0xa89994; // "  Admits he is partially responsible for the team's slump in form"
    }
    0xa89970 // "  Thinks the club is underachieving"
}

/// Mood, part 4: the league: underachieving (flag 0x10 of a player at a club below the expectations:
/// `transfer_tab_league_position`), more potential (no flag 0x10, more than 5 matches, the club well below), the new
/// manager (grievance 0x10000000: his reputation and days at the club), the successful club (no grievances, happy, a
/// player at a club above the expectations: joined, being there, the style of play by his ratings against the
/// competition's average, the league position, performing well) and the style of play he dislikes.
unsafe fn transfer_tab_mood_league(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let pl = p.player();
    if !pl.is_null() && !club.is_null() && st.club_state(TRANSFER_TAB_CLUB_FORM) < -1 && st.flags() & 0x10 != 0 {
        let s = transfer_tab_league_position(st);
        lang(JOB_TEXT, s);
        st.line();
    }
    if st.flags() & 0x10 == 0
        && st.matches_played > 5
        && !pl.is_null()
        && !club.is_null()
        && st.club_state(TRANSFER_TAB_CLUB_FORM) < -2
        && p.loyalty() > 10
        && p.professionalism() > 10
    {
        st.say(if p.pressure() > 0xf {
            0xa89938 // "  Feels the team has more potential than it is showing"
        } else {
            0xa898fc // "  Will strive to help the club improve it's league position"
        });
    }
    if c.grievances() & 0x10000000 != 0 && !club.is_null() && !club.manager().is_null() {
        let m = club.manager();
        let s = if st.days_since(m.addr() + staff::DATE_JOINED_CLUB) < 0x5a {
            let np = m.non_player();
            if !np.is_null()
                && st.club_state(TRANSFER_TAB_CLUB_FORM) < 0
                && np.current_reputation() < club.reputation().wrapping_sub(0x2ee)
                && p.loyalty() < 10
            {
                0xa898bc // "  Not convinced the club's new manager can turn the club around"
            } else if np.current_reputation() < club.reputation().wrapping_sub(0x4e2) {
                0xa89880 // "  Worried by his manager's lack of experience at this level"
            } else {
                0xa89844 // "  Waiting to see how the club's new manager changes things"
            }
        } else {
            0xa89814 // "  Has lost confidence in his manager's ability"
        };
        st.say(s);
    }
    let stats = read_usize(G_PLAYER_STATS);
    let average = || {
        let d = read_f64(stats + 0x18);
        if !(d < 0.0 || d > 0.0) { read_f64(rdata::F64_6_8) } else { read_f64(stats + 0x10) / d }
    };
    if c.grievances() == 0 && c.happiness() > 0x19 && !pl.is_null() {
        if club.is_null() {
            return;
        }
        if st.club_state(TRANSFER_TAB_CLUB_FORM) > 2 && p.loyalty() > 10 {
            covered!("tel08_3 transfer tab successful club");
            let rating = exe::playerstats_get_stat(stats, pl, 6, 1, 6.5) as f32;
            let other = exe::playerstats_get_stat(stats, pl, 6, 0x11, 6.5) as f32;
            let s = if st.days_joined() < 0x1e {
                0xa897e4 // "  Is pleased to have joined a successful club"
            } else if st.days_joined() < 0x5a {
                0xa897bc // "  Is pleased to be at a successful club"
            } else if rating > read_f32(rdata::F32_5) && (c.matches_left_out() as i8) <= 1 {
                0xa89734 // "  Is happy to see the club performing well in the league"
            } else if rating > read_f32(rdata::F32_5) && !(average() + read_f64(rdata::F64_0_25) >= other as f64) {
                0xa89798 // "  Enjoys the team's style of play"
            } else if (c.matches_left_out() as i8) > 1 && p.loyalty() > 0xf {
                0xa89770 // "  Is proud of the clubs league position"
            } else {
                0xa89734
            };
            st.say(s);
            return;
        }
    }
    if !club.is_null()
        && c.happiness() > 0x19
        && p.loyalty() < 10
        && p.pressure() < 10
        && p.ambition() > 0xf
        && p.professionalism() < 6
        && !pl.is_null()
        && pl.attacker() == 0x14
        && pl.current_reputation() > 0x128e
        && pl.current_reputation() as i32 > club.reputation() as i32 - 0x2ee
    {
        let rating = exe::playerstats_get_stat(stats, pl, 6, 1, 6.5) as f32;
        let other = exe::playerstats_get_stat(stats, pl, 6, 0x11, 6.5) as f32;
        if rating > read_f32(rdata::F32_5) && (c.matches_left_out() as i8) > 1 && average() - read_f64(rdata::F64_0_35) > other as f64 {
            st.say(0xa8970c); // "  Doesn't like the team's style of play"
        }
    }
}

/// Mood, part 5: the club's valuation of him (flag 8, no future transfer: outraged, disappointed, upset, unrealistic
/// or pressured by the grievances, temperament, professionalism, pressure and age), the club's finances (grievance
/// 0x400000), the style of play (flag 0x1000), the transfer listing (listed by the club: the squad status he deserves,
/// days at the club and character; listed by request: the grievance 1 and character; or the forthcoming transfer),
/// the agent (within 210 days of the expiry), the stepping stone (a young ambitious player at a small club), absent
/// without permission and the lost place in the team (more matches left out than loyalty / 5 + 1).
unsafe fn transfer_tab_mood_listing(st: &mut TransferTab) {
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let age = p.age() as i8;
    if st.flags() & 8 != 0 && !st.future_now() {
        let s = if c.grievances() & 0x10181 != 0 {
            if p.temperament() < 5 && p.professionalism() < 0xf {
                Some(0xa896d4) // "  Is outraged by the clubs unrealistic valuation of him"
            } else if p.professionalism() > 0xf {
                Some(0xa89698) // "  Is disappointed by the clubs unrealistic valuation of him"
            } else if p.pressure() < 5 {
                Some(0xa89660) // "  Is upset by the clubs unrealistic valuation of him"
            } else {
                Some(0xa895f4) // "  Thinks the clubs valuation of him is unrealistic"
            }
        } else if age > 0x12 {
            Some(if p.pressure() < 6 { 0xa89628 } else { 0xa895f4 }) // "  Feels pressured by the clubs high valuation of him"
        } else if p.pressure() < 6 {
            Some(0xa89628)
        } else {
            None
        };
        if let Some(s) = s {
            st.say(s);
        }
    }
    if c.grievances() & 0x400000 != 0 {
        let pl = p.player();
        let s = if p.ambition() > 0xf
            && age > 0x15
            && matches!(st.squad_bits(), 0x10 | 0x20)
            && pl.current_ability() as i32 > p.loyalty() as i32 + 100
            && pl.current_reputation() > 0xcb2
        {
            0xa895ac // "  Doesn't think a club in this financial state can fulfil his ambitions"
        } else if c.morale() < -75 && p.pressure() < 10 {
            0xa89584 // "  Unsettled by the financial upheaval"
        } else if p.pressure() < 6 {
            0xa89558 // "  Worried by the club's financial plight"
        } else if exe::staff_contract_wants_to_leave(c) != 0 {
            0xa89514 // "  Wants to join a club that has a more secure financial position"
        } else {
            0xa894dc // "  Concerned about the financial stability of the club"
        };
        st.say(s);
    }
    if st.flags() & 0x1000 != 0 {
        st.say(if st.future_now() {
            0xa8949c // "  Hopes he is joining a club which respects his style of play"
        } else if c.leaving_on_bosman() == 0 && exe::staff_is_listed_by_request(p, 0usize) == 0 {
            0xa89464 // "  Thinks the manager should accept his style of play"
        } else {
            0xa8942c // "  Wants to join a club which respects his style of play"
        });
    }
    transfer_tab_listed(st);
    if !club.is_null()
        && !p.player().is_null()
        && c.leaving_on_bosman() == 0
        && st.days_to_expiry() < 0xd2
        && p.loyalty() < 6
        && p.professionalism() < 10
        && age < 0x20
        && p.player().current_reputation() > club.reputation()
        && p.player().current_reputation() > 0x109a
    {
        st.say(0xa891a8); // "  Has been told by his agent that it is time to move on"
    }
    let pl = p.player();
    if club.reputation() < 0x128e
        && !pl.is_null()
        && age < 0x1a
        && pl.potential_ability() as i32 + p.ambition() as i32 * 2 > 0x78
        && p.loyalty() < 6
        && p.professionalism() < 10
        && p.ambition() > 10
        && age < 0x1c
    {
        st.say(if st.squad_bits() == 0x70 {
            0xa8913c // "  Thinks his manager is misjudging his ability"
        } else {
            0xa8916c // "  Thinks this club is a stepping stone towards his future"
        });
    }
    if exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0 {
        st.say(0xa89120); // "  Absent without permission"
    }
    if !pl.is_null()
        && exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) == 0
        && c.unhappy_flag() as i32 > p.loyalty() as i32 / 5 + 1
        && matches!(st.squad_bits(), 0x10 | 0x20)
        && age > 0x14
        && age < 0x22
        && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 2
    {
        let s = if p.professionalism() > 10 && pl.work_rate() > 10 {
            if p.loyalty() > 10 && age > 0x1c && !pl.is_null() && pl.current_reputation() > 0x1482 && club.reputation() > 0x1a5e {
                Some(0xa890f0) // "  Wants to be more involved in the first team"
            } else if exe::staff_contract_wants_to_leave(c) != 0 {
                Some(0xa890b0) // "  Eager to move in order to find regular first team football"
            } else {
                Some(0xa89080) // "  Is willing to fight for his place in the team"
            }
        } else if p.professionalism() < 6 && p.temperament() < 10 {
            Some(0xa89058) // "  Upset at losing his place in the team"
        } else {
            None
        };
        if let Some(s) = s {
            st.say(s);
        }
    }
}

/// The transfer listing comments of `transfer_tab_mood_listing`.
unsafe fn transfer_tab_listed(st: &mut TransferTab) {
    let p = st.p;
    let c = st.contract;
    if exe::staff_is_transfer_listed(p, 0usize) != 0
        && c.leaving_on_bosman() == 0
        && !st.future_now()
        && c.grievances() == 0
        && exe::staff_contract_wants_to_leave(c) == 0
    {
        covered!("tel08_3 transfer tab listed by club");
        let bits = st.squad_bits();
        let out_of_plans = bits == 0 || bits == 0x80 || st.deserved() == 6 || st.deserved() == 5;
        let s = if out_of_plans {
            if p.ambition() > 0xf {
                0xa89328 // "  Believes he can force his way back into the manager's plans"
            } else if p.pressure() < 10 && p.loyalty() > 0xf {
                0xa89408 // "  Doesn't want to leave the club"
            } else {
                0xa89300 // "  Disappointed at being transfer listed"
            }
        } else {
            if st.days_joined() > 0x28a && p.loyalty() > 0xf && p.professionalism() > 10 {
                st.say(0xa89408);
            }
            if p.loyalty() > 10 && p.temperament() > 10 && p.professionalism() > 10 {
                0xa893e4 // "  Shocked at being transfer listed"
            } else if p.temperament() < 6 {
                0xa893c0 // "  Angry at being transfer listed"
            } else if p.ambition() > 10 && st.deserved() == 1 {
                0xa89390 // "  Thinks he is too good to be transfer listed"
            } else if p.pressure() < 6 && st.deserved() == 1 {
                0xa89368 // "  Surprised at being transfer listed"
            } else {
                0xa89300
            }
        };
        st.say(s);
        return;
    }
    if (exe::staff_is_transfer_listed(p, 0usize) != 0 || exe::staff_is_listed_by_request(p, 0usize) != 0)
        && p.loyalty() < 6
        && p.professionalism() < 10
        && c.grievances() & 1 != 0
    {
        let s = if p.loyalty() == 1 && p.ambition() > 0xf && p.temperament() < 10 {
            0xa892d0 // "  Looking forward to moving to a better club"
        } else if p.ambition() > 0xf && p.temperament() < 6 {
            0xa892ac // "  Looking forward to moving clubs"
        } else {
            0xa89288 // "  Pleased at being transfer listed"
        };
        st.say(s);
        return;
    }
    if exe::staff_is_listed_by_request(p, 0usize) != 0 && c.grievances() == 0 && exe::staff_contract_wants_to_leave(c) == 0 {
        st.say(if st.future_now() {
            if p.pressure() < 10 {
                0xa8925c // "  Nervous about his forthcoming transfer"
            } else if p.loyalty() > 10 {
                0xa89238 // "  Feels he is leaving a great club"
            } else {
                0xa89208 // "  Feels it is a shame he has to leave the club"
            }
        } else {
            0xa891e0 // "  Would now like to stay at the club"
        });
    }
}

/// Mood, part 6: the pay (grievance 0x1000000 at a club that is not a national team: his wage against 0.9 × the
/// club's highest wage, his world reputation and character), homesick or ending his career at home (grievance
/// 0x20000, over 32), wanting to leave or to be listed (grievance 0x10000, no future transfer), no support from the
/// manager (0x80000) and the role in the team (0x4000: ability and ambition against professionalism, reputation
/// against the club).
unsafe fn transfer_tab_mood_pay(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    if !club.is_null() && exe::database_is_national_team(club) == 0 && c.grievances() & 0x1000000 != 0 {
        let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        let s = if finance != 0 && (read_i32(finance + finance::HIGHEST_WAGE) as f64 * read_f64(rdata::F64_0_9)) < c.wage() as f64 {
            0xa89034 // "  Feels he deserves to be paid more"
        } else if p.loyalty() < 10 && p.professionalism() < 5 && !p.player().is_null() && p.player().world_reputation() > 0x128e {
            0xa88fe0 // "  Feels that his pay must be on a par with the highest at the club if he is to stay"
        } else if p.ambition() > 0xf && p.loyalty() < 6 {
            0xa88fa4 // "  Demands that his pay matches the top players at the club"
        } else {
            0xa88f68 // "  Feels his pay should match the top players at the club"
        };
        st.say(s);
    }
    if c.grievances() & 0x20000 != 0 {
        if (p.age() as i8) > 0x20 {
            let n = read_usize(p.addr() + staff::NATION);
            let g = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
            lang(JOB_TEXT, 0xa88f28usize, g, g, n + nation::SHORT_NAME); // "  Wants to end career in {}<%s - Home Nation>{}"
        } else {
            lang(JOB_TEXT, 0xa88f18usize); // "  Is homesick "
        }
        st.line();
    }
    if c.grievances() & 0x10000 != 0 && !st.future_now() {
        st.say(if p.professionalism() < 10 && p.temperament() < 5 {
            0xa88ef0 // "  Wants to leave the club immediately"
        } else {
            0xa88ed0 // "  Wants to be transfer listed "
        });
    }
    if c.grievances() & 0x80000 != 0 {
        st.say(0xa88ea4); // "  Feels he has no support from his manager"
    }
    if c.grievances() & 0x4000 != 0 {
        let pl = p.player();
        st.say(
            if p.temperament() < 6
                && p.ambition() > 10
                && !pl.is_null()
                && pl.current_ability() as i32 + p.ambition() as i32 > p.professionalism() as i32 + 0x5a
                && pl.current_reputation() > club.reputation()
            {
                0xa88e78 // "  Expects a more important role in the team"
            } else {
                0xa88e40 // "  Feels he deserves a more important role in the team"
            },
        );
    }
}

/// The disliked staff of the person (`staff_get_disliked_staff`, at most 3) at his club, each a line with a button
/// 0x7db to him, then, when the disliked one is transfer listed, the glad / taking action / harmony line; "Unsettled"
/// when he dislikes nobody. The first list (grievance 0x800) takes only players (job 11) and says disruptive,
/// selfish, arrogant or dislikes; the second (flag 1 without 0x800) says immature, hopes he will learn, or irritated.
unsafe fn transfer_tab_disliked(st: &mut TransferTab, second: bool) {
    let lang = staff_tab_lang();
    let p = st.p;
    let mut list = [0usize; 50];
    let mut count: i8 = 0;
    exe::staff_get_disliked_staff(p, list.as_mut_ptr() as usize, &mut count as *mut i8 as usize);
    let mut k: i8 = 0;
    while k < count {
        let d = Staff(list[k as usize]);
        if (second || d.job_for_club() == 0xb) && read_usize(d.addr() + staff::CLUB_JOB) == read_usize(p.addr() + staff::CLUB_JOB) {
            covered!("tel08_3 transfer tab disliked");
            let mut buf = [0u8; 0x1f4];
            let n = transfer_tab_name(d, &mut buf, 0x1f4, 1);
            let dp = d.player();
            let s = if dp.is_null() {
                if second { 0xa88a90 } else { 0xa88cc4 } // "  Irritated by <%s>" / "  Dislikes <%s>"
            } else if second {
                if p.professionalism() > 0xf && d.professionalism() < 6 && (d.age() as i8) < 0x18 {
                    0xa88b24 // "  Thinks <%s> is immature"
                } else {
                    0xa88ad0 // "  Hopes <%s> will learn from his mistakes"
                }
            } else if p.professionalism() > 0xf && d.professionalism() < 6 {
                0xa88de0 // "  Thinks <%s> is a disruptive influence on the team"
            } else {
                let arrogant = dp.current_ability() as i32 > 100 - p.ambition() as i32 && d.ambition() > 0xf && d.professionalism() < 6;
                if dp.goalkeeper() != 0x14 && dp.teamwork() < 6 {
                    if arrogant { 0xa88d90 } else { 0xa88d44 } // "  Feels <%s> is selfish and arrogant" / "... a selfish player"
                } else if arrogant {
                    0xa88d00 // "  Feels <%s> is arrogant"
                } else {
                    0xa88cc4
                }
            };
            lang(JOB_TEXT, s, n);
            st.button(0x7db, d.addr());
            st.pad();
            st.row += 1;
            if exe::staff_is_transfer_listed(d, 0usize) != 0 {
                if p.temperament() < 6 && p.professionalism() < 10 {
                    lang(JOB_TEXT, 0xa88c74usize, n); // "  Will be glad to see the back of <%s>"
                } else if read_i32(p.addr()) % 3 != 0 {
                    lang(JOB_TEXT, 0xa88c10usize, n); // "  Is glad to see the manager taking action against <%s>"
                } else {
                    lang(JOB_TEXT, 0xa88b9cusize, 100u32, n); // "  Thinks that <%s>{} leaving the club will restore dressing-room harmony"
                }
                st.button(0x7db, d.addr());
                st.pad();
                st.row += 1;
            }
        }
        k += 1;
    }
    if count == 0 {
        lang(JOB_TEXT, 0xa88b90usize); // "  Unsettled"
        st.value(JOB_TEXT);
        st.pad();
        st.row += 1;
    }
}

/// Mood, part 7: the disliked staff (grievance 0x800, then flag 1 without 0x800, each after more than id % 7 + 4 days
/// at the club: `transfer_tab_disliked`), the assistant manager (grievance 0x1000), the referees of the club's nation
/// (flag 0x20, another region than his nation's) and the pride in the club he favours (no grievances or flags, loyal).
unsafe fn transfer_tab_mood_dislikes(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    if c.grievances() & 0x800 != 0 && st.days_joined() > read_i32(p.addr()) % 7 + 4 {
        transfer_tab_disliked(st, false);
    }
    if c.grievances() & 0x1000 != 0 && !club.is_null() && club.assistant_manager() != 0 {
        st.say(0xa88b68); // "  Dislikes the club's Assistant Manager"
    }
    if st.flags() & 1 != 0 && c.grievances() & 0x800 == 0 && st.days_joined() > read_i32(p.addr()) % 7 + 4 {
        transfer_tab_disliked(st, true);
    }
    if !club.is_null() {
        let n = read_usize(club.addr() + club::NATION);
        let home = read_usize(p.addr() + staff::NATION);
        if n != 0 && n != home && read_u8(home + nation::ACTUAL_REGION) != read_u8(n + nation::ACTUAL_REGION) && st.flags() & 0x20 != 0 {
            let g = read_i8(n + nation::GENDER_NAME) as i32;
            // "  Feels that referees in {}<%s - Nation Name (eg. England)>{} are victimising him"
            lang(JOB_TEXT, 0xa88a3cusize, g, g, n + 4);
            st.line();
        }
    }
    if c.grievances() == 0 && st.flags() == 0 && p.loyalty() > 0xf && exe::staff_favours_club(p, club) != 0 {
        let g = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32;
        if c.matches_left_out() != 0 {
            lang(JOB_TEXT, 0xa889f8usize, g, g, club.addr() + club::SHORT_NAME); // "  Proud to be playing at {}<%s>{}"
        } else {
            lang(JOB_TEXT, 0xa889bcusize, g, g, club.addr() + club::SHORT_NAME); // "  Proud to be at {}<%s>{}"
        }
        st.button(0x7d5, club.addr());
        st.pad();
        st.row += 1;
    }
}

/// Mood, part 8: the manager: his praise (a loyal, professional player under no grievances or flags, not a youth or
/// part-time job, at a club whose board trusts the manager, in good form and happy: a tactical genius, proud to play
/// for him, hopes he stays, extremely competent), his dislike (grievance 0x2000: tactics or team selection by the
/// manager's ratings when the board's confidence is low), the squad status (above what he deserves: indispensable,
/// the rotation, or the value and contribution he wants acknowledged under a 10000 wage), the rumours the manager
/// may leave (the manager favours him, board confidence under 4000) and the board dismissing him (board confidence
/// under 4250: sure he will turn it around, foolish or wrong to dismiss).
unsafe fn transfer_tab_mood_manager(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let mm = read_usize(G_MANAGER_MANAGER);
    let m = if club.is_null() { Staff(0) } else { club.manager() };
    if c.grievances() == 0
        && st.flags() == 0
        && !matches!(p.job_for_club(), 0xd | 0xf | 5)
        && p.loyalty() > 0xf
        && p.professionalism() > 0xf
        && !m.is_null()
        && !m.non_player().is_null()
        && mm != 0
        && !club.is_null()
        && transfer_tab_board_confidence(club) > 0x186a
        && st.club_state(TRANSFER_TAB_CLUB_FORM) > 0
        && c.happiness() > 0
    {
        covered!("tel08_3 transfer tab manager praise");
        let mut buf = [0u8; 0x190];
        let n = transfer_tab_name(m, &mut buf, 0x190, 5);
        let s = if (p.age() as i8) < 0x14 && st.winning_run != 0 && m.non_player().current_reputation() > 0x1c52 {
            0xa8897c // "  Thinks <%s> is a tactical genius"
        } else if exe::staff_is_favoured_by(m, p) != 0 {
            if c.matches_left_out() != 0 && p.ambition() > 0xf {
                0xa88940 // "  Proud to be playing for <%s>"
            } else {
                0xa888e8 // "  Hopes that <%s> will remain manager for a very long time"
            }
        } else {
            0xa888a4 // "  Thinks <%s> is extremely competent"
        };
        lang(JOB_TEXT, s, n);
        st.line();
    }
    if c.grievances() & 0x2000 != 0 {
        let s = if !club.is_null() && !m.is_null() && !m.non_player().is_null() && p.temperament() < 5 && mm != 0 && transfer_tab_board_confidence(club) < 0x109a {
            let np = m.non_player();
            if exe::non_player_attribute_rating(np, np.tactics()) < 10 {
                0xa88874 // "  Thinks the manager is tactically incompetant"
            } else if exe::non_player_attribute_rating(np, np.judgement()) < 10 {
                0xa88844 // "  Thinks the manager's team selection is poor"
            } else {
                0xa88824 // "  Dislikes the club's Manager"
            }
        } else {
            0xa88824
        };
        st.say(s);
    }
    if c.grievances() & 1 == 0 && exe::staff_is_transfer_listed(p, 0usize) == 0 {
        let deserved = st.deserved() as i8;
        let sq = st.squad() as i8;
        if sq > deserved
            && sq != 0
            && sq != 8
            && (sq != 3 || (p.ambition() > 0xc && p.professionalism() < 0xf && p.temperament() < 0xf))
            && st.deserved() != 6
            && st.deserved() != 5
        {
            let pl = p.player();
            if st.deserved() == 1 && !pl.is_null() && pl.current_reputation() > 0xcb2 && p.professionalism() < 10 && p.temperament() < 6 {
                st.say(0xa887f8); // "  Believes he is indispensable to the club"
            } else if st.squad() == 3 && c.unhappy_flag() > 1 {
                st.say(0xa887c8); // "  Is not happy with the squad rotation system"
            } else {
                transfer_tab_undervalued(st);
            }
        }
    }
    if !m.is_null()
        && exe::staff_is_favoured_by(m, p) != 0
        && c.grievances() == 0
        && p.loyalty() > 10
        && mm != 0
        && transfer_tab_board_confidence(club) < 0xfa0
    {
        let mut buf = [0u8; 0x190];
        let n = transfer_tab_name(m, &mut buf, 0x190, 5);
        lang(JOB_TEXT, 0xa88774usize, n); // "  Is unhappy at rumours that <%s> may leave the club"
        st.line();
    }
    let pl = p.player();
    if st.flags() == 0
        && c.grievances() == 0
        && !m.is_null()
        && mm != 0
        && !pl.is_null()
        && transfer_tab_board_confidence(club) < 0x109a
        && p.loyalty() > 0xf
    {
        let mut buf = [0u8; 0x190];
        let n = transfer_tab_name(m, &mut buf, 0x190, 5);
        let s = if st.club_state(TRANSFER_TAB_CLUB_FORM) < 0 {
            0xa885d4 // "  Is sure his manager will turn the club around"
        } else if pl.current_reputation() as i32 > club.reputation() as i32 + 0x4e2 && p.ambition() > 10 && p.professionalism() < 10 {
            0xa88720 // "  Thinks the board would be foolish to dismiss <%s>"
        } else {
            0xa88604 // "  Thinks the board would be wrong to dismiss <%s>"
        };
        if s == 0xa885d4 {
            lang(JOB_TEXT, s);
        } else {
            lang(JOB_TEXT, s, n);
        }
        st.line();
    }
}

/// The undervalued player of `transfer_tab_mood_manager` (a squad status above the one he deserves, earning under
/// 10000): re-assess his value, acknowledge his contribution, or more important than the manager indicates.
unsafe fn transfer_tab_undervalued(st: &mut TransferTab) {
    let p = st.p;
    let c = st.contract;
    let pl = p.player();
    let not_first = |st: &TransferTab| st.squad() != 3 && st.squad() != 2;
    if p.professionalism() > 0xe && !pl.is_null() && pl.current_reputation() > 0x128e && not_first(st) && c.wage() < 10000 {
        st.say(0xa886e4); // "  Hopes the manager will re-assess his value to the team"
    } else if !pl.is_null() && p.professionalism() < 6 && pl.current_reputation() > 0xea6 && c.wage() < 10000 {
        st.say(0xa8869c); // "  Believes the manager should acknowledge his contribution to the team"
    } else if p.pressure() < 10 && !pl.is_null() && pl.current_reputation() > 0x109a && not_first(st) && c.wage() < 10000 {
        st.say(0xa88654); // "  Believes he is more important to the team than the manager indicates"
    }
}

/// Mood, part 9: the contract's status bits. Bit 2 alone set with others (no future transfer): turning professional (a
/// part-time ambitious youngster with potential), then always one of loyalty rewarded, a manager who appreciates him,
/// the agent looking for a club or thinking of moving; bit 1: a bigger club (the future transfer's club against his
/// own) or furthering his career; bit 2: the agent, being paid what he deserves, the forthcoming move, or the contract
/// (disliking the club, extending, negotiating or a new contract).
unsafe fn transfer_tab_mood_contract(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let g = c.grievances();
    if g & 2 != 0 && g != 2 && !st.future_now() {
        covered!("tel08_3 transfer tab contract status");
        let pl = p.player();
        if c.contract_type() & 0x80 != 0
            && c.grievances() & 1 == 0
            && p.ambition() > 0xf
            && p.loyalty() < 0xf
            && !pl.is_null()
            && pl.potential_ability() as i32 > 0x50 - p.ambition() as i32
            && (p.age() as i8) < 0x19
        {
            st.say(if p.loyalty() >= 10 {
                0xa88554 // "  Would like the opportunity to turn professional"
            } else if p.ambition() > 0xe && p.professionalism() < 10 {
                0xa885a8 // "  Feels he should be a professional player"
            } else {
                0xa88588 // "  Wants to turn professional"
            });
        }
        let agent = || {
            if p.temperament() < 5 && !p.player().is_null() && p.player().current_reputation() > 0x1194 {
                0xa88488 // "  Has instructed his agent to look for a more suitable club to join"
            } else {
                0xa8846c // "  Thinking of moving clubs"
            }
        };
        let s = if !club.is_null()
            && !pl.is_null()
            && p.loyalty() > 10
            && pl.current_reputation() > 0x109a
            && pl.current_reputation() > club.reputation().wrapping_sub(0x113)
        {
            if p.professionalism() > 5 && st.days_joined() > 0x190 {
                0xa88520 // "  Thinks his loyalty to the club should be rewarded"
            } else if !club.manager().is_null() {
                if p.temperament() >= 5 {
                    0xa8846c
                } else if read_usize(G_MANAGER_MANAGER) != 0 && transfer_tab_board_confidence(club) < 0xfa0 {
                    0xa884cc // "  Hopes that a manager who appreciates his contribution takes over the club soon"
                } else {
                    agent()
                }
            } else {
                agent()
            }
        } else if p.temperament() >= 6 || p.professionalism() >= 10 || p.loyalty() >= 10 {
            0xa8846c
        } else if !p.player().is_null() && p.player().current_reputation() > 0x1194 {
            0xa88488
        } else {
            0xa8846c
        };
        st.say(s);
        return;
    }
    if c.grievances() & 1 != 0 {
        let pl = p.player();
        let s = if !pl.is_null() && pl.current_reputation() > 0x1194 && p.professionalism() < 10 && p.temperament() < 5 {
            if st.future != 0 {
                0xa8843c // "  Relieved to be moving to a bigger club soon"
            } else {
                0xa88404 // "  Wants to move to a bigger club as soon as possible"
            }
        } else if st.future != 0 {
            let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), read_i32(st.future + future_transfer::OFFER_SLOT));
            if slot != 0
                && read_usize(slot) != 0
                && club.reputation() as i32 + 0x2ee < club_at(read_i32(read_usize(slot) + offer::BUYER_CLUB_ID) as isize).reputation() as i32
            {
                0xa883e0 // "  Looking forward to his big move"
            } else {
                0xa892ac // "  Looking forward to moving clubs"
            }
        } else if (p.age() as i8) < 0x1c && p.professionalism() > 0xf {
            0xa883a8 // "  Feels that he needs to leave to further his career"
        } else {
            0xa88384 // "  Wants to move to a bigger club"
        };
        st.say(s);
    }
    if c.grievances() & 2 != 0 {
        let pl = p.player();
        let s = if p.loyalty() < 10 && p.professionalism() < 5 && !pl.is_null() && pl.world_reputation() > 0x128e {
            0xa88348 // "  His agent believes he can get a better contract elsewhere"
        } else if p.temperament() < 5 && pl.current_reputation() > 0x128e {
            0xa88324 // "  Wants to be paid what he deserves"
        } else if st.future_now() {
            if p.loyalty() < 6 && p.professionalism() < 10 {
                0xa88308 // "  Glad to be moving clubs"
            } else if p.loyalty() > 0xf {
                0xa882e8 // "  Sad to be leaving the club"
            } else {
                0xa882d0 // "  Looking to the future"
            }
        } else {
            let dislikes = exe::staff_dislikes_club(p, club) != 0;
            let age = p.age() as i8;
            if dislikes && age < 0x20 {
                if p.loyalty() < 10 && p.professionalism() < 10 {
                    0xa88348
                } else if c.grievances() == 2 {
                    0xa88298 // "  Considering moving clubs at the end of his contract"
                } else {
                    0xa8827c // "  Considering his options"
                }
            } else if age > 0x20 && st.days_to_expiry() < 0x5a {
                0xa88258 // "  Would like to extend his contract"
            } else if c.morale() > 0x19 && c.grievances() == 2 && st.days_to_expiry() < p.loyalty() as i32 * 10 + 0x7d {
                0xa88258
            } else if p.loyalty() > 10 && p.professionalism() > 10 {
                0xa8822c // "  Would like to negotiate a new contract"
            } else {
                0xa88214 // "  Wants a new contract"
            }
        };
        lang(JOB_TEXT, s);
        st.line();
    }
}

/// Mood, part 10: the grievances about his position (0x2000000, not listed: by days at the club and temperament, then
/// the manager tactically inept), his preferred position (flag 0x40 without 0x2000000), trust (0x8000000), the
/// strongest positions (flag 0x100), not listening (flag 0x80), the fringe or being substituted (0x4000000), unfair
/// treatment (0x200), a teammate's treatment (0x400), being obstructed from leaving (0x800000), the first team regular
/// (4), the media (0x40000: the story kind, the supporters' and the media's view by days at the club), the new
/// challenge (0x100) and the manager (8).
unsafe fn transfer_tab_mood_grievances(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let age = p.age() as i8;
    if c.grievances() & 0x2000000 != 0 && exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0 {
        let mut pos = [0u8; 0x200];
        let pb = pos.as_mut_ptr() as usize;
        position_mask_get_text(exe::staff_get_natural_position_mask(p), pb, 0);
        let s = if st.days_joined() < 0x3c {
            if p.temperament() < 6 {
                0xa881a8 // "  Wouldn't have joined the club if he'd known he wouldn't play as a <%s>"
            } else {
                0xa88110 // "  Wants to play as a <%s>"
            }
        } else if p.temperament() < 6 && p.professionalism() < 6 {
            0xa88150 // "  Wants to move to a club where he can play as a <%s>"
        } else {
            0xa88110
        };
        lang(JOB_TEXT, s, pb);
        st.line();
        if p.professionalism() < 6
            && p.pressure() > 10
            && !p.player().is_null()
            && age > 0x14
            && p.player().current_reputation() > 0x1676
            && c.grievances() & 0x2000 == 0
        {
            st.say(0xa880e4); // "  Thinks the manager is tactically inept"
        }
    }
    if exe::staff_is_transfer_listed_or_requested(p, 0usize) == 0 && st.flags() & 0x40 != 0 && c.grievances() & 0x2000000 == 0 {
        let mut pos = [0u8; 0x200];
        let pb = pos.as_mut_ptr() as usize;
        position_mask_get_text(exe::staff_get_natural_position_mask(p), pb, 0);
        // "  Feels his best position is playing as a <%s>" / "  Would like to play as a <%s>"
        lang(JOB_TEXT, if p.professionalism() > 5 { 0xa88094usize } else { 0xa88050 }, pb);
        st.line();
    }
    if c.grievances() & 0x8000000 != 0 {
        st.say(0xa88024); // "  Feels he can no longer trust his manager"
    }
    if st.flags() & 0x100 != 0 {
        st.say(if p.temperament() < 6 {
            0xa87fd8 // "  Angry that the manager is not putting players in their strongest position"
        } else if p.professionalism() > 10 {
            0xa87f84 // "  Thinks the team would perform better if players were in their strongest positions"
        } else {
            0xa87f38 // "  Upset that the manager doesn't field players in their strongest position"
        });
    }
    if st.flags() & 0x80 != 0 {
        st.say(0xa87f04); // "  Feels the manager doesn't listen to his players"
    }
    if c.grievances() & 0x4000000 != 0 {
        st.say(
            if p.ambition() > 0xf && age > 0x17 && c.happiness() < 0 && !matches!(st.squad(), 0 | 8 | 7 | 1 | 2) {
                0xa87ed4 // "  Wants to be more than just a fringe player"
            } else {
                0xa87eb4 // "  Unhappy at being substituted"
            },
        );
    }
    if c.grievances() & 0x200 != 0 {
        let pl = p.player();
        st.say(if st.days_joined() < 0xe {
            0xa87e78 // "  Feels he was brought to the club under false pretences"
        } else if st.days_joined() < 0x3c {
            0xa87e40 // "  Feels he made the wrong decision in joining the club"
        } else if p.pressure() < 6 && p.professionalism() < 0xf {
            0xa87e10 // "  Feels he has been victimised by his manager"
        } else if p.loyalty() > 5
            && p.professionalism() < 0xb
            && p.temperament() < 0xb
            && age > 0x18
            && !pl.is_null()
            && pl.leadership() > 8
            && pl.current_reputation() > 0x1194
        {
            0xa87de0 // "  Feels he should be treated with more respect"
        } else {
            0xa87db8 // "  Feels he has been treated unfairly"
        });
    }
    if c.grievances() & 0x400 != 0 {
        st.say(if p.temperament() > 10 && p.pressure() > 10 {
            0xa87d78 // "  Believes the manager should be more patient with his staff"
        } else if p.pressure() < 5 && p.professionalism() < 10 {
            0xa87d2c // "  Believes the manager's treatment of players is affecting the team morale"
        } else {
            0xa87d08 // "  Unhappy at treatment of teammate"
        });
    }
    if c.grievances() & 0x800000 != 0 && !st.future_now() {
        st.say(if exe::staff_is_transfer_listed_or_requested(p, 0usize) != 0 {
            0xa87cb8 // "  Feels he cannot trust the manager as he is obstructing his leaving the club"
        } else {
            0xa87c70 // "  Feels the manager is obstructing any chance of his leaving the club"
        });
    }
    if c.grievances() & 4 != 0 {
        let pl = p.player();
        let s = if st.future_now()
            && !pl.is_null()
            && p.pressure() < 10
            && pl.current_reputation() > 0xea6
            && pl.current_reputation() > club.reputation().wrapping_sub(0x2ee)
        {
            0xa87c1c // "  Thinks the manager is leaving him out of the team because he's leaving the club"
        } else if !club.is_null()
            && !pl.is_null()
            && pl.current_reputation() > 0xea6
            && pl.current_reputation() > club.reputation()
            && p.professionalism() < 0xf
        {
            if p.temperament() < 6 {
                0xa87bd0 // "  Is angry that his manager doesn't consider him to be a first team regular"
            } else if p.ambition() > 10 {
                if read_i32(p.addr()) % 3 == 0 {
                    0xa87b80 // "  Is shocked that his manager doesn't consider him to be a first team regular"
                } else {
                    0xa87b38 // "  Is shocked that his manager doesn't treat him as a first team regular"
                }
            } else {
                0xa87b0c // "  Feels he should be a first team regular"
            }
        } else {
            0xa87b0c
        };
        st.say(s);
    }
    if !p.player().is_null() && c.grievances() & 0x40000 != 0 {
        transfer_tab_media(st);
    }
    if c.grievances() & 0x100 != 0 {
        st.say(if st.future_now() {
            0xa87794 // "  Is looking forward to a new challenge"
        } else {
            0xa877bc // "  Wants a new challenge"
        });
    }
    if !club.is_null() && !club.manager().is_null() && c.grievances() & 8 != 0 {
        st.say(0xa87778); // "  Unhappy with his manager"
    }
}

/// The media comment of `transfer_tab_mood_grievances` (grievance 0x40000 of a player): by the finable story kind
/// (`get_finable_story_kind`), else by the supporters' view (`club_dislikes_staff`, `club_favours_staff`), loyalty,
/// pressure and days at the club, else the (local) media being overly critical.
unsafe fn transfer_tab_media(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let kind = get_finable_story_kind(read_usize(G_MEDIA), p.addr());
    let critical = || if club.reputation() > 0x1388 { 0xa87810 } else { 0xa877d4 }; // "  Feels the (local) media have been overly critical of him"
    let s = if kind != 0 {
        if kind == 8 {
            0xa87810
        } else if kind == 0x13 || (p.loyalty() <= 0xf && p.professionalism() <= 10) {
            0xa87ae8 // "  Has vented his feelings publicly"
        } else {
            0xa87abc // "  Feels he has been misquoted by the media"
        }
    } else if st.days_joined() < 0x5a {
        if exe::club_dislikes_staff(p, club) != 0 {
            if p.loyalty() > 0xf {
                0xa87a98 // "  Wants to win over the supporters"
            } else if p.loyalty() < 5 {
                0xa87a60 // "  Is fed up with the sniping from a minority of fans"
            } else {
                0xa87a30 // "  Hopes the supporters will give him a chance"
            }
        } else if exe::club_favours_staff(p, club) != 0 {
            if p.loyalty() > 0xf {
                0xa879fc // "  Cares only about what the supporters think of him"
            } else if p.loyalty() < 5 {
                0xa879cc // "  Dislikes the media criticism he has recieved"
            } else {
                0xa8799c // "  Wants to prove the media wrong for the fans"
            }
        } else {
            critical()
        }
    } else if exe::club_dislikes_staff(p, club) != 0 {
        if p.loyalty() > 0xf {
            0xa87974 // "  Wants to prove the supporters wrong"
        } else if p.loyalty() < 5 {
            0xa87944 // "  Has had enough of the fans' treatment of him"
        } else if c.grievances() == 0x40000 {
            0xa87910 // "  Wants the supporters to give him another chance"
        } else {
            0xa878d4 // "  Wants to move to a club with more appreciative supporters"
        }
    } else if exe::club_favours_staff(p, club) != 0 {
        if p.loyalty() > 0xf && p.pressure() > 5 {
            // the original writes this text and then overwrites it
            lang(JOB_TEXT, 0xa879fcusize);
        }
        if p.pressure() < 6 {
            0xa878a8 // "  Just wants the media to leave him alone"
        } else if p.loyalty() < 5 {
            0xa87874 // "  Unhappy with the media criticism he has recieved"
        } else {
            0xa87844 // "  Aims to prove the media wrong for the fans"
        }
    } else {
        critical()
    };
    st.say(s);
}

/// Mood, part 11: the selection (grievance 0x10): a player out of the board's favour wanting a new manager (the text
/// is written and then overwritten, as in the original), then by the matches left out: back in the team (glad,
/// worried, hoping to stay or not convinced, with "Unsettled" or "Wants a long run" first when 0x10 is the only
/// grievance) or out of it (reputation, waiting, ready, happy to wait, fed up, forcing his way, first team football).
unsafe fn transfer_tab_mood_selection(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    if c.grievances() & 0x10 == 0 {
        return;
    }
    covered!("tel08_3 transfer tab selection");
    let pl = p.player();
    if !pl.is_null()
        && p.loyalty() < 6
        && p.professionalism() < 6
        && pl.current_ability() > 0x50
        && pl.current_reputation() > 0x109a
        && read_usize(G_MANAGER_MANAGER) != 0
        && transfer_tab_board_confidence(club) < 0x109a
    {
        // "  Hopes the club's next manager will appreciate his talent more" (overwritten below, never shown)
        lang(JOB_TEXT, 0xa87738usize);
    }
    let only = |c: Contract| c.grievances() == 0x10;
    let unsettled = |st: &mut TransferTab| {
        if only(st.contract) {
            st.say(0xa88b90); // "  Unsettled"
        }
    };
    let s = if c.matches_left_out() as i8 > 1 {
        if c.grievances() & 0x6046008 != 0 || exe::staff_is_transfer_listed_or_requested(p, club) != 0 {
            unsettled(st);
            0xa87650 // "  Isn't convinced that he will stay in the managers plans"
        } else if p.professionalism() > 10 && p.pressure() > 10 {
            if only(c) {
                st.say(0xa87710); // "  Wants a long run in the first team"
            }
            0xa876ec // "  Is glad to be back in the team"
        } else if p.pressure() < 6 {
            unsettled(st);
            0xa876b4 // "  Is worried that he may be dropped from the team again"
        } else {
            unsettled(st);
            0xa8768c // "  Hopes to stay in the managers plans"
        }
    } else {
        let rep = pl.current_reputation();
        if rep > 0x186a && rep as i32 > club.reputation() as i32 - 0x4e2 && p.ambition() > 10 && p.professionalism() < 6 {
            0xa8760c // "  Feels his reputation should guarantee his place in the first team"
        } else if !club.is_null() && !pl.is_null() && p.ambition() > 0xe && rep < club.reputation() {
            let r = (read_i32(p.addr()) % 8) as i16;
            if p.professionalism() < 6
                && p.loyalty() < 6
                && pl.potential_ability() > (p.ambition() as i16).wrapping_add(r).wrapping_add(100)
            {
                unsettled(st);
                0xa875cc // "  May look elsewhere if he doesn't get first team football soon"
            } else if p.professionalism() < 0xf && (p.age() as i8) < 0x17 {
                if p.determination() < 10 && p.pressure() < 6 {
                    0xa875a4 // "  Worried about his future at the club"
                } else if p.pressure() <= 10 && p.loyalty() <= 10 {
                    0xa87578 // "  Feels he is ready for first team football"
                } else if c.morale() > 0 {
                    0xa87548 // "  Is happy to wait for his chance in the team"
                } else {
                    0xa88b90 // "  Unsettled"
                }
            } else {
                unsettled(st);
                if c.grievances() & 0x101 != 0 {
                    0xa87518 // "  Fed up waiting for a chance in the first team"
                } else {
                    0xa874e8 // "  Trying to force his way into the first team"
                }
            }
        } else {
            0xa874cc // "  Wants first team football"
        }
    };
    st.say(s);
}

/// Mood, part 12: life abroad (grievance 0x20 with a nation for the contract's club: time to move on after 720 days,
/// a mistake, or the lifestyle by the nationality adjective), personal problems (0x40), a team-mate's remarks
/// (0x200000), the manager's handling of the media (the second media response, a first team player), discipline
/// (flag 2), motivation (morale under −25), pressure (0x100000: handling it, rested, past his peak, worried, or in
/// English the club relying on him or jaded) and training (0x8000).
unsafe fn transfer_tab_mood_personal(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let age = p.age() as i8;
    let n = read_usize(club_at(c.club() as isize).addr() + club::NATION);
    if c.grievances() & 0x20 != 0 && n != 0 {
        if st.days_joined() > 0x2d0 {
            lang(JOB_TEXT, 0xa874acusize); // "  Feels it is time to move on"
        } else if p.temperament() < 6 && p.professionalism() < 0xf {
            let g = read_i8(n + nation::GENDER_NAME) as i32;
            lang(JOB_TEXT, 0xa8745cusize, g, g, n + 4); // "  Feels he may have made a mistake in moving to {}<%s>{}"
        } else {
            let mut adj = [0u8; 0xc8];
            let a = adj.as_mut_ptr() as usize;
            exe::nation_get_nationality_adjective(n, a, 0xc8, 0x20);
            let g = read_i8(n + nation::GENDER_NAME) as i32;
            lang(JOB_TEXT, 0xa87408usize, g, g, a); // "  Finding it hard to adjust to the {}<%s>{} lifestyle"
        }
        st.line();
    }
    if c.grievances() & 0x40 != 0 {
        st.say(
            if p.pressure() < 5 && p.professionalism() < 6 && p.temperament() < 10 && p.player().world_reputation() > 0x109a {
                0xa873e4 // "  Is being treated for depression"
            } else {
                0xa873cc // "  Has personal problems"
            },
        );
    }
    if c.grievances() & 0x200000 != 0 {
        st.say(if p.pressure() < 6 {
            0xa873a4 // "  Feels hurt by his team-mate's remarks"
        } else {
            0xa87378 // "  Is unhappy about his team-mate's remarks"
        });
    }
    if exe::media_has_response_two_for_player(read_usize(G_MEDIA), p) != 0
        && p.pressure() > 0xf
        && p.temperament() < 6
        && p.player().current_reputation() > 0xea6
        && age > 0x12
        && matches!(st.squad(), 1 | 2)
    {
        st.say(if p.temperament() < 6 && p.professionalism() < 10 && p.pressure() > 5 {
            0xa87344 // "  Outraged by his manager's handling of the media"
        } else if p.pressure() < 6 {
            0xa87314 // "  Hurt by his manager's comments to the media"
        } else {
            0xa872e0 // "  Upset with his manager's handling of the media"
        });
    }
    if st.flags() & 2 != 0 {
        st.say(0xa872b0); // "  Worried by the lack of managerial discipline"
    }
    let pl = p.player();
    if !club.is_null()
        && !pl.is_null()
        && c.morale() < -25
        && pl.work_rate() < 10
        && p.professionalism() < 10
        && pl.current_reputation() as i32 > club.reputation() as i32 + 0xfa
    {
        st.say(0xa87288); // "  Is having trouble motivating himself"
    }
    if c.grievances() & 0x100000 != 0 {
        let s = if age < 0x17 && p.pressure() < 10 && p.professionalism() < 6 {
            Some(0xa87244) // "  Is having trouble handling the pressure of first team football"
        } else if !pl.is_null() && age < 0x17 && p.professionalism() > 0xf && pl.stamina() < 0xf {
            Some(0xa87220) // "  Feels that he needs to be rested"
        } else if age > 0x1e && p.pressure() < 6 {
            Some(0xa871fc) // "  Believes that he is past his peak"
        } else if age <= 0x1e && p.pressure() < 6 && c.grievances() & 0x20181 == 0 {
            Some(0xa875a4) // "  Worried about his future at the club"
        } else if exe::langlib_get_language() == 0 {
            Some(if ((c.matches_left_out() as i8) > 4 || c.happiness() > 0x4b) && age < 0x1b {
                0xa871d0 // "  Feels the club relies upon him too much"
            } else {
                0xa871c8 // "  Jaded"
            })
        } else {
            None
        };
        if let Some(s) = s {
            st.say(s);
        }
    }
    if c.grievances() & 0x8000 != 0 {
        st.say(if p.pressure() < 10 && p.professionalism() < 10 && age < 0x18 {
            0xa87190 // "  Feels his training regime is harming his development"
        } else {
            0xa87178 // "  Unhappy with training"
        });
    }
}

/// Mood, part 13: the outlook. With a future transfer: `transfer_tab_outlook_moving`. Otherwise not being allowed to
/// move (grievance 0x80), then the Bosman exit (with its date), the clarification wanted by a player without a squad
/// status (a button 5 when the human controls him), or, with no grievances, morale not below −loyalty and no
/// transfer request: anxious for a new contract, the manager's favour (helping the club, proving him wrong, a lost
/// career, the mentor, enthusiasm, developing), the loan, and the stay (`transfer_tab_outlook_stay`) or "Unknown".
unsafe fn transfer_tab_mood_outlook(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    if st.future_now() {
        transfer_tab_outlook_moving(st);
        return;
    }
    if c.grievances() & 0x80 != 0 {
        st.say(if p.temperament() < 6 && p.professionalism() < 0xf {
            0xa87144 // "  Is angry about not being allowed to move clubs"
        } else if p.professionalism() > 0xe {
            0xa87110 // "  Is frustrated at not being allowed to move clubs"
        } else {
            0xa870e0 // "  Is unhappy at not being allowed to move clubs"
        });
    }
    if c.leaving_on_bosman() != 0 {
        covered!("tel08_3 transfer tab bosman");
        let mut buf = [0u8; 0xc8];
        let b = buf.as_mut_ptr() as usize;
        let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x54a650usize);
        short_date(c.addr() + contract::CONTRACT_EXPIRES, b, 0xc8, 0xff);
        lang(JOB_TEXT, 0xa870a4usize, 100u32, b); // "  Leaving the club under the Bosman ruling on {}<%s - date>"
        st.line();
        return;
    }
    if st.squad() == 0 && p.job_for_club() == 0xb {
        lang(JOB_TEXT, 0xa87080usize); // "  Wants clarification on his future"
        let own = p.club_job();
        if (!own.is_null()
            && exe::human_manager_current_controls_club(own, 0, 1) != 0
            && exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) == 0)
            || exe::human_manager_current_controls_club(club_at(c.club() as isize), 0, 1) != 0
        {
            st.button(5, 0);
        } else {
            st.value(JOB_TEXT);
        }
        st.pad();
        st.row += 1;
        return;
    }
    if c.grievances() != 0 || (c.morale() as i32) < -(p.loyalty() as i32) || c.transfer_status() & 8 != 0 {
        return;
    }
    let pl = p.player();
    if p.pressure() < 6
        && (p.age() as i8) < 0x18
        && !st.future_now()
        && exe::staff_is_transfer_listed_or_requested(p, club) == 0
        && pl.current_reputation() < 0xcb2
        && pl.current_reputation() < club.reputation().wrapping_add(0xfa)
        && st.days_to_expiry() < 0xc8 - p.pressure() as i32
    {
        st.say(0xa8705c); // "  Anxious to sign a new contract"
    }
    if exe::staff_is_favoured_by(club.manager(), p) != 0 {
        transfer_tab_outlook_favoured(st);
    }
    if exe::staff_is_loan_listed(p, 0usize) != 0
        && (p.age() as i8) < 0x18
        && !pl.is_null()
        && pl.current_reputation() < 0x109a
        && p.professionalism() > 0xf
        && p.pressure() > 10
        && !matches!(st.squad(), 4 | 1 | 2 | 3)
    {
        st.say(0xa86e78); // "  Hoping to gain first team experience from going on loan"
    }
    let dispute: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x569e30usize);
    if exe::staff_is_transfer_listed_or_requested(p, club) == 0 && dispute(read_usize(G_DISPUTE), p.addr(), 0) == 0 {
        let s = transfer_tab_outlook_stay(st);
        st.say(s);
    } else if st.row == st.first {
        staff_tab_lang()(JOB_TEXT, 0xa86cb8usize); // "  Unknown"
        st.wrapped(G_GUI_COLOUR_YELLOW);
        st.row += 1;
    }
}

/// The manager's favour of `transfer_tab_mood_outlook` (the club's manager favours a person with no grievances).
unsafe fn transfer_tab_outlook_favoured(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let pl = p.player();
    let enthusiastic = 0xa86eec; // "  Enthusiastic about his future at the club"
    let convinced = 0xa86f18; // "  Convinced he will still make it as a professional"
    if !pl.is_null() {
        if p.loyalty() > 0xf
            && !st.future_now()
            && exe::staff_is_transfer_listed_or_requested(p, club) == 0
            && pl.current_reputation() as i32 > club.reputation() as i32 - 0x2ee
            && pl.current_ability() > 100i16.wrapping_sub(p.ambition() as i16)
            && club.reputation() < 0x128e
        {
            st.say(0xa8702c); // "  Wants to help the club fulfill its potential"
        }
        let rep = pl.current_reputation();
        if exe::media_has_response_two_for_player(read_usize(G_MEDIA), p) == 0 && (rep < 0xcb2 || rep < club.reputation()) && (p.age() as i8) < 0x18 {
            if exe::staff_is_transfer_listed_or_requested(p, club) == 0 {
                st.say(0xa86eb4); // "  Thinks his manager will help him develop as a player"
                lang(JOB_TEXT, enthusiastic); // written and not shown
                return;
            }
            if exe::staff_is_transfer_listed(p, 0usize) == 0 {
                st.say(enthusiastic);
                return;
            }
            if p.ambition() > 0xf {
                st.say(0xa87004); // "  Determined to prove his manager wrong"
                return;
            }
            let age = p.age() as i8;
            if age < 0x13 && pl.current_reputation() < 0x9c4 && p.ambition() < 6 {
                st.say(0xa86fc8); // "  Believes his ambition for a professional career is over"
                return;
            }
            let m = club.manager();
            if !m.is_null()
                && st.days_since(m.addr() + staff::DATE_JOINED_CLUB) > 0x258
                && age < 0x1c
                && st.days_joined() > 0x258
                && p.temperament() < 6
                && pl.aggression() > 0xf
            {
                st.say(0xa86f98); // "  Is angry that his mentor has abandoned him"
                return;
            }
            if p.professionalism() > 0xf {
                let g = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32;
                lang(JOB_TEXT, 0xa86f4cusize, g, g, club.addr() + club::SHORT_NAME); // "  Realises that his career at {}<%s>{} is over"
                st.line();
                return;
            }
            if age < 0x13 && p.ambition() > 0xf && !pl.is_null() && pl.current_reputation() < 0x9c4 {
                st.say(convinced);
            }
            return;
        }
    }
    if exe::staff_is_transfer_listed(p, 0usize) != 0 && (p.age() as i8) < 0x13 && p.ambition() > 0xf && !pl.is_null() && pl.current_reputation() < 0x9c4 {
        st.say(convinced);
        return;
    }
    if exe::staff_is_transfer_listed(p, 0usize) == 0 {
        lang(JOB_TEXT, enthusiastic); // written and not shown
    }
}

/// The stay comment of `transfer_tab_mood_outlook` (not listed, not in dispute): favouring the club (player-manager,
/// staying on after playing, enthusiastic, enjoying life, impressing the new manager, enjoying playing, content),
/// with contract flags (considering options, willing to stay), committing his future, happy with the conditions, or
/// happy to stay.
unsafe fn transfer_tab_outlook_stay(st: &TransferTab) -> usize {
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let age = p.age() as i8;
    let happy = 0xa86cc4; // "  Happy to stay at the club"
    if exe::staff_favours_club(p, club) != 0 && p.loyalty() > 5 {
        let job = p.job_for_club();
        if job == 0xc {
            return 0xa86e50; // "  Happy in his role as player-manager"
        }
        if !p.non_player().is_null() && !p.player().is_null() && age > 0x20 && job == 0xb {
            return 0xa86e0c; // "  Would like to remain at the club after his playing career is over"
        }
        if age < 0x19 {
            return 0xa86eec; // "  Enthusiastic about his future at the club"
        }
        if exe::database_is_player_job(job as i8) == 0 {
            return 0xa86dec; // "  Is enjoying life at the club"
        }
        if st.days_joined() < 0x1e {
            return 0xa86dc8; // "  Wants to impress his new manager"
        }
        let enjoying = 0xa86da4; // "  Is enjoying playing for the club"
        if st.squad() == 1 || (st.squad() == 2 && c.happiness() > 0x19 && st.days_joined() > 0x2d) {
            return enjoying;
        }
        if st.squad() == 3 && c.unhappy_flag() > 1 {
            return happy;
        }
        if st.squad() == 3 {
            return enjoying;
        }
        return if age >= 0x1f { happy } else { 0xa86d84 }; // "  Content to stay at the club"
    }
    if st.flags() != 0 {
        return if p.loyalty() < 6 && p.professionalism() < 10 {
            0xa8827c // "  Considering his options"
        } else if p.loyalty() < 6 {
            0xa86d5c // "  Willing to stay at the club currently"
        } else {
            0xa86d3c // "  Willing to stay at the club"
        };
    }
    if club.is_null() {
        return happy;
    }
    let pl = p.player();
    if p.loyalty() > 0xf && c.morale() > 0x19 && !pl.is_null() && pl.current_reputation() < club.reputation() && age < 0x19 {
        return 0xa86d10; // "  Happy to commit his future to the club"
    }
    if p.loyalty() < 5
        && p.professionalism() < 5
        && !pl.is_null()
        && pl.current_reputation() > 0x109a
        && pl.current_reputation() as i32 > club.reputation() as i32 - 0x5dc
        && age < 0x1f
    {
        return 0xa86ce0; // "  Happy with conditions at the club currently"
    }
    happy
}

/// The outlook of a person with a future transfer (`transfer_tab_mood_outlook`): not in dispute and with no grievances,
/// with morale not below −50 − loyalty: the loan listing (gaining experience, unsure of the gain, too good to be
/// listed, disappointed) or insecure, jaded, wanting to extend, unsettled or unknown; with low morale: considering his
/// options, the loan listing, or unsure.
unsafe fn transfer_tab_outlook_moving(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let club = st.club;
    let c = st.contract;
    let age = p.age() as i8;
    let dispute: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x569e30usize);
    let unknown = 0xa86b14; // "  Unknown<%s - COMMENT - unknown future>"
    let unsure = 0xa86c7c; // "  Unsure"
    let experience = 0xa86e78; // "  Hoping to gain first team experience from going on loan"
    let not_sure = 0xa86c88; // "  Not sure what he will gain by going on loan"
    let too_good = 0xa86c3c; // "  Feels a player of his ability shouldn't be listed for loan"
    if dispute(read_usize(G_DISPUTE), p.addr(), 0) == 0 && c.grievances() == 0 && c.morale() as i32 >= -0x32 - p.loyalty() as i32 {
        covered!("tel08_3 transfer tab outlook moving");
        let pl = p.player();
        let (s, comment) = if exe::staff_is_loan_listed(p, 0usize) != 0 {
            if age < 0x18 {
                if p.professionalism() > 0xf && p.pressure() > 10 && !pl.is_null() && pl.current_reputation() < 0xcb2 && !matches!(st.squad(), 4 | 1 | 2 | 3) {
                    (experience, false)
                } else if p.loyalty() < 6
                    && p.professionalism() < 10
                    && !pl.is_null()
                    && pl.current_ability() as i32 > p.professionalism() as i32 - p.ambition() as i32 + 100
                {
                    (not_sure, false)
                } else {
                    (unsure, false)
                }
            } else if !pl.is_null()
                && p.professionalism() < 10
                && p.ambition() > 0xf
                && pl.current_ability() as i32 > p.professionalism() as i32 - p.ambition() as i32 + 100
                && pl.current_reputation() > 0xcb2
            {
                (too_good, false)
            } else if p.professionalism() < 10 && p.temperament() < 10 {
                (0xa86c14, false) // "  Disappointed at being listed for loan"
            } else {
                (unknown, true)
            }
        } else if p.pressure() < 5 && age < 0x17 {
            (0xa86be0, true) // "  Feels insecure<%s - COMMENT - Insecure future>"
        } else if p.loyalty() < 5 && age > 0x1f {
            (0xa86bac, true) // "  Jaded<%s - COMMENT - Jaded opinion on situation>"
        } else if p.pressure() < 10
            && !pl.is_null()
            && pl.current_reputation() < 0xcb2
            && pl.current_reputation() < club.reputation().wrapping_add(0xfa)
            && st.days_to_expiry() < p.pressure() as i32 + 0x1e
        {
            (if age < 0x15 { 0xa86be0 } else { 0xa86b68 }, true) // "  Would like to extend his contract<%s - COMMENT - Insecure future>"
        } else if st.row != st.first {
            (0xa86b40, true) // "  Unsettled<%s - COMMENT - Unsettled>"
        } else {
            (unknown, true)
        };
        if comment {
            lang(JOB_TEXT, s, TRANSFER_TAB_COMMENT);
        } else {
            lang(JOB_TEXT, s);
        }
        st.line();
        return;
    }
    if dispute(read_usize(G_DISPUTE), p.addr(), 0) != 0 || c.grievances() != 0 {
        return;
    }
    let s = if c.morale() < -75 && p.temperament() < 0xf && p.professionalism() < 10 && p.ambition() > 5 && !st.future_now() {
        0xa8827c // "  Considering his options"
    } else if exe::staff_is_loan_listed(p, 0usize) != 0 {
        if age < 0x18 {
            if p.professionalism() > 0xf && p.pressure() > 10 {
                experience
            } else if p.loyalty() < 6 && p.professionalism() < 10 {
                not_sure
            } else {
                unsure
            }
        } else if p.professionalism() < 10 && p.ambition() > 0xf && p.player().current_reputation() > 0xcb2 {
            too_good
        } else if p.professionalism() < 10 && p.temperament() < 10 {
            0xa86af0 // "  Upset at being listed for loan"
        } else {
            lang(JOB_TEXT, unknown, TRANSFER_TAB_COMMENT);
            st.line();
            return;
        }
    } else {
        unsure
    };
    st.say(s);
}

/// The Potential Fees section: the percentage of the next transfer fee, the first team appearance clause and the
/// international appearance clause (after the first, or after N appearances with the count so far), when any exists.
unsafe fn transfer_tab_potential_fees(st: &mut TransferTab) {
    let lang = staff_tab_lang();
    let p = st.p;
    let tm = read_usize(G_TRANSFER_MANAGER);
    let percentage = find_next_sale_percentage(tm, p.addr());
    let league = find_first_team_appearance_money(tm, p.addr());
    let international = find_international_appearance_money(tm, p.addr());
    if percentage == 0 && league == 0 && international == 0 {
        return;
    }
    covered!("tel08_3 transfer tab potential fees");
    lang(JOB_TEXT, 0xa86a54usize); // "  Potential Fees"
    st.label(JOB_TEXT);
    let first = st.row;
    if percentage != 0 {
        lang(JOB_TEXT, 0xa86a20usize, read_i8(percentage) as i32); // "  <%d - Percentage(e.g.25)>%% of next transfer fee"
        st.wrapped(G_GUI_COLOUR_YELLOW);
        st.row += 1;
    }
    let mut buf = [0u8; 0xc8];
    for (clause, kind, once, many) in [(league, 2, 0xa869e4usize, 0xa86974usize), (international, 4, 0xa86938, 0xa868c8)] {
        if clause == 0 {
            continue;
        }
        let b = transfer_tab_cash(read_i32(clause + appearance_clause::AMOUNT), &mut buf);
        if st.row != first {
            st.label(0);
        }
        let games = read_i8(clause + appearance_clause::GAMES);
        if games == 1 {
            lang(JOB_TEXT, once, b); // "  <%s - Cash> after first league appearance" / "... first international cap"
        } else {
            let so_far = get_clause_count(tm, p.addr(), kind) as i16 as i32;
            lang(JOB_TEXT, many, b, games as i32, so_far); // "  <%s> after <%d> league appearances (<%d> so far)" / "... international caps"
        }
        st.wrapped(G_GUI_COLOUR_YELLOW);
        st.row += 1;
    }
}

/// Draws the Transfer tab of the staff profile (the person of screen parameter 0) between screen rows `top` and
/// `bottom`.
///
/// Without transfer information (a person whose `job_for_club` is 1 or 2, or an id from the person count − 16 up),
/// only "There is no transfer information available." in font 3. Otherwise a two-column table (weights 2 and 7, as
/// many rows as fit in font 2), labels in pale grey and values in yellow, in sections: the details
/// (`transfer_tab_details`: full name, availability, value, second nationality, fluent and basic languages), Offers
/// (`transfer_tab_offers`, with the offers listed in screen parameter 3 and the viewer's club taken from the human of
/// the current screen slot), Interested (`transfer_tab_interested`), Future (`transfer_tab_future`: the contract offer
/// being considered, retirement, and the long list of the person's mood comments by his contract's grievances and
/// flags, his character, his club and its league form, his manager and the board, each a wrapped line) and Potential
/// Fees (`transfer_tab_potential_fees`), then empty rows to the end, the area's scroll row from screen parameter 0xb and
/// the area's index in screen parameter 0xc. Each contract lookup but the first also leaves the contract in
/// `G_CONTRACT_LOOKUP_RESULT`.
///
/// Kept from the original: a few texts are written into the buffer and then overwritten without being shown (the
/// supporters he cares about, the next manager, the enthusiasm after the development line); the "club" checks of the
/// mood comments (a contract always has one); the future transfer's missing offer slot shows the "Error" box (line
/// 0x1902).
/// Original: `staff_screens_draw_transfer_tab` (0x87dd80, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_draw_transfer_tab(top: i32, bottom: i32) {
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let weights: [u8; 2] = [2, 7];
    let p = Staff(get_screen_param(0));
    let offers = get_screen_param(3);
    let manager = tel26_slot_manager();
    let viewer = if manager.is_null() { Club(0) } else { manager.club_job() };
    let job = p.job_for_club();
    let count = read_i32(G_STAFF_COUNT);
    if job == 1 || job == 2 || read_i32(p.addr()) >= count - 0x10 || (!p.is_null() && read_i32(p.addr()) >= count) {
        covered!("tel08_3 transfer tab without information");
        staff_tab_no_information(top, bottom, 0xa8689c); // "There is no transfer information available."
        exe::drop_llist(l);
        return;
    }
    covered!("tel08_3 transfer tab");
    let rows = staff_tab_rows(top, bottom, 2);
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let mut st = TransferTab {
        area,
        row: 0,
        first: 0,
        p,
        contract: Contract(0),
        club: Club(0),
        viewer,
        future: 0,
        list: l,
        matches_played: 0,
        position: 0,
        final_position: 0,
        losing_run: 0,
        winning_run: 0,
    };
    transfer_tab_details(&mut st);
    transfer_tab_offers(&mut st, offers);
    transfer_tab_interested(&mut st);
    transfer_tab_future(&mut st);
    transfer_tab_potential_fees(&mut st);
    while st.row < rows as i32 {
        staff_tab_cell(area, 0, st.row, 2, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, 0);
        staff_tab_cell(area, 1, st.row, 2, 0, 0xc, 2, G_GUI_COLOUR_PALE_GREY, 0);
        st.row += 1;
    }
    let scroll: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x4567a0usize);
    scroll(G_SCREEN_MANAGER, area, get_screen_param(0xb) as u32);
    set_screen_param(0xc, area as u16 as i16 as isize as usize);
    exe::drop_llist(l);
}

/// Comparer for `staff_screens_draw_transfer_tab`: both draw side by side and are compared by the display list
/// (`compare_match_screen_build`, which takes the screen's parameter block, so the area index of parameter 0xc) plus
/// the contract lookup result the tab leaves (`G_CONTRACT_LOOKUP_RESULT`).
pub(crate) unsafe extern "C" fn compare_staff_screens_draw_transfer_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I0%%);
    let extra = [(G_CONTRACT_LOOKUP_RESULT, 4)];
    compare_match_screen_build("staff profile transfer tab", &extra, || o(top, bottom), || staff_screens_draw_transfer_tab(top, bottom));
}
