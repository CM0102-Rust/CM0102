# As escapadas da 320a leva (itens de simulacao da rodada 2 das telas: tel39_3..8, tel41_1..4, tel42_2; ENG ate 2002-06-01).
# A comparacao longa (ate 2002-06-01) roda nas bancadas 3, 4 e 5 (levas/320/conf_escapadas_N.txt); o resultado entra num
# commit seguinte, como na 316a.

def _par(pt, en):
    return (pt, en)


ESCAPADAS = {
    'tel39_6_mesma_frase': _par(
        'LACUNA MEDIDA. Troca `&&` por `||` no valor que `Text::add_link` devolve quando a mesma frase ja tem vinculo; '
        'nada e acrescentado nos dois casos, e o valor so muda quando so um de tipo e dado coincide (dois objetos de mesmo '
        'nome na mesma noticia). Medido: comparacao ate 2002-06-01 com 0 divergencias em 10^9 comparacoes; o caso nao ocorre '
        'na carreira de teste.',
        'MEASURED GAP. Turns `&&` into `||` in the value `Text::add_link` returns when the same phrase already has a link; '
        'nothing is added either way, and the value only changes when just one of kind and data matches (two objects with '
        'the same name in one news item). Measured: comparison to 2002-06-01 with 0 divergences in 10^9 comparisons; the '
        'case does not happen in the test career.'),
    'tel39_7_sem_texto': _par(
        'EM CONFIRMACAO. O construtor de Text com texto de origem deixa de copiar o texto; o comparador do construtor compara '
        'o conteudo e acusaria. Na comparacao ate 2001-08-20 o construtor com origem nao rodou; o ramo roda antes de '
        '2002-06-01 (contador da carreira). A comparacao longa decide.',
        'BEING CONFIRMED. The Text constructor with a source text stops copying it; the constructor\'s comparer compares '
        'the contents and would catch it. In the comparison to 2001-08-20 the constructor with a source did not run; the '
        'branch runs before 2002-06-01 (career counter). The long comparison decides.'),
    'tel39_8_uma_linha': _par(
        'EM CONFIRMACAO. A lista de comentaristas le so a primeira linha do arquivo. Os comentaristas so aparecem no texto '
        'das partidas assistidas, que a carreira de ferias nao assiste; o comparador do construtor roda so a original. '
        'A comparacao longa decide se algo os le ate 2002-06-01.',
        'BEING CONFIRMED. The pundit list reads only the first line of the file. Pundits only appear in the text of '
        'watched matches, which the holiday career does not watch; the constructor\'s comparer runs only the original. '
        'The long comparison decides whether anything reads them before 2002-06-01.'),
    'tel41_2_trailing_zeros': _par(
        'EM CONFIRMACAO. A tabela ordenada de clubes deixa de cortar os clubes com valor 0 no fim. As tabelas ordenadas '
        'sao pedidas pela midia e pelos premios no fim de cada mes; a comparacao ate 2001-08-20 so passa pelo fim de julho, '
        'com poucos jogos. A comparacao longa decide.',
        'BEING CONFIRMED. The sorted club table stops dropping the trailing clubs with value 0. Sorted tables are asked '
        'for by the media and awards at each month end; the comparison to 2001-08-20 only crosses the end of July, with '
        'few games. The long comparison decides.'),
    'tel41_2_group_filter': _par(
        'EM CONFIRMACAO. A tabela ordenada de jogadores inverte o filtro de posicao. Mesmo motivo da trailing_zeros: os '
        'pedidos com filtro de posicao vem dos premios por posicao, depois de agosto. A comparacao longa decide.',
        'BEING CONFIRMED. The sorted player table inverts the position filter. Same reason as trailing_zeros: requests '
        'with a position filter come from the position awards, after August. The long comparison decides.'),
    'tel41_3_flag_cleared': _par(
        'EM CONFIRMACAO. O envio das noticias de recorde nao apaga a marca de pendente. Roda no fim de cada competicao; a '
        'marca so seria lida de novo no fim seguinte da mesma competicao, na temporada seguinte. Provavel lacuna.',
        'BEING CONFIRMED. Sending the record news does not clear the pending mark. It runs at the end of each competition; '
        'the mark would only be read again at the same competition\'s next end, the following season. Likely a gap.'),
    'tel41_4_instances': _par(
        'EM CONFIRMACAO. O destrutor das estatisticas nao baixa o contador de instancias. O destrutor so roda quando uma '
        'competicao troca de estatisticas, o que pode nao ocorrer ate 2002-06-01 (o leitor ja avisou). Provavel lacuna.',
        'BEING CONFIRMED. The statistics destructor does not lower the instance count. The destructor only runs when a '
        'competition swaps statistics, which may not happen before 2002-06-01 (the reader warned). Likely a gap.'),
    'tel41_4_fallback_best': _par(
        'EM CONFIRMACAO. Nas posicoes previstas, o caminho de reserva usa a pior posicao no lugar da melhor. O caminho de '
        'reserva so roda quando o clube nao tem projecao propria; a irma tel41_4_projected_best foi ACUSADA em 2001-267. '
        'A comparacao longa decide.',
        'BEING CONFIRMED. In the projected positions, the fallback path uses the worst position instead of the best. The '
        'fallback only runs when the club has no projection of its own; the sibling tel41_4_projected_best was CAUGHT at '
        '2001-267. The long comparison decides.'),
    'tel42_2_posicao': _par(
        'EM CONFIRMACAO. O registro da temporada de liga no historico do clube grava a posicao + 1. Nada na carreira le o '
        'historico; o comparador do registro acusa, mas a primeira liga termina em maio de 2002. A irma da copa, '
        'tel42_2_semente, foi ACUSADA pelo comparador (a Charity Shield termina em agosto de 2001). A comparacao longa decide.',
        'BEING CONFIRMED. The league season record in the club history stores position + 1. Nothing in the career reads '
        'the history; the record\'s comparer catches it, but the first league ends in May 2002. The cup sibling, '
        'tel42_2_semente, was CAUGHT by the comparer (the Charity Shield ends in August 2001). The long comparison decides.'),
}  # nome: (explicacao pt, explicacao en)
