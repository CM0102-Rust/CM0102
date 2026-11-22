// ---------- batch %%LEVA%%: the Web Sites screen (contract_screens.cpp) ----------

/// Rule: building the Web Sites screen: the title, the scrolling list with the developers' and the official
/// messageboard's addresses, the publisher rows and the fan club links, and the Back/Next buttons.
///
/// A random background picture (`background_set_random_picture`); the game title (heading colours 0 and 1); the menu
/// bar (4, 0); the subtitle "Web Sites"; the list area (0x176 / the font height rows, from y 0x91); the heading "Sports
/// Interactive" (not translated, the row counter from 0); "The developers" with www.sigames.com and "Official
/// Messageboard" with community.sigames.com; the publisher rows (`web_sites_add_publisher_rows`) and the fan club links
/// (`web_sites_add_fan_club_links`); the list scrolled to parameter 0 and parameter 1 = the area; then the Back/Next
/// buttons.
/// Original: `web_sites_screen_build` (0x513a20, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn web_sites_screen_build() {
    covered!("tel34_2 web sites screen");
    exe::background_set_random_picture();
    tel34_game_title();
    exe::menubar_build(4, 0);
    tel34_lang_text(0x9b53b4); // "Web Sites"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 0);
    tel34_credits_list_area(0x176, 0x91);
    tel34_copy_to_text(0x9b1240); // "Sports Interactive"
    write_u32(CREDITS_ROW, 0);
    tel34_credits_title_rows();
    tel34_lang_text(0x9b53a4); // "The developers"
    tel34_web_site_row(0x9b5394); // "www.sigames.com"
    tel34_lang_text(0x9b537c); // "Official Messageboard"
    tel34_web_site_row(0x9b5364); // "community.sigames.com"
    web_sites_add_publisher_rows();
    web_sites_add_fan_club_links();
    tel34_credits_list_done();
    tel26_back_next();
}

/// Comparer for `web_sites_screen_build`: the Web Sites screen side by side over the display list and the row state.
pub(crate) unsafe extern "C" fn compare_web_sites_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    compare_by_display_list_with("Web Sites screen (web_sites_screen_build)", &[CREDITS_STATE], || o(), || web_sites_screen_build());
}
