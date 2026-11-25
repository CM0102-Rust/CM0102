# The award shortlist news text (award_manager.cpp; reader rxf, round 6, 25/11/2026). Simulation (news text writer).
# - 0x414420 AWARD_MANAGER::write_shortlist_news(news, out, body): thiscall, ret 0xc (SEH frame only for a local Date
#   built by Date::Date() 0x54a110 and never read: not reproduced). Called by NewsItem::write_text (0x77bd5b, news
#   type 0x1c, with [0xad9c60] = the award manager; gra_1 calls it by address). The map calls the third argument
#   "is_vote_invite": it is the body flag (0 headline, 1 body), as in the other writers of that switch.
# - Calls: NewsItem::get_slot 0x785830 thiscall(k) ret 4 (slots 0..4, then 5..9 when slot 4 is set); langlib_sprintf
#   into 0xde1f64 (JOB_TEXT); Text::set_string 0x4ae660, append_string 0x4ae770, add_link 0x4ae8a0 (ret 0xc);
#   Date::Date(day, year) 0x549ef0 ret 8; AWARD_SHORTLIST::get_vote_status 0x418140 thiscall(manager) ret 4 (ax);
#   fill_staff_name_ref 0x7e4920 cdecl(staff, out[4]: the three name ids, then the id); News::set_item_slot 0x786d30
#   thiscall on [G_NEWS](manager, item id, slot, value, 0) ret 0x14; database_format_name_from_ids 0x53e140 cdecl(7);
#   database_staff_matches_names 0x53e990 cdecl(5).
# Same bytes in cm0102-1080. Comparer: compare_text_writer (the vote written into the stored item gets the same values
# from the original and from the copy).
# Doubt for the proof: whether the ENG career up to 2002-06-01 writes the text of a shortlist news (type 0x1c, the
# England year awards' shortlists in the spring); the mutation is caught by the comparer only when it runs.
SAVE = False
ROTINAS = [('write_shortlist_news', 0x414420, [0x6a, 0xff, 0x68, 0x6b, 0x3d, 0x95, 0x00])]
DOCS = ('o texto da notícia da lista de indicados a um prêmio',
        'the award shortlist news text',
        [('write_shortlist_news', 'AWARD_MANAGER::write_shortlist_news', '0x414420', 'award_manager.cpp', 'thiscall, ret 0xc',
          'a manchete "<prêmio> shortlist decided" ou o corpo "You have been invited to vote in this year\'s <prêmio> award."; quando o técnico humano da vez já votou nessa lista (anunciada na data da notícia), o voto é gravado na notícia guardada dele e o corpo acrescenta "You voted for <clube> <jogador>." com os vínculos; sem voto, os dois vínculos do prêmio',
          'the headline "<award> shortlist decided" or the body "You have been invited to vote in this year\'s <award> award."; when the current human manager has already voted in that shortlist (announced on the news date), the vote is written into his stored news item and the body adds "You voted for <club> <player>." with the links; without a vote, the award\'s two links')])
