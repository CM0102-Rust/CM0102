# Mutacoes (mwb_3): a Copa de Gales com a disciplina das copas escocesas (outras suspensoes); as quartas uma semana antes
# (10/3 -> 3/3, depois do sorteio de 11/2); a Liga de Gales do fim do banco para o comeco (outra ordem, outro sorteio).
MUTACOES = [
    ('mwb3_disciplina_escocesa',
     'discipline: 0x28, // mwb3: the Welsh Cup discipline rule',
     'discipline: 0x19, // mutation: the Scottish rule',
     'mwb_3 new welsh cup'),
    ('mwb3_quartas_uma_semana_antes',
     'cql10r((0xb, 1, 1, 6), (0xa, 2, 1, 5, 1, 0), 0x78, (0, 3, 3), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // mwb3: the Welsh Cup quarter-finals',
     'cql10r((0xb, 1, 1, 6), (3, 2, 1, 5, 1, 0), 0x78, (0, 3, 3), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // mutation: on 3/3',
     'mwb_3 welsh cup rounds'),
    ('mwb3_liga_de_gales_ao_contrario',
     'let order: Vec<i32> = (0..read_usize(G_CLUB_COUNT) as i32).collect(); // mwb3: the League of Wales in database order',
     'let order: Vec<i32> = (0..read_usize(G_CLUB_COUNT) as i32).rev().collect(); // mutation: from the end of the database',
     'mwb_3 welsh cup entrants'),
]
