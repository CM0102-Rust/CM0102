# As convocações para o jogo de seleções e duas notícias (national_teams.cpp, Cup.cpp):
# NationalTeams::select_squads_for_fixture (0x76bab0, 1.363 chamadas na cobertura de 17/10: a cada jogo de seleção
# perto da data, pelo fix_man 0x5aa680 e pela 0x76b100; pede o elenco 0x76c120, 1.007 chamadas, quando o último tem
# mais de 14 dias ou não está marcado), national_teams_post_player_called_up_news (0x76fd10, 106 chamadas: a notícia
# do convocado; passageira) e Cup::add_team_result_news (0x519690, 1.148 chamadas: a virtual +0x44 da copa e a
# notícia do desfecho; passageira). A 0x76bab0 chama a 0x54cab0 (original, a data duas semanas antes), a escolha do
# elenco 0x76c120 (original, aloca e mexe nos elencos) e rotinas em Rust (datas, 0x5409b0, 0x76d880, 0x76c060); a
# 0x76fd10 e a 0x519690 só rotinas em Rust (a virtual da copa pode ser original). Quadros SEH sem destrutor (a data
# local e as notícias locais, como na original). Iguais no cm0102-1080.exe.
# Comparadores: a 0x76bab0 com a original antes e a cópia depois em ensaio (sem 0x76c120, 0x76d880 e 0x76c060; da
# 0x76d880, só o byte +0x6b que ela zera) nos registros das duas nações (+0x32..+0x6c); quando a cópia pede um
# elenco, vale a original. A 0x76fd10 com a cópia em ensaio (sem entregar) no [0xae1080]; a 0x519690 só a original
# (a virtual acrescenta registros, as entregas alocam; a cópia leva o ramo).
SAVE = False
ROTINAS = [('convocar_selecoes_para_o_jogo', 0x76bab0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_do_convocado', 0x76fd10, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('noticia_do_desfecho_na_copa', 0x519690, [0x6a, 0xff, 0x68, 0xdb, 0x75, 0x95, 0x00])]
DOCS = ('as convocações para o jogo de seleções e duas notícias: escolher os elencos dos dois lados, a notícia do convocado e a do desfecho na copa',
        "the call-ups for an international match and two news items: picking both sides' squads, the called-up player news item and the cup result news item",
        [('convocar_selecoes_para_o_jogo', 'NationalTeams::select_squads_for_fixture', '0x76bab0', 'national_teams.cpp', 'thiscall, ret 4',
          'a partir de duas semanas antes do jogo, para cada lado: elenco principal, B/olímpico ou sub-21 de até 14 dias e marcado só leva a data até o jogo; senão pede um novo (0x76c120); dispensa os acima da idade do sub-21 e apaga os repetidos',
          'from two weeks before the match, for each side: a senior, B/Olympic or under-21 squad at most 14 days old and flagged only moves its date up to the match; otherwise asks for a new one (0x76c120); releases the over-age under-21 players and clears duplicates'),
         ('noticia_do_convocado', 'national_teams_post_player_called_up_news', '0x76fd10', 'national_teams.cpp', 'stdcall, ret 0xc',
          'com técnico humano no clube do jogador, a notícia 0x21 (o jogador, a seleção e a vaga) ao técnico do clube +0x39 e ao do clube do contrato',
          "with a human manager at the player's club, news item 0x21 (the player, the national side and the slot) to the manager of club +0x39 and of the contract club"),
         ('noticia_do_desfecho_na_copa', 'Cup::add_team_result_news', '0x519690', 'Cup.cpp', 'thiscall, ret 0x14',
          'fora dos amistosos, a virtual +0x44 da competição e a notícia 0x10 (clube, competição, desfecho, categoria do resultado, o jogo, a rodada seguinte), entregue pelo alcance, à região da competição (final e semifinal) ou ao técnico do clube',
          'outside friendlies, the competition virtual +0x44 and news item 0x10 (club, competition, outcome, result category, the match, the next round), delivered by scope, to the competition region (final and semi-final) or to the club manager')])
CAMPOS = [('registro da nação (NationalTeams +4, 0x6e bytes) +0x32/+0x46/+0x5a: a marca de hoje dos elencos principal, B e sub-21; valor (data); grava (convocar_selecoes_para_o_jogo)',
           'nation record (NationalTeams +4, 0x6e bytes) +0x32/+0x46/+0x5a: the today stamp of the senior, B and under-21 squads; value (date); writes (convocar_selecoes_para_o_jogo)'),
          ('registro da nação +0x3a/+0x4e/+0x62: a data dos elencos; valor (data); lê e grava (convocar_selecoes_para_o_jogo)',
           'nation record +0x3a/+0x4e/+0x62: the squad dates; value (date); reads and writes (convocar_selecoes_para_o_jogo)'),
          ('registro da nação +0x43/+0x57/+0x6b: a marca de elenco convocado; valor; lê (convocar_selecoes_para_o_jogo)',
           'nation record +0x43/+0x57/+0x6b: the squad called-up flag; value; reads (convocar_selecoes_para_o_jogo)'),
          ('partida +0x28/+0x2a: a data do jogo; +0x40 a marca; +0x4d/+0x4e os bits do jogo (1 sem convocar, 2 sem dispensar, 0x100 B, 0x200 sub-21); valor; lê (convocar_selecoes_para_o_jogo)',
           'fixture +0x28/+0x2a: the match date; +0x40 the flag; +0x4d/+0x4e the match bits (1 no call-up, 2 no release, 0x100 B, 0x200 under-21); value; reads (convocar_selecoes_para_o_jogo)'),
          ('NationalTeams +0x36: o vetor das nações com convocados (byte por id); ponteiro; lê (convocar_selecoes_para_o_jogo)',
           'NationalTeams +0x36: the array of nations with called-up players (byte per id); pointer; reads (convocar_selecoes_para_o_jogo)'),
          ('[0xae1080] o último contrato lido; ponteiro; grava (noticia_do_convocado)',
           '[0xae1080] the last contract read; pointer; writes (noticia_do_convocado)'),
          ('pessoa +0x39: o clube da seleção do jogador; ponteiro; lê (noticia_do_convocado)',
           "staff +0x39: the player's national-side club; pointer; reads (noticia_do_convocado)"),
          ('copa +0x3c / +0xa3: as rodadas e o vetor delas (0x68 bytes, +7 a rodada); valor e ponteiro; lê (noticia_do_desfecho_na_copa)',
           'cup +0x3c / +0xa3: the rounds and their array (0x68 bytes, +7 the round); value and pointer; reads (noticia_do_desfecho_na_copa)')]
INTERESSA = ['CMRTE']
