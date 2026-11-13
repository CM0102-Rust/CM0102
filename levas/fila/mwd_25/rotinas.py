# The Turkish First Division table and the lowest division's promotion (tur_first.cpp). Reader mwd (12/11/2026), round 2
# (the table 2 calls: creation, when the 2001 merge runs, and the turnover; the promotion 1 call at the turnover).
# - 0x8fddc0 TurFirst::create_table: in 2001 the clubs of the five category B groups (0x9cf8e4..0x9cf8f4) move into
#   category B (0x9cf8e0), the group kept in +0x60; 18 top-flight clubs; no prize fund (+0xe1 = 2), 1,000,000 TV money.
# - 0x8fe3a0 tur_first_promote_lower_clubs_to_third_tier: 10 up from the lowest division (0x9cf8cc; the nation pointer
#   only tested for null); the relegated of category B and of its stages 0..3 down, their +0x60 cleared. The map says
#   cdecl(void) but ecx (the First Division) is passed as this to 0x6830b0 and 0x6831a0: thiscall, ret 0.
# Comparers: only the original runs (the table: the planned clubs and the prize fields; the promotion: the relegated
# clubs checked when there are at least 10 candidates). Equal in cm0102-1080.exe.
# Depends on: fill_league_table_rows (mwd_16).
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('tur_first_create_table', 0x8fddc0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tur_first_promote_lower_clubs_to_third_tier', 0x8fe3a0, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00])]
DOCS = ('a tabela da Primeira Divisão turca e o acesso da divisão mais baixa', "the Turkish First Division table and the lowest division's promotion",
        [('tur_first_create_table', 'TurFirst::create_table', '0x8fddc0', 'tur_first.cpp', 'thiscall, ret 0',
          'regra: a Primeira Divisão turca tem 18 clubes, os da elite na ordem do banco; cheia, sem fundo de prêmios (modo 2) e a TV paga 1 milhão a cada clube; em 2001 também passa os clubes dos cinco grupos da categoria B para a própria categoria B, guardando o grupo na divisão reserva do clube',
          "rule: the Turkish First Division has 18 clubs, those of the top flight in database order; when full, no prize fund (mode 2) and TV pays 1,000,000 to each club; in 2001 it also moves the clubs of the five category B groups into category B itself, keeping the group in the club's reserve division"),
         ('tur_first_promote_lower_clubs_to_third_tier', 'tur_first_promote_lower_clubs_to_third_tier', '0x8fe3a0', 'tur_first.cpp', 'thiscall (ecx = a Primeira Divisão), ret 0',
          'regra: na virada turca, os dez melhores clubes da divisão mais baixa (ordenados por força, os 30 melhores embaralhados) sobem à segunda divisão categoria B, e os de situação 3 na categoria B e nos seus quatro grupos descem à divisão mais baixa, com a divisão reserva zerada; com menos de 10 candidatos ninguém se move',
          'rule: at the Turkish turnover, the ten best clubs of the lowest division (sorted by strength, the best 30 shuffled) go up into the second division category B, and the clubs of status 3 in category B and its four groups go down to the lowest division, their reserve division cleared; with fewer than 10 candidates nobody moves')])
CAMPOS = [('liga +0x3e, +0xb1, +0x28, +0xe1, +0xc; clube +0x53 nação, +0x57 divisão, +0x60 divisão reserva; linha da tabela +0x37', 'league +0x3e, +0xb1, +0x28, +0xe1, +0xc; club +0x53 nation, +0x57 division, +0x60 reserve division; table row +0x37')]
