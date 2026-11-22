// ---------- batch %%LEVA%%: the credits screen (contract_screens.cpp, part of it): the Testing, Feral, Eidos and Translation rows ----------

/// The credits screen's build state (contract_screens.cpp globals, set by `credits_screen_build` 0x513180: the
/// sections of page 0, these four among them, then those of page 1; only the page shown adds its rows).
pub(crate) mod tel32_credits {
    /// u16: the row counter (the headings carry it in their value).
    pub(crate) const ROW: usize = 0xae1868;
    /// u32: the item number of the next row (the left cell of a row shares it with the right one).
    pub(crate) const ITEM: usize = 0xae186c;
    /// u16: the list area the rows go into (a byte follows at 0xae1872; a word field, its high half is not kept).
    pub(crate) const AREA: usize = 0xae1870;
    /// u16: the area of the section headings.
    pub(crate) const HEADING_AREA: usize = 0xae1874;
    /// u32: the column of the next name of a two-column list (0 left, 1 right).
    pub(crate) const COLUMN: usize = 0xae1878;
    /// u16: the page of the sections being built (0 for these four).
    pub(crate) const PAGE: usize = 0xae187c;
    /// u16: the page shown (screen parameter 2).
    pub(crate) const SHOWN_PAGE: usize = 0xae187e;
    /// The bytes from `ROW` to `SHOWN_PAGE` (the comparer's extra region).
    pub(crate) const STATE_SIZE: usize = 0x18;
}

/// The colour word of the role column (0xaebdde).
pub(crate) const TEL32_COLOUR_BDDE: usize = 0xaebdde;
/// The empty text of the exe (0xad9c64).
pub(crate) const TEL32_NO_TEXT: u32 = 0xad9c64;

/// Rows are added only when the sections' page is the page shown.
unsafe fn tel32_shown() -> bool {
    read_u16(tel32_credits::PAGE) == read_u16(tel32_credits::SHOWN_PAGE)
}

/// Takes the next item number (`tel32_credits::ITEM`, then one more).
unsafe fn tel32_next_item() -> u32 {
    let i = read_u32(tel32_credits::ITEM);
    write_u32(tel32_credits::ITEM, i.wrapping_add(1));
    i
}

/// Moves the row counter on by `k`.
unsafe fn tel32_bump_row(k: u16) {
    write_u16(tel32_credits::ROW, read_u16(tel32_credits::ROW).wrapping_add(k));
}

/// The rows' area, zero-extended (the original pushes the register with only its low half loaded; `Guio::init`
/// keeps the word).
unsafe fn tel32_area() -> u32 {
    read_u16(tel32_credits::AREA) as u32
}

/// `Display::add_item` (0x55f990) on the current slot's Display: the 18 fields.
unsafe fn tel32_item(f: [u32; 18]) {
    tel26_add_item(screens_current_display(G_SCREEN_MANAGER), f);
}

/// A text cell of the rows' area: x from `x1` to `x2`, item `item`, font 3, colour `colour`, text `text`.
unsafe fn tel32_cell(x1: u32, x2: u32, item: u32, colour: u32, text: u32) {
    tel32_item([1, x1, 0, x2, 0, 0, item, 1, 0, 0, 1, 3, colour, text, 0, 0, 0, tel32_area()]);
}

/// Rule: an empty row across the list.
///
/// The row counter goes up by 1; on the shown page, an empty text cell from x 0x82 to 0x2f8 with the next item
/// number, in the 0xae3184 colour.
unsafe fn tel32_blank_row() {
    tel32_bump_row(1);
    if tel32_shown() {
        let i = tel32_next_item();
        tel32_cell(0x82, 0x2f8, i, job_colour(JOB_COLOUR_3184), TEL32_NO_TEXT);
    }
}

/// Rule: the section heading whose text is already in the screens' text buffer (`JOB_TEXT`).
///
/// The heading item goes into `tel32_credits::HEADING_AREA` (kind 0x10, colours 0xae31aa, 0xaebdec and 0xaebdea):
/// flags 0x2002 when the item number equals screen parameter 0 and the page is the shown one (the selected section
/// [inference]), otherwise flags 2 with the value 0x64 and the page in the high word over the row counter in the low
/// one. On the shown page, two cells as wide as the text in font 3 plus 0x82 (from x 0x82), each with the next item
/// number: the text in the 0xaebdec colour, then an empty one with style 0xc400 and that colour as its first colour.
/// The row counter goes up by 2.
unsafe fn tel32_heading() {
    let shown = tel32_shown();
    let bdea = job_colour(JOB_COLOUR_BDEA);
    let bdec = job_colour(JOB_COLOUR_BDEC);
    let c31aa = job_colour(JOB_COLOUR_31AA);
    let heading_area = read_u16(tel32_credits::HEADING_AREA) as u32;
    let text = JOB_TEXT as u32;
    if read_u32(tel32_credits::ITEM) as usize == get_screen_param(0) && shown {
        covered!("tel32_1 the selected heading");
        tel32_item([0x2002, 0, 0, 0, 0, 0, 0, 0x10, c31aa, bdec, 1, 1, bdea, text, 0, 0, 0, heading_area]);
    } else {
        let value = (read_u16(tel32_credits::PAGE) as u32) << 16 | read_u16(tel32_credits::ROW) as u32;
        tel32_item([2, 0, 0, 0, 0, 0, 0, 0x10, c31aa, bdec, 1, 1, bdea, text, 0, 0x64, value, heading_area]);
    }
    if shown {
        // the original passes the whole dword at 0xae1870 here; Guio::init keeps the word
        let area = read_u32(tel32_credits::AREA);
        let x2 = (exe::font_text_width(3usize, JOB_TEXT as *const u8) as u32).wrapping_add(0x82);
        let i = tel32_next_item();
        tel32_item([1, 0x82, 0, x2, 0, 0, i, 1, 0, 0, 1, 3, bdec, text, 0, 0, 0, area]);
        let x2 = (exe::font_text_width(3usize, JOB_TEXT as *const u8) as u32).wrapping_add(0x82);
        let i = tel32_next_item();
        tel32_item([1, 0x82, 0, x2, 0, 0, i, 0xc400, bdec, 0, 0, 0, 0, 0, 0, 0, 0, area]);
    }
    tel32_bump_row(2);
}

/// Rule: a row with a name on the left and the role (already in `JOB_TEXT`) on the right.
///
/// The row counter goes up by 1; on the shown page, the name from x 0x82 to 0x1b8 (0xae3184 colour) with the current
/// item number, without taking it, and the role from 0x1c2 to 0x2f8 (0xaebdde colour) taking it.
unsafe fn tel32_role_row(name: u32) {
    tel32_bump_row(1);
    if tel32_shown() {
        tel32_cell(0x82, 0x1b8, read_u32(tel32_credits::ITEM), job_colour(JOB_COLOUR_3184), name);
        let i = tel32_next_item();
        tel32_cell(0x1c2, 0x2f8, i, job_colour(TEL32_COLOUR_BDDE), JOB_TEXT as u32);
    }
}

/// A role row whose role is translated (`langlib_sprintf` of the role text, no arguments).
unsafe fn tel32_translated_role_row(role: u32, name: u32) {
    comp_lang_format()(JOB_TEXT, role as usize);
    tel32_role_row(name);
}

/// Rule: a list of names in two columns.
///
/// The column starts at 0. For each name, on the shown page: in the left column the name goes from x 0x82 to 0x1b8
/// with the current item number (not taken), in the right one from 0x1c2 to 0x2f8 taking it (0xae3184 colour), and
/// the column flips (1 − column). Then, when the column is 0, the row counter goes up by 1: once per two names on the
/// shown page, but once per name on the other one (the column does not flip there; a quirk of the original: the
/// row values of the hidden page's later headings do not match the rows the page has when shown [inference]).
unsafe fn tel32_name_columns(names: &[u32]) {
    write_u32(tel32_credits::COLUMN, 0);
    for &name in names {
        if tel32_shown() {
            if read_u32(tel32_credits::COLUMN) == 0 {
                tel32_cell(0x82, 0x1b8, read_u32(tel32_credits::ITEM), job_colour(JOB_COLOUR_3184), name);
            } else {
                let i = tel32_next_item();
                tel32_cell(0x1c2, 0x2f8, i, job_colour(JOB_COLOUR_3184), name);
            }
            write_u32(tel32_credits::COLUMN, 1u32.wrapping_sub(read_u32(tel32_credits::COLUMN)));
        }
        if read_u32(tel32_credits::COLUMN) == 0 {
            tel32_bump_row(1);
        }
    }
}


/// The Testing names (exe texts), in the order of the original.
const TEL32_TESTING_NAMES: [u32; 32] = [
    0x9b18d0, // Dan Gravell
    0x9b18c0, // Glenn Wakeford
    0x9b18a4, // Eamonn 'Whitewashman' Kenny
    0x9b1898, // John Holt
    0x9b1888, // Stephen Norrie
    0x9b187c, // Chris Lake
    0x9b1868, // Rohit Alimchandani
    0x9b185c, // Tom Sheldon
    0x9b1850, // Alex Bell
    0x9b182c, // Stuart 'DisplacedSeagull' Fairway
    0x9b181c, // Steven Buckley
    0x9b1810, // Bambo Akani
    0x9b17fc, // Franck Zuffellato
    0x9b1898, // John Holt
    0x9b17ec, // Mark Cosgrove
    0x9b17dc, // Peter Blackwell
    0x9b17c0, // Chris 'SpursLodger' Bedwell
    0x9b17b4, // Will Tully
    0x9b179c, // Paul 'Keeper' Stewart
    0x9b1790, // James Yorke
    0x9b1780, // Adrian Osborne
    0x9b1770, // Andrew Jolly
    0x9b1758, // Jared 'Hoopy' McGeehan
    0x9b1748, // Marco Camilleri
    0x9b1738, // Graham Stark
    0x9b172c, // Robert Pang
    0x9b171c, // David Stainer
    0x9b1708, // Craig 'Ter' Hunter
    0x9b16f0, // Gordon 'Gudge' Dunbar
    0x9b16d4, // Jeroen 'Reddiablo' Thijssen
    0x9b16bc, // Francis 'Teaboy' Cole
    0x9b16b0, // Daniel Nash
];

/// Rule: the Testing section: an empty row, the "Testing" heading (translated) and the testers' names in two columns.
///
/// The rows through the credits row helpers (`tel32_blank_row`, `tel32_heading`, then the names or the role rows);
/// the texts are the exe's own, passed by address as the original does.
/// Original: `credits_add_testing_rows` (0x4e92f0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_testing_rows() {
    covered!("tel32_1 testing rows");
    tel32_blank_row();
    comp_lang_format()(JOB_TEXT, 0x9b18dcusize); // "Testing"
    tel32_heading();
    tel32_name_columns(&TEL32_TESTING_NAMES);
}

/// The Feral rows: (role, name), exe texts, in the order of the original.
const TEL32_FERAL_ROLES: [(u32, u32); 1] = [
    (0x9b18f4, 0x9b18e4), // Producer: David Stephen
];

/// Rule: the Feral Interactive section: an empty row, the heading (copied, not translated) and the producer row.
///
/// The rows through the credits row helpers (`tel32_blank_row`, `tel32_heading`, then the names or the role rows);
/// the texts are the exe's own, passed by address as the original does.
/// Original: `credits_add_feral_rows` (0x4eb110, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_feral_rows() {
    covered!("tel32_1 feral rows");
    tel32_blank_row();
    tel26_copy_text(JOB_TEXT as *mut u8, 0x9b1900 as *const u8); // "Feral Interactive"
    tel32_heading();
    for &(role, name) in TEL32_FERAL_ROLES.iter() {
        tel32_translated_role_row(role, name);
    }
}

/// The Eidos rows: (role, name), exe texts, in the order of the original.
const TEL32_EIDOS_ROLES: [(u32, u32); 24] = [
    (0x9b18f4, 0x9b1b70), // Producer: Andrew Norman
    (0x9b1b5c, 0x9b1b50), // Associate Producer: Steve Didd
    (0x9b1b3c, 0x9b1b28), // Marketing Manager: Jonathan Rosenblatt
    (0x9b1b1c, 0x9b1b0c), // PR Manager: Steve Starvis
    (0x9b1af4, 0x9b1ae4), // Localisation QA Manager: Marco Vernetti
    (0x9b1ac8, 0x9b1ab8), // Localisation Lead Tester: Arnaud Messager
    (0x9b1aa8, 0x9b1a98), // Localisation: Maike Köhler
    (0x9b1aa8, 0x9b1a88), // Localisation: Iain Willows
    (0x9b1a6c, 0x9b1a5c), // Group Localisation Manager: Flavia Timiani
    (0x9b1118, 0x9b1a4c), // QA Manager: Chris Rowley
    (0x9b1a34, 0x9b1a28), // Assistant QA Manager: John Ree
    (0x9b1a18, 0x9b1b50), // QA Co-ordinator: Steve Didd
    (0x9b1a14, 0x9b1a08), // QA: Ron Jackson
    (0x9b1a14, 0x9b19f8), // QA: Tony Peterken
    (0x9b1a14, 0x9b19e4), // QA: Scott Sutherland
    (0x9b1a14, 0x9b19d8), // QA: Joe Neate
    (0x9b1a14, 0x9b19cc), // QA: Ray Mullen
    (0x9b1a14, 0x9b19c0), // QA: Steve Addis
    (0x9b1a14, 0x9b19b0), // QA: Marlon Grant
    (0x9b1a14, 0x9b19a0), // QA: Lawrence Day
    (0x9b1a14, 0x9b1990), // QA: Jon Redington
    (0x9b1a14, 0x9b1980), // QA: Markus Poltorp
    (0x9b1970, 0x9b1960), // MC Co-ordinator: Jason Walker
    (0x9b1954, 0x9b1944), // Mastering: Phil Spencer
];

/// Rule: the Eidos Interactive section: an empty row, the heading (copied), the role rows (roles translated) and, in English only, the photos thanks row (printed without translation).
///
/// The rows through the credits row helpers (`tel32_blank_row`, `tel32_heading`, then the names or the role rows);
/// the texts are the exe's own, passed by address as the original does.
/// Original: `credits_add_eidos_rows` (0x4eb480, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_eidos_rows() {
    covered!("tel32_1 eidos rows");
    tel32_blank_row();
    tel26_copy_text(JOB_TEXT as *mut u8, 0x9b1b80 as *const u8); // "Eidos Interactive"
    tel32_heading();
    for &(role, name) in TEL32_EIDOS_ROLES.iter() {
        tel32_translated_role_row(role, name);
    }
    if exe::langlib_get_language() != 0 {
        return;
    }
    covered!("tel32_1 the English only row");
    comp_crt_format()(JOB_TEXT, 0x9b1928usize); // "Tickets and Man City photos"
    tel32_role_row(0x9b1914); // Julie and Debbie
}

/// The Translation rows: (role, name), exe texts, in the order of the original.
const TEL32_TRANSLATION_ROLES: [(u32, u32); 13] = [
    (0x9b1c38, 0x9b1c30), // Danish: Babel
    (0x9b1c38, 0x9b1c20), // Danish: Peter Højholt
    (0x9b1c18, 0x9b1c00), // Dutch: Ruud van de Moosdijk
    (0x9b1bf8, 0x9b1570), // French: Emmanuel Rochard
    (0x9b1bf0, 0x9b1be4), // German: Thomas Knie
    (0x9b1bf0, 0x9b1bd4), // German: Alexander Barth
    (0x9b1bcc, 0x9b14f4), // Italian: Alberto Scotta 'Panoz'
    (0x9b1bc0, 0x9b1458), // Norwegian: Vegard Rønningen
    (0x9b1bc0, 0x9b146c), // Norwegian: Tore Ludvigsen
    (0x9b1bb4, 0x9b1418), // Portuguese: 'Zé' Guilherme Chieira
    (0x9b1bac, 0x9b1358), // Spanish: Iván Abella Villar
    (0x9b1ba4, 0x9b1328), // Swedish: Jan Juhlin
    (0x9b1ba4, 0x9b1b94), // Swedish: David Landerin
];

/// Rule: the translation section: an empty row, the "Translation and Proof-Reading" heading (translated) and one row per translator with the language as the role (translated).
///
/// The rows through the credits row helpers (`tel32_blank_row`, `tel32_heading`, then the names or the role rows);
/// the texts are the exe's own, passed by address as the original does.
/// Original: `credits_add_translation_rows` (0x4eccd0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_translation_rows() {
    covered!("tel32_1 translation rows");
    tel32_blank_row();
    comp_lang_format()(JOB_TEXT, 0x9b1c40usize); // "Translation and Proof-Reading"
    tel32_heading();
    for &(role, name) in TEL32_TRANSLATION_ROLES.iter() {
        tel32_translated_role_row(role, name);
    }
}

/// Comparer for `credits_add_testing_rows`: side by side over the display list (`compare_match_screen_build`) plus the credits
/// build state (`tel32_credits`).
pub(crate) unsafe extern "C" fn compare_credits_add_testing_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    compare_match_screen_build("credits Testing rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE)], || o(), || credits_add_testing_rows());
}

/// Comparer for `credits_add_feral_rows`: side by side over the display list (`compare_match_screen_build`) plus the credits
/// build state (`tel32_credits`).
pub(crate) unsafe extern "C" fn compare_credits_add_feral_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_match_screen_build("credits Feral rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE)], || o(), || credits_add_feral_rows());
}

/// Comparer for `credits_add_eidos_rows`: side by side over the display list (`compare_match_screen_build`) plus the credits
/// build state (`tel32_credits`).
pub(crate) unsafe extern "C" fn compare_credits_add_eidos_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I2%%);
    compare_match_screen_build("credits Eidos rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE)], || o(), || credits_add_eidos_rows());
}

/// Comparer for `credits_add_translation_rows`: side by side over the display list (`compare_match_screen_build`) plus the credits
/// build state (`tel32_credits`).
pub(crate) unsafe extern "C" fn compare_credits_add_translation_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I3%%);
    compare_match_screen_build("credits Translation rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE)], || o(), || credits_add_translation_rows());
}
