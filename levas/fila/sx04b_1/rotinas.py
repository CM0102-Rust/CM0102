# A noticia do vencedor do premio do mes e o seu texto (month_award.cpp; leitor sx04b, rodada 5, 23/11/2026). Rodam na
# carreira ENG: MONTH_AWARD::set_winner (0x7671c0, ja Rust) posta a noticia a cada vencedor do mes (jogador, jovem e
# tecnico do mes da Inglaterra, na virada de cada mes); o texto (virtual +0xc da vtable 0x96da00) e escrito pelo
# AWARD_MANAGER::write_award_news (0x4143f0, Rust) quando a noticia tipo 0x1b e lida (manchete e corpo).
# Convencoes conferidas na desmontagem (cada push e cada [esp+k] seguidos ate a chamada):
# - 0x767330: thiscall(vaga em byte), ret 4; quadro SEH sem destrutor; NewsItem::NewsItem 0x77bbe0 thiscall(0x1b, 0);
#   set_slot 0x785790 thiscall(k, valor) ret 8; utils_random 0x90cfc0 cdecl(3); formation_get_position_mask 0x5b7d20
#   cdecl(ficha) -> ax (movsx); staff_get_player_strength_flags 0x546ab0 cdecl(pessoa, clube);
#   transfer_manager_collect_player_news_recipients 0x8d1da0 thiscall(pessoa, 0x48, vetor, 0x1e) -> al;
#   News::send_to_division_managers 0x7862d0 / send_to_nation_managers 0x7863f0 / send_to_club_manager 0x7861e0
#   thiscall(&noticia, alvo) ret 8 (o G_NEWS relido a cada envio).
# - 0x767580: thiscall(noticia, Text, corpo), ret 0xc; get_slot 0x785830 thiscall(k) ret 4 (vagas 2..7, 9..0xe);
#   write_position_name 0x5461b0 cdecl(posicoes, buf, 0xc8, 1); database_format_name_from_ids 0x53e140 cdecl (estilos 1
#   e 5, 0xc8 bytes); write_player_description 0x5465a0 cdecl(pessoa, posicoes, qualidades, buf, 0xc8, genero, 1);
#   date_get_month_name 0x54d3b0 cdecl (le so o byte); langlib_sprintf 0x66f4e0 cdecl no 0xde1f64; Text::set_string
#   0x4ae660 e Text::add_link 0x4ae8a0 thiscall; database_staff_matches_names 0x53e990 cdecl(pessoa, n1, n2, n3, 0).
#   A tabela de saltos 0x767ad8 (5 entradas: 0x76792a, 0x767949, 0x767975, 0x767991, 0x7679ad) escolhe a manchete.
# Prologos sem chamada relativa; iguais no cm0102-1080.exe.
# Defeitos do original: (0x767330) vencedor com ficha de jogador e sem clube (-1) le as qualidades com o registro
# antes da tabela de clubes; (0x767580) o genero da descricao vai num dword cujo so o byte baixo foi escrito.
# Comparadores: 0x767330 a original e depois o ensaio da copia (rehearse_news: preenche e sorteia sem enviar) no mesmo
# gerador; os sorteios da copia tem de ser o comeco dos da original; 0x767580 por compare_text_writer.
# Usa award_base, NewsPost, NewsSink, news_post e rehearse_news do sx05b_5: integrar depois dele.
SAVE = False
ROTINAS = [('month_award_post_winner_news', 0x767330, [0x6a, 0xff, 0x68, 0xab, 0xdb, 0x95, 0x00]),
           ('month_award_write_news', 0x767580, [0x81, 0xec, 0x80, 0x02, 0x00, 0x00])]
DOCS = ('a notícia do vencedor do prêmio do mês e o seu texto',
        "the month award winner's news and its text",
        [('month_award_post_winner_news', 'MONTH_AWARD::post_winner_news', '0x767330', 'month_award.cpp', 'thiscall, ret 4',
          'regra: monta a notícia tipo 0x1b do vencedor de uma vaga do mês (a nação e o índice do prêmio, o vencedor e os ids dos seus nomes, o clube, o mês e o ano, a marca 0x420e9, um sorteio de 0 a 2 que escolhe o modelo do texto, se não tem ficha de jogador, com ficha as posições e as qualidades, se é jogador-técnico) e a envia aos técnicos da liga do prêmio (ou, sem liga, aos da nação da competição do prêmio) e a cada clube interessado no jogador que não joga nessa liga. Defeito: vencedor com ficha e sem clube lê as qualidades com o registro antes da tabela de clubes',
          "rule: builds the type 0x1b news of a month slot's winner (the award's nation and index, the winner and his name ids, the club, the month and the year, the mark 0x420e9, a draw from 0 to 2 that picks the text template, whether he has no player sheet, with one his positions and strengths, whether he is a player-manager) and sends it to the managers of the award's league (or, without one, to those of the award competition's nation) and to each club interested in the player that does not play in that league. Defect: a winner with a sheet and no club reads his strengths with the record before the club table"),
         ('month_award_write_news', 'MONTH_AWARD::write_news', '0x767580', 'month_award.cpp', 'thiscall, ret 0xc',
          'regra: a manchete por (vencedor / (sorteio + 1)) % 6: "leva o prêmio", "eleito o <prêmio>", "prêmio para", "fica com o prêmio", "ganha o prêmio de <mês>" ou "<prêmio> decidido"; o corpo: sem clube "ganhou o <prêmio> do mês de <mês>"; com clube, ficha de jogador e sorteio 1 ou 2, com a descrição do jogador ("player-boss" se jogador-técnico) num de dois modelos; senão "<clube> <jogador> ganhou o <prêmio> de <mês>"; depois os vínculos do clube, do jogador (se os nomes ainda batem) e do prêmio. Defeito: o gênero da descrição vai num dword com resto de pilha',
          'rule: the headline by (winner / (draw + 1)) % 6: "takes award", "named as <award>", "award for", "scoops prize", "wins <month> award" or "<award> award decided"; the body: without a club "has won the <award> award for the month of <month>"; with a club, a player sheet and draw 1 or 2, with the player\'s description ("player-boss" for a player-manager) in one of two templates; otherwise "<club> <player> has won the <award> award for <month>"; then the links of the club, the player (when the names still match) and the award. Defect: the description\'s gender goes in a dword with stack leftover')])
CAMPOS = [('prêmio (AWARD) +0xd índice da nação, +0x11 índice do prêmio na nação; valores; lê (month_award_post_winner_news)',
           'award (AWARD) +0xd nation index, +0x11 award index in the nation; values; reads (month_award_post_winner_news)'),
          ('competição de pessoas (TStaffComp, 0x65 bytes) +4 nome, +0x37 gênero, +0x38 nome curto, +0x52 gênero, +0x57 nação; lê (month_award_post_winner_news, month_award_write_news)',
           'staff competition (TStaffComp, 0x65 bytes) +4 name, +0x37 gender, +0x38 short name, +0x52 gender, +0x57 nation; reads (month_award_post_winner_news, month_award_write_news)')]
INTERESSA = []
