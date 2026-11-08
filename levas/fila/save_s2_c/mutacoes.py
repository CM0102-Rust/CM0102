# Mutacao pela prova do save: a ligacao de gravar (0x524d30) e o step_towards 1 da gravacao do .qst; o
# a4 trocado cria o conteiner com o id 0x16 no cabecalho (o .qst tem 0x17): conteudo diferente.
# A ligacao de carregar so roda no load_game_data (a etapa de carregar da prova ainda nao existe).
# O free_game_data roda no fim do inicio rapido (0x827b77), depois do .qst gravado; antes dele, so
# na abertura (o que ele zera a criacao do jogo refaz): sem mutacao que mude o arquivo com certeza.
MUTACOES = [
    ('sv2_banco_a4', "    gravar_banco(save, 1, a4, tipo)", "    gravar_banco(save, 1, a4 ^ 1, tipo)"),
]
