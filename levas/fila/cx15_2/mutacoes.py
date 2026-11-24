# Mutacoes (cx15_2), carreira ENG ate 2002-06-01. Nenhuma abre caixa.
# - os pontos de um resultado europeu dobrados: o coeficiente das nacoes muda na virada do ano (roll_coefficients, em
#   janeiro de 2002), e com ele o ranking, as vagas e os clubes classificados que o update_season redistribui;
# - as vagas da Liga dos Campeoes e da Copa da UEFA trocadas: muda o progresso dos clubes nas estatisticas no fim das
#   ligas europeias (e quem a previsao poe nas vagas);
# - no prazo de 1 de agosto, a inscricao do passo 1 de novo: as marcas de inscricao zeradas e a escolha da fase
#   preliminar em vez da fase de grupos (muda quem pode jogar nas copas europeias).
MUTACOES = [
    ('cx15_2_pontos_dobrados',
     '    write_i16(at, read_i16(at).wrapping_add(points as u8 as i8 as i16));',
     '    write_i16(at, read_i16(at).wrapping_add(2 * (points as u8 as i8 as i16))); // mutation',
     'cx15_2 nation points'),
    ('cx15_2_vagas_trocadas',
     '            *champions_league = read_i8(e + uefa_seeding_nation::CHAMPIONS_LEAGUE_PLACES);\n            *uefa_cup = read_i8(e + uefa_seeding_nation::UEFA_CUP_PLACES);',
     '            *champions_league = read_i8(e + uefa_seeding_nation::UEFA_CUP_PLACES); // mutation\n            *uefa_cup = read_i8(e + uefa_seeding_nation::CHAMPIONS_LEAGUE_PLACES);',
     'cx15_2 nation places'),
    ('cx15_2_agosto_como_julho',
     '    } else if is_date_today(group_stage, G_CURRENT_DATE) {\n        Some(2)',
     '    } else if is_date_today(group_stage, G_CURRENT_DATE) {\n        Some(1) // mutation',
     'cx15_2 registration deadline'),
]
