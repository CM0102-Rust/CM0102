# Mutações (tel44_1). O texto não volta para a simulação: acusadas pela comparação (compare_text_writer compara o
# texto e os vínculos), como as de texto aceitas em 12/11/2026; a carreira ENG até 2002-06-01 entrega pelo menos uma
# notícia de recorde de competição (o ramo "tel41_3 record news sent" da base).
# - o vínculo do clube com outro tipo (0x7d6): todos os tipos passam por ele;
# - o gênero do time trocado (^ 1): todos os tipos o usam no formato.
MUTACOES = [
    ('tel44_1_club_link',
     "exe::text_add_link(out, self.name.as_ptr(), LINK_CLUB, self.club); // tel44_1: the club's link",
     "exe::text_add_link(out, self.name.as_ptr(), 0x7d6, self.club); // mutation: the club's link with another type",
     'tel44_1 comp record news text'),
    ('tel44_1_team_gender',
     "CompRecordTeam { club, name, gender: record_gender(club) } // tel44_1: the team's gender",
     "CompRecordTeam { club, name, gender: record_gender(club) ^ 1 } // mutation: the other gender",
     'tel44_1 comp record news text'),
]
