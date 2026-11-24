# As sedes do Mundial de Clubes e da Copa das Confederacoes, sorteadas de novo (host_country.cpp; leitor cx05, rodada 5,
# 24/11/2026). Simulacao com as 8 ligas e ATE 2003-08-15: o unico chamador e 0x929db0 (world_club_champ.cpp, a virada
# do Mundial de Clubes, cuja primeira edicao e em julho e agosto de 2003, cx04_6); 0 chamadas na cobertura do mundo ate
# 2002-08-01. Duvida para a prova: a data exata da virada do Mundial de Clubes; se nao rodar ate 2003-08-15, deixar
# para a auditoria.
# - 0x5fab70 HostCountry::redraw_world_club_and_confed_hosts thiscall() ret 0: utils_random 0x90cfc0 (cdecl). Prologo
#   sub esp, 0x258 (6 bytes).
# Igual no cm0102-1080.exe (inteira).
# Comparador: so a original (sorteia), com o plano: a sede da secao da Oceania e a Australia ou a Nova Zelandia.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('redraw_world_club_and_confed_hosts', 0x5fab70, [0x81, 0xec, 0x58, 0x02, 0x00, 0x00])]
DOCS = ('as sedes do Mundial de Clubes e da Copa das Confederações', 'the Club World Championship and Confederations Cup hosts',
        [('redraw_world_club_and_confed_hosts', 'HostCountry::redraw_world_club_and_confed_hosts', '0x5fab70', 'host_country.cpp', 'thiscall, ret 0',
          'regra: a cada edição o Mundial de Clubes e a Copa das Confederações ganham uma sede sorteada entre as nações de liga forte (padrão 13 ou mais) que não sejam a Inglaterra, a Espanha nem da Oceania; a sede da eliminatória da Oceania alterna entre a Austrália e a Nova Zelândia',
          'rule: each edition the Club World Championship and the Confederations Cup get a host drawn among the nations with a strong league (standard 13 or more) other than England, Spain and the Oceania nations; the Oceania qualifying section\'s host alternates between Australia and New Zealand')])
CAMPOS = []
INTERESSA = []
