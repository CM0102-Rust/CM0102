# Mutacoes (sx05b_13), pela carreira com a Suecia ate 2002-06-01.
# - a equipe da semana so da Premier Division (a da First Division nunca e montada);
# - as regras suecas sem a marca +0x15 (o comparador lado a lado acusa; a elegibilidade muda).
MUTACOES = [
    ('sx05b_13_so_premier',
     '    nation_awards_team_of_the_week_of(this, award, &[7, 9]);',
     '    nation_awards_team_of_the_week_of(this, award, &[7]); // mutation'),
    ('sx05b_13_sem_marca',
     '    write_u32(this + transfer_rules::SKIP_CONTRACT_TYPE_5_INELIGIBILITY, 1);\n    covered!("sx05b_13 swedish rules");',
     '    covered!("sx05b_13 swedish rules"); // mutation: no flag',
     'sx05b_13 swedish rules'),
]
