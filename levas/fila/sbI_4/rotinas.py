# O gerente dos elencos (squad_manager.cpp; leitor sbI, rodada 5, 24/11/2026). SAVE com CARGA: o que a rotina faz de
# observavel e na carga (le o ano das inscricoes do squad_manager.dat do .qst); na criacao do jogo so espera o CD e
# zera o ano, que o init_new_game 0x859dc0 reescreve logo depois (sem mutacao possivel pela carreira).
# - 0x859aa0 SQUAD_MANAGER::SQUAD_MANAGER thiscall(save) ret 4 -> this: na criacao (setup, passo 0x1d, new 2, save 0) e
#   na carga (passo 0x21, com o caminho do save). O CD pelo find_game_cd (0x859ade e 0x859b21: o patch sem CD do
#   cm0102-1080 e do portatil troca esses 3 bytes; a rotina e desviada nos dois exes pelos dois hashes; fora eles, igual
#   no cm0102-1080.exe). A carga SQUAD_MANAGER::load 0x859bc0 thiscall(this, save) ret 4 (cx15_14, chamada pelo
#   endereco: sem dependencia), o retorno ignorado. O mapa chama a rotina de find_cm_cd_drive: e o construtor (o setup a
#   chama com o new de 2 bytes e guarda o this em G_SQUAD_MANAGER).
# Prologo sub esp, 0x474 (6 bytes). Sem relativas. Usa o find_game_cd (sx02b_1, ja no HEAD desde a leva 330).
# Comparador: so a original (espera o CD e le o save); sem save, o ano 0.
SAVE = True
CARGA = True
ROTINAS = [('new_squad_manager', 0x859aa0, [0x81, 0xec, 0x74, 0x04, 0x00, 0x00])]
DOCS = ('o gerente dos elencos', 'the squad manager',
        [('new_squad_manager', 'SQUAD_MANAGER::SQUAD_MANAGER', '0x859aa0', 'squad_manager.cpp', 'thiscall, ret 4',
          'monta o gerente dos elencos: confere o CD, zera o ano das inscrições e, com um jogo salvo, lê o ano dele',
          'builds the squad manager: checks the CD, clears the registration year and, with a saved game, reads the year from it')])
CAMPOS = []
INTERESSA = []
