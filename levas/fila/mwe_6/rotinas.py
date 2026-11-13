# The tables of the Norwegian, Swedish and Finnish Premier Divisions (reader mwe, whole-world round, 12/11/2026). Each
# runs when its league is created (the constructor) and at each season turnover (the start_new_season of the league,
# at the end of 2001):
# - 0x7925e0 NorPrm::create_table (288 bytes, 14 clubs, share 50,000);
# - 0x890430 SwePrm::create_table (288 bytes, 14 clubs, share 50,000);
# - 0x595b70 FinPrm::create_table (288 bytes, 12 clubs, share 35,000).
# The body of eng_prm_fill_league_table (cqi_5), checked instruction by instruction, with no TV money. Equal in
# cm0102-1080.exe (whole).
# Conventions checked: called directly by the constructor and the turnover (mov ecx, esi; call) -> thiscall, ret 0;
# inside, malloc (push; add esp, 4 -> cdecl), League::init_table_row (push 0; push 0; push club; push row -> thiscall,
# ret 0x10; the row pushed in eax with only al set: the routine reads the byte), League::calc_prize_fund (push share
# -> thiscall, ret 4), the error box.
# Comparer: eng_prm_compare_league_table (only the original: it allocates; the plan of the division's clubs in
# database order, the count, the return value, the rows, +0xe1 = 1 and the prize fund).
# Determinism: no stack, heap or address read as a value (the high bytes of the pushed row are not read).
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('nor_prm_create_table', 0x7925e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('swe_prm_create_table', 0x890430, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('fin_prm_create_table', 0x595b70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as tabelas das primeiras divisões da Noruega, da Suécia e da Finlândia',
        'the tables of the Norwegian, Swedish and Finnish Premier Divisions',
        [('nor_prm_create_table', 'NorPrm::create_table', '0x7925e0', 'nor_prm.cpp', 'thiscall, ret 0',
          'a Tippeligaen tem 14 clubes, os da divisão na ordem do banco de dados; o prêmio pela cota de 50.000 por posição; sem dinheiro da TV',
          'the Tippeligaen has 14 clubs, those of the division in database order; the prize fund by a share of 50,000 per place; no TV money'),
         ('swe_prm_create_table', 'SwePrm::create_table', '0x890430', 'swe_prm.cpp', 'thiscall, ret 0',
          'a Allsvenskan tem 14 clubes; o prêmio pela cota de 50.000 por posição; sem dinheiro da TV',
          'the Allsvenskan has 14 clubs; the prize fund by a share of 50,000 per place; no TV money'),
         ('fin_prm_create_table', 'FinPrm::create_table', '0x595b70', 'fin_prm.cpp', 'thiscall, ret 0',
          'a Veikkausliiga tem 12 clubes; o prêmio pela cota de 35.000 por posição; sem dinheiro da TV',
          'the Veikkausliiga has 12 clubs; the prize fund by a share of 35,000 per place; no TV money')])
CAMPOS = []
INTERESSA = []
