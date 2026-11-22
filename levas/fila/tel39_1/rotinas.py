# As ultimas pecas do perfil da pessoa (staff_screens.cpp, 0x88b910..0x88c360; leitor tel39, rodada 2 das telas,
# 22/11/2026): o menu do elenco e do historico, a ordem das linhas de texto, o dialogo do apelido e o pedido de
# observacao.
# Convencoes conferidas na desmontagem (cada push contra o esp e o ret de cada rotina):
# - 0x88b910 staff_screens_add_squad_and_history_menu cdecl(pessoa, lado do clube (byte), cor, cor do texto,
#   tratador do elenco, tratador do historico) (6 argumentos: o 5o em [esp+0x3c] e o 6o em [esp+0x40] na entrada do
#   item); quadro SEH pela LLIST local (0x68d020/0x68d040). O eax de saida e o que o destruidor da lista deixa
#   (nenhum chamador conhecido o usa; aqui 0). Date::get_month 0x54cfb0 e get_day_of_month 0x54cf70 sao thiscall sem
#   argumento: os pushes antes delas sao os argumentos do sprintf.
# - 0x88bc50 staff_screens_compare_text_rows cdecl(a, b) -> eax; por endereco (qsort) em 0x879b93, 0x879ba6,
#   0x879bb9, 0x87a779, 0x87adb7.
# - 0x88bc70 build_nickname_dialog e 0x88c100 build_scout_request_dialog cdecl(); 0x88bee0 e 0x88c360 cdecl(widget)
#   -> eax (os tratadores dos dialogos abertos pelo perfil).
# - Chamadas: build_squad_list_for_staff_history 0x484980 (Rust), history_his_file_exists 0x5f2500, sprintf 0x944c9f,
#   database_format_staff_name 0x53dba0, utils_string_compare 0x90c1d0, get_heading_colour 0x839590,
#   gui_add_text_area 0x5ed070 (aqui com 14 argumentos: o vinculo do nome), HUMAN_MANAGER 0x6003d0 (ret 8),
#   0x600740, 0x600870 e 0x600090 (ret 0xc), SCOUT_MANAGER 0x7f9290 (ret 8), free da CRT 0x9452ca.
# Usa do motor.rs: squad_menu_section e squad_menu_entry (o menu do elenco do clube, tel06_2), tel26_*, tel36_langlib,
# tel36_staff_param, manager_menu_bar, JOB_COLOUR_BDF0, compare_match_screen_build e compare_by_display_list_with.
# Comparadores: o menu e os dois dialogos lado a lado sobre a lista de exibicao; a ordem lado a lado (o sinal);
# nos tratadores, digitar o apelido (0x32) e o No do pedido lado a lado sobre os parametros; guardar/apagar o apelido,
# liberar o buffer, o pedido de observacao e a barra de menu so a original.
# Defeito do original: o dialogo do apelido le strlen do buffer sem testar se o parametro 1 e nulo.
TELAS = True
ROTINAS = [('staff_screens_add_squad_and_history_menu', 0x88b910, [0x6a, 0xff, 0x68, 0xd8, 0x2f, 0x96, 0x00]),
           ('staff_screens_compare_text_rows', 0x88bc50, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0x4c, 0x24, 0x04]),
           ('staff_screens_build_nickname_dialog', 0x88bc70, [0x51, 0x55, 0x56, 0x57, 0x6a, 0x00]),
           ('staff_screens_handle_nickname_dialog_action', 0x88bee0, [0x51, 0x53, 0x55, 0x56, 0x57]),
           ('staff_screens_build_scout_request_dialog', 0x88c100, [0x81, 0xec, 0x9c, 0x01, 0x00, 0x00]),
           ('staff_screens_handle_scout_request_action', 0x88c360, [0x66, 0x8b, 0x15, 0x7a, 0x7c, 0xb7, 0x00])]
DOCS = ('as últimas peças do perfil: o menu do elenco e do histórico, a ordem das linhas, o diálogo do apelido e o pedido de observação',
        "the profile's last pieces: the squad and history menu, the rows order, the nickname dialog and the scout request",
        [('staff_screens_add_squad_and_history_menu', 'staff_screens_add_squad_and_history_menu', '0x88b910', 'staff_screens.cpp', 'cdecl',
          'o botão de menu do perfil: o elenco do clube (ou da seleção) da pessoa para ir direto a outro jogador (até 28 nomes, 26 com histórico) e "Histórico" quando a pessoa tem um histórico escrito (a chave "nome sobrenome d.m.aaaa")',
          'the profile\'s menu button: the squad of the person\'s club (or national side) to jump to another player (up to 28 names, 26 with a history) and "History" when the person has a written history (the key "first second d.m.yyyy")'),
         ('staff_screens_compare_text_rows', 'staff_screens_compare_text_rows', '0x88bc50', 'staff_screens.cpp', 'cdecl',
          'a ordem de duas linhas de texto (utils_string_compare modo 1), para o qsort das listas do perfil',
          'the order of two text rows (utils_string_compare mode 1), for the qsort of the profile lists'),
         ('staff_screens_build_nickname_dialog', 'staff_screens_build_nickname_dialog', '0x88bc70', 'staff_screens.cpp', 'cdecl',
          'monta o diálogo do apelido: o nome como título, a caixa do apelido e Cancelar, Remover e Ok (Remover e Ok cinzentos sem apelido)',
          'builds the nickname dialog: the name as the title, the nickname entry and Cancel, Remove and Ok (Remove and Ok greyed with no nickname)'),
         ('staff_screens_handle_nickname_dialog_action', 'staff_screens_handle_nickname_dialog_action', '0x88bee0', 'staff_screens.cpp', 'cdecl',
          'trata o diálogo do apelido: digitar redesenha; Remover tira o apelido; Ok grava (troca ou acrescenta, salvo se for igual ao nome comum); Cancelar fecha; o buffer é liberado ao fechar; o resto vai para a barra de menu',
          'handles the nickname dialog: typing redraws; Remove takes the nickname away; Ok stores it (changing or adding, unless it equals the common name); Cancel closes; the buffer is freed on closing; the rest goes to the menu bar'),
         ('staff_screens_build_scout_request_dialog', 'staff_screens_build_scout_request_dialog', '0x88c100', 'staff_screens.cpp', 'cdecl',
          'monta o pedido de observação: avaliar o jogador agora (olheiros livres) ou observá-lo quando houver olheiro, nas cores do clube do técnico, com Não e Sim',
          "builds the scout request: assess the player now (scouts free) or watch him when scouts are available, in the manager's club colours, with No and Yes"),
         ('staff_screens_handle_scout_request_action', 'staff_screens_handle_scout_request_action', '0x88c360', 'staff_screens.cpp', 'cdecl',
          'trata o pedido de observação: Não fecha, Sim pede o relatório do olheiro e fecha; o resto vai para a barra de menu',
          'handles the scout request: No closes, Yes asks for the scout report and closes; the rest goes to the menu bar')])
