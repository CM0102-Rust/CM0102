# Mutacoes (cqc_12): as tres na 0x76b100 (516 chamadas ate 2002-06-01). Sem as convocacoes dos jogos de daqui a duas
# semanas (0x76bab0), as selecoes jogam com outros elencos (a impressao das partidas muda); com as reservas refeitas
# em 1o de fevereiro em vez de 1o de janeiro, as convocacoes de janeiro (os amistosos de fevereiro de 2002) saem das
# reservas de agosto; sem marcar os clubes com jogo em volta da data, a tabela (+0x24) fica nula e a remarcacao dos
# jogos dos clubes com convocados (0x7777e0) nao roda. Podem escapar: a segunda, se as reservas de agosto e de janeiro
# derem as mesmas convocacoes; a terceira, se nenhum clube com convocados tiver jogo a remarcar perto das datas (os
# contadores mostram quantas vezes cada trecho rodou).
# Na 0x76c120 (01/11/2026): a vaga 23 da correcao com codigo de goleiro (so a carreira acusa: a original le lixo ali e o
# comparador deixa as vagas da correcao fora; roda a partir de 29/05/2002, entao so na carreira ate 2002-06-01); a
# idade do sub-21 (> 0x15 no lugar de > 0x17: muda a lista da escala, o comparador e a carreira acusam); o convocado
# sem o time (+0x24; so a carreira acusa: o comparador confere a decisao, a lista e os codigos). Pode escapar: a do
# goleiro, se a escala der o mesmo jogador para a vaga 23 com os dois codigos.
MUTACOES = [
    ('cqc12_sem_convocar', '        convocar(this, p); // cqc12: as convocações para o jogo',
     '        // mutacao: sem convocar',
     'cqc_12 convoca'),
    ('cqc12_reservas_em_fevereiro', '    if cqc12_e_hoje(dma(pt, 1, 0, ano, u32::MAX), hoje) // cqc12: 1º de janeiro',
     '    if cqc12_e_hoje(dma(pt, 1, 1, ano, u32::MAX), hoje) // mutacao: 1o de fevereiro',
     'cqc_12 refaz as reservas'),
    ('cqc12_vaga23_goleiro', '            c[i] = CQC12_EXTRAS[i - 22]; // cqc12: as vagas da correção',
     '            c[i] = 1; // mutacao: goleiro',
     'cqc_12 correção do elenco de 23'),
    ('cqc12_idade_do_sub21', '    if t == 2 && cqc12_b(j + 0x18) as i8 > 0x17 { // cqc12: a idade do sub-21',
     '    if t == 2 && cqc12_b(j + 0x18) as i8 > 0x15 { // mutacao: 21',
     'cqc_12 escala do elenco'),
    ('cqc12_convocado_sem_time', '            cqc12_pd(j + 0x24, elenco); // cqc12: o convocado ganha o time',
     '            // mutacao: sem o time',
     'cqc_12 escala do elenco'),
    ('cqc12_sem_marcar', '            cqc12_marcar_tres_dias(this, d); // cqc12: os clubes com jogo em volta da data',
     '            // mutacao: sem marcar',
     'cqc_12 marca os clubes'),
]
