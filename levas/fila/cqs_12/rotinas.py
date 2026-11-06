# A Copa América: as 12 seleções, as posições previstas e a temporada nova (goldcup.cpp, em intercomp\). Frente das
# competições (Agente E, 06/11/2026), carreira de 8 ligas (a Copa América de 2001 fica dentro da janela).
# - 0x5e1cd0 (GoldCup::set_up_team_list, 1092 bytes; 1 chamada até 2002-06-01): as 12 (fixas em 2001). As três
#   mutações (trocas de vaga entre os grupos).
# - 0x5e2340 (GoldCup::set_predicted_positions) e 0x5e1530 (GoldCup::start_new_season): passageiras.
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas (lição da 258a): as 12 são thiscall, ret 0 (mov ecx, esi; call, do dia); as previsões e a
# temporada nova são as virtuais +0x5c e +8 (thiscall, ret 0). Dentro: 0x53b3d0 (push nação; add esp -> cdecl),
# 0x5fa730 (5 pushes; mov ecx, [0xb5e134] -> thiscall, ret 0x14), 0x945333 (4 pushes; add esp -> cdecl), 0x4a2540
# (push posição; push clube; mov ecx, [+8] -> thiscall, ret 8), 0x687970 (push 0; mov ecx -> thiscall, ret 4),
# 0x4a1c50 (push 1; mov ecx, [+8] -> thiscall, ret 4), a virtual 0 das fases (push 1; mov ecx -> thiscall), a
# virtual +0x8c (thiscall, ret 0), 0x944e46, 0x9452ca e a caixa.
# Comparadores: as 12 lado a lado (o +0xf2 e o [0xb67a34]; a lista da cópia comparada nas 12 vagas e liberada); as
# previsões só a original, conferindo a posição de cada uma na estatística (0x4a4850, +0x72) pela ordem de reputação;
# a temporada nova só a original (libera e destrói), conferindo o ano, +0x45, +0x30 e +0x4c.
# Determinismo: nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('cqs12_copa_america_selecoes', 0x5e1cd0, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('cqs12_copa_america_previstas', 0x5e2340, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('cqs12_copa_america_temporada_nova', 0x5e1530, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a Copa América: as 12 seleções, as posições previstas e a temporada nova',
        'the Copa América: the 12 nations, the predicted positions and the new season',
        [('cqs12_copa_america_selecoes', 'GoldCup::set_up_team_list', '0x5e1cd0', 'goldcup.cpp', 'thiscall, ret 0', 'regra: a Copa América tem 12 seleções; em 2001 uma lista fixa (os grupos intercalados: A = Colômbia e as vagas 3, 6 e 9; B com o México na vaga 4 e o Peru na 7; C com a Bolívia na 11); nos outros anos as sedes na frente, depois as sul-americanas e as convidadas (o Japão em 1999 [inferência: a nação de 0x9cf36c], nos outros anos a de 0x9cf280, e o México sempre), sem repetir as sedes, até 12, ordenadas pela reputação; sem 12, erro', 'rule: the Copa América has 12 nations; in 2001 a fixed list (the groups interleaved: A = Colombia and places 3, 6 and 9; B with Mexico in place 4 and Peru in 7; C with Bolivia in 11); in other years the hosts first, then the South American nations and the guests (Japan in 1999 [inference: the nation of 0x9cf36c], in other years that of 0x9cf280, and Mexico always), without repeating the hosts, up to 12, sorted by reputation; without 12, an error'),
         ('cqs12_copa_america_previstas', 'GoldCup::set_predicted_positions', '0x5e2340', 'goldcup.cpp', 'thiscall, ret 0', 'regra: as posições previstas da Copa América pela reputação: 1, 2, 3, 3, quatro 5, 9 e três 10', 'rule: the Copa América predicted positions by reputation: 1, 2, 3, 3, four 5ths, 9 and three 10ths'),
         ('cqs12_copa_america_temporada_nova', 'GoldCup::start_new_season', '0x5e1530', 'goldcup.cpp', 'thiscall, ret 0', 'regra: a Copa América é a cada 2 anos: ao acabar, a tabela e as fases são desfeitas, a temporada vai para o histórico da estatística, o ano sobe 2 e as regras são montadas de novo', 'rule: the Copa América is every 2 years: when it ends, the table and stages are undone, the season goes to the statistics history, the year goes up 2 and the rules are set up again')])
CAMPOS = [('copa +0x40 ano (2001 = 0x7d1), +0xee lista das seleções, +0xf2 a conta (12); valor/ponteiro; lê e grava (as 12)', 'cup +0x40 year (2001 = 0x7d1), +0xee nation list, +0xf2 its count (12); value/pointer; reads and writes (the 12)'),
          ('nação +0x71 continente (o +0 dele é o id), +0 id; seleção +0x53 nação; ponteiro/id; lê (as 12)', 'nation +0x71 continent (its +0 is the id), +0 id; national team +0x53 nation; pointer/id; reads (the 12)'),
          ('estatística (+8) posição prevista da entrada do clube (+0x72); valor; grava (previsões)', 'statistics (+8) predicted position of the club entry (+0x72); value; writes (predictions)'),
          ('copa +0x4c, +0xba tabela das rodadas, +0x45, +0x30; valor/ponteiro; grava (temporada nova)', 'cup +0x4c, +0xba round table, +0x45, +0x30; value/pointer; writes (new season)')]
INTERESSA = ['CMRTE']
