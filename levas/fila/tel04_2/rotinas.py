# The friendly tour and the tournament settings (club_screens.cpp, part 4b; reader tel04, the screens round, 21/11/2026).
# Uses the helpers of tel04_1 (club_screens_*, past meetings constants) and of tel26_1: it goes after both.
# Conventions checked at the call sites (every push against esp):
# - 0x4719a0 open_friendly_tour_nation_screen cdecl(club) ret 0; open_screen(0x471a90, 0x472180, 1, 0, 0).
# - 0x471a90 friendly_tour_nation_screen_build cdecl(void), 0x22c of locals; the map's 0x472070 "fragment" is its tail
#   (Cancel and Next), to the ret at 0x472173. 0x944e46 malloc / 0x9452ca free cdecl; 0x945333 qsort(list, n, 4,
#   0x540080); 0x5c52d0 friendly_club_reputation_tier (ret 4, ecx = [0xae2c74] at the call: called as thiscall here).
# - 0x472180 friendly_tour_nation_screen_handle_event cdecl(short) ret 0; 0x4701f0 by address cdecl(club, 0xb, nation).
# - 0x472380 friendly_tour_club_screen_build cdecl(void); 0x472850 friendly_tour_club_screen_handle_event cdecl(short).
# - 0x472900 open_configure_tournament_screen cdecl(settings, arg): open_screen(0x472a00, 0x4735f0, 1, 0, 0), a new
#   0xd1-byte copy (rep movsd 0x34 + movsb, no null check).
# - 0x472a00 configure_tournament_screen_build cdecl(void): 0x5e4800 graphics_rgb_to_pixel cdecl 4 args; 0x41c0d0
#   background_set_picture cdecl(0); 0x803fd0 add_text_item thiscall 11 args ret 0x2c; 0x8ac600 draw_pitch cdecl 11
#   pushes (the last the float 4.0); the edit field add_item 0x80018 over the copy's name (15 characters).
# - 0x4735f0 configure_tournament_screen_handle_event cdecl(short) ret 0: ids 1..4 read inline, 5..7 by 0x468b10
#   Display::get_widget_id, 8 by 0x468b40 get_widget_action_id (the same id); the values by 0x4739a0.
# - 0x4739a0 Display::get_widget_data thiscall(display, short) ret 4.
# The 1080 exe changes the screen-wide area of 0x472a00 (0x31f/0x257 at 0x472adc/0x472ad7): read from the running exe.
# Defects of the original: none found.
# Comparers: the openers only the original, the params (and the settings copy) checked; the builds side by side by the
# display list (Configure: plus the copy, which a cup's build corrects, and the green made once); the inputs side by
# side for the clicks that only write params, the tour record's +0x103 or the settings copy; get_widget_data side by
# side by value.
# How to reach it: a club's fixtures > Arrange Tour (the nations: Filter Available Only / All Nations, choose, Next;
# Cancel); the tour's club choice (from the tour dates' Next when the nation's clubs are asked; Cancel); Arrange
# Tournament > Configure (every choice, the name field, Ok and Cancel).
TELAS = True
ROTINAS = [('open_friendly_tour_nation_screen', 0x4719a0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('friendly_tour_nation_screen_build', 0x471a90, [0x81, 0xec, 0x2c, 0x02, 0x00, 0x00]),
           ('friendly_tour_nation_screen_handle_event', 0x472180, [0x53, 0x55, 0x56, 0x8b, 0x74, 0x24, 0x10]),
           ('friendly_tour_club_screen_build', 0x472380, [0x81, 0xec, 0x2c, 0x02, 0x00, 0x00]),
           ('friendly_tour_club_screen_handle_event', 0x472850, [0x56, 0x57, 0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00]),
           ('open_configure_tournament_screen', 0x472900, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('configure_tournament_screen_build', 0x472a00, [0xa0, 0x40, 0xab, 0xad, 0x00]),
           ('configure_tournament_screen_handle_event', 0x4735f0, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('display_get_widget_data', 0x4739a0, [0x66, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('a excursão de amistosos e as definições do torneio (club_screens.cpp, parte 4)',
        'the friendly tour and the tournament settings (club_screens.cpp, part 4)',
        [('open_friendly_tour_nation_screen', 'open_friendly_tour_nation_screen', '0x4719a0', 'club_screens.cpp', 'cdecl',
          'abre a escolha da nação da excursão do clube (sem clube, a caixa de club_screens.cpp 12948): parâmetros clube, nação escolhida 0, rolagem e área 0, "só disponíveis" ligado',
          'opens the tour nation choice of the club (no club: the club_screens.cpp 12948 box): params club, chosen nation 0, scroll and area 0, "available only" on'),
         ('friendly_tour_nation_screen_build', 'friendly_tour_nation_screen_build', '0x471a90', 'club_screens.cpp', 'cdecl',
          'monta a escolha da nação: título nas cores do clube, o filtro (só disponíveis / todas), as nações com continente, importância até 2 e clubes (menos a do clube) em ordem de nome, duas por linha com o continente; a nação fechada pela regra da excursão (nível do clube abaixo de 4: só a própria região; abaixo de 5: só o próprio continente) fica cinza ou some com o filtro; a escolhida realçada; Cancelar e Avançar (só com nação)',
          'builds the nation choice: title in the club\'s colours, the filter (available only / all), the nations with a continent, importance up to 2 and clubs (but the club\'s own) by name, two per row with the continent; a nation closed by the tour rule (club tier below 4: own region only; below 5: own continent only) is grey or left out by the filter; the chosen one highlighted; Cancel and Next (only with a nation)'),
         ('friendly_tour_nation_screen_handle_event', 'friendly_tour_nation_screen_handle_event', '0x472180', 'club_screens.cpp', 'cdecl',
          'responde ao clique: guarda a rolagem; alterna o filtro; escolhe a nação; cancela; avança para as datas da excursão (tipo 0xb com a nação); o resto vai para a barra de menus',
          'answers a click: saves the scroll; toggles the filter; chooses the nation; cancels; goes on to the tour dates (kind 0xb with the nation); the rest goes to the menu bar'),
         ('friendly_tour_club_screen_build', 'friendly_tour_club_screen_build', '0x472380', 'club_screens.cpp', 'cdecl',
          'monta a escolha do clube da excursão: "Arrange Tour Of <nação>", Cancelar e Avançar, os clubes da nação (sem seleções) em ordem de nome, dois por linha com a seleção e a divisão ligadas',
          'builds the tour club choice: "Arrange Tour Of <nation>", Cancel and Next, the nation\'s clubs (no national teams) by name, two per row with the national team and the division linked'),
         ('friendly_tour_club_screen_handle_event', 'friendly_tour_club_screen_handle_event', '0x472850', 'club_screens.cpp', 'cdecl',
          'responde ao clique: guarda a rolagem; Cancelar zera o +0x103 do registro da excursão e sai; o resto vai para a barra de menus',
          'answers a click: saves the scroll; Cancel clears the tour record\'s +0x103 and leaves; the rest goes to the menu bar'),
         ('open_configure_tournament_screen', 'open_configure_tournament_screen', '0x472900', 'club_screens.cpp', 'cdecl',
          'abre a configuração do torneio sobre as definições (sem elas, a caixa de club_screens.cpp 13437): parâmetros as definições, a marca de torneio novo e uma cópia de trabalho dos 0xd1 bytes',
          'opens the tournament configuration over the settings (none: the club_screens.cpp 13437 box): params the settings, the new-tournament flag and a working copy of the 0xd1 bytes'),
         ('configure_tournament_screen_build', 'configure_tournament_screen_build', '0x472a00', 'club_screens.cpp', 'cdecl',
          'monta a configuração: o verde das escolhas feito uma vez; a copa ganha desempate e 4 times; sem a marca, uma área da tela inteira por baixo; a caixa, "Configure Tournament", Cancelar e Ok, o campo; o nome (campo de 15 letras) e a tabela: tipo (liga/copa), pontos por vitória (3/2), critério (saldo, gols pró, vitórias), prorrogação (nenhuma, 30 min, gol de ouro), desempate (nenhum/pênaltis), times (4/3); a escolha atual em amarelo, as que não valem para a copa em cinza',
          'builds the configuration: the choices\' green made once; a cup gets a decider and 4 teams; without the flag, a screen-wide area below; the box, "Configure Tournament", Cancel and Ok, the pitch; the name (a 15-character field) and the table: type (league/cup), points for a win (3/2), tie-break (goal difference, goals for, games won), extra time (none, 30 min, golden goal), decider (none/penalties), teams (4/3); the current choice in yellow, the ones that do not apply to a cup in grey'),
         ('configure_tournament_screen_handle_event', 'configure_tournament_screen_handle_event', '0x4735f0', 'club_screens.cpp', 'cdecl',
          'responde ao clique: Cancelar libera a cópia (e as definições de um torneio novo); Ok dá o nome "CM Cup" à cópia sem nome, copia sobre as definições e a libera; as escolhas gravam tipo, pontos, critério, prorrogação, desempate e times na cópia (refaz a tela); o resto devolve 0',
          'answers a click: Cancel frees the copy (and a new tournament\'s settings); Ok names an unnamed copy "CM Cup", copies it over the settings and frees it; the choices store the type, points, tie-break, extra time, decider and teams in the copy (rebuild); the rest returns 0'),
         ('display_get_widget_data', 'Display::get_widget_data', '0x4739a0', 'club_screens.cpp', 'thiscall',
          'devolve o valor do item da lista de exibição (0 para -1)',
          'returns the value of a display list widget (0 for -1)')])
