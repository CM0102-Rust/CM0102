# O gestor de treino no jogo salvo: o training.dat carregado e gravado, e o destrutor (training_manager.cpp; leitor
# sx21b, rodada 5, 24/11/2026). 0 chamadas na carreira: a gravacao pelo save_game_data (0x82f479), a carga pelo
# construtor (0x8b1006, com o caminho do save), o destrutor quando os dados do jogo sao liberados (0x830094: carregar
# outro jogo ou sair). Item do save, com a carga do FeriasTesteSave (CARGA).
# Convencoes conferidas: 0x8b10d0 e 0x8b1450 thiscall(save) ret 4 (eax 1 ou 0); 0x8b1070 thiscall ret 0. 0x9326e0
# zopen cdecl(save, "training.dat" 0xa8ccc0, 0 lendo / 1 gravando, 0x16, 4); 0x932c30 zread e 0x932e10 zwrite
# cdecl(buf, tamanho, quantidade, item); 0x932b00 zclose cdecl(&item); 0x944e46 malloc; 0x9452ca free; 0x9462bb eh
# vector destructor iterator stdcall(this, 0x9b, 0xa7, 0x5a53c0).
# O bloco: as entradas de treino (6 bytes por ficha de jogador), a semana (+0x688d), as 167 programacoes, o dword
# +0x6529 (a contagem de treino de posicao: a carga a copia para +0x6525; a gravacao grava o +0x6529), os itens de 8
# bytes e as 96 vagas de treinador de 9 bytes. Uma leitura ou escrita curta: a caixa (training_manager.cpp, linhas
# 0x7f..0xc8 na carga, 0xe2..0x118 na gravacao), 0, e o item fica aberto (como a original).
# Comparadores: so a original (leem e gravam o save, alocam e liberam); a prova e a do save.
SAVE = True
CARGA = True
ROTINAS = [
    ('load_training_dat', 0x8b10d0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
    ('save_training_dat', 0x8b1450, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x04, 0x02, 0x00, 0x00]),
    ('destroy_training_manager', 0x8b1070, [0x56, 0x8b, 0xf1, 0x8b, 0x86, 0x1d, 0x65, 0x00, 0x00]),
]
DOCS = ('o gestor de treino no jogo salvo: o training.dat carregado e gravado, e o destrutor',
        "the training manager in the saved game: training.dat loaded and saved, and the destructor",
        [('load_training_dat', 'load_training_dat', '0x8b10d0', 'training_manager.cpp', 'thiscall, ret 4',
          'regra: do item training.dat: as entradas de treino de cada ficha de jogador (bloco novo), a semana, as 167 programações, a contagem e os itens de treino de posição (bloco novo), as 96 vagas de treinador; 1 se leu tudo',
          "rule: from the training.dat item: each player sheet's training entry (a new block), the week, the 167 schedules, the position training count and items (a new block), the 96 coach slots; 1 when everything was read"),
         ('save_training_dat', 'save_training_dat', '0x8b1450', 'training_manager.cpp', 'thiscall, ret 4',
          'regra: grava no item training.dat os mesmos blocos, na mesma ordem; 1 se gravou tudo',
          'rule: writes the same blocks into the training.dat item, in the same order; 1 when everything was written'),
         ('destroy_training_manager', 'destroy_training_manager', '0x8b1070', 'training_manager.cpp', 'thiscall, ret 0',
          'regra: libera as entradas de treino e os itens de treino de posição e destrói as 167 programações',
          'rule: frees the training entries and the position training items and destroys the 167 schedules')])
CAMPOS = []
INTERESSA = []
