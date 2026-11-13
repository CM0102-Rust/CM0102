# Mutations (mwe_9): each First Division turnover two years ahead (the Swedish, Norwegian and Finnish seasons turn
# over at the end of 2001: the year and the 2002 schedule from April change).
MUTACOES = [
    ('mwe9_superettan_dois_anos',
     'if !rebuild_season(this, 0x88ef90, 0xa8b53c, [0x1aa, 0x1b1, 0x1b7, 0x1be], 1) { // mwe9: the Superettan next year',
     'if !rebuild_season(this, 0x88ef90, 0xa8b53c, [0x1aa, 0x1b1, 0x1b7, 0x1be], 2) { // mutation: two years ahead',
     'mwe_9 superettan turnover'),
    ('mwe9_primeira_norueguesa_dois_anos',
     'if !rebuild_season(this, 0x7909a0, 0xa69740, [0x2c9, 0x2d0, 0x2d6, 0x2dd], 1) { // mwe9: the Norwegian First Division next year',
     'if !rebuild_season(this, 0x7909a0, 0xa69740, [0x2c9, 0x2d0, 0x2d6, 0x2dd], 2) { // mutation: two years ahead',
     'mwe_9 norwegian first division turnover'),
    ('mwe9_ykkonen_dois_anos',
     'this.set_year(this.year().wrapping_add(1)); // mwe9: the Ykkonen next year',
     'this.set_year(this.year().wrapping_add(2)); // mutation: two years ahead',
     'mwe_9 ykkonen turnover'),
]
