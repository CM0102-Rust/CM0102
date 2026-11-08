# Mutações da 239a leva (itens dbv_1). Uso: python mutacoes.py mutacoes\leva239.py --prova-save 239
MUTACOES = [
    ('dbv1_habilidade_x50', '            ww(j + player_sheet::CURRENT_REPUTATION, e20.wrapping_mul(50));', '            ww(j + player_sheet::CURRENT_REPUTATION, e20.wrapping_mul(49)); // mutacao'),
    ('dbv1_reputacao_x50', '            let a = w(nj + non_player::WORLD_REPUTATION).wrapping_mul(50);', '            let a = w(nj + non_player::WORLD_REPUTATION).wrapping_mul(49); // mutacao'),
    ('dbv1_metade', '                    set_attr_cell_value(&mut v3, c, (x as i32 / 2) as i16);', '                    set_attr_cell_value(&mut v3, c, (x as i32 / 3) as i16); // mutacao'),
    ('dbv1_sorteio_potencial', '            if exe::utils_random(2000) == 0 {', '            if exe::utils_random(1999) == 0 { // mutacao'),
    ('dbv1_molde_250', '                    count -= 1; // the 250 drawn', '                    count -= 2; // mutacao: 125 sorteados'),
]
