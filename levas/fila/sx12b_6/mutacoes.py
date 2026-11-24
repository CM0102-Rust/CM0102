# Mutacoes (sx12b_6): nenhuma. Os oito comparadores so rodam a original (os inicializadores registram o atexit, os
# destrutores liberam memoria): nenhum comparador acusaria uma mutacao, e o efeito de um destrutor errado (vazamento,
# liberacao dupla) so aparece como queda. A prova e o modo Rust: o jogo abrir (os inicializadores rodam no _initterm),
# carregar um jogo salvo pelo menu com outro jogo aberto (free_game_data chama os destrutores) e sair sem cair.
MUTACOES = []
