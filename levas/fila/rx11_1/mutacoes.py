# Mutacoes (rx11_1), ENG ate 2005-12-31 (a campanha de 2006): os textos nao voltam a simulacao; as duas sao acusadas
# pelo comparador lado a lado (compare_text_writer), que conta como acusada.
# - africana: o vinculo da Copa depois do do clube num mata-mata (a ordem dos vinculos muda);
# - CONCACAF: "qualify for final" no lugar de "have qualified from the semi final stage" (o texto muda).
MUTACOES = [
    ('rx11_1_africana_vinculos_trocados',
     "link_fate_competition(text, world_cup); // rx11_1: the World Cup's link first in a tie",
     "link_fate_club(text, club); // mutation: the club twice",
     'rx11_1 african tie fate'),
    ('rx11_1_concacaf_texto_curto',
     'qualifying_fate_line(text, from_stage, club, Some((comp + competition::SHORT_GENDER_NAME, comp + competition::NAME)));',
     'qualifying_fate_line(text, in_short, club, Some((comp + competition::SHORT_GENDER_NAME, comp + competition::NAME))); // mutation',
     'rx11_1 concacaf group fate'),
]
