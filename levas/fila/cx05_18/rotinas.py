# As sedes das finais europeias de clubes e a ordem da tabela de sedes (host_country.cpp; leitor cx05, rodada 5,
# 24/11/2026). Simulacao, carreira ENG com ATE 2002-08-01: HostCountry::daily_update (ja em Rust) chama as sedes das
# finais em 1 de julho fora de 2001 (1 chamada na cobertura do mundo ate 2002-08-01) e ordena a tabela com o
# comparador em 1 de janeiro (o construtor, fora daqui, tambem; e o find_hosts busca com ele).
# - 0x5fad20 HostCountry::pick_european_final_hosts thiscall() ret 0: utils_random 0x90cfc0 (cdecl). Prologo sub esp,
#   0x204 (6 bytes).
# - 0x5fb3f0 host_country_compare_entries cdecl(a, b) -> int (o comparador do qsort 0x945333 e do bsearch). Prologo mov
#   edx, [esp+4]; push esi (5 bytes).
# Fica de fora o construtor 0x5f97c0 (HostCountry::HostCountry): ele espera o CD do jogo (GetLogicalDriveStrings,
# GetDriveType, a busca do cm0102.exe no CD e a caixa "Insert CD") e e diferente no cm0102-1080.exe (o teste do CD
# esta trocado); portar a espera do CD ou fazer uma copia por exe e decisao da sessao principal.
# Iguais no cm0102-1080.exe (as duas inteiras).
# Comparadores: as sedes so a original (sorteia), com o plano: tres sedes europeias distintas; o comparador lado a lado
# no valor (puro).
SAVE = False
ATE = '2002-08-01'
ROTINAS = [('pick_european_final_hosts', 0x5fad20, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
           ('host_country_compare_entries', 0x5fb3f0, [0x8b, 0x54, 0x24, 0x04, 0x56])]
DOCS = ('as sedes das finais europeias e a ordem da tabela de sedes', 'the European finals\' hosts and the host table order',
        [('pick_european_final_hosts', 'HostCountry::pick_european_final_hosts', '0x5fad20', 'host_country.cpp', 'thiscall, ret 0',
          'regra: as sedes das finais da Liga dos Campeões, da Copa da UEFA e da Recopa saem ao acaso entre as nações europeias desenvolvidas, de liga forte (padrão acima de 7) e com estádio nacional de mais de 40 mil lugares; com menos de três candidatas, a caixa de erro',
          'rule: the hosts of the Champions League, UEFA Cup and Cup Winners\' Cup finals are drawn at random among the developed European nations with a strong league (standard above 7) and a national stadium of more than 40,000 seats; with fewer than three candidates, the error box'),
         ('host_country_compare_entries', 'host_country_compare_entries', '0x5fb3f0', 'host_country.cpp', 'cdecl',
          'a ordem da tabela de sedes: pelo ano da edição e, no mesmo ano, pelo dia da decisão',
          'the host table order: by the edition year and, in the same year, by the decision day')])
CAMPOS = []
INTERESSA = []
