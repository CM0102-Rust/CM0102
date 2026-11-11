# Mutacoes (cqv_7), nos inscritos da Copa da Inglaterra (juntados uma vez, em 3 de setembro de 2001, antes da primeira
# rodada): sem as 60 trocas da primeira divisao (120 sorteios a menos e outra ordem de entrada); a segunda divisao antes
# da terceira (os clubes entram em outras rodadas); os de fora da liga sem a ordem pela injury_word_0c (outros 38 clubes e
# sem os sorteios do embaralhamento).
MUTACOES = [
    ('cqv7_sem_as_trocas_da_primeira',
     'for _ in 0..0x3c { // cqv7: 60 swaps in the first division',
     'for _ in 0..0 { // mutacao: sem as trocas',
     'cqv_7 first division shuffled'),
    ('cqv7_segunda_antes_da_terceira',
     'fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, &[a, b, x1, x2]); fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, &[x1, x2]); // cqv7: the third and the second',
     'fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, &[x1, x2]); fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, &[a, b, x1, x2]); // mutacao: a segunda antes',
     'cqv_7 first division shuffled'),
    ('cqv7_fora_da_liga_sem_reputacao',
     'exe::comp_util_sort_and_shuffle_top_clubs(v.as_mut_ptr() as usize + FA_CUP_NON_LEAGUE_START as usize * 4, nout as u32, non_league_places as i16 as u16 as u32); // cqv7: the non-league clubs by reputation',
     'let _ = non_league_places; // mutacao: sem a ordem pela injury_word_0c',
     'cqv_7 first division shuffled'),
]
