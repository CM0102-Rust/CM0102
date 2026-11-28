# Os jogos fixos das campanhas de 2002 das eliminatorias e a campanha sul-americana (wc_europe_league.cpp,
# wc_south_american_league.cpp; leitor r7_06, rodada 7, 28/11/2026). Estavam fora (cx17_7/FORA.md, rx08_11/FORA.md)
# por serem codigo morto, o motivo continua valendo e esta conferido na desmontagem: so os construtores (0x918200 e
# 0x9202b0, ja em Rust) chamam as cargas, e so quando o ano do jogo nao e anterior ao da campanha, que nunca e antes de
# 2004 (qualifying_campaign_year(ano).max(0x7d4)); os construtores so sao chamados pelo setup_set_game_start_date
# (0x830990/0x830a56) com o ano da data de inicio (2001 ou 2002). So um exe com o ano de inicio mudado chega aqui: o
# YearChanger do CM0102Patcher, em anos 4k+1 (2005, 2009...); nos anos 4k ele mesmo pula a chamada (0x918473 e 0x9204b8
# viram jmp). Escritas porque a rodada 5 pede o exe inteiro (destino: o exe independente).
# Tipo: TELAS, so para entrar numa leva sem mutacoes pela carreira (nenhuma carreira de teste nem o Lucas jogando
# passam por aqui: a comparacao nunca dispara). A prova e a emulacao (unicorn: original e copia da DLL compilada no
# mesmo mundo falso), ver PRONTO.txt. Se a sessao principal preferir, segurar no SEGURADOS.txt.
# - 0x91db60 WcEuropeLeague::add_fixed_result thiscall(fase, dia, mes, ano, casa, gols, fora, gols) ret 0x20; quadro
#   SEH de 0x464 (o registro do jogo na pilha, 0x4f bytes); caixas nas linhas 0xa32 (sem vetor de fases, fase depois
#   da atual ou fase sem objeto) e 0xa39 (ano fora de 2000..2001 ou fase fora de -1..7).
# - 0x922350 WcSouthAmericanLeague::add_fixed_fixture thiscall(os mesmos 8) ret 0x20; caixas 0x3c7 (fase depois da
#   atual ou fase sem objeto, o vetor lido sem conferir nulo) e 0x3ce (ano fora ou fase diferente de -1).
#   As duas: Date::Date 0x549f70 (dia, mes, ano, -1), Date::get_weekday 0x54a3f0 (5 ou 6 -> fatia 1, senao 2; chamado
#   de novo quando o primeiro nao e 5), database_get_national_team 0x53b3d0 (cdecl, nacao = [0xae23a8] + id x 0x122),
#   Stadium::set_fixture_venue 0x85c260 ([0xdd7ef4]; 0, registro, -1), a data contra G_GAME_START_DATE 0xdd6ec8
#   (depois dela: gols 0xff), FIXTURE_MANAGER::add_fixture 0x5aa680 ([0xae2a58]; registro, 1); com resultado, a
#   virtual +4 (registro, 0) e League::sort_table 0x6827d0 (0) da fase (a europeia) ou da secao; a europeia termina
#   sempre com WcEuropeLeague::fill_runners_up_table 0x91a240.
# - 0x9216d0 WcSouthAmericanLeague::load_2002_campaign thiscall ret 0: virtual +0x5c, +0x45 = 1 e 90 chamadas a
#   0x922350 (a tabela foi tirada da desmontagem por script: rascunho r7_06\extrai_r7_06.py).
# Defeito do original (as duas que montam o jogo): leem a palavra das regras do elenco (+0x4d) do registro na pilha
# antes de escreve-la e guardam os bits 12..15 (o nibble alto de +0x4e) da pilha nao iniciada; a copia deixa 0.
# Prologos: mov eax, fs:[0] (6 bytes, sem relativo) nas duas que montam o jogo; push esi; mov esi, ecx; mov eax, [esi]
# (5 bytes) na carga. As tres iguais no cm0102-1080.exe (inteiras).
TELAS = True
ROTINAS = [('wc_europe_league_add_fixed_result', 0x91db60, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('wc_south_american_league_add_fixed_fixture', 0x922350, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('wc_south_american_league_load_2002_campaign', 0x9216d0, [0x56, 0x8b, 0xf1, 0x8b, 0x06])]
DOCS = ('os jogos fixos das campanhas de 2002 das eliminatorias e a campanha sul-americana',
        'the fixed matches of the 2002 qualifying campaigns and the South American campaign',
        [('wc_europe_league_add_fixed_result', 'WcEuropeLeague::add_fixed_result', '0x91db60', 'wc_europe_league.cpp', 'thiscall, ret 0x20',
          'regra: um jogo real das eliminatorias europeias de 2002 entra no calendario com o placar; se ja foi jogado ate a data de inicio do jogo, a tabela do grupo recebe o resultado; a tabela dos segundos colocados e refeita sempre. O registro do jogo: a competicao da secao, o ano da temporada, o prefixo da rodada da fase (ou da secao na fase -1), as regras e a categoria da secao, reservas e substituicoes da secao, a data (fatia 1 no fim de semana, 2 no meio), as selecoes das duas nacoes, o estadio pelo Stadium; data depois do inicio do jogo: sem gols. Caixas: sem fases, fase depois da atual ou sem objeto (linha 0xa32); ano fora de 2000-2001 ou fase fora de -1..7 (0xa39)',
          "rule: a real match of the 2002 European qualifying goes into the calendar with its score; when it was played by the game's start date, the group table takes the result; the runners-up table is redone every time. The match record: the section's competition, the season year, the round prefix of the stage (or of the section for stage -1), the section's rules and category, its substitutes and substitutions, the date (slice 1 at the weekend, 2 midweek), the two nations' national teams, the venue from the Stadium; a date after the game's start: no goals. Boxes: no stages, a stage after the current one or without an object (line 0xa32); a year outside 2000-2001 or a stage outside -1..7 (0xa39)"),
         ('wc_south_american_league_add_fixed_fixture', 'WcSouthAmericanLeague::add_fixed_fixture', '0x922350', 'wc_south_american_league.cpp', 'thiscall, ret 0x20',
          'regra: um jogo real das eliminatorias sul-americanas de 2002 entra no calendario com o placar; se ja foi jogado ate a data de inicio do jogo, a tabela da secao recebe o resultado. O registro como o europeu, com o prefixo da secao e o indice de fase 0xff. Caixas: fase depois da atual ou sem objeto (0x3c7); ano fora de 2000-2001 ou fase diferente de -1 (0x3ce)',
          "rule: a real match of the 2002 South American qualifying goes into the calendar with its score; when it was played by the game's start date, the section's table takes the result. The record as the European one, with the section's round prefix and stage index 0xff. Boxes: a stage after the current one or without an object (0x3c7); a year outside 2000-2001 or a stage other than -1 (0x3ce)"),
         ('wc_south_american_league_load_2002_campaign', 'WcSouthAmericanLeague::load_2002_campaign', '0x9216d0', 'wc_south_american_league.cpp', 'thiscall',
          'regra: as eliminatorias sul-americanas de um jogo cujo ano ja chegou ao da campanha carregam a campanha real de 2002: as posicoes previstas, a secao ativa e os 90 jogos de marco de 2000 a novembro de 2001 com os placares conhecidos quando o jogo foi feito (os 15 das tres ultimas rodadas sem placar). Codigo morto no jogo como ele e: a campanha nunca e antes de 2004 e o jogo comeca em 2001',
          "rule: the South American qualifying of a game whose year has reached the campaign's loads the real 2002 campaign: the predicted positions, the section active and the 90 matches of March 2000 to November 2001 with the scores known when the game was made (the 15 of the last three rounds without a score). Dead code in the game as it is: the campaign is never before 2004 and the game starts in 2001")])
