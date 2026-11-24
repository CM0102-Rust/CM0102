# Mutacoes (sx09_4). As tres rodam na lingua da carreira de teste (ingles): o nome da hora do dia vai para os textos
# (news.cpp 0x77c6b6, match_screens.cpp), os generos para a escolha do artigo nos textos traduzidos. O texto nao
# volta para a simulacao: o comparador de cada rotina (buffer ou retorno lado a lado) as acusa.
MUTACOES = [
    ('sx09_maiuscula',
     'let names = if capital != 0 { &TIME_OF_DAY_NAMES } else { &TIME_OF_DAY_NAMES_LOWER }; // sx09_4: the case',
     'let names = if capital == 0 { &TIME_OF_DAY_NAMES } else { &TIME_OF_DAY_NAMES_LOWER }; // mutation: the case swapped',
     'sx09_4 time of day name'),
    ('sx09_sem_genero',
     '        _ => 0, // sx09_4: no gender',
     '        _ => 1, // mutation: a gender for every language',
     'sx09_4 time of day gender'),
    ('sx09_lingua_3',
     '(language as i32 == LANGUAGE_3) as u32 // sx09_4: language 3',
     '(language as i32 != LANGUAGE_3) as u32 // mutation: every language but 3',
     'sx09_4 weekday gender'),
]
