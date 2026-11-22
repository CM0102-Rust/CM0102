# Mutations (tel25_5). The automatic career opens no screen: the chain runs none; these prove the comparer in a
# session of Lucas's (JOGAR_COMPARANDO.bat; club screen > Training): every build of the training screen is compared.
# - every contracted player shown as "Free" (the value column's test inverted);
# - the "Central" filter asks the right-side position bit (caught with Filter > Central ticked).
MUTACOES = [
    ('tel25_5_value_free', '    if club != 0 && value != 0 {', '    if club == 0 && value != 0 {'),
    ('tel25_5_central_filter', '(0x40, 0x80), (0x80, 0x200), (0x100, 0x800)]', '(0x40, 0x80), (0x80, 0x800), (0x100, 0x800)]'),
]
