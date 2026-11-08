# Mutacoes pela prova do save (o .qst gravado pelo inicio rapido). O resumo (build_save_game_summary)
# e as regras FIFA (o ultimo item) estao no caminho da gravacao; todas mudam o conteudo
# descomprimido: o texto das nacoes e a versao no general.dat (fora de +0xf70/+0xf74, que a prova
# ignora: tempo de jogo e time() nao recebem mutacao, escapariam) e o tamanho do item das regras.
# Os tecnicos do resumo nao: o .qst nao tem tecnico humano (so zera o nome). A carga das regras so
# roda no load_game_data (a etapa de carregar da prova ainda nao existe): sem mutacao.
MUTACOES = [
    ('sv2_resumo_primeira', "            if primeira {\n                formatar(", "            if !primeira {\n                formatar("),
    ('sv2_resumo_versao', "    copy_text(resumo.add(0x7d0), versao());", "    copy_text(resumo.add(0x7d1), versao());"),
    ('sv2_fifa_segundo_byte', "    if gravar(0xa83608, 1, 1, z) != 1 {", "    if 1 != 1 {"),
]
