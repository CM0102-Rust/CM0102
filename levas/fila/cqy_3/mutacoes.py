# Mutacoes (cqy_3). Os nomes das fases sao texto (noticias e telas): a carreira nao le o texto, entao as mutacoes sao
# acusadas pelo comparador de cada rotina (lado a lado, o texto da original contra o da copia), como diz o guia
# (COMO_ESCREVER_UM_ITEM: "uma que escapa da carreira mas e acusada pelo comparador da rotina conta como acusada"):
# - a forma 0 (nome completo) com a tabela dos nomes curtos;
# - a "First Round" (id 10, a mais comum nas copas) com o texto de "Replay";
# - o rotulo competicao + fase + rodada com a fase e a rodada trocadas.
MUTACOES = [
    ('cqy3_nome_completo_com_a_tabela_curta',
     '0 => &STAGE_NAMES_FULL, // cqy3: the full names',
     '0 => &STAGE_NAMES_SHORT, // mutation: the short names',
     'cqy_3 stage name'),
    ('cqy3_primeira_fase_com_o_texto_de_replay',
     '    (10, 0x9a7c04), // First Round',
     '    (10, 0x9a7c3c), // mutation: Replay',
     'cqy_3 stage name'),
    ('cqy3_rotulo_com_fase_e_rodada_trocadas',
     'lang_sprintf(buf, 0x9a7cc0, cg, cg, sg, sg, rg, rg, pn, ps, pr);',
     'lang_sprintf(buf, 0x9a7cc0, cg, cg, sg, sg, rg, rg, pn, pr, ps); // mutation: stage and round swapped',
     'cqy_3 label competition stage round'),
]
