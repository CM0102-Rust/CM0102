# Mutacoes para o comparador (TELAS: a cadeia nao roda; a prova e o Lucas jogando comparando, Clube -> Recordes):
# - rx08_9_tipos_deslocados: o rotulo do tipo k vira o do k + 1: o comparador dos rotulos acusa.
# - rx08_9_cash_dobrado: o Cash guarda o dobro: o comparador do Cash acusa.
MUTACOES = [
    ('rx08_9_tipos_deslocados',
     'club_records_write_label(&CLUB_RECORDS_TYPE_LABELS, kind, buf, size, 0x793, 0x87f) // rx08_9: the record type labels',
     'club_records_write_label(&CLUB_RECORDS_TYPE_LABELS[1..], kind, buf, size, 0x793, 0x87f) // rx08_9 mutation',
     'rx08_9 record type label'),
    ('rx08_9_cash_dobrado',
     'write_i32(this, value); // rx08_9: the amount',
     'write_i32(this, value.wrapping_mul(2)); // rx08_9 mutation',
     'rx08_9 period label'),
]
