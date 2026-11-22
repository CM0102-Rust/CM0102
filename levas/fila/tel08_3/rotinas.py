# The staff profile's Transfer tab (staff_screens.cpp, part 3; reader tel08b, the screens round, 21/11/2026).
# - 0x87dd80 staff_screens_draw_transfer_tab (31275 bytes, 0x87dd80..0x8857aa): cdecl(int top, int bottom), ret 0, SEH
#   frame (an LLIST, 0x2194 bytes of locals). Called by the staff profile builder (0x870700) for its Transfer tab. The
#   queue list (tel08) gave it 10884 bytes: the map's "pieces" 0x880804 and 0x884ea2 (list tel09) are its middle and
#   end, there is no prologue at either. Split into named pieces: transfer_tab_details, _offers (_offer_list),
#   _interested, _future, the mood comments (_mood and its 13 parts: _interest, _favoured_team_mate, _sales, _squad,
#   _league, _listing, _pay, _dislikes, _manager, _contract, _grievances, _selection, _personal, _outlook) and
#   _potential_fees; the whole routine is hooked.
# Conventions checked at the call sites: 0x804160 add_area_text_by_ref thiscall ret 0x2c (area, column, &row, style,
#   colour, text, 0x209, "  "; it only reads the row); 0x569e30 DISPUTE::is_in_dispute thiscall ret 8 (al);
#   0x56a030 get_dispute_club_id thiscall ret 4; 0x776380 player_dislikes_national_team thiscall ret 4; 0x450aa0
#   Cash::format_display_amount thiscall ret 0xc (0x450a90 Cash::Cash only stores the int); 0x54a650
#   get_short_date_string thiscall ret 0xc; 0x4567a0 set_area_scroll_row thiscall ret 8; 0x7c6830
#   get_language_fluency (p, 0, language, 1); 0x7c1e30 PlayerStats::get_stat (player, 6, 1 or 0x11, 6.5f), stored as a
#   float; 0x842180 collect_interested_clubs (p, &list, 1, 1, 0) on [[0xde1f54] + 0x856]; 0x4cc9e0
#   find_contract_offer (person id, club id); 0x4ca470 get_deserved_squad_status (p, contract); cdecl 0x544520 (p, 0 or
#   the club), 0x5444a0, 0x544590, 0x544600, 0x547790/0x5477d0/0x547d30/0x547d60 (p, club), 0x547810 (manager, p),
#   0x544410, 0x5480f0 (at most 3), 0x6181d0 (nation, out, 0xc8, 0x20). The trivial getters 0x5d5a30/40/50 and
#   0x8857b0/c0/d0/e0/f0 are read as the contract fields (happiness 0x3d, 0x3e, grievances 0x45, morale 0x44, matches
#   left out 0x3f, squad status >> 4, Bosman 0x49, transfer status & 8); 0x885800 (board confidence) as the board
#   record's word. Already in Rust and called directly: 0x53cee0, 0x549dd0, 0x5482f0, 0x548da0, 0x7357c0, 0x8c3e50,
#   0x8d0dd0, 0x8d0ea0, 0x8d0f00.
# Uses the helpers of tel26_1 (tel26_add_area, tel26_error_box, tel26_slot_manager, job_colour, JOB_TEXT) and tel08_1
#   (staff_tab_*, contract_tab_lookup): it goes after both (and after tel08_2, which also follows them).
# Kept from the original: three texts written and then overwritten without being shown (the supporters he cares
#   about, the next manager, the enthusiasm after the development line); the club checks of the mood comments (a
#   contract always has a club); the error box when a future transfer has no offer slot (line 0x1902); stack garbage
#   in the high halves of the colour words (zero here) and of the rows dword (Display::add_area reads a byte).
# Comparer: side by side by the display list (compare_match_screen_build: the screen's parameter block, so also the
#   area index in parameter 0xc) plus G_CONTRACT_LOOKUP_RESULT. Nothing else is written to the game: the LLIST of
#   interested clubs is local to each side; a future transfer whose offer slot is missing would show the error box
#   twice (only a broken save).
# How to reach it: a person's profile (click a name) > Transfer tab. Paths: a player of the human's club (Value button,
#   the offers with buttons 0x13, the mood comments, Potential Fees after a sale with clauses), a player of another
#   club (offers seen only when shortlisted or known to him, Interested, the trial offer), a player with a transfer
#   arranged (future transfer line), a retired or retiring player, a player with nickname or common name (Full
#   Name), a player with a second nationality, a manager or chairman (no transfer information).
TELAS = True
ROTINAS = [('staff_screens_draw_transfer_tab', 0x87dd80, [0x6a, 0xff, 0x64, 0xa1, 0x00, 0x00, 0x00])]
DOCS = ('a aba Transferência do perfil da pessoa',
        "the staff profile's Transfer tab",
        [('staff_screens_draw_transfer_tab', 'staff_screens_draw_transfer_tab', '0x87dd80', 'staff_screens.cpp', 'cdecl, SEH',
          'Desenha a aba Transferência do perfil (a pessoa do parâmetro 0) entre top e bottom: sem informação ("There is no transfer information available.") para cargo de clube 1 ou 2 ou id a partir da contagem de pessoas − 16; senão a tabela de duas colunas: nome completo (com nome comum ou apelido do humano), disponibilidade (listado pelo clube, a pedido, para empréstimo ou desconhecida), valor (cargo de jogador sem contrato de experiência; botão 5 quando o humano controla o clube), segunda nacionalidade (botão para a seleção), línguas fluentes (fluência ≥ 8) e básicas (4 a 7); Ofertas: nenhuma sem propostas, a transferência futura acertada (clube e data), as propostas da lista do parâmetro 3 que o espectador pode ver (do clube humano, do clube que propõe, conhecidas do jogador ou de jogador observado: valor, troca, empréstimo, transferência livre; botões 0x13, 0xe ou 0x7d5) e a proposta de experiência do clube do espectador; Interessados (os clubes do gerente de listas); Futuro: a proposta de contrato em estudo, aposentadoria da seleção, aposentadoria (aposentado, sem futuro, se aposenta no fim da temporada), sem contrato "-", senão os comentários do humor pelas queixas e marcas do contrato, o caráter (ambição, lealdade, pressão, profissionalismo, temperamento), o clube e a forma na liga, o técnico e a diretoria, cada um uma linha quebrada, e "Unknown" quando nada foi escrito; Taxas potenciais (porcentagem da próxima venda e cláusulas de jogos na liga e na seleção); linhas vazias até o fim, a rolagem do parâmetro 0xb e o índice da área no parâmetro 0xc; cada consulta ao contrato, menos a primeira, fica em G_CONTRACT_LOOKUP_RESULT',
          'Draws the profile\'s Transfer tab (the person of parameter 0) between top and bottom: no information ("There is no transfer information available.") for club job 1 or 2 or an id from the person count - 16 up; else the two-column table: full name (with a common name or the human\'s nickname), availability (listed by the club, by request, for loan or unknown), value (a player job without a trial contract; button 5 when the human controls the club), second nationality (a button to the national team), fluent (fluency >= 8) and basic (4 to 7) languages; Offers: none without bids, the arranged future transfer (club and date), the offers of the list in parameter 3 the viewer may see (the human\'s club, the bidding club, known to the player or a shortlisted player: value, part-exchange, loan, free transfer; buttons 0x13, 0xe or 0x7d5) and the viewer club\'s trial offer; Interested (the shortlist manager\'s clubs); Future: the contract offer being considered, international retirement, retirement (retired, no future, retiring at the end of the season), without a contract "-", else the mood comments by the contract\'s grievances and flags, the character (ambition, loyalty, pressure, professionalism, temperament), the club and its league form, the manager and the board, each a wrapped line, and "Unknown" when nothing was written; Potential Fees (the next sale percentage and the league and international appearance clauses); empty rows to the end, the scroll of parameter 0xb and the area index in parameter 0xc; each contract lookup but the first is left in G_CONTRACT_LOOKUP_RESULT')])
