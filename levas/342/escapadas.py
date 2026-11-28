# As escapadas da 342a leva (a carga e a gravacao do save: premios, disciplina, tecnicos humanos, noticias, notas,
# arbitros, busca de jogadores, taticas, orgaos, ofertas, treino e os dados extras das copas). A prova da leva e a
# prova do save (prova_save.py --leva 342): o save de ferias gravado e carregado, comparado item a item com a base
# feita com as rotinas desligadas. Estas mutacoes mexem em campos ou listas que esse save de teste (agosto de 2001,
# Inglaterra) nao tem preenchidos, ou que a gravacao seguinte reescreve igual: a gravacao e a carga sairam iguais.

_PT = (' O save de teste e de agosto de 2001: a prova do save so ve o que ele guarda. Fica para a prova de um save mais '
       'adiantado (com copas no meio, vagas vazias e listas cheias).')
_EN = (' The test save is from August 2001: the save proof only sees what it keeps. Left for the proof with a later save '
       '(cups halfway, empty slots and full lists).')

ESCAPADAS = {
    'sx12b_noticias_de_recuperacao_perdidas': (
        'EM CONFIRMACAO. A contagem de noticias de recuperacao do tecnico humano fica em 0 na carga: o save de teste nao '
        'tem nenhuma.' + _PT,
        'BEING CONFIRMED. The human manager\'s recovery news count stays at 0 on loading: the test save has none.' + _EN),
    'sx12b_instrucao_de_equipe_sempre': (
        'EM CONFIRMACAO. A carga aceita qualquer palavra de instrucao de equipe do tecnico humano: so muda um valor fora '
        'do intervalo, que o save de teste nao tem.' + _PT,
        'BEING CONFIRMED. The load accepts any team instruction word of the human manager: it only changes an '
        'out-of-range value, which the test save does not have.' + _EN),
    'sx14b_2_sem_ultimatos': (
        'EM CONFIRMACAO. A carga da configuracao dos tecnicos deixa de ler a lista de ultimatos: o save de teste nao tem '
        'ultimato nenhum.' + _PT,
        'BEING CONFIRMED. The managers\' configuration load stops reading the ultimatum list: the test save has no '
        'ultimatum.' + _EN),
    'sx16_8_quinze_vagas': (
        'EM CONFIRMACAO. A carga dos arbitros percorre 15 listas em vez de 16: so muda a ultima, vazia no save de '
        'teste.' + _PT,
        'BEING CONFIRMED. The officials load walks 15 lists instead of 16: it only changes the last one, empty in the test '
        'save.' + _EN),
    'sx16_10_ultima_nota_zerada': (
        'EM CONFIRMACAO. A carga das estatisticas dos arbitros zera a ultima nota: so muda quando ha nota guardada.' + _PT,
        'BEING CONFIRMED. The officials statistics load zeroes the last rating: it only changes when a rating is '
        'kept.' + _EN),
    'sx16_11_fila_vazia': (
        'EM CONFIRMACAO. A carga da aposentadoria trata uma fila como vazia: so muda quando essa fila tem registros.' + _PT,
        'BEING CONFIRMED. The retirement load treats a queue as empty: it only changes when that queue has records.' + _EN),
    'sx17_1_tipo_de_lista': (
        'EM CONFIRMACAO. A busca de jogadores nasce com o tipo de lista 3 em vez de 4: a carga seguinte o reescreve com o '
        'do save.' + _PT,
        'BEING CONFIRMED. The player search is built with list type 3 instead of 4: the following load rewrites it with '
        'the saved one.' + _EN),
    'sx17_1_registro_fora_da_lista': (
        'EM CONFIRMACAO. O registro da busca de jogadores nao entra na lista global dos olheiros: so pesa quando a lista '
        'e percorrida, nas telas dos olheiros.' + _PT,
        'BEING CONFIRMED. The player search record does not join the scouts\' global list: it only matters when the list '
        'is walked, on the scouts\' screens.' + _EN),
    'sx19b_3_mandante_trocado': (
        'EM CONFIRMACAO. A carga de uma oferta poe o visitante no lugar do mandante: o save de teste nao tem oferta '
        'desse tipo.' + _PT,
        'BEING CONFIRMED. An offer\'s load puts the away club in the home club\'s place: the test save has no offer of '
        'that kind.' + _EN),
    'sx21b_5_rival_3_bytes': (
        'EM CONFIRMACAO. Um campo de 4 bytes passa a ser gravado com 3: so muda quando o byte alto e diferente de zero, '
        'o que o save de teste nao tem.' + _PT,
        'BEING CONFIRMED. A 4-byte field is written with 3: it only changes when its high byte is not zero, which the '
        'test save does not have.' + _EN),
    'cx02_13_fa_cup_le_por_cima': (
        'EM CONFIRMACAO. A carga dos dados extras da Copa da Inglaterra le um byte no campo errado: os dois valem o mesmo '
        'no save de teste, antes das rodadas da copa.' + _PT,
        'BEING CONFIRMED. The FA Cup extra data load reads a byte into the wrong field: both are equal in the test save, '
        'before the cup rounds.' + _EN),
    'cx02_14_vaga_vazia_vira_clube': (
        'EM CONFIRMACAO. Uma vaga vazia de uma copa vira o clube 0 na carga: o save de teste nao tem vaga vazia.' + _PT,
        'BEING CONFIRMED. An empty slot of a cup becomes club 0 on loading: the test save has no empty slot.' + _EN),
    'cx02_16_vaga_vazia_zero': (
        'EM CONFIRMACAO. Uma vaga vazia de uma copa e gravada como 0 em vez de -1: o save de teste nao tem vaga '
        'vazia.' + _PT,
        'BEING CONFIRMED. An empty slot of a cup is written as 0 instead of -1: the test save has no empty slot.' + _EN),
    'cx02_17_le_substituicoes_nas_reservas': (
        'EM CONFIRMACAO. A carga de uma fase de copa le as substituicoes no campo das reservas: os dois valem o mesmo no '
        'save de teste.' + _PT,
        'BEING CONFIRMED. A cup stage load reads the substitutions into the substitutes\' field: both are equal in the '
        'test save.' + _EN),
    'cx03_11_vaga_livre_vira_clube_zero': (
        'EM CONFIRMACAO. Uma vaga livre da Copa da Liga escocesa vira o clube 0 na carga: o save de teste nao tem vaga '
        'livre.' + _PT,
        'BEING CONFIRMED. A free slot of the Scottish League Cup becomes club 0 on loading: the test save has no free '
        'slot.' + _EN),
    'cx03_11_isentos_zerados': (
        'EM CONFIRMACAO. A carga da Copa da Liga escocesa zera um contador de isentos: so muda quando ele nao e zero.' + _PT,
        'BEING CONFIRMED. The Scottish League Cup load zeroes an exemption counter: it only changes when it is not '
        'zero.' + _EN),
}  # nome: (explicacao pt, explicacao en)
