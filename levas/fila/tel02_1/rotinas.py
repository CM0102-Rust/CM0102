# The club screen's next match and fixtures tabs and the Spanish badge pictures (club_screens.cpp, part 2; reader
# tel02, the screens round, 21/11/2026).
# Conventions checked in the disassembly and at the call sites:
# - 0x45ccf0 club_screen_draw_next_match_tab cdecl(top, bottom) ret 0, SEH frame (Text 0x4ae630, LLIST 0x68d040 x2);
#   9.8 KB up to the ret at 0x45f362 (the map's 2932 bytes are too short); the team news jump table at 0x45f364
#   (cases 0..4, 5 = default) is read from the exe. Five stdcall routines are called with ecx loaded and are ported as
#   thiscall with that ecx (same stack). No random generator.
# - 0x45f380 club_screen_draw_fixtures_tab cdecl(top, bottom) ret 0, SEH frame; it ALLOCATES the day list (operator new
#   0xe, LLIST) the first time and keeps it in screen param 0xd. The cm0102-1080.exe changes the imm32 of push 0x257
#   (0x46041c) and push 0x31f (0x460421): the copy reads both from the exe.
# - 0x460450 find_club_badge_picture_path cdecl(club, out) -> 0/1 (opens the picture "rb" and closes it).
# Comparers: the two tabs side by side over the display list (compare_by_display_list_with); fixtures with the list's
# 0xe bytes, and only the original when param 0xd = 0 (it allocates); the badge side by side on the answer and `out`.
# DEFECTS of the original kept (candidates for docs/DEFEITOS_DO_ORIGINAL.md):
# - next match: the first pass reads the selection limit struct before transfer_manager_get_selection_limit fills it
#   (stack leftover; the layout does not change); the blank cell before the non-EU and over-age limits can never be
#   drawn (one limit row at most);
# - fixtures: the shown year (param 0xc) goes down as a dword (+0xffff), its high word gaining 1, and goes that way to
#   key_nation_format_club_season and Date::Date; the row count goes as the low byte of a dword whose high bytes are
#   the key nation entry pointer's; a failed operator new reads [0+2]; "<<" is enabled when the shown year is past the
#   first year with fixtures OR past the current year.
TELAS = True
ROTINAS = [('club_screen_draw_next_match_tab', 0x45ccf0, [0x6a, 0xff, 0x68, 0x48, 0x5a, 0x95, 0x00]),
           ('club_screen_draw_fixtures_tab', 0x45f380, [0x6a, 0xff, 0x68, 0x84, 0x5a, 0x95, 0x00]),
           ('find_club_badge_picture_path', 0x460450, [0x51, 0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x14])]
DOCS = ('a tela do clube (1): as abas do próximo jogo e dos jogos e os escudos dos clubes espanhóis',
        "the club screen (1): the next match and fixtures tabs and the Spanish clubs' badges",
        [('club_screen_draw_next_match_tab', 'club_screen_draw_next_match_tab', '0x45ccf0', 'club_screens.cpp', 'cdecl',
          'a aba do próximo jogo da tela do clube: sem jogo, a mensagem de que não há jogos (de reservas, de seleção B); com jogo, as setas para o jogo anterior (até 24 semanas para seleções, 8 para clubes, botão só se o último ainda não foi jogado) e o seguinte, o Progress (liga que não é de reservas), o Past Meetings, o adversário com (Home)/(Away), e a tabela montada em duas passadas (a primeira só conta as linhas para decidir a largura com a barra de rolagem): a fase (link para a competição), a data (hoje, amanhã, em N dias), o estádio e a cidade, o agregado ou a série melhor de três, o limite de estrangeiros, não comunitários ou de idade, as regras do desempate (gols fora, prorrogação ou gol de ouro, pênaltis, desempate americano, novo jogo, o cabeça de chave que passa), os reservas, o último confronto, o tempo (até 2 semanas antes), o gramado (no dia), o árbitro com o rigor e as notícias dos dois times (seleção, inelegível, suspenso, machucado, em dúvida, estreia)',
          "the club screen's next match tab: with no fixture, the no forthcoming matches message (reserve, 'B' international); with one, the arrows to the previous fixture (up to 24 weeks back for national teams, 8 for clubs, a button only when the last one is unplayed) and the next one, Progress (a league that is not a reserve one), Past Meetings, the opponent with (Home)/(Away), and the table built in two passes (the first only counts the rows to choose the width with the scroll bar): the round (a link to the competition), the date (today, tomorrow, in N days), stadium and city, the aggregate or the best of three series, the foreign, non-EU or age squad limit, the tie rules (away goals, extra time or golden goal, penalties, US-style shoot-out, replay, the seeded team going through), the substitutes, the last meeting, the weather (within 2 weeks), the pitch (on match day), the referee with his strictness and both teams' news (international duty, ineligible, suspended, injured, doubtful, debut)"),
         ('club_screen_draw_fixtures_tab', 'club_screen_draw_fixtures_tab', '0x45f380', 'club_screens.cpp', 'cdecl',
          'a aba de jogos da tela do clube: a navegação de temporada (<< e >>, pelo primeiro e o último ano com jogos do clube; clube de temporada que atravessa o ano, antes do dia do fim da temporada da nação, mostra a temporada anterior), Cancelar e Marcar com o menu de amistoso em casa, fora, excursão e torneio (só no clube do humano, fora da vista só de leitura), o título da temporada e a lista dia a dia, do começo da temporada a um ano depois: os jogos do dia (o jogo seguinte do clube vira a linha para onde a lista rola) e, nos dias sem jogo, fora da vista só de leitura, o amistoso, a excursão e o torneio marcados, cada um com a sua marca de cancelar; a lista é criada e guardada no parâmetro 0xd na primeira vez; no fim rola até o jogo seguinte (-7) ou ao parâmetro 0x10, guarda a área no 0x11 e, cancelando, cobre a tela com um botão (id 0x19)',
          "the fixtures tab of the club screen: the season navigation (<< and >>, by the club's first and last years with fixtures; a club whose season spans two years shows the previous season before its nation's season end day), Cancel and Arrange with the home friendly, away friendly, tour and tournament menu (only for the human's club, outside the read-only view), the season caption and the day by day list from the season start to a year later: the day's fixtures (the club's next fixture becomes the row the list scrolls to) and, on days without a fixture, outside the read-only view, the arranged friendly, tour and tournament, each with its cancel mark; the list is created and kept in param 0xd the first time; at the end it scrolls to the next fixture (-7) or to param 0x10, keeps the area in param 0x11 and, while cancelling, covers the screen with a button (id 0x19)"),
         ('find_club_badge_picture_path', 'find_club_badge_picture_path', '0x460450', 'club_screens.cpp', 'cdecl',
          'o escudo de um clube espanhol: pelo nome em inglês (do próprio clube, ou do que a ligação com o time B devolve), a primeira entrada igual da tabela de escudos dá o arquivo em CM3_PICTURES; se ele abre, o caminho é copiado e a resposta é 1; senão 0',
          "a Spanish club's badge: by the English name (of the club itself, or of the one the B team link returns), the first equal entry of the badge table gives the file under CM3_PICTURES; if it opens, the path is copied and the answer is 1; otherwise 0")])
