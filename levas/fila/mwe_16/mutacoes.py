# Mutations (mwe_16): each cup turnover two years ahead (the year of the new edition, and with it its dates, change
# at the turnover of each cup inside the career).
MUTACOES = [
    ('mwe16_copa_finlandesa_dois_anos',
     'start_new_cup_season(Cup(this), 0x592a20, 0x9c8158, [0x206, 0x20d, 0x214], 1, "mwe_16 finnish cup turnover")',
     'start_new_cup_season(Cup(this), 0x592a20, 0x9c8158, [0x206, 0x20d, 0x214], 2, "mwe_16 finnish cup turnover") /* mutation: two years ahead */',
     'mwe_16 finnish cup turnover'),
    ('mwe16_copa_norueguesa_dois_anos',
     'start_new_cup_season(Cup(this), 0x78f890, 0xa69708, [0x1e8, 0x1ef, 0x1f6], 1, "mwe_16 norwegian cup turnover")',
     'start_new_cup_season(Cup(this), 0x78f890, 0xa69708, [0x1e8, 0x1ef, 0x1f6], 2, "mwe_16 norwegian cup turnover") /* mutation: two years ahead */',
     'mwe_16 norwegian cup turnover'),
    ('mwe16_copa_portuguesa_dois_anos',
     'start_new_cup_season(Cup(this), 0x7ce220, 0xa786f0, [0x20d, 0x214, 0x21b], 1, "mwe_16 portuguese cup turnover")',
     'start_new_cup_season(Cup(this), 0x7ce220, 0xa786f0, [0x20d, 0x214, 0x21b], 2, "mwe_16 portuguese cup turnover") /* mutation: two years ahead */',
     'mwe_16 portuguese cup turnover'),
]
