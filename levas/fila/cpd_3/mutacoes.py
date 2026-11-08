# Mutacoes da competicao atual e do substituto. As duas do aviso tiram a chamada 0x6a3770 (zera e sorteia na linha do
# tecnico em [[0xb67a44]], que entra na impressao do banco): acusam se o caminho com a competicao seguinte rodar (ramos
# "seleção a competição seguinte" e "seleção continente a seguinte vira a atual"); a do continente (4 chamadas) PODE
# ESCAPAR se nenhuma nacao tiver a seguinte esperando. A do substituto inverte o filtro das posicoes (140 chamadas):
# outro jogador e convocado (pessoas na impressao do banco), deve acusar.
MUTACOES = [
    ('cpd_atual_aviso', '        notify_manager_manager(nacao); // o aviso da competição seguinte', '        // mutacao: sem o aviso da competicao seguinte'),
    ('cpd_continente_aviso', '                notify_manager_manager(nacao); // o aviso do continente', '                // mutacao: sem o aviso do continente'),
    ('cpd_substituto_posicao', 'as u8 & mascara & 0x7f == 0 { // as posições de quem sai', 'as u8 & mascara & 0x7f != 0 { // mutacao: as outras posicoes'),
]
