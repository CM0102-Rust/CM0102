# The club screens (6c): the club screen's action button (club_screens.cpp, part 6; reader tel06, 21/11/2026).
# - 0x482a40 add_club_screen_action_button cdecl(club, colour, colour_text) ret 0 -> 1/0; 0x804030 add_button_item
#   thiscall(12 dwords) ret 0x30; 0x838de0 cdecl(); 0x7f8420 thiscall -> eax; 0x7f8500 thiscall(club) ret 4;
#   0x5c68b0/0x5c6920/0x5c6830 thiscall on [0xae2c74].
# NEEDS A LINE IN rust/src/detours.rs: the prologue is push ebx/esi/edi (3 bytes) then call rel32 0x838de0, so the
#   detour needs the entry (0x482a40, 3) in RELATIVAS (the trampoline redoes the call); prologos.py checks it. Kept in
#   an item of its own so it can be left out alone (the sma_9 precedent left 0x6306e0 original for the same reason).
# Identical in cm0102-1080.exe. Depends on: tel26_1 (tel26_add_button, JOB_TEXT).
# Comparer: over the display list (compare_match_screen_build) and the return.
TELAS = True
ROTINAS = [('add_club_screen_action_button', 0x482a40, [0x53, 0x56, 0x57, 0xe8, 0x98, 0x63, 0x3b, 0x00])]
DOCS = ('as telas do clube (6c): o botão de ação da tela do clube', 'the club screens (6c): the club screen\'s action button',
        [('add_club_screen_action_button', 'add_club_screen_action_button', '0x482a40', 'club_screens.cpp', 'cdecl',
          'o botão de ação da tela do clube, o primeiro que valer: "Take Control" enquanto o humano cria o técnico (clube não humano e que pode ser dirigido), "Scout Club" (a busca atual sem clube, o clube fora das buscas, não seleção), convidar o clube para o amistoso, o pedido de datas ou o torneio do humano ("Uninvite Club" se já convidado) quando o humano não o dirige, "Apply for Job" (sem técnico e pode ser dirigido)',
          'the club screen\'s action button, the first that applies: "Take Control" while the human creates his manager (a club not human-run that can be managed), "Scout Club" (the current search without a club, the club not in the searches, not a national team), inviting the club to the human\'s friendly, dates request or tournament ("Uninvite Club" when already invited) when the human does not run it, "Apply for Job" (no manager and it can be managed)')])
