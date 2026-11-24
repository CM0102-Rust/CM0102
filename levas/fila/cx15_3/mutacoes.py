# Mutacoes (cx15_3), carreira ENG ate 2002-06-01 (o ranking e refeito em 1/1/2002). Nenhuma abre caixa.
# - as nacoes do menor coeficiente para o maior: o ranking das selecoes e as vagas por nacao se invertem;
# - os clubes de uma nacao pela pior posicao primeiro: outros clubes recebem as vagas europeias;
# - os classificados pela competicao de menor reputacao primeiro: so a ordem da lista da noticia muda (a carreira pode
#   nao acusar; o comparador da rotina acusa na comparacao).
MUTACOES = [
    ('cx15_3_coeficiente_invertido',
     '    if x > y {\n        -1\n    } else if !(x >= y) {\n        1',
     '    if x > y {\n        1 // mutation\n    } else if !(x >= y) {\n        -1',
     'cx15_3 nations by coefficient'),
    ('cx15_3_pior_posicao_primeiro',
     '    let position = read_i8(a + european_club_entry::LAST_POSITION) as i32 - read_i8(b + european_club_entry::LAST_POSITION) as i32;',
     '    let position = read_i8(b + european_club_entry::LAST_POSITION) as i32 - read_i8(a + european_club_entry::LAST_POSITION) as i32; // mutation'),
    ('cx15_3_competicao_menor_primeiro',
     '        return competition_reputation(kb as i32) as i32 - competition_reputation(ka as i32) as i32;',
     '        return competition_reputation(ka as i32) as i32 - competition_reputation(kb as i32) as i32; // mutation',
     'cx15_3 qualified clubs by competition'),
]
