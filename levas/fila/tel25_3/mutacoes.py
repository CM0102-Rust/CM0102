# Mutations (tel25_3). The automatic career opens no screen: the chain runs none; these prove the comparers in a
# session of Lucas's (JOGAR_COMPARANDO.bat; the competition screen's Print popup, and a club's transfers printout):
# - the competition printout's period starts a year later (caught by its plan check);
# - the transfers sorted latest first (caught by the date order comparer).
MUTACOES = [
    ('tel25_3_period_year', 'return (day as u32, year);\n    }\n    (0, year)', 'return (day as u32, year + 1);\n    }\n    (0, year)'),
    ('tel25_3_date_order', '        covered!("tel25_3 transfer earlier");\n        return -1;', '        covered!("tel25_3 transfer earlier");\n        return 1;'),
]
