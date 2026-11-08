# Mutacoes (cqk_7): o destino de cada clube no fim das fases (slot 18) do Centro-Oeste, Goiano, Mineiro, Paranaense e
# Carioca (os estaduais do primeiro semestre de 2002). Os registros de carreira do tecnico (o historico) mudam, e a
# memoria do jogo (impressao diaria) com eles.
# - Estaduais: o campeao do mata-mata nao ganha o registro de campeao.
# - Estaduais: o rebaixado dos pontos corridos nao ganha o registro de rebaixamento.
# - Estaduais: o classificado dos pontos corridos nao ganha o registro de classificacao. ESCAPOU na carreira de 8
#   ligas ate 2002-06-01 (04/11), como os outros registros de classificacao: fica como lacuna medida.
# Escaparam na carreira de 8 ligas ate 2002-06-01 (04/11) e sairam: o destino 0 em vez de 5 do campeao (nada le o
# destino antes da virada); o peso da escala do eliminado (o porte da 0x86c000 ignora o ultimo argumento); sem o
# registro de eliminacao do estadual e, na NSL, o peso 0x50 e sem o registro de classificacao (o historico desses
# tecnicos nao chega a impressao antes de 2002-06-01 [inferencia: tecnicos sem a ficha de historico +0xcf]).
MUTACOES = [
    ('cqk7_campeao_estadual', 'cqk7_campeao(clube, comp); // cqk7: o registro do campeão do estadual',
     '// mutacao: sem o registro do campeão do estadual', 'cqk_7 state league champion'),
    ('cqk7_rebaixado_estadual', 'cqk7_rebaixado(clube, comp); // cqk7: o rebaixado do estadual',
     '// mutacao: sem o registro do rebaixado do estadual', 'cqk_7 state league relegated'),
    ('cqk7_classificado_estadual', 'cqk7_classificou(clube, comp, 0, 0x82, 0x32); // cqk7: o classificado do estadual',
     '// mutacao: sem o registro do classificado do estadual', 'cqk_7 state league qualified'),
]
