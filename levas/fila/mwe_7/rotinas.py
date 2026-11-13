# The tables of the Swedish, Finnish and Norwegian First Divisions (reader mwe, whole-world round, 12/11/2026). Each
# runs when its league is created and at each season turnover (the end of 2001):
# - 0x88ef90 SweFirst::create_table (256 bytes): 16 clubs;
# - 0x593a70 FinFirst::create_table (272 bytes): the North group, 10 clubs (division and reserve division +0x60);
# - 0x7909a0 NorFirst::create_table (352 bytes): 16 clubs, or 14 with 2 relegated when the database has exactly 14.
# No prize fund nor TV money (unlike the Premier Divisions); the return value is sete (filled). Equal in
# cm0102-1080.exe (whole).
# Conventions checked: called directly by the constructor and the turnover (mov ecx, esi; call) -> thiscall, ret 0;
# inside, malloc (push; add esp, 4 -> cdecl), League::init_table_row (push 0; push 0; push club; push row ->
# thiscall, ret 0x10; the row in eax with only al set), the error box.
# Comparer: compare_first_division_table (only the original: it allocates; the plan of the accepted clubs in database
# order, the count, the return value and the rows; the Norwegian one also the relegated count with 14).
# Determinism: no stack, heap or address read as a value.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('swe_first_create_table', 0x88ef90, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('fin_first_create_table', 0x593a70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nor_first_create_table', 0x7909a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as tabelas das segundas divisões da Suécia, da Finlândia e da Noruega',
        'the tables of the Swedish, Finnish and Norwegian First Divisions',
        [('swe_first_create_table', 'SweFirst::create_table', '0x88ef90', 'swe_first.cpp', 'thiscall, ret 0',
          'a Superettan tem 16 clubes, os da divisão na ordem do banco de dados; sem prêmio nem dinheiro da TV',
          'the Superettan has 16 clubs, those of the division in database order; no prize fund nor TV money'),
         ('fin_first_create_table', 'FinFirst::create_table', '0x593a70', 'fin_first.cpp', 'thiscall, ret 0',
          'a tabela da fase de liga da Ykkönen é o grupo Norte: 10 clubes, os da divisão cujo grupo regional é o Norte, na ordem do banco de dados; sem prêmio nem dinheiro da TV (o grupo Sul é outra fase)',
          'the Ykkönen league-stage table is the North group: 10 clubs, those of the division whose regional group is the North one, in database order; no prize fund nor TV money (the South group is another stage)'),
         ('nor_first_create_table', 'NorFirst::create_table', '0x7909a0', 'nor_first.cpp', 'thiscall, ret 0',
          'a primeira divisão norueguesa tem 16 clubes; se o banco de dados tiver exatamente 14 clubes na divisão, são 14 e só 2 caem; os da divisão na ordem do banco de dados; sem prêmio nem dinheiro da TV',
          'the Norwegian First Division has 16 clubs; when the database has exactly 14 clubs in the division, 14 and only 2 relegated; those of the division in database order; no prize fund nor TV money')])
CAMPOS = [('clube +0x60 divisão de reservas: o grupo regional da Ykkönen (Norte ou Sul); ponteiro; lê (fin_first_create_table)',
           'club +0x60 reserve division: the Ykkönen regional group (North or South); pointer; reads (fin_first_create_table)')]
INTERESSA = []
