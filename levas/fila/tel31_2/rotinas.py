# As estatísticas das competições (2): um recorde como texto (a tela de recordes) e como notícia (comp_screens.cpp;
# leitor tel31, rodada das telas, 21/11/2026; grupo novo da varredura).
# Usa o que o tel31_1 define (comp_stats_obj, COMP_STATS_CPP, ftol): entra depois dele; se o tel31_1 sair, este sai.
# Convenções conferidas na desmontagem: format_record_text thiscall(stats, records, kind byte, Text*) ret 0xc;
# write_record_to_news cdecl(first, records, kind byte, NewsItem*). Chamadas: 0x66f4e0 langlib_sprintf cdecl variádica
# (os `{}` pegam um int: o gênero do nome do clube, club::SHORT_GENDER_NAME); 0x54a650 Date::get_short_date_string
# thiscall(date, buf, 0x14, 0xff) ret 0xc; 0x6697c0 key_nation_format_comp_season cdecl(year, comp id, buf, 0x14);
# 0x7e4ca0 format_venue_marker cdecl(venue, buf, 5); 0x4bffc0 stage name cdecl(prefix, name, buf, 0xc8, 1); 0x53e140
# format_name_from_ids cdecl (7); 0x53e990 staff_matches_names cdecl (5); 0x540340 years_between_dates cdecl(end,
# start, buf, 5); 0x4a4850 find_club_entry thiscall ret 4; 0x4a7f20 Date::operator== thiscall(date, lo, hi) ret 8;
# 0x729500 fixture_entry_matches thiscall (5) ret 0x14; Text::clear 0x4afa40, set_string 0x4ae660 ret 4, add_link
# 0x4ae8a0 ret 0xc; NewsItem::set_slot 0x785790 thiscall(slot byte, value) ret 8; 0x7e4de0/0x7e5160/0x7e5420 os
# escritores de partida, nome e período cdecl(record, news, &slot).
# Rotinas inteiras iguais no cm0102-1080.exe. format_record_text com texto ou registro nulo ou tipo fora de 1..22
# chama a original (caixa de erro).
# Defeito do original: no tipo 9 (o resultado do registro em +0x75) o lado do mando é lido em +0xa4, o byte do
# registro seguinte (tipo 10), e não em +0x85; a cópia mantém (DEFEITOS_DO_ORIGINAL).
TELAS = True
ROTINAS = [('comp_stats_format_record_text', 0x4a5690, [0x81, 0xec, 0xbc, 0x05, 0x00, 0x00]),
           ('comp_stats_write_record_to_news', 0x4ad9c0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
DOCS = ('as estatísticas das competições (2): um recorde como texto e como notícia',
        'the competition statistics (2): a record as text and as news',
        [('comp_stats_format_record_text', 'comp_stats_format_record_text', '0x4a5690', 'comp_screens.cpp', 'thiscall',
          'escreve um recorde da competição numa linha de texto com ligações: "-" quando vazio; senão a frase do tipo (recordes de time, públicos, resultados, sequências, recordes de jogador), com o clube, o adversário, a fase, a partida e a pessoa ligados; as sequências invicta e sem vitória terminam em "?" enquanto continuam',
          'writes a competition record as one line of text with links: "-" when empty; otherwise the kind\'s sentence (team records, attendances, results, runs, player records), with the club, the opponent, the stage, the match and the person linked; the unbeaten and winless runs end in "?" while still going'),
         ('comp_stats_write_record_to_news', 'comp_stats_write_record_to_news', '0x4ad9c0', 'comp_screens.cpp', 'cdecl',
          'põe um recorde nas casas de uma notícia: o primeiro valor, o tipo e os campos do tipo em ordem (contagens, clubes, anos, partidas, períodos e nomes)',
          'puts a record into a news item\'s slots: the first value, the kind and the kind\'s fields in order (counts, clubs, years, matches, date spans and names)')])
