# O registro de uma partida jogada nas estatísticas da competição (Comp_Stats::record_match_result, comp_screens.cpp;
# leitor tel43, rodada 5, 23/11/2026). Item de simulação, uma rotina grande (9,3 KB) inteira, partida em pedaços com
# nome. Roda em toda partida da carreira: chamada por League::add_result (0x685f4a, já em Rust) e pela copa
# (0x51a299), com o resultado já aplicado.
# Prova pela desmontagem: começa em 0x49f570 (push -1; push 0x9566e7, o quadro de exceção), termina no ret 0x10 de
# 0x4a19e3 (nops até 0x4a19f0, o início de update_season_end_records); sem tabela de saltos.
# Convenções conferidas: thiscall(stats, fixture, match record, pontos da casa, pontos de fora) ret 0x10; os pontos são
# dwords de que só o short é lido (bx). Chamadas: Date::Date 0x54a110 thiscall; fill_match_record_from_fixture 0x7e4750
# cdecl(fixture, club, out); find_club_entry 0x4a4850 / add_club_entry 0x4aceb0 thiscall ret 4; match_record_result_code
# 0x7e56d0 cdecl (al); Friendly::is_friendly_competition 0x5c6710 thiscall(friendly, comp record) ret 4;
# comp_util_is_league_division 0x4b0700 cdecl(comp id); Date::operator= 0x417c10 thiscall(dst, tmp, src) ret 8;
# match_record_assign 0x448110 thiscall ret 4 (o original copia em linha os mesmos campos); match_record_is_bigger_win
# 0x7e4a30 e match_record_has_more_goals 0x7e4be0 cdecl(a, b); utils_random 0x90cfc0 cdecl; Media::post_team_media_story
# 0x74b060 thiscall(media, club, kind, person, fixture, variant, -1) ret 0x18; FinanceTable::get_club_finance 0x5a0590
# thiscall(table, club) ret 4; news_post_fa_disciplinary_fine 0x58a4d0 cdecl(club, char level, amount) (fora da base:
# FN_ no codigo.rs); Finance::pay_league_fine 0x599850 thiscall ret 4; find_player_entry 0x4a4a30 ret 4 /
# add_player_entry 0x4ad030 ret 8; fill_staff_name_ref 0x7e4920 cdecl; Date::get_difference 0x54cc30 thiscall(date, out,
# dword, dword) ret 0xc; accumulate_match_player_totals 0x4a25f0 thiscall ret 4. As cópias em linha dos recordes de
# sempre conferidas campo a campo contra attendance_record_assign, run_record_assign, comp_run_record_assign,
# record_holder_assign e player_age_record_assign (as mesmas sub-cópias).
# Prólogo: push -1; push 0x9566e7 (7 bytes, fronteira de instrução; sem call/jmp relativo; sem RELATIVAS).
# Defeitos do original: (1) a multa da federação: o nível são 3 bits com sinal e o aumento testa "< 4", sempre
# verdadeiro: o nível 3 vira 4, lido de volta como -4, e o clube nunca mais é multado; (2) no nível 3 o nível vai para
# 0x58a4d0 num dword cujo resto a rotina nunca escreveu (a chamada só lê o byte). Lembrete: a história da mídia passa a
# variante num dword com o índice da busca nos bytes altos (post_team_media_story só lê o byte).
ROTINAS = [('comp_stats_record_match_result', 0x49f570, [0x6a, 0xff, 0x68, 0xe7, 0x66, 0x95, 0x00])]
DOCS = ('as estatísticas das competições: o registro de uma partida jogada',
        "the competition statistics: recording a played match",
        [('comp_stats_record_match_result', 'Comp_Stats::record_match_result', '0x49f570', 'comp_screens.cpp', 'thiscall',
          'cada partida jogada entra nas estatísticas da competição: para cada clube, mais um jogo, o público em casa, a forma (geral, em casa, fora), as sequências (vitórias, derrotas, invicta, sem vencer), gols, cartões e totais do time; para cada jogador que entrou, jogos, gols, pênaltis, assistências, cartões, melhor em campo, nota, passes, desarmes, dribles, chutes e minutos. Os recordes da temporada (público maior e menor, maior goleada, jogo com mais gols, sequências, pontos, gols, cartões, artilheiro, gols numa partida, assistências, melhor em campo, cartões de jogador, mais novo e mais velho) passam aos de sempre quando os batem, com notícia a partir do terceiro fim de temporada (quase todos só em divisão de liga). O fim de uma sequência sem vencer (por vitória) ou invicta (por derrota) de mais de 10 jogos pode virar história da mídia, e os cartões de um clube rendem multa da federação acima do limite do seu nível (65, 90, 115 e 140 pontos, amarelo 1 e vermelho 4)',
          "every played match goes into the competition statistics: for each club, one more game, the home crowd, the form (all, home, away), the runs (wins, defeats, unbeaten, winless), goals, cards and team totals; for each player who took part, games, goals, penalties, assists, cards, man of the match, rating, passes, tackles, dribbles, shots and minutes. The season's records (highest and lowest attendance, biggest win, highest scoring match, runs, points, goals, cards, top scorer, goals in a match, assists, man of the match, player cards, youngest and oldest) pass to the all-time ones when they beat them, with news from the third season end (most only in a league division). The end of a winless run (by a win) or an unbeaten run (by a defeat) of more than 10 games may become a media story, and a club's cards bring an FA fine above its level's threshold (65, 90, 115 and 140 points, yellow 1 and red 4)")])
