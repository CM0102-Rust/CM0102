# Mutacoes (rx09_3), com a Espanha e a Suecia ate 2002-09-01 e a base NORMAL:
# - todo grupo de 19 ou mais com os lugares do grupo curto (rebaixa 3): os grupos 2 a 4 rebaixam um a menos no fim de
#   maio de 2002 e a virada muda;
# - dois sobem de cada grupo do acesso: a virada de 2002 muda (quatro clubes a mais na Segunda);
# - a repescagem do rebaixamento com os 15os em vez dos 16os: outros clubes jogam e caem (a carreira e o plano acusam).
MUTACOES = [
    ('rx09_3_grupo_curto_sempre',
     'let short = count == 0x13; // rx09_3: a group of 19 relegates three',
     'let short = count >= 0x13; // mutation: every group relegates three',
     'rx09_3 segunda b group stage'),
    ('rx09_3_dois_sobem',
     '&[1, 0, 0, 0], stage as u32, 0x14, 0x22); // rx09_3: one goes up from each play-off group',
     '&[2, 0, 0, 0], stage as u32, 0x14, 0x22); // mutation: two go up',
     'rx09_3 segunda b promotion play-off group'),
    ('rx09_3_repescagem_dos_15os',
     '*slot = aus_nsl_table_row_club(segunda_b_group(l, g as u8), 15); // rx09_3: the sixteenth of each group',
     '*slot = aus_nsl_table_row_club(segunda_b_group(l, g as u8), 14); // mutation: the fifteenth',
     'rx09_3 segunda b relegation play-off'),
]
