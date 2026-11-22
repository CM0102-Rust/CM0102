# Mutations (tel03_3): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat (open a club's History twice, so the lists and the record array are already kept in the
# params and the tabs run side by side):
# - the Attendances default type 3 -> 4: another heading and other rows when the stored type does not apply (caught by
#   the tab's display-list comparer);
# - up to 7 years a row in the honours: other rows for a club with 8 or more titles of one competition (Competitions >
#   View > Honours).
MUTACOES = [
    ('tel03_3_publico_tipo',
     '    history_season_tab(top, bottom, 5, 0xa, 0xb, 0xc, HistoryPeriod::Fixed(1), HistoryDefault::Type(3)); // tel03_3 the attendances default type',
     '    history_season_tab(top, bottom, 5, 0xa, 0xb, 0xc, HistoryPeriod::Fixed(1), HistoryDefault::Type(4)); // tel03_3 mutation',
     'tel03_3 competitions tab'),
    ('tel03_3_anos_por_linha',
     '            if years < 8 { // tel03_3 up to 8 years a row',
     '            if years < 7 { // tel03_3 mutation',
     'tel03_3 honours years on a new row'),
]
