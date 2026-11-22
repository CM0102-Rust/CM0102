// ---------- batch %%LEVA%%: the credits screen (contract_screens.cpp, part of it): opening it; the Sports Interactive, Jask/Xbox and Research rows ----------

/// The credits screen's build and input functions (`credits_screen_build`, `credits_screen_input`).
pub(crate) const CREDITS_SCREEN_BUILD: usize = 0x513180;
pub(crate) const CREDITS_SCREEN_INPUT: usize = 0x5133c0;
/// The second text buffer of the screens (a name or a role translated apart from `JOB_TEXT`) [inference].
pub(crate) const CREDITS_SECOND_TEXT: usize = 0xae1098;

/// Rule: opening the credits screen starts it on its first page.
///
/// `exe::screenmanager_open_screen` with `CREDITS_SCREEN_BUILD` and `CREDITS_SCREEN_INPUT` (flag 0, no leave or exit
/// function); when it opened, screen parameters 0, 1 and 2 are set to 0 (no copy).
/// Original: `credits_screen_open` (0x4e5920, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_screen_open() {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, CREDITS_SCREEN_BUILD, CREDITS_SCREEN_INPUT, 0, 0usize, 0) == 0 {
        return;
    }
    covered!("tel33_2 credits opened");
    for k in 0..3 {
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, k, 0usize, 0);
    }
}

/// Comparer for `credits_screen_open`: only the original runs (it opens a screen), then the plan is checked: the
/// current screen builds and answers as the credits screen and its parameters 0, 1 and 2 are 0.
pub(crate) unsafe extern "C" fn compare_credits_screen_open() {
    original_fn::<unsafe extern "C" fn()>(%%I0%%)();
    let node = current_screen_node(G_SCREEN_MANAGER);
    let ok = node != 0
        && read_usize(node + screen_node_fields::BUILD) == CREDITS_SCREEN_BUILD
        && read_usize(node + screen_node_fields::INPUT) == CREDITS_SCREEN_INPUT
        && (0..3).all(|k| exe::screenmanager_get_screen_param(G_SCREEN_MANAGER, k) == 0);
    SCOREBOARD.record(ok, || format!("credits_screen_open: the current screen is not the credits screen on its first page (node {:#x})", node));
}

/// Rule: a credits row with both texts given (a name or a role translated into one of the text buffers).
///
/// As `tel32_role_row`: the row counter goes up by 1; on the shown page, `name` from x 0x82 to 0x1b8 (0xae3184
/// colour) with the current item number, without taking it, and `role` from 0x1c2 to 0x2f8 (0xaebdde colour)
/// taking it.
unsafe fn credits_text_row(name: u32, role: u32) {
    tel32_bump_row(1);
    if tel32_shown() {
        tel32_cell(0x82, 0x1b8, read_u32(tel32_credits::ITEM), job_colour(JOB_COLOUR_3184), name);
        let i = tel32_next_item();
        tel32_cell(0x1c2, 0x2f8, i, job_colour(TEL32_COLOUR_BDDE), role);
    }
}

/// (role, name), exe texts, in the order of the original.
const SPORTS_INTERACTIVE_CHAIRMAN_ROWS: [(u32, u32); 2] = [
    (0x9b1234, 0x99b240), // Chairman: Oliver Collyer
    (0x9b1234, 0x99b170), // Chairman: Paul Collyer
];

/// (role, name), exe texts, in the order of the original.
const SPORTS_INTERACTIVE_DESIGN_ROWS: [(u32, u32); 7] = [
    (0x9b11d0, 0x99b3d4), // Design & Programming: Paul Norman
    (0x9b11d0, 0x99b074), // Design & Programming: Kevin Turner
    (0x9b11d0, 0x99adf0), // Design & Programming: Marc Vaughan
    (0x9b11d0, 0x99acbc), // Design & Programming: Svein Kvernoey
    (0x9b11d0, 0x99ab28), // Design & Programming: Keith Flannery
    (0x9b11d0, 0x9b11c0), // Design & Programming: Miles Dinneen
    (0x9b11d0, 0x9b11b4), // Design & Programming: Sam Deane
];

/// (role, name), exe texts, in the order of the original.
const SPORTS_INTERACTIVE_OTHER_ROWS: [(u32, u32); 6] = [
    (0x9b119c, 0x99aebc), // Research Co-ordination: Mark Woodger
    (0x9b119c, 0x9b118c), // Research Co-ordination: Peter Sottrel
    (0x9b1174, 0x99af88), // Internet Development: Marc Duffy
    (0x9b1164, 0x9b1154), // Sound Effects: Graham Miles
    (0x9b1138, 0x9b1124), // Manual & Written History: Alexander Deacon
    (0x9b1118, 0x9b1108), // QA Manager: Joe O'Rielly
];

/// (role, name), exe texts, in the order of the original.
const JASK_XBOX_ROWS: [(u32, u32); 1] = [
    (0x9b1260, 0x9b1254), // Xbox Design & Programming: Simon Page
];

/// (role, name), exe texts, in the order of the original.
const RESEARCH_ROWS_ARGENTINA_TO_JAPAN: [(u32, u32); 18] = [
    (0x9b1698, 0x9b1688), // Argentina: Daniel Szmid
    (0x9b167c, 0x9b166c), // Australia: David Bintley
    (0x9b1664, 0x9b1654), // Austria: Gregor Eigner
    (0x9b164c, 0x9b1638), // Belgium: Fabrice Leclercq
    (0x9b164c, 0x9b1628), // Belgium: Kristof Terreur
    (0x9b1620, 0x9b1610), // Brazil: Ndongo Bastos
    (0x9b1608, 0x9b15fc), // China: Frank Ng
    (0x9b15f4, 0x9b15e0), // Croatia: Tihomir Kovacevic
    (0x9b15d0, 0x9b15c0), // Czech Republic: Tony Grasser
    (0x9b15b8, 0x9b15a4), // Denmark: Jesper Krogsgaard
    (0x9b159c, 0x9b158c), // Finland: Jarkko Nyman
    (0x9b1584, 0x9b1570), // France: Emmanuel Rochard
    (0x9b1568, 0x9b1558), // Germany: Ronald Demohn
    (0x9b1550, 0x9b153c), // Greece: Spyros Giannoutsos
    (0x9b1534, 0x9b1524), // Holland: René Schipper
    (0x9b1534, 0x9b1514), // Holland: Martin Barelds
    (0x9b150c, 0x9b14f4), // Italy: Alberto Scotta 'Panoz'
    (0x9b14ec, 0x9b14dc), // Japan: Yoshimi Kano
];

/// (role, name), exe texts, in the order of the original.
const RESEARCH_ROWS_MEXICO_TO_YUGOSLAVIA: [(u32, u32); 21] = [
    (0x9b14bc, 0x9b14a8), // Mexico: Martin del Palacio
    (0x9b1494, 0x9b1484), // Northern Ireland: Gordon Smyth
    (0x9b147c, 0x9b146c), // Norway: Tore Ludvigsen
    (0x9b147c, 0x9b1458), // Norway: Vegard Rønningen
    (0x9b1450, 0x9b143c), // Poland: Krzysztof Scibiorek
    (0x9b1430, 0x9b1418), // Portugal: 'Zé' Guilherme Chieira
    (0x9b1404, 0x9b13f4), // Republic of Ireland: Darren Matthews
    (0x9b13ec, 0x9b13dc), // Russia: Alex Cieszynscy
    (0x9b13ec, 0x9b13c8), // Russia: Maxim Yevtushenko
    (0x9b13bc, 0x9b13ac), // Scotland: Stuart Milne
    (0x9b14d0, 0x9b1394), // South Korea: South Korea CM Fan Club
    (0x9b1384, 0x9b1374), // South Africa: Sean Riskowitz
    (0x9b136c, 0x9b1358), // Spain: Iván Abella Villar
    (0x9b136c, 0x9b133c), // Spain: Honorino Zamora, 'Nordin'
    (0x9b1334, 0x9b1328), // Sweden: Jan Juhlin
    (0x9b1334, 0x9b1310), // Sweden: Andreas Broström Ling
    (0x9b1304, 0x9b12f4), // Switzerland: Christian Guei
    (0x9b12ec, 0x9b12dc), // Turkey: Türker Tozar
    (0x9b12d4, 0x9b12c8), // U.S.A.: Tony Roche
    (0x9b12c0, 0x9b12b0), // Wales: Neil Vaughan
    (0x9b12a4, 0x9b1290), // Yugoslavia: Dusan Stamenkovic
];

/// Rule: the Sports Interactive section: the heading (copied, not translated, no empty row first) and the rows of
/// name and translated role.
///
/// Through the credits row helpers (`tel32_heading`, `tel32_translated_role_row`), the texts by address as the
/// original does: the two chairmen; the managing director (the role translated with the empty text as its argument,
/// the name "Miles"); seven design and programming rows; Grant Appleyard, whose name is translated into `JOB_TEXT`
/// and the role into `CREDITS_SECOND_TEXT` (`credits_text_row`); six more rows (research co-ordination, internet
/// development, sound effects, manual and written history, QA manager).
/// Original: `credits_add_sports_interactive_rows` (0x4e5970, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_sports_interactive_rows() {
    covered!("tel33_2 sports interactive rows");
    tel26_copy_text(JOB_TEXT as *mut u8, 0x9b1240 as *const u8); // "Sports Interactive"
    tel32_heading();
    for &(role, name) in SPORTS_INTERACTIVE_CHAIRMAN_ROWS.iter() {
        tel32_translated_role_row(role, name);
    }
    comp_lang_format()(JOB_TEXT, 0x9b11f0usize, TEL32_NO_TEXT as usize); // "Managing Director<%s - COMMENT - ..>"
    tel32_role_row(0x9b11e8); // Miles
    for &(role, name) in SPORTS_INTERACTIVE_DESIGN_ROWS.iter() {
        tel32_translated_role_row(role, name);
    }
    comp_lang_format()(JOB_TEXT, 0x99a960usize); // "Grant Appleyard"
    comp_lang_format()(CREDITS_SECOND_TEXT, 0x9b11d0usize); // "Design & Programming"
    credits_text_row(JOB_TEXT as u32, CREDITS_SECOND_TEXT as u32);
    for &(role, name) in SPORTS_INTERACTIVE_OTHER_ROWS.iter() {
        tel32_translated_role_row(role, name);
    }
}

/// Rule: the Jask Consultants section: an empty row, the "Jask Consultants" heading (translated) and the Xbox design
/// and programming row.
///
/// Through the credits row helpers (`tel32_blank_row`, `tel32_heading`, `tel32_translated_role_row`).
/// Original: `credits_add_jask_xbox_rows` (0x4e6a70, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_jask_xbox_rows() {
    covered!("tel33_2 jask xbox rows");
    tel32_blank_row();
    comp_lang_format()(JOB_TEXT, 0x9b127cusize); // "Jask Consultants"
    tel32_heading();
    for &(role, name) in JASK_XBOX_ROWS.iter() {
        tel32_translated_role_row(role, name);
    }
}

/// Rule: the Research section: an empty row, the "Research" heading (translated) and one row per researcher with
/// the nation (translated) as the role.
///
/// Through the credits row helpers; the nations from Argentina to Japan, then South Korea with the name translated
/// too (the nation into `JOB_TEXT`, "Doo-Jin Sa" into `CREDITS_SECOND_TEXT`, `credits_text_row` with the name on the
/// left), then Mexico to Yugoslavia (South Korea again, for the fan club, between Scotland and South Africa).
/// Original: `credits_add_research_rows` (0x4e6dc0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_research_rows() {
    covered!("tel33_2 research rows");
    tel32_blank_row();
    comp_lang_format()(JOB_TEXT, 0x9b16a4usize); // "Research"
    tel32_heading();
    for &(role, name) in RESEARCH_ROWS_ARGENTINA_TO_JAPAN.iter() {
        tel32_translated_role_row(role, name);
    }
    comp_lang_format()(JOB_TEXT, 0x9b14d0usize); // "South Korea"
    comp_lang_format()(CREDITS_SECOND_TEXT, 0x9b14c4usize); // "Doo-Jin Sa"
    credits_text_row(CREDITS_SECOND_TEXT as u32, JOB_TEXT as u32);
    for &(role, name) in RESEARCH_ROWS_MEXICO_TO_YUGOSLAVIA.iter() {
        tel32_translated_role_row(role, name);
    }
}

/// Comparer for `credits_add_sports_interactive_rows`: side by side over the display list (`compare_match_screen_build`)
/// plus the credits build state (`tel32_credits`) and the second text buffer.
pub(crate) unsafe extern "C" fn compare_credits_add_sports_interactive_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_match_screen_build("credits Sports Interactive rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE), (CREDITS_SECOND_TEXT, 0x100)],
        || o(), || credits_add_sports_interactive_rows());
}

/// Comparer for `credits_add_jask_xbox_rows`: side by side over the display list (`compare_match_screen_build`) plus the
/// credits build state (`tel32_credits`).
pub(crate) unsafe extern "C" fn compare_credits_add_jask_xbox_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I2%%);
    compare_match_screen_build("credits Jask/Xbox rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE)], || o(), || credits_add_jask_xbox_rows());
}

/// Comparer for `credits_add_research_rows`: side by side over the display list (`compare_match_screen_build`) plus the
/// credits build state (`tel32_credits`) and the second text buffer.
pub(crate) unsafe extern "C" fn compare_credits_add_research_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I3%%);
    compare_match_screen_build("credits Research rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE), (CREDITS_SECOND_TEXT, 0x100)],
        || o(), || credits_add_research_rows());
}
