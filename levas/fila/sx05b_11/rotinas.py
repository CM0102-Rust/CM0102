# O jogador que some sem licenca (AWOL) e a noticia dele (awol.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa NewsPost,
# NewsSink, news_post e rehearse_news (sx05b_5): integrar depois dele. O texto da noticia (0xbc5) e o sx05_5.
# start_absence e chamada pelo AWOL::daily_update (0x41a454/0x41a4a2, ja Rust em awol.rs), pelas transferencias
# negadas (0x8c1603), pelas lesoes (0x6311cc), pela midia (0x7377c2) e pelo jogo (0x5d3320, 0x5a606d, 0x7a3bc8): roda
# na carreira ENG, raro. Sem LIGAS.
# Convencoes conferidas na desmontagem (cada push e cada [esp+k] seguidos ate a chamada):
# - 0x419840 thiscall(pessoa, clube, tipo (byte)) ret 0xc: a marca (bit 1) na tabela +0xc; as tres reputacoes da
#   ficha (+0xb, +9, +0xd, nessa ordem) x (1 - utils_random(0x29) x 0.01 (f64 0x967898), com 1.0 de 0x967890) pelo
#   _ftol; staff_add_disliked_staff 0x547af0 cdecl(quem, de quem, 1): a pessoa do tecnico (+0xcf), o tecnico dela sem
#   tecnico humano (human_manager_is_human_club 0x600a50 cdecl(clube, 1, 1, 0, 0)), os 50 do elenco (+0xd7) com
#   random(lealdade +0x59) != 0 e random(profissionalismo +0x5b) == 0, o presidente (+0xbf); o registro de 12 bytes
#   (id, a data de hoje [0xae2c90] por Date::operator= 0x417c10 e Date::add_days 0x54c5b0 thiscall(saida, dias) ret
#   8: tipo 4 random(0x1f) + 2, senao random(0x96) + 0x15) por LLIST::add_tail 0x68d0d0 na lista +0; 0x419ad0
#   thiscall(pessoa, clube ou o dela, 2/4/1, -1/dias/-1) ret 0x10;
# - 0x419ad0 thiscall(pessoa, clube, tipo (byte), dias (word)) ret 0x10: NewsItem 0x77bbe0 (0xbc5, 0); as vagas 0..8
#   (os ids de nome lidos de +0x33 dos registros de nome, sem conferir nulo); a reputacao mundial (word com sinal +0xd
#   da ficha) acima de 7500 vai para todos (0x786780); senao a nacao do clube (0x7863f0, com nacao) e, com a
#   nacionalidade (+0x1a) diferente, essa nacao (mesmo nula).
# Comparadores: start_absence so a original (sorteios, reputacoes, antipatias, lista e noticia) com o plano conferido
# (a marca e o registro); post_news a original posta e a copia ensaia.
SAVE = False
ROTINAS = [('awol_start_absence', 0x419840, [0x6a, 0xff, 0x68, 0xb0, 0x3e, 0x95, 0x00]),
           ('awol_post_news', 0x419ad0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o jogador que some sem licença e a notícia dele',
        'a player going AWOL and its news',
        [('awol_start_absence', 'AWOL::start_absence', '0x419840', 'awol.cpp', 'thiscall, ret 0xc',
          'o jogador some: sem pessoa, ficha, id válido ou com a marca de ausente já posta, nada; senão a marca, as reputações atual, nacional e mundial cortadas cada uma de 0 a 40% por sorteio; no clube, ele antipatiza com o técnico, o técnico com ele (sem técnico humano), cada colega quando o sorteio da lealdade não dá 0 e o do profissionalismo dá 0, e o presidente com ele; a ausência (o id e a volta: hoje mais 2 a 32 dias no tipo 4, senão 21 a 170) vai para a lista; a notícia com o tipo 2 (do tipo 1), 4 (com os dias) ou 1',
          'the player goes AWOL: without a person, a sheet, a valid id or with the AWOL flag already set, nothing; otherwise the flag, his current, home and world reputations each cut by 0 to 40 percent by a draw; at his club, he dislikes the manager, the manager him (no human manager), each squad mate when a draw of his loyalty is not 0 and one of his professionalism is 0, and the chairman him; the absence (the id and the return: today plus 2 to 32 days for kind 4, else 21 to 170) goes to the list; the news with kind 2 (from kind 1), 4 (with the days) or 1'),
         ('awol_post_news', 'awol_post_news', '0x419ad0', 'awol.cpp', 'thiscall, ret 0x10',
          'a notícia do jogador ausente (tipo 0xbc5): o id, os ids de nome, o clube dado, o clube dele, o tipo, os dias e a nação do clube; reputação mundial acima de 7500 vai para todos os técnicos; senão para a nação do clube e, se a nacionalidade for outra, para ela',
          "the AWOL news (type 0xbc5): the id, the name ids, the given club, his club, the kind, the days and the club's nation; a world reputation above 7500 goes to every manager; otherwise to the club's nation and, when his nationality is another, to it")])
CAMPOS = []
INTERESSA = []
