# The staff profile's Contract tab (staff_screens.cpp, part 2; reader tel08, the screens round, 21/11/2026).
# - 0x87cd90 staff_screens_draw_contract_tab (4007 bytes, 0x87cd90..0x87dd36): cdecl(int top, int bottom), ret 0, SEH
#   frame (a Text). Called by the staff profile builder (0x870700) for its Contract tab.
# Conventions checked at the call sites: 0x55fba0 Display::add_area thiscall ret 0x2c (the rows read as a byte);
#   0x804100 add_area_text ret 0x24; 0x8040a0 add_area_button ret 0x2c; 0x5ed070 gui_add_text_area cdecl;
#   0x4aeaf0 Text::draw_wrapped thiscall ret 0x20; 0x4ae590/0x4ae630 Text ctor/dtor; 0x4ae660 set_string ret 4;
#   0x4ae8a0 add_link ret 0xc; 0x4cf5a0/0x4cf6d0 get_contract/get_loan_contract thiscall ret 4; 0x85e750
#   format_wage_text thiscall ret 0xc; 0x85e8f0 format_end_date_text thiscall ret 4; 0x85f830 format_bonus_text
#   (uses ecx = the contract) ret 0x10; 0x85f940 format_release_clause_name ret 0x10 (ecx = the contract, unused);
#   0x84b330 squad_status_text ret 0xc (ecx = [[0xde1f54] + 0x856]); 0x54a650 get_short_date_string thiscall ret
#   0xc; 0x54ba20 get_long_date_string ret 0xc; 0x868210 check_fifa_protected_period thiscall ret 0xc; 0x8c7260
#   find_work_permit thiscall ret 4; 0x8c7560 player_needs_work_permit thiscall ret 0xc; 0x795750 find_note ret 8;
#   0x7957b0 format_note_text ret 0xc; 0x43f460 Cash::get_display_amount thiscall ret 4; cdecl: 0x441c10,
#   0x600c90, 0x53b620, 0x66f4d0, 0x66f4e0, 0x944c9f, 0x9466d0 (strncpy, pure Rust here), 0x5e5cd0.
# Uses the helpers of tel26_1 (tel26_area_text, tel26_area_button, tel26_add_area, tel26_text_area, job_colour,
#   JOB_TEXT, tel26_slot_manager): it goes after it. tel08_2 uses the helpers of this item (staff_tab_*).
# Defects of the original (kept, docs/DEFEITOS_DO_ORIGINAL.md candidates): every clause line shows the same amount
#   (contract::RELEASE_FEE); Loan Details and Work Permit read the club contract without a null check.
# Comparer: side by side by the display list (compare_match_screen_build) plus G_CONTRACT_LOOKUP_RESULT.
# How to reach it: a person's profile (click a name) > the Contract tab; paths: a player of the human's club
#   (Squad Status button), a loan (Loan Details and the two restrictions), bonuses and clauses, a work permit, a note
#   (Notes button), a manager or chairman (no contract information).
TELAS = True
ROTINAS = [('staff_screens_draw_contract_tab', 0x87cd90, [0x6a, 0xff, 0x68, 0xf6, 0x2e, 0x96, 0x00])]
DOCS = ('a aba Contrato do perfil da pessoa',
        "the staff profile's Contract tab",
        [('staff_screens_draw_contract_tab', 'staff_screens_draw_contract_tab', '0x87cd90', 'staff_screens.cpp', 'cdecl, SEH',
          'Desenha a aba Contrato do perfil (a pessoa do parâmetro 0) entre as linhas top e bottom: sem informação ("There is no contract information available.") para presidente (cargo 1), cargo 2, cargo de seleção 5 sem clube ou com cargo de clube diferente de 11, ou id a partir da contagem de pessoas − 16; senão a tabela de duas colunas: tipo de contrato (parcial/integral; mensal, normal, experiência, empréstimo), salário, para jogador de contrato normal com as regras da FIFA valendo o início (a data se depois da mudança de regras, senão "assinado antes do novo sistema" ou N/A) e a proteção (sim, ou os anos que faltam), vencimento, situação no elenco (botão quando o humano controla o clube; não em contrato de experiência), detalhes do empréstimo (clube e data, e as restrições de copa e de jogar contra o dono), bônus (os três ≥ 0 ou nenhum), cláusulas (cada cláusula marcada com a multa rescisória em dinheiro, vermelha para a marca 2, ou nenhuma), visto de trabalho (jogos jogados e perdidos e a porcentagem), a nota do humano (botão 0x2c, até 499 caracteres) e linhas vazias até o fim; cada consulta ao contrato fica em G_CONTRACT_LOOKUP_RESULT',
          "Draws the profile's Contract tab (the person of parameter 0) between rows top and bottom: no information (\"There is no contract information available.\") for a chairman (job 1), job 2, national job 5 without a club or with a club job other than 11, or an id from the person count - 16 up; else the two-column table: contract type (part/full time; monthly, normal, trial, loan), wages, for a player on a normal contract with the FIFA rules in force the start (the date when after the rule change, else \"signed before the new system\" or N/A) and the protection (yes, or the years left), expiry, squad status (a button when the human controls the club; not on a trial contract), loan details (club and date, and the cup and parent-club restrictions), bonuses (the three >= 0 or none), clauses (each set clause with the release fee as money, red for mark 2, or none), work permit (games played and missed and the share), the human's note (button 0x2c, up to 499 characters) and empty rows to the end; each contract lookup is left in G_CONTRACT_LOOKUP_RESULT")])
