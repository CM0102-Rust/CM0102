# O jogo entre seleções (national_teams.cpp, comp.cpp): NationalTeams::record_international_result (0x76b600, 5.160
# chamadas na cobertura de 17/10, pela 0x6b5f00 a cada jogo do dia; o ramo dos jogos internacionais roda: a primeira
# convocação 0x86cc50, só por ele, conta 128..256 na base da 257ª), o nome do elenco da seleção (0x775610, passageira:
# telas e textos) e as rodadas com jogos conhecidos (0x48d410, passageira, virtual da competição). A 0x76b600 chama
# rotinas já em Rust (0x53b900, 0x5409b0, 0x86cc50, 0x8ce920) e três originais: a data igual (0x4a7f20, pura), o
# ranking da FIFA (0x58cf70) e a notícia aos clubes (0x7706c0). Sem SEH nas três. Iguais no cm0102-1080.exe.
# Comparadores: a 0x76b600 em ensaio (a cópia sem 0x58cf70, 0x86cc50, 0x8ce920 e 0x7706c0) lado a lado com a original
# nas regiões que ela grava por conta própria (jogos da nação, +0x44 dos tipos, +0x1a..+0x24 das pessoas, +0xc/+0xd do
# registro geral); a 0x775610 lado a lado na saída e na devolução; a 0x48d410 pelo valor.
SAVE = False
ROTINAS = [('resultado_entre_selecoes', 0x76b600, [0x83, 0xec, 0x60, 0x56, 0x57]),
           ('nome_do_elenco_da_selecao', 0x775610, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('rodada_com_jogos_conhecidos', 0x48d410, [0x8b, 0x54, 0x24, 0x04, 0x8b, 0xc1])]
DOCS = ('o jogo entre seleções: as convocações, os jogos e gols pela seleção, o nome do elenco e as rodadas conhecidas',
        'the international match: call-ups, caps and international goals, the squad label and the known rounds',
        [('resultado_entre_selecoes', 'NationalTeams::record_international_result', '0x76b600', 'national_teams.cpp', 'thiscall, ret 8',
          'entre duas seleções: conta o jogo de cada nação; pela competição, marca o registro geral, troca a nação ou, nos jogos oficiais, soma o ranking da FIFA e os jogos e gols pela seleção de quem jogou; zera o +0x44 do tipo na data dele e manda a notícia aos clubes',
          "between two national sides: counts each nation's match; by competition, marks the general record, swaps the nationality or, in real internationals, scores the FIFA ranking and adds caps and goals to everyone who played; clears the type's +0x44 on its date and sends the news to the clubs"),
         ('nome_do_elenco_da_selecao', 'NationalTeams::format_squad_label', '0x775610', 'national_teams.cpp', 'thiscall, ret 0x14',
          'escreve o nome do elenco (Main, B, Under 21 ou Under 23), em minúsculas quando pedido',
          'writes the squad name (Main, B, Under 21 or Under 23), in lower case when asked'),
         ('rodada_com_jogos_conhecidos', 'Comp::are_round_fixtures_known', '0x48d410', 'comp.cpp', 'thiscall, ret 8',
          'a fase é liga, ou a rodada é antes da atual, ou é a atual e última com o código 0x8c',
          'the stage is a league, or the round is before the current one, or is the current last round with code 0x8c')])
CAMPOS = [('pessoa ([0xae23bc] + 0x6e × id) +0x22: os jogos pela seleção; valor; lê e grava +1 (resultado_entre_selecoes)',
           'person ([0xae23bc] + 0x6e x id) +0x22: international caps; value; reads and writes +1 (resultado_entre_selecoes)'),
          ('pessoa +0x23: os gols pela seleção; valor; lê e grava + os da vaga (+0x21) (resultado_entre_selecoes)',
           'person +0x23: international goals; value; reads and adds the slot goals (+0x21) (resultado_entre_selecoes)'),
          ('pessoa +0x1a / +0x1e: a nação e a segunda nação; ponteiro; lê e troca (resultado_entre_selecoes)',
           'person +0x1a / +0x1e: nationality and second nationality; pointer; reads and swaps (resultado_entre_selecoes)'),
          ('registro geral ([0xae2d0c] + 0x4f × id) +0xc bit 2 e +0xd; valor; grava (resultado_entre_selecoes)',
           'general record ([0xae2d0c] + 0x4f x id) +0xc bit 2 and +0xd; value; writes (resultado_entre_selecoes)'),
          ('registro da nação nas seleções ([NationalTeams+4] + 0x6e × id) +2: os jogos da nação; valor; grava +1; +0x3a + 20 × tipo a data do tipo; lê; +0x44 + 20 × tipo; valor; grava 0 (resultado_entre_selecoes)',
           "nation record in the national teams ([NationalTeams+4] + 0x6e x id) +2: the nation's match count; value; writes +1; +0x3a + 20 x type the type date; reads; +0x44 + 20 x type; value; writes 0 (resultado_entre_selecoes)"),
          ('resumo do jogo + 0x625 × lado + 0x190 + 0x49 × vaga: +0 id da pessoa, +0x21 gols, +0x32 entrou (reserva); +0x153 a vaga do destaque; valor; lê (resultado_entre_selecoes)',
           'match summary + 0x625 x side + 0x190 + 0x49 x slot: +0 person id, +0x21 goals, +0x32 came on (sub); +0x153 the star slot; value; reads (resultado_entre_selecoes)'),
          ('competição +0x30 / +0xc / +0x43 / +0xab / +0x3c / +0xa3: as fases, o tipo, a rodada atual, as rodadas e os registros de 0x68 bytes (+7 o código); lê (rodada_com_jogos_conhecidos)',
           'competition +0x30 / +0xc / +0x43 / +0xab / +0x3c / +0xa3: stages, type, current round, rounds and 0x68-byte round records (+7 the code); reads (rodada_com_jogos_conhecidos)')]
INTERESSA = ['CMRTE', 'CMScout']
