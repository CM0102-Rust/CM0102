# Telas do clube (2): a carreira automatica nao abre telas; mutacoes para provar o comparador numa sessao do Lucas com
# o JOGAR_COMPARANDO.bat:
# - a aba Tactics com o tipo 4 em vez de 3 (tela do clube de qualquer clube: cai no comparador da montagem);
# - o saldo das transferencias futuras sem descontar as que chegam (aba Transferencias, visao Future Transfers).
MUTACOES = [
    ('tel01_2_aba_tatica',
     'set_tab(&mut tabs, 5, 3, 0);',
     'set_tab(&mut tabs, 5, 4, 0);'),
    ('tel01_2_saldo_futuro',
     'total = total.wrapping_sub(read_i32(rec + 0x14));',
     'total = total.wrapping_add(0);'),
]
