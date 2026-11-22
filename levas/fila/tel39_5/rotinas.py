# As copias de recorde e a comparacao de datas do comp_screens.cpp (leitor tel39, rodada 2 das telas, 22/11/2026).
# Convencoes conferidas na desmontagem e nos pontos de chamada:
# - 0x4a1fb0 player_age_record_assign: thiscall(this, origem), ret 4, devolve this; copia os 0x3f bytes: a data em +0
#   e em +0x18 pelo Date::operator= (0x417c10, retorno por valor num local), 4 dwords em +8..+0x14 e o registro de
#   partida em +0x20 em linha (igual ao match_record_assign 0x448110, ja Rust). Chamada pelo
#   Club_Records::end_season_for_nation (0x447f25, 0x447f38), pelo Club_Records::build_history_list (0x451031...),
#   pelo Comp_Stats::archive_season_and_reset (0x4a1e43, 0x4a1e56) e pelo Comp_Stats::build_season_records_list
#   (0x4ab2e6...): roda na carreira no fim da temporada.
# - 0x4ab5b0 attendance_record_assign: thiscall(this, origem), ret 4, devolve this: o dword +0 e o
#   match_record_assign (0x448110) em +4. Chamada pelo Club_Records::build_history_list (0x450ecf) e pelo
#   Comp_Stats::build_season_records_list (0x4ab401, 0x4ab40d).
# - 0x4a7f20 Date::operator==: thiscall(this, Date por valor: 8 bytes na pilha), ret 8, eax 0/1; compara o ano (+2)
#   e depois o dia (+0); o terceiro campo nao. Chamada pelas telas das competicoes (0x4a6a48...), pelos contratos
#   (0x4cda0e, 0x4dd282), pelas noticias e pela midia (0x76b9b0, 0x77a6ec, 0x79539a, 0x7af213) e outras: roda na
#   carreira.
# Comparadores: as copias lado a lado sobre o registro e o retorno; a comparacao lado a lado.
# Pode nao rodar ate 2002-06-01: o build_season_records_list e o build_history_list sao das telas de historico;
#   a 0x4ab5b0 talvez so rode nelas (a mutacao dela pode escapar da carreira).
ROTINAS = [('player_age_record_assign', 0x4a1fb0, [0x83, 0xec, 0x08, 0x8d, 0x44, 0x24, 0x00]),
           ('attendance_record_assign', 0x4ab5b0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('date_equals', 0x4a7f20, [0x66, 0x8b, 0x44, 0x24, 0x06])]
DOCS = ('as cópias de recorde e a comparação de datas do comp_screens.cpp: o recorde de jogador mais novo ou mais velho, o recorde de público e Date::operator==',
        "comp_screens.cpp's record copies and date comparison: the youngest or oldest player record, the attendance record and Date::operator==",
        [('player_age_record_assign', 'player_age_record_assign', '0x4a1fb0', 'comp_screens.cpp', 'thiscall',
          'copia um recorde de jogador mais novo ou mais velho (0x3f bytes): a idade como data, o recordista, a data de nascimento e a partida',
          "copies a youngest or oldest player record (0x3f bytes): the age as a date, the holder, his birth date and the match"),
         ('attendance_record_assign', 'attendance_record_assign', '0x4ab5b0', 'comp_screens.cpp', 'thiscall',
          'copia um recorde de público (o maior e o menor): o público e o registro da partida',
          'copies an attendance record (the highest and the lowest): the attendance and the match record'),
         ('date_equals', 'Date::operator==', '0x4a7f20', 'comp_screens.cpp', 'thiscall',
          'duas datas do jogo são iguais quando o ano e o dia do ano batem (o campo do ano bissexto não entra)',
          'two game dates are equal when the year and the day of the year match (the leap-year field is not compared)')])
