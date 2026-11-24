# Mutacoes (cx17_3), ENG ate 2005-12-31: as de texto sao acusadas pela comparacao (compare_text_writer), nao pela
# carreira (o texto nao volta a simulacao):
# - a classificada sem a frase inteira com os vinculos na ordem trocada (o do clube primeiro);
# - a selecao da repescagem com a linha de outra formula (0xad4658, "Qualified For World Cup");
# - os potes do sorteio da Copa de 2006 em ordem inversa (a semeadura de 15/12/2005): o comparador lado a lado e a
#   carreira (o sorteio muda) acusam.
MUTACOES = [
    ('cx17_3_vinculos_trocados',
     'exe::text_add_link(text, (world_cup + competition::SHORT_NAME) as *const u8, 0x7d0, world_cup); // cx17_3: the World Cup\'s link first\n            exe::text_add_link(text, club_name as *const u8, 0x7d5, club);',
     'exe::text_add_link(text, club_name as *const u8, 0x7d5, club); // mutation: the club\'s link first\n            exe::text_add_link(text, (world_cup + competition::SHORT_NAME) as *const u8, 0x7d0, world_cup);',
     'cx17_3 south american qualified'),
    ('cx17_3_repescagem_outra_linha',
     'sprintf(0xde1f64, 0xad4d2c, club_gender, club_gender, club_name);',
     'sprintf(0xde1f64, 0xad4658, club_gender, club_gender, club_name); // mutation: another line',
     'cx17_3 south american to the play-off'),
    ('cx17_3_potes_invertidos',
     'read_i8(a + 4) as i32 - read_i8(b + 4) as i32 // cx17_3: the pot byte',
     'read_i8(b + 4) as i32 - read_i8(a + 4) as i32 // mutation: the pots reversed',
     'cx17_3 pot order'),
]
