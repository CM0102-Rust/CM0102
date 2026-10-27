# Mutacoes da copa (cpb_1). A do vice grava o registro de campeao (0x86a800, que sobe mais a reputacao do clube, do
# tecnico e do elenco): as finais da Copa da Liga e da FA Cup (fev. e maio de 2002) caem antes de 2002-06-01, a
# carreira deve acusar pela reputacao. As outras duas so mudam o historico da competicao (0x4afce0) e a semeadura da
# UEFA (0x905790), que nao entram na impressao (fora do banco) e so pesam na temporada seguinte: PODEM ESCAPAR da
# carreira (11 e 2 chamadas); o comparando acusa na primeira chamada (a fita traz os argumentos).
MUTACOES = [
    ('cpb_vice_como_campeao', '        2 => registro3(0x86b0b0), // vice', '        2 => registro3(0x86a800), // mutacao: o registro de campeao'),
    ('cpb_honras_sem_vice', '                2 => vice = cpb1_u32(e),', '                2 => campeao = cpb1_u32(e),'),
    ('cpb_semi_so_um', "                    segundo = id(time);\n                    break 'lacos;", "                    segundo = primeiro;\n                    break 'lacos;"),
]
