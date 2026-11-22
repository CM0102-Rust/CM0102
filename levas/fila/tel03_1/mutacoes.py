# Mutations (tel03_1): the screens batch runs no mutations in the chain; these are for a session of Lucas with
# JOGAR_COMPARANDO.bat (caught by the routines' comparers on first use):
# - the current month block 8 bytes further on (compare_value!: the club Info tab, income view);
# - the widget's own id ignored: every click answers with the injected id (compare_value! on any screen click).
MUTACOES = [
    ('tel03_1_mes_corrente',
     '    this + finance::MONTH_PLAYERS_BOUGHT // tel03_1 the current month block',
     '    this + finance::MONTH_PLAYERS_BOUGHT + 8 // tel03_1 mutation',
     'tel03_1 cash set'),
    ('tel03_1_id_injetado',
     "    let id = if widget == -1 { 0 } else { // tel03_1 the widget's own id",
     "    let id = if widget == -1 || true { 0 } else { // tel03_1 mutation",
     'tel03_1 cash set'),
]
