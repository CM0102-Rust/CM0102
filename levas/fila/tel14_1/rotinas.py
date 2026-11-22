# The tactics screen (tactics_screens.cpp, 1; reader tel14, the screens round, 21/11/2026): opening it, a team data
# block's destructor and leaving without applying.
# Conventions checked in the disassembly (0x899a50..0x899e99, 0x8a2c60..0x8a2c94): tactics_screen_open cdecl(club,
# reserve, match, arg4, record index, penalty mode), ret 0 (the constructor 0x8a7440 thiscall(this, club, match,
# reserve, arg4, record, penalty) ret 0x18; open_screen thiscall ret 0x14); Tactics_Team_Data::~Tactics_Team_Data
# thiscall, ret 0; tactics_screen_close_without_apply cdecl, ret 0.
# The same in the cm0102-1080.exe (whole routines).
# The other tel14 items (tel14_2 the build, tel14_3 the event handler) use the names of this one (tactics_screen::*,
# TACTICS_SCREENS_CPP, the *_FN addresses): this item goes first. Uses tel26_error_box and display_method of tel26_1.
# Defects of the original: tactics_screen_open reads READ_ONLY of a null object when the allocation fails.
TELAS = True
ROTINAS = [('tactics_screen_open', 0x899a50, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('tactics_team_data_destructor', 0x899e50, [0x6a, 0xff, 0x68, 0x58, 0x34, 0x96, 0x00]),
           ('tactics_screen_close_without_apply', 0x8a2c60, [0x56, 0x6a, 0x00, 0xb9, 0x50, 0x44, 0xb7, 0x00])]
DOCS = ('a tela de tática (1): abrir, o destrutor de um bloco de dados do time e sair sem aplicar',
        "the tactics screen (1): opening it, a team data block's destructor and leaving without applying",
        [('tactics_screen_open', 'tactics_screen_open', '0x899a50', 'tactics_screens.cpp', 'cdecl',
          'abre a tática de um clube: sem clube, a caixa de erro da linha 0x17c; arg4 e o modo dos pênaltis juntos, a da linha 0x183. Senão constrói o objeto da tela (0xc4e2 bytes) a partir do clube, da partida, da marca de reservas, do índice do registro da partida e do modo dos pênaltis e abre a tela (a montagem das reservas pelo salto 0x89f700 quando a marca vem ligada); só de leitura, como tela repetível que sai sem aplicar; senão, de instância única que aplica ao fechar. Se a tela não abre, destrói e libera o objeto; se abre, os parâmetros 0 a 10 (o objeto, o clube, a partida, as reservas, arg4, 0, 0, o registro, os pênaltis, -1 e a coluna inicial: 6, a nota da partida, numa partida; senão 2, a condição)',
          "opens a club's tactics: no club shows the error box of line 0x17c; arg4 together with the penalty mode, the box of line 0x183. Otherwise it builds the screen object (0xc4e2 bytes) from the club, the match, the reserve flag, the match record index and the penalty mode and opens the screen (the reserve builder through the 0x89f700 thunk when the flag is set); read-only, as a repeatable screen that leaves without applying; otherwise single-instance, applying on close. If the screen does not open, the object is destroyed and freed; if it opens, parameters 0 to 10 (the object, the club, the match, the reserve flag, arg4, 0, 0, the record, the penalty mode, -1 and the first column: 6, the match rating, in a match; else 2, the condition)"),
         ('tactics_team_data_destructor', 'Tactics_Team_Data::~Tactics_Team_Data', '0x899e50', 'tactics_screens.cpp', 'thiscall',
          'destrói um bloco de dados do time da tela: o destrutor de cada um dos 20 registros de jogador da partida (0x1be bytes)',
          'destroys a team data block of the screen: the destructor of each of its 20 match player records (0x1be bytes)'),
         ('tactics_screen_close_without_apply', 'tactics_screen_close_without_apply', '0x8a2c60', 'tactics_screens.cpp', 'cdecl',
          'sai da tela sem gravar as mudanças: destrói e libera o objeto do parâmetro 0, quando há, e zera o parâmetro',
          'leaves the screen without writing the edits back: destroys and frees the object of parameter 0, when there is one, and clears the parameter')])
