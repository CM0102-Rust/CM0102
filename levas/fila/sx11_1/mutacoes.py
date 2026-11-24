# Mutações (sx11_1): a carreira cria o jogo do zero e passa pelos dias até a data de início.
# - sem a avaliação dos elencos no fim (Shortlist_Manager, "Assessing Club Squads"): as listas de observação dos clubes
#   do computador começam vazias e as transferências do começo da temporada mudam;
# - sem a forma física inicial dos elencos (Injury::init_squad_fitness): as partidas das primeiras rodadas mudam.
MUTACOES = [
    ('sx11_1_sem_avaliacao',
     '    start_date_call0(start_date_fns::ASSESS_CLUB_SQUADS, read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856)); // the Shortlist_Manager',
     '    let _ = start_date_fns::ASSESS_CLUB_SQUADS; // mutation: no squad assessment'),
    ('sx11_1_sem_forma',
     '    start_date_call0(start_date_fns::INJURY_INIT_SQUAD_FITNESS, read_usize(G_INJURY));',
     '    let _ = start_date_fns::INJURY_INIT_SQUAD_FITNESS; // mutation: no squad fitness'),
]
