# Mutações da 269a leva (itens cqc_6). Uso: python mutacoes.py mutacoes\leva269.py --carreira 2002-06-01
MUTACOES = [
    ('cqc6_bonus_ao_mandante', 's = exe::utils_random(read_i16(read_usize(f + fixture::AWAY_CLUB) + club::REPUTATION) as i32 + 0x5dc); // cqc6: the away side with the bonus', 's = exe::utils_random(read_i16(read_usize(f + fixture::HOME_CLUB) + club::REPUTATION) as i32 + 0x5dc); // mutacao: o mandante com o bonus', 'cqc_6 match for TV'),
    ('cqc6_tv_no_dia_seguinte', 'write_u16(r + fixture::DAY, read_i16(pa) as u16); // cqc6: the TV day', 'write_u16(r + fixture::DAY, read_i16(pa).wrapping_add(1) as u16); // mutacao: o dia seguinte ao da TV', 'cqc_6 match for TV'),
    ('cqc6_conflito_nao_zera', 's = 0; // cqc6: conflito na janela', '// mutacao: o conflito na janela nao zera a nota', 'cqc_6 conflict in the window'),
]
