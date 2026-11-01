# Mutacoes (cqc_12): as tres na 0x76b100 (516 chamadas ate 2002-06-01). Sem as convocacoes dos jogos de daqui a duas
# semanas (0x76bab0), as selecoes jogam com outros elencos (a impressao das partidas muda); com as reservas refeitas
# em 1o de fevereiro em vez de 1o de janeiro, as convocacoes de janeiro (os amistosos de fevereiro de 2002) saem das
# reservas de agosto; sem marcar os clubes com jogo em volta da data, a tabela (+0x24) fica nula e a remarcacao dos
# jogos dos clubes com convocados (0x7777e0) nao roda. Podem escapar: a segunda, se as reservas de agosto e de janeiro
# derem as mesmas convocacoes; a terceira, se nenhum clube com convocados tiver jogo a remarcar perto das datas (os
# contadores mostram quantas vezes cada trecho rodou).
MUTACOES = [
    ('cqc12_sem_convocar', '        convocar(this, p); // cqc12: as convocações para o jogo',
     '        // mutacao: sem convocar',
     'cqc_12 convoca'),
    ('cqc12_reservas_em_fevereiro', '    if cqc12_e_hoje(dma(pt, 1, 0, ano, u32::MAX), hoje) // cqc12: 1º de janeiro',
     '    if cqc12_e_hoje(dma(pt, 1, 1, ano, u32::MAX), hoje) // mutacao: 1o de fevereiro',
     'cqc_12 refaz as reservas'),
    ('cqc12_sem_marcar', '            cqc12_marcar_tres_dias(this, d); // cqc12: os clubes com jogo em volta da data',
     '            // mutacao: sem marcar',
     'cqc_12 marca os clubes'),
]
