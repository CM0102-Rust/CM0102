# Mutacoes (cqz_13), na campanha europeia de 2004 montada na criacao: oito grupos em vez de nove sem isentos (a tabela de
# rodadas e as fases mudam na memoria do jogo, o banco); a lista com 49 vagas (fica abaixo da contagem e a rotina devolve
# 0); o sorteio antecipado para o primeiro dia completo (grupos e jogos ja em 2001).
MUTACOES = [
    ('cqz13_oito_grupos',
     '0 => [8, 9, 0xa], // cqz13: nine groups without exempt nations',
     '0 => [7, 8, 9], // mutation: eight groups',
     'cqz_13 european pool'),
    ('cqz13_lista_de_49',
     "if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_EUROPE) && count < 0x32 {",
     "if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_EUROPE) && count < 0x31 { // mutation: 49 slots",
     'cqz_13 european pool'),
    ('cqz13_sorteio_ja',
     '    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {\n        return 0;\n    }\n'
     '    if today_year == year && today_day == day && league.active() == 0 && full != 0 {\n        covered!("cqz_13 european draw");',
     '    if false && (today_year < year || (today_year == year && (today_day as i16) < day as i16)) {\n        return 0;\n    }\n'
     '    if league.active() == 0 && full != 0 { // mutation: the draw at once\n        covered!("cqz_13 european draw");',
     'cqz_13 european day'),
]
