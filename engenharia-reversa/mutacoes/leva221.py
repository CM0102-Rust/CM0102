# Mutações da 221a leva (itens save_s4_a). Uso: python mutacoes.py mutacoes\leva221.py --prova-save 221
MUTACOES = [
    ('sv4_hora_do_dia', '        (G_CURRENT_DAY_SLICE, 1, 0x23ef, 0xa815d0),', '        (0xae2cfc, 1, 0x23ef, 0xa815d0),'),
    ('sv4_data_do_jogo', '                    exe::date_operator_3d(G_CURRENT_DATE, (tmp.as_mut_ptr()) as usize, data_do_jogo.as_ptr() as usize);', '                    let _ = (&mut tmp, &data_do_jogo);'),
    ('sv4_iniciar_0x22', '                    exe::reservecomp_create_sub_comps(o);', '                    let _ = (iniciar, o);'),
]
