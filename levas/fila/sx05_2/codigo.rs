/// A year ratings entry (`year_ratings_consts::ENTRY_SIZE` bytes: +0 staff id, +4 nation id, +8/+0xa/+0xc the three
/// weighted rating sums, +0xe matches, +0xf goals, +0x11 assists, +0x13 cards), as the sort modes read it.
#[derive(Clone, Copy)]
pub(crate) struct YearEntry(pub(crate) usize);

impl YearEntry {
    pub(crate) unsafe fn person(self) -> Staff {
        staff_at(read_i32(self.0) as isize)
    }
    pub(crate) unsafe fn nation_id(self) -> i32 {
        read_i32(self.0 + 4)
    }
    /// The weighted rating sum `k` (0, 1 or 2: the +8, +0xa or +0xc word).
    pub(crate) unsafe fn rating_sum(self, k: usize) -> i32 {
        read_i16(self.0 + 8 + 2 * k) as i32
    }
    pub(crate) unsafe fn matches(self) -> i8 {
        read_i8(self.0 + 0xe)
    }
    pub(crate) unsafe fn goals(self) -> i32 {
        read_i16(self.0 + 0xf) as i32
    }
    pub(crate) unsafe fn assists(self) -> i32 {
        read_i16(self.0 + 0x11) as i32
    }
    /// Yellow cards + 2 × red cards.
    pub(crate) unsafe fn cards(self) -> i32 {
        read_i16(self.0 + 0x13) as i32
    }
}

/// The qsort order of two year ratings entries by their scores: an entry with fewer than 3 matches goes after one
/// with 3 or more (two such are equal); otherwise the higher score first (-1), the lower (or an unordered pair) after
/// (1), equal 0.
pub(crate) unsafe fn year_entries_order(a: YearEntry, b: YearEntry, score: unsafe fn(YearEntry) -> f64) -> i32 {
    if a.matches() < 3 {
        return (b.matches() >= 3) as i32;
    }
    if b.matches() < 3 {
        return -1;
    }
    covered!("sx05_2 year order by score");
    let (sa, sb) = (score(a), score(b));
    if sa > sb {
        -1
    } else if !(sa >= sb) {
        1
    } else {
        0
    }
}

/// The average-rating score of sort modes 0 to 2: (goals + 2 × the mode's rating sum + the other two sums) / matches,
/// plus 2 × matches × 0.02; × 1.2 for a player rated above 14 as an attacker, else × 1.05 above 14 as an attacking
/// midfielder.
pub(crate) unsafe fn year_rating_score(e: YearEntry, doubled: usize) -> f64 {
    let n = e.matches() as f64;
    let total = e.goals() + e.rating_sum(0) + e.rating_sum(1) + e.rating_sum(2) + e.rating_sum(doubled);
    let mut score = total as f64 / n + (n * 0.02 + n * 0.02);
    let sheet = e.person().player();
    if !sheet.is_null() {
        if sheet.attacker() > 14 {
            covered!("sx05_2 attacker bonus");
            score *= 1.2f32 as f64;
        } else if sheet.attacking_midfielder() > 14 {
            score *= 1.05f32 as f64;
        }
    }
    score
}

/// Orders two year ratings entries for rating mode 0: the score of `year_rating_score` with the +8 rating sum doubled
/// (`year_entries_order`: at least 3 matches, the higher score first).
/// Original: `year_ratings_compare_mode_0` (0x931f90, award/year_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn year_ratings_compare_mode_0(a: usize, b: usize) -> i32 {
    year_entries_order(YearEntry(a), YearEntry(b), |e| year_rating_score(e, 0))
}

/// Orders two year ratings entries for rating mode 1: the score of `year_rating_score` with the +0xa rating sum
/// doubled.
/// Original: `year_ratings_compare_mode_1` (0x9320e0, award/year_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn year_ratings_compare_mode_1(a: usize, b: usize) -> i32 {
    year_entries_order(YearEntry(a), YearEntry(b), |e| year_rating_score(e, 1))
}

/// Orders two year ratings entries for rating mode 2: the score of `year_rating_score` with the +0xc rating sum
/// doubled, then × the player's current ability × 0.00625 (when he has a player sheet).
/// Original: `year_ratings_compare_mode_2` (0x932230, award/year_ratings.cpp, cdecl).
pub(crate) unsafe extern "C" fn year_ratings_compare_mode_2(a: usize, b: usize) -> i32 {
    year_entries_order(YearEntry(a), YearEntry(b), |e| {
        let score = year_rating_score(e, 2);
        let sheet = e.person().player();
        if sheet.is_null() {
            return score;
        }
        covered!("sx05_2 mode 2 by ability");
        score * sheet.current_ability() as f64 * (0.00625f32 as f64)
    })
}

/// A comparer of the year ratings sort modes: both orders side by side, the original's returned.
pub(crate) unsafe fn compare_year_order(index: usize, name: &str, a: usize, b: usize, rust: unsafe extern "C" fn(usize, usize) -> i32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(index)(a, b);
    let v = rust(a, b);
    SCOREBOARD.record(r == v, || format!("{}(person {}, person {}) original={} rust={}", name, read_i32(a), read_i32(b), r, v));
    r
}

pub(crate) unsafe extern "C" fn compare_year_ratings_compare_mode_0(a: usize, b: usize) -> i32 {
    compare_year_order(%%I0%%, "year_ratings_compare_mode_0", a, b, year_ratings_compare_mode_0)
}

pub(crate) unsafe extern "C" fn compare_year_ratings_compare_mode_1(a: usize, b: usize) -> i32 {
    compare_year_order(%%I1%%, "year_ratings_compare_mode_1", a, b, year_ratings_compare_mode_1)
}

pub(crate) unsafe extern "C" fn compare_year_ratings_compare_mode_2(a: usize, b: usize) -> i32 {
    compare_year_order(%%I2%%, "year_ratings_compare_mode_2", a, b, year_ratings_compare_mode_2)
}
