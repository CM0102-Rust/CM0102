# Mutacoes (cx06_3), com as 8 ligas ate 2002-06-01. Os tres comparadores rodam so a original: as mutacoes sao para a
# carreira (sorteios, partidas e banco).
# - a edicao das eliminatorias vai para 2003 (resto 1 no lugar de 0): o sorteio de 10/03/2002 nao acontece (sem os 20
#   sorteios por pote do draw_groups) e as selecoes nao entram nas eliminatorias;
# - so 45 inscritos (0x2d no lugar de 0x33): o sorteio usa 5 potes em vez de 6 (40 sorteios a menos) e 6 selecoes
#   europeias ficam de fora;
# - a seleçao mais forte nao recebe as eliminatorias como competicao atual (NationalTeams::set_current_competition), e o
#   calendario de amistosos dela muda.
MUTACOES = [
    ('cx06_3_edicao_errada',
     '    while (year as i32 - 0x7c6) % 4 != 0 {',
     '    while (year as i32 - 0x7c6) % 4 != 1 { // mutation',
     'cx06_3 euro qualifiers built'),
    ('cx06_3_inscritos_45',
     '    let count = 0x33 - read_u8(this + euro_champ_qual::GROUP_FORMAT) as i8 as i32;',
     '    let count = 0x2d - read_u8(this + euro_champ_qual::GROUP_FORMAT) as i8 as i32; // mutation',
     'cx06_3 qualifiers entrants'),
    ('cx06_3_sem_competicao_atual',
     '        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), team.nation(), l.competition());',
     '        if k != 0 { exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), team.nation(), l.competition()); } // mutation',
     'cx06_3 qualifiers predicted band'),
]
