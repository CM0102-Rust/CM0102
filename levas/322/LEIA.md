# 322ª leva — as últimas peças do perfil: o menu do elenco e do histórico, a ordem das linhas, o diálogo do apelido e o pedido de observação; as miudezas das telas: as opções das fotos de fundo, a cópia de um Text, o destruidor de uma lista, os botões das notícias de contrato, a tela de espera, o destruidor de uma data estática e o registro dos filtros da busca; o gerente de telas: os registros dos jogadores e técnicos, a sessão de rede, os pacotes de clique e de rolagem e a mensagem de estado; a vida do objeto de rede e os pacotes dele: destruir um TcpIp e um Network, fechar a conexão de um jogador, mandar um pacote, montar e destruir um buffer de rede; o lado de rede do Display: o texto de um campo de entrada devolvido a quem chamou e o clique de um jogador remoto mandado ao servidor; os técnicos humanos de um jogo restaurado: quem fica com as telas e quem vai para o computador; as listas do histórico do clube das telas: temporadas de liga, campanhas de copa, as competições de cada lista e o registro da liga atual; os arquivos de história escrita (.his): ler um para a tela de história, existência, categoria, título, a conferência -verifyhist e a ordem por título

**Rotinas:**
- `staff_screens_add_squad_and_history_menu()` = 0x88b910 (staff_screens.cpp, cdecl): o botão de menu do perfil: o elenco do clube (ou da seleção) da pessoa para ir direto a outro jogador (até 28 nomes, 26 com histórico) e "Histórico" quando a pessoa tem um histórico escrito (a chave "nome sobrenome d.m.aaaa");
- `staff_screens_compare_text_rows()` = 0x88bc50 (staff_screens.cpp, cdecl): a ordem de duas linhas de texto (utils_string_compare modo 1), para o qsort das listas do perfil;
- `staff_screens_build_nickname_dialog()` = 0x88bc70 (staff_screens.cpp, cdecl): monta o diálogo do apelido: o nome como título, a caixa do apelido e Cancelar, Remover e Ok (Remover e Ok cinzentos sem apelido);
- `staff_screens_handle_nickname_dialog_action()` = 0x88bee0 (staff_screens.cpp, cdecl): trata o diálogo do apelido: digitar redesenha; Remover tira o apelido; Ok grava (troca ou acrescenta, salvo se for igual ao nome comum); Cancelar fecha; o buffer é liberado ao fechar; o resto vai para a barra de menu;
- `staff_screens_build_scout_request_dialog()` = 0x88c100 (staff_screens.cpp, cdecl): monta o pedido de observação: avaliar o jogador agora (olheiros livres) ou observá-lo quando houver olheiro, nas cores do clube do técnico, com Não e Sim;
- `staff_screens_handle_scout_request_action()` = 0x88c360 (staff_screens.cpp, cdecl): trata o pedido de observação: Não fecha, Sim pede o relatório do olheiro e fecha; o resto vai para a barra de menu;
- `background_free_config_tables()` = 0x41bad0 (background.cpp, cdecl): libera as tabelas do pics.cfg: das 11 seções, cada subseção (as duas listas de fotos e o nome) e o vetor de subseções, zerando ponteiros e contagens;
- `background_set_pictures_disabled()` = 0x41bf80 (background.cpp, cdecl): grava a opção da foto fixa (0xdd6bda) como o inverso do argumento;
- `background_pictures_disabled()` = 0x41bfa0 (background.cpp, cdecl): devolve o inverso da opção da foto fixa (1 quando 0xdd6bda é 0);
- `llist_scalar_deleting_destructor()` = 0x497910 (comp_screens.cpp, thiscall): destrói uma lista ligada e libera a memória dela quando o bit 0 das opções está ligado;
- `text_construct_copy()` = 0x4ae5c0 (comp_text.cpp, thiscall): constrói um Text como cópia de outro: o texto e cada vínculo;
- `contract_screens_news_offer_contract_action()` = 0x4dc9b0 (contract_screens.cpp, cdecl): o botão "Oferecer contrato" da notícia: abre a oferta de contrato ao jogador da vaga 2;
- `contract_screens_news_negotiate_action()` = 0x4e5770 (contract_screens.cpp, cdecl): o botão "Negociar" da notícia de contrato: abre a negociação com a pessoa da vaga 0; depois de uma proposta de transferência aceita (tipo 0xfa4), sobre a proposta da vaga 7;
- `match_screens_draw_waiting_screen()` = 0x727fa0 (match_screens.cpp, cdecl): a tela de espera dos jogadores (jogo em rede): só tira a foto de fundo;
- `match_stats_static_date_atexit()` = 0x72aa70 (match_screens.cpp, cdecl): o destruidor vazio da data estática da 0x729ff0;
- `search_filter_record_init()` = 0x81b800 (search_screens.cpp, thiscall): constrói o registro dos filtros da busca de jogadores (0x11 bytes em PLAYER_SEARCH +0x293) com os valores padrão;
- `screen_manager_get_local_addresses()` = 0x7fc7c0 (scrman.cpp, thiscall, ret 4): os endereços da máquina, pela sessão de rede (repassa à Network::get_local_addresses);
- `screen_manager_start_network_session()` = 0x7ff500 (scrman.cpp, thiscall, ret 8): guarda os ganchos de entrada e saída, fecha a sessão antiga e abre uma nova cujas conexões chamam os registros de jogador; devolve se abriu (defeito: sem memória devolve "abriu" sem sessão);
- `scrman_add_player_record()` = 0x7ff630 (scrman.cpp, cdecl): quem entra ganha um registro novo no canal dele (o canal cujo primeiro registro é dele, ou o primeiro vazio): até 16 por canal, os outros sobem um lugar; com gancho de entrada o gancho roda e o canal fica marcado para o monitor; sem gancho o registro novo vai para trás do atual; num cliente de rede nada;
- `scrman_remove_player_records()` = 0x7ff7b0 (scrman.cpp, cdecl): quem sai perde todos os registros dos canais que são dele; só no anfitrião;
- `screen_manager_add_manager_record()` = 0x7ff8f0 (scrman.cpp, thiscall, ret 0xc): um técnico ganha um registro novo no canal atual; com a chave do técnico o registro vai para trás do atual e leva a chave, e os ganchos da sessão ficam como estavam;
- `screen_manager_send_clear_click()` = 0x800000 (scrman.cpp, thiscall): apaga o clique na fila da tela atual (comando 0x11), no Display local ou no do jogador remoto;
- `screen_manager_set_pending_click_widget()` = 0x800160 (scrman.cpp, thiscall, ret 4): o próximo clique esquerdo na tela atual devolve o widget dado (comando 0x13);
- `screen_manager_send_area_scroll_row()` = 0x800330 (scrman.cpp, thiscall, ret 8): quando uma área rola, a primeira linha visível vira um pacote (comando 0xf), mandado à tela remota quando o canal 0 mostra o Display remoto; no jogo local só enche o buffer;
- `screen_manager_send_network_status_message()` = 0x802f80 (scrman.cpp, thiscall, ret 0x10): uma mensagem de estado (texto e dois números, a barra de progresso) vai para a tela do canal dado, ou de todos com -1: aplicada ao Display local para um registro local, mandada ao jogador para um remoto;
- `tcpip_destructor()` = 0x77a870 (national_teams_screens.cpp, thiscall): fecha todos os soquetes e libera a tabela de conexões (só pelo desenrolar de uma exceção na montagem do Network);
- `network_destructor()` = 0x77a890 (national_teams_screens.cpp, thiscall): hospedando, despede-se dos servidores da lista ("CM3 LIST SERVER BYE" com o endereço, por difusão; defeito: mais 50 bytes do que estava na pilha); depois fecha os soquetes e libera a tabela;
- `network_release_slot_socket()` = 0x77ac00 (national_teams_screens.cpp, thiscall, ret 4): fecha a conexão do jogador 1 a 16 (outro número nada); a vaga não fica marcada livre;
- `network_send_packet()` = 0x77ac30 (national_teams_screens.cpp, thiscall, ret 0xc): manda um pacote, antecedido do tamanho em 4 bytes, à conexão de um jogador ou, hospedando, a todas para o jogador 0; conexão achada fechada fica livre (e, hospedando, o gancho de saída é avisado); tamanho 0 ou menos dá a caixa de erro, 50000 ou mais devolve 0; devolve se o último envio deu certo;
- `network_buffer_construct()` = 0x77b630 (national_teams_screens.cpp, thiscall, ret 4): um buffer de rede começa vazio com a capacidade dada e a memória própria; sem memória o jogo para (caixa de erro, limpeza, saída);
- `network_buffer_destruct()` = 0x77b6e0 (national_teams_screens.cpp, thiscall): libera a memória do buffer, quando há, e zera o ponteiro;
- `guio_copy_text_to_caller()` = 0x567b00 (display.cpp, thiscall): um campo de entrada de texto com buffer de quem chamou recebe o texto de volta, com o terminador;
- `display_send_widget_click()` = 0x568560 (display.cpp, thiscall, ret 8): o clique de um jogador remoto vai ao servidor: comando 6 com o widget e todos os campos atuais dele e, quando foi solto sobre outro widget, o retângulo e os campos desse outro; falhando, a caixa "Network Error" e 0;
- `setup_assign_restored_network_managers()` = 0x8387e0 (Setup.cpp, cdecl): ao restaurar um jogo salvo, cada técnico humano sem senha (num jogo sem nome de rede) fica com as telas: o primeiro com o registro atual, os outros com um registro cada; quem tem senha e não está de férias vai para o controle do computador até retomar; devolve 1 (abrir o Manager Status) com algum técnico travado ou de férias com senha;
- `club_history_get_league_history()` = 0x443380 (club_history.cpp, thiscall, ret 4): monta a lista das temporadas de liga do clube, a mais nova primeiro: a temporada atual na frente se a liga ainda não acabou, depois as gravadas no arquivo, da última para a primeira;
- `club_history_get_cup_history()` = 0x443520 (club_history.cpp, thiscall, ret 4): monta a lista das campanhas de copa do clube: primeiro o progresso em cada copa em andamento (que não é liga, divisão nem amistoso) onde ele chegou a uma fase, depois as gravadas, da última para a primeira;
- `club_history_get_league_comp_ids()` = 0x443770 (club_history.cpp, thiscall, ret 4): junta as competições distintas de uma lista de temporadas de liga e as devolve numa lista nova, "ordenadas" por uma comparação que nunca ordena;
- `club_history_get_cup_comp_ids()` = 0x443a20 (club_history.cpp, thiscall, ret 4): o mesmo para uma lista de campanhas de copa;
- `club_history_get_current_league_record()` = 0x444190 (club_history.cpp, cdecl): acha o clube na tabela atual da sua divisão (no Brasil, de fevereiro a junho, a estadual, quando pedido), da fase atual para trás, só fases-liga com mais de 7 rodadas (qualquer uma na Polônia; na Argentina só a liga), e preenche o registro com divisão, ano, posição e totais; diz também se a temporada acabou e em que fase achou;
- `compare_int_ids()` = 0x444500 (club_history.cpp, cdecl): comparação do qsort que nunca ordena: 0 iguais, 1 diferentes;
- `history_load_his_file()` = 0x5f1e50 (History.cpp, cdecl): lê o .his: {title:} o título (parâmetro 1), {section:} uma seção nova, {pic:} uma foto que passa a valer para os parágrafos seguintes; o texto vira parágrafos (uma quebra de linha = espaço, duas = fim do parágrafo); grava seções, fotos e contagens nos parâmetros 2 a 6;
- `history_his_file_exists()` = 0x5f2500 (History.cpp, cdecl): diz se cm3_history\<nome>.his existe, só em jogo de um jogador na tela local;
- `history_get_his_file_category()` = 0x5f2580 (History.cpp, cdecl): lê a marca {cat:} ou {category:} e devolve o índice entre CLUB, NATION, COMP, LEAGUE e PLAYER, ou -1;
- `history_get_his_file_title()` = 0x5f2700 (History.cpp, cdecl): lê a marca {title:} num texto fixo e o devolve, ou 0;
- `verify_history_files()` = 0x5f2840 (History.cpp, cdecl): com -verifyhist, confere cada .his contra o banco pela categoria (clube e nação contra os nomes dos clubes, competição e liga contra as competições, jogador contra "nome sobrenome dia.mês.ano") e mostra uma caixa para cada um sem par;
- `history_compare_entries_by_title()` = 0x5f3410 (History.cpp, cdecl): ordem da lista de história escrita pelo título, entrada nula por último;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (881a6c2e).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 6edd502b, original 388c7666 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 321ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 322`
