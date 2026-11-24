/// `staff_add_disliked_staff(person, other, force)`: puts `other` among the person's three disliked staff (preferences
/// `staff_preferences()`: `preferences::STAFF_DISLIKED_STAFF1`..+0x30).
///
/// Nothing when `other` is the person, or with a null person, preferences or other. Already disliked: nothing; one of
/// the three favoured staff (`preferences::STAFF_FAVOURITE_STAFF1`..+0x24): stops being one and the routine stops;
/// otherwise the first free disliked slot; with no free slot, with `force`, a random slot (`random`, 3)
/// (`database_add_to_preference_list`). Used after the match (the media's ratings of a player, the fights and the
/// reserves' complaints), by the injuries and the transfers.
/// Original: `staff_add_disliked_staff` (0x547af0, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_add_disliked_staff(person: usize, other: usize, force: u32) {
    let person = Staff(person);
    if person.addr() == other || person.is_null() {
        return;
    }
    let pref = person.staff_preferences();
    if pref == 0 || other == 0 {
        return;
    }
    let disliked = pref + preferences::STAFF_DISLIKED_STAFF1;
    database_add_to_preference_list(disliked, pref + preferences::STAFF_FAVOURITE_STAFF1, other, list3_contains(disliked, other), force)
}

/// Comparer for `staff_add_disliked_staff`: the favoured and disliked staff (24 bytes) and the random generator, side by
/// side.
pub(crate) unsafe extern "C" fn compare_staff_add_disliked_staff(person: usize, other: usize, force: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(%%I0%%);
    let pref = if person != 0 { Staff(person).staff_preferences() } else { 0 };
    let reg = preference_regions(if pref != 0 { pref + preferences::STAFF_FAVOURITE_STAFF1 } else { 0 });
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(person, other, force),
        || crate::rng::without_counting(|| staff_add_disliked_staff(person, other, force)));
    SCOREBOARD.record(same, || format!("staff_add_disliked_staff({:x}, {:x}, {}) {:x?} x {:x?}", person, other, force, mo, mr));
}

/// `staff_add_favoured_club(person, club, force)`: puts the club among the person's three favoured clubs (preferences
/// `staff_preferences()`: `preferences::STAFF_FAVOURITE_CLUBS1`..+0xc).
///
/// Null person, preferences or club: nothing. The club already favours the person (`club::FAV_STAFF1`..+0xa3):
/// nothing; the club is one of the person's disliked clubs (`preferences::STAFF_DISLIKED_CLUBS1`..+0x18): it stops
/// being one and the routine stops; otherwise the first free favoured slot (without checking whether it is already
/// among the favoured, as in the original); with no free slot, with `force`, a random slot (`random`, 3)
/// (`database_add_to_preference_list`). Used after the match (the player pleased with his club) and by the transfers.
/// Original: `staff_add_favoured_club` (0x547d90, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_add_favoured_club(person: usize, club: usize, force: u32) {
    let person = Staff(person);
    let club = Club(club);
    if person.is_null() {
        return;
    }
    let pref = person.staff_preferences();
    if pref == 0 || club.is_null() {
        return;
    }
    let club_favours_person = list3_contains(club.addr() + club::FAV_STAFF1, person.addr());
    database_add_to_preference_list(pref + preferences::STAFF_FAVOURITE_CLUBS1, pref + preferences::STAFF_DISLIKED_CLUBS1, club.addr(), club_favours_person, force)
}

/// Comparer for `staff_add_favoured_club`: the favoured and disliked clubs (24 bytes) and the random generator, side by
/// side.
pub(crate) unsafe extern "C" fn compare_staff_add_favoured_club(person: usize, club: usize, force: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32)>(%%I1%%);
    let pref = if person != 0 { Staff(person).staff_preferences() } else { 0 };
    let reg = preference_regions(if pref != 0 { pref + preferences::STAFF_FAVOURITE_CLUBS1 } else { 0 });
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(person, club, force),
        || crate::rng::without_counting(|| staff_add_favoured_club(person, club, force)));
    SCOREBOARD.record(same, || format!("staff_add_favoured_club({:x}, {:x}, {}) {:x?} x {:x?}", person, club, force, mo, mr));
}
