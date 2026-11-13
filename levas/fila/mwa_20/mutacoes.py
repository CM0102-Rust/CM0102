# Mutacoes (mwa_20), nas tabelas de jogos da Irlanda do Norte, carreira ENG IRL NIR ate 2002-08-01:
# - a primeira rodada uma semana depois (12/8 -> 19/8: duas rodadas no mesmo dia);
# - o caso do Natal deslocado (k + 2: outras datas de Natal);
# - a repescagem uma semana depois (5/5 -> 12/5).
MUTACOES = [
    ('mwa20_primeira_rodada_semana_depois',
     '(12, 7, 0, 5, 1, &[(-1, -1, -1)]), // mwa20: the first Northern Irish matchday, 12 August',
     '(19, 7, 0, 5, 1, &[(-1, -1, -1)]), // mutation: a week later',
     'mwa_20 northern irish premier schedule'),
    ('mwa20_natal_deslocado',
     'let k = (read_u16(t + 19 * 0x41).wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(0xd); // mwa20: the Christmas case',
     'let k = (read_u16(t + 19 * 0x41).wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(0xb); // mutation: another case',
     'mwa_20 northern irish premier schedule'),
    ('mwa20_repescagem_semana_depois',
     'cql10r((0x1d, 3, 1, 0), (5, 4, 1, 3, 1, 0), 0xa0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // mwa20: the play-off on 5 May',
     'cql10r((0x1d, 3, 1, 0), (0xc, 4, 1, 3, 1, 0), 0xa0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // mutation: a week later',
     'mwa_20 northern irish play-off round'),
]
