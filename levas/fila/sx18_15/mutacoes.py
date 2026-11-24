# Mutacoes (sx18_15), pela carreira com LIGAS 'ENG BEL HOL GER DEN' ate 2002-08-01: as suspensoes tiram jogadores das escalacoes seguintes;
# as de construtor (escala) sao acusadas pelo comparador do construtor (os campos contra a classe).
MUTACOES = [
    ('sx18_15_direto_so_na_copa',
     '|straight| [if straight { league } else { 0xff }, cup, 0xff, 0xff]) // sx18_15',
     '|_straight| [0xff, cup, 0xff, 0xff]) // mutation',
     'sx18_15 german cup cards'),
    ('sx18_15_limite_20',
     'threshold = if threshold == 0xf { 0x1e } else { threshold + 0xe }; // sx18_15',
     'threshold = if threshold == 0xf { 0x14 } else { threshold + 0xe }; // mutation',
     'sx18_15 danish points ban'),
]
