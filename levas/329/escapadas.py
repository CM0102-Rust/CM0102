# As escapadas da 329a leva. Na cadeia, a confirmacao comparando de varias delas acusou por engano a noticia 0x24
# (o escritor original 0x770d20 le um dword de pilha nunca iniciado; o comparador agora roda so a original para ela).
# Refeitas depois da correcao (levas/329/reconf.txt e reconf_pt.txt): a sx07_valor_sem_assinatura foi acusada; estas
# nao divergem ate 2001-08-20. A comparacao longa (ate 2002-06-01, bancada 3: levas/329/conf_escapadas_3.txt) decide,
# e o resultado entra num commit seguinte, como na 320a, 324a e 328a.

_PT = ' A comparacao longa decide.'
_EN = ' The long comparison decides.'

ESCAPADAS = {
    'sx03_3_limites_40': (
        'EM CONFIRMACAO. O registro de 42 bytes copiado so ate 40: os dois ultimos bytes quase sempre ja coincidem.' + _PT,
        'BEING CONFIRMED. The 42-byte record copied only up to 40: the last two bytes nearly always already match.' + _EN),
    'sx03_4_sem_jogador_mundial': (
        'EM CONFIRMACAO. Sem o jogador do ano mundial: o premio e de 6 de maio, depois da confirmacao curta.' + _PT,
        'BEING CONFIRMED. No world player of the year: the award is on 6 May, after the short confirmation.' + _EN),
    'sx07_codigo_365': (
        'EM CONFIRMACAO. O prazo de 91 dias trocado por 365: so muda contratos que terminam entre os dois prazos.' + _PT,
        'BEING CONFIRMED. The 91-day limit swapped for 365: it only changes contracts ending between the two.' + _EN),
    'sx07_recusa_sem_motivos': (
        'EM CONFIRMACAO. A recusa sem os motivos: o texto so e escrito quando a noticia chega a um tecnico humano.' + _PT,
        'BEING CONFIRMED. The refusal without the reasons: the text is only written when the item reaches a human manager.' + _EN),
    'sx05_1_uma_decimal': (
        'EM CONFIRMACAO. Uma casa decimal em vez de duas: so muda valores com o segundo digito decimal diferente de 0.' + _PT,
        'BEING CONFIRMED. One decimal place instead of two: it only changes values whose second decimal digit is not 0.' + _EN),
    'sx07_nunca_em_casa': (
        'EM CONFIRMACAO. O jogo nunca em casa: na carga em portugues a divergencia era o erro do sx02_2 (o vendedor no lugar '
        'do comprador), corrigido; sem ele a carga curta nao diverge.' + _PT,
        'BEING CONFIRMED. The match never at home: in the Portuguese load the divergence was the sx02_2 error (the seller '
        'in place of the buyer), now fixed; without it the short load does not diverge.' + _EN),
}  # nome: (explicacao pt, explicacao en)
