# A fase final da Eurocopa: o construtor, a lista de seleções e o dia (euro_champ.cpp). Leitor cqz (12/11/2026), lista de
# 8 ligas.
# - 0x57a890 (EuroChamp::EuroChamp), 0x57bd80 (EuroChamp::set_up_team_list), 0x57c620 (EuroChamp::update_for_date).
# O construtor: a edição no ciclo de 4 anos a partir de 1992 começando no ano seguinte ao do jogo (2004), +0x4b = 1,
# +0xd9 = 0x28 antes da carga; a lista de seleções antes das regras (retorno não lido); sem estatísticas a caixa e
# segue; no fim, edição a menos de um ano: +0x36 = 16, senão +0x45 = 0 (no outro caso o +0x45 fica como estava);
# +0x3e = 0 e +0xb1 = 0. A lista: livro de 16 vagas de 6 bytes (calloc), as sedes (status 1) e depois uma lista fixa
# de 14 seleções nas vagas 2 a 15 (sem status nem contagem: com uma sede só, a vaga 1 fica vazia). O dia: sem `full`,
# só o dia da liga; com ele, a semeadura no dia +0x13e/+0x140 (comparação de 32 bits: o dia com sinal contra o word sem
# sinal) e o sorteio no dia do sorteio com a tabela vazia.
# Convenções conferidas: o construtor thiscall, ret 8; as outras thiscall, ret 0 e ret 8; 0x945501 (2 pushes ->
# cdecl), 0x5fa730 (5 pushes; mov ecx -> thiscall, ret 0x14), 0x53b3d0 (cdecl), 0x57c710 e 0x57c1d0 (ret 0), 0x6847c0
# (ret 8), 0x549ef0 (lea ecx, local; 2 pushes -> ret 8).
# Comparadores: só a original nas três (alocam, sorteiam), com o plano.
# Cuidado na integração: usa compare_nations_tournament_ctor e oceania_nations_draw_date (motor.rs, já em Rust).
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('construct_euro_champ', 0x57a890, [0x6a, 0xff, 0x68, 0x89, 0x87, 0x95, 0x00]),
           ('euro_champ_set_up_team_list', 0x57bd80, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('euro_champ_update_for_date', 0x57c620, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a fase final da Eurocopa: o construtor, a lista de seleções e o dia',
        'the European Championship finals: the constructor, the team list and the day',
        [('construct_euro_champ', 'EuroChamp::EuroChamp', '0x57a890', 'euro_champ.cpp', 'thiscall, ret 8', 'regra: a Eurocopa é de quatro em quatro anos a partir de 1992; a edição do jogo é a seguinte ao ano do jogo (num jogo de 2001, a de 2004); 4 fases; a lista de seleções, as regras e as estatísticas; a fase final do próprio ano já tem as 16 inscritas', "rule: the European Championship is every four years from 1992; the game's edition is the one after the game's year (in a 2001 game, 2004); 4 stages; the team list, the rules and the statistics; the finals of the game's own year already have their 16 entrants"),
         ('euro_champ_set_up_team_list', 'EuroChamp::set_up_team_list', '0x57bd80', 'euro_champ.cpp', 'thiscall, ret 0', 'regra: antes das eliminatórias, a Eurocopa tem 16 vagas: as sedes primeiro (já classificadas) e uma lista fixa de 14 seleções (Itália, Finlândia, Espanha, Escócia, Romênia, Alemanha, França, País de Gales, Inglaterra, Dinamarca, Áustria, Croácia, Bulgária e Noruega)', 'rule: before qualifying, the European Championship has 16 places: the hosts first (already qualified) and a fixed list of 14 nations (Italy, Finland, Spain, Scotland, Romania, Germany, France, Wales, England, Denmark, Austria, Croatia, Bulgaria and Norway)'),
         ('euro_champ_update_for_date', 'EuroChamp::update_for_date', '0x57c620', 'euro_champ.cpp', 'thiscall, ret 8', 'regra: na data da semeadura (10 de janeiro do ano da fase final) as seleções vão para os potes; na data do sorteio (10 de fevereiro), sem tabela ainda, os grupos são sorteados; depois o dia da liga', 'rule: on the seeding date (10 January of the finals year) the nations go into pots; on the draw date (10 February), with no table yet, the groups are drawn; then the league day')])
CAMPOS = [('Eurocopa +0x13e/+0x140 data da semeadura (dia do ano, ano relativo), +0x14 livro de inscritas (6 bytes por vaga: +0 seleção, +4 status), +0x36/+0x38 inscritas/vagas, +0x4b, +0xb1; valor/ponteiro; lê e grava', 'European Championship +0x13e/+0x140 seeding date (day of year, relative year), +0x14 entrant book (6 bytes per slot: +0 team, +4 status), +0x36/+0x38 entrants/places, +0x4b, +0xb1; value/pointer; reads and writes')]
