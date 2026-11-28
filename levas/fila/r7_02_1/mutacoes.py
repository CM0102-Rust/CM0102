# Mutações (r7_02_1), na carga do banco na criação do jogo: cada uma muda o banco que a prova do save grava no .qst
# (a criação do zero carrega o banco uma vez; com e sem a rotina). No jogo real comparando, o comparador da carga as
# acusa também.
# - as últimas seleções (uma por nação) com um terço da reputação da seleção da nação, não a metade (213 clubes);
# - a divisão "Reserve" (club_comp 87, divisão de 10 clubes no club.dat) não é tirada dos clubes;
# - as 127 competições de reserva com reputação 1 (vão para o .qst depois das competições de nação);
# - as pessoas livres e as de reserva nascidas em 1972 (0x7b4), não 1971 (as linhas vão para o .qst).
MUTACOES = [
    ('r702_selecoes_um_terco',
     '        write_i16(c + club::REPUTATION, (read_i16(team + club::REPUTATION) as i32 / 2) as i16);',
     '        write_i16(c + club::REPUTATION, (read_i16(team + club::REPUTATION) as i32 / 3) as i16); // mutation',
     'r7_02_1 load database'),
    ('r702_reserva_mantida',
     '        if division != 0 && load_db_same_text(division + competition::NAME, LOAD_DB_RESERVE_TEXT) {',
     '        if division != 0 && load_db_same_text(division + competition::NAME, LOAD_DB_RESERVE_TEXT) && false { // mutation',
     'r7_02_1 reserve division cleared'),
    ('r702_reserva_reputacao_1',
     '        write_u16(c + competition::REPUTATION, 0); // r7_02_1: reserve competition reputation',
     '        write_u16(c + competition::REPUTATION, 1); // mutation',
     'r7_02_1 linked layout'),
    ('r702_livres_1972',
     '    write_u16(p + staff::YEAR_OF_BIRTH, 0x7b3); // r7_02_1: born 1971',
     '    write_u16(p + staff::YEAR_OF_BIRTH, 0x7b4); // mutation',
     'r7_02_1 free people rows'),
]
