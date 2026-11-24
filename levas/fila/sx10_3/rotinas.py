# Os mapas da nevoa (fog of war): o que cada tecnico humano sabe de cada pessoa (fog_of_war.cpp; leitor sx10, rodada
# 5, 23/11/2026): FOG_OF_WAR_MAP::FOG_OF_WAR_MAP 0x5aff00, FOG_OF_WAR::add_manager_map 0x5afd70,
# FOG_OF_WAR_MAP::daily_update 0x5b0260, FOG_OF_WAR_MAP::get_knowledge_level 0x5b02f0,
# fog_of_war_initial_knowledge_level 0x5b03e0, fog_of_war_minimum_knowledge_level 0x5b06e0.
# TELAS: nada disto roda na carreira de teste, que tem a mascara de atributos ("attribute masking") desligada (a
# cobertura da carreira do mundo da 0 chamadas ao construtor do mapa e ao add_manager_map; o get_knowledge_level so e
# chamado depois de conferir a mascara: Database 0x543280, 0x599514, 0x8ea322). Rodam so num jogo com a mascara
# ligada, e a prova e o Lucas jogando comparando (nao sao telas: o tipo TELAS e o que leva a prova ao jogo dele).
# Inicio e fim conferidos na desmontagem (todas sem tabela de saltos): 0x5aff00..0x5b0000 (ret 4), 0x5afd70..0x5afdd7
# (moldura SEH, ret 4), 0x5b0260..0x5b02ec (ret), 0x5b02f0..0x5b03cf (ret 4), 0x5b03e0..0x5b06db (ret 8),
# 0x5b06e0..0x5b09c3 (ret 8).
# Chamadores: o construtor do mapa por 0x5af764, 0x5af8ca (FOG_OF_WAR::load) e 0x5afdba (add_manager_map);
# add_manager_map por 0x5af684 (construtor da nevoa) e 0x822e76 (Setup: o tecnico humano acrescentado, com a mascara
# ligada); daily_update por 0x5afcef (FOG_OF_WAR::daily_update, ja Rust); get_knowledge_level por 0x5432c6, 0x599514,
# 0x8ea322; o nivel inicial so pelo construtor do mapa; o minimo so pelo daily_update.
# Convencoes conferidas: thiscall ret 4, ret 4, ret 0, ret 4, ret 8, ret 8 (os dois niveis recebem (tecnico, pessoa),
# o ecx do mapa nao e lido); malloc 0x944e46 cdecl; langlib_sprintf 0x66f4e0 cdecl(buf, texto); a barra
# ScreenManager::send_network_status_message 0x802f80 thiscall(0xb74450, texto, feito, total, -1); Date::operator=
# 0x417c10 thiscall(destino, &tmp, 0xae2c90); Date::days_since 0x54cdb0 thiscall(0xae2c90, data lo, data hi);
# CONTRACT_MANAGER::get_contract 0x4cf5a0 thiscall([0xae1050], pessoa), o resultado guardado em 0xae1080 pela rotina;
# club_get_linked_b_team 0x540a50 cdecl(clube, &saida, 1); player_stats_years_since_at_club 0x7c6e30 stdcall(pessoa,
# clube) -> word (ja Rust; o original passa o ecx = 0xb74350, nao lido).
# Comparadores: o construtor do mapa e o add_manager_map so a original (alocam; a barra de progresso); o daily_update
# lado a lado sobre a grade e 0xae1080; os tres niveis pelo valor (compare_value!).
TELAS = True
ROTINAS = [('fog_of_war_map_new', 0x5aff00, [0xa1, 0x64, 0x23, 0xae, 0x00]),
           ('fog_of_war_add_manager_map', 0x5afd70, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('fog_of_war_map_daily_update', 0x5b0260, [0xa1, 0x64, 0x23, 0xae, 0x00]),
           ('fog_of_war_map_get_knowledge_level', 0x5b02f0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('fog_of_war_initial_knowledge_level', 0x5b03e0, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('fog_of_war_minimum_knowledge_level', 0x5b06e0, [0x83, 0xec, 0x08, 0x53, 0x8b, 0x5c, 0x24, 0x10])]
DOCS = ('os mapas da névoa: o que cada técnico humano sabe de cada pessoa',
        'the fog of war maps: what each human manager knows of each person',
        [('fog_of_war_map_new', 'FOG_OF_WAR_MAP::FOG_OF_WAR_MAP', '0x5aff00', 'fog_of_war.cpp', 'thiscall, ret 4',
          'aloca a grade de 9 bytes por pessoa; com grade e técnico, guarda o técnico e dá a cada pessoa a data de hoje e o nível inicial de conhecimento, com a barra "Initialising game data" a cada 1000 pessoas (sem grade ou sem técnico o campo do técnico fica como estava)',
          'allocates the grid of 9 bytes per person; with a grid and a manager, keeps the manager and gives each person today\'s date and the initial knowledge level, with the "Initialising game data" bar every 1000 people (without a grid or a manager the manager field is left as it was)'),
         ('fog_of_war_add_manager_map', 'FOG_OF_WAR::add_manager_map', '0x5afd70', 'fog_of_war.cpp', 'thiscall, ret 4',
          'cria o mapa de um técnico humano e o põe na vaga dele (total de pessoas - id - 1), por cima do que houver',
          'creates a human manager\'s map and puts it in his slot (people count - id - 1), over whatever is there'),
         ('fog_of_war_map_daily_update', 'FOG_OF_WAR_MAP::daily_update', '0x5b0260', 'fog_of_war.cpp', 'thiscall',
          'para cada pessoa: conhecida e sem novidade há mais de 540 dias, ganha a data de hoje e desce um nível; depois sobe ao nível mínimo, se estiver abaixo',
          'for each person: known and not seen for more than 540 days, gets today\'s date and drops a level; then rises to the minimum level when below it'),
         ('fog_of_war_map_get_knowledge_level', 'FOG_OF_WAR_MAP::get_knowledge_level', '0x5b02f0', 'fog_of_war.cpp', 'thiscall, ret 4',
          'sem pessoa ou sem técnico: 0; pessoa fora da tabela ou máscara desligada: 3; a pessoa no clube do técnico (ou com contrato com ele, ou no clube ligado), ou na mesma seleção: 3; senão o nível guardado',
          'no person or no manager: 0; a person outside the table or masking off: 3; the person at the manager\'s club (or under contract to it, or at the linked club), or in the same national team: 3; otherwise the stored level'),
         ('fog_of_war_initial_knowledge_level', 'fog_of_war_initial_knowledge_level', '0x5b03e0', 'fog_of_war.cpp', 'thiscall, ret 8',
          'quem não é jogador e a mesma seleção: 3; famoso (reputação da nação / 10 × convocações acima de 8,5, ou reputação mundial acima de 5250): 3; mesma divisão e mais de 19 anos: 3; no clube do técnico há menos de 2 anos: 3; ligado à nação do técnico pelo clube: 3 com reputação acima de 4250, 2 acima de 21 anos ou no clube há menos de 3 anos, senão 1 (23 anos para o clube na nação do técnico e para a mesma nação); sem ligação de nação: 1 se o técnico dirige a seleção de uma nação pequena (menos de 1000 pessoas) que é a da pessoa, senão 0',
          'non-players and the same national team: 3; famous (nation reputation / 10 × caps above 8.5, or world reputation above 5250): 3; same division and older than 19: 3; at the manager\'s club less than 2 years ago: 3; linked to the manager\'s nation by club: 3 with reputation above 4250, 2 older than 21 or at the club less than 3 years ago, otherwise 1 (23 years for the club in the manager\'s nation and for the same nation); with no nation link: 1 when the manager runs the national team of a small nation (fewer than 1000 people) that is the person\'s, otherwise 0'),
         ('fog_of_war_minimum_knowledge_level', 'fog_of_war_minimum_knowledge_level', '0x5b06e0', 'fog_of_war.cpp', 'thiscall, ret 8',
          'famoso, no clube do técnico, na mesma seleção, com contrato com o clube do técnico ou num clube ligado a ele: 3; mesma divisão: 2; clubes na mesma nação, a mesma nação ou o clube do técnico na nação da pessoa: 2 com reputação acima de 4250, 1 acima de 21 anos; o clube da pessoa na nação do técnico: 1 com reputação acima de 4250; senão 0',
          'famous, at the manager\'s club, in the same national team, under contract to the manager\'s club or at a club linked to it: 3; same division: 2; clubs in the same nation, the same nation or the manager\'s club in the person\'s nation: 2 with reputation above 4250, 1 older than 21; the person\'s club in the manager\'s nation: 1 with reputation above 4250; otherwise 0')])
