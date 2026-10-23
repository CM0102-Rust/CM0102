# Mutações da 221a leva (itens save_s4_a). Uso: python mutacoes.py mutacoes\leva221.py --prova-save 221
MUTACOES = [
    ('sv4_hora_do_dia', '        (0xae2c88, 1, 0x23ef, 0xa815d0),', '        (0xae2cfc, 1, 0x23ef, 0xa815d0),'),
    ('sv4_data_do_jogo', '                    copia_data(0xae2c90, tmp.as_mut_ptr(), data_do_jogo.as_ptr() as usize);', '                    let _ = (&mut tmp, &data_do_jogo);'),
    ('sv4_iniciar_0x22', '                    iniciar(o);', '                    let _ = (iniciar, o);'),
]
