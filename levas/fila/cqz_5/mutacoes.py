# Mutacoes (cqz_5). Os sorteios das tres copas caem depois de 2002-06-01 (a Copa da Asia de 2004, a Olimpiada de 2004,
# a Copa das Confederacoes de 2003) e o dia da liga de uma copa inativa devolve 0 sem mexer em nada: ate la a rotina
# nao tem efeito. Cada mutacao antecipa o sorteio da sua copa para o primeiro dia que a chamar (a copa ganha grupos,
# tabela e jogos na memoria do jogo, o banco, ja em 2001): o mesmo trecho do molde compartilhado, com a copa escolhida
# pelo endereco do sorteio.
_TRECHO = ('    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {\n        return 0;\n    }\n'
           '    if today_year == year && today_day == day && this.active() == 0 && (!plan.draw_needs_light_day || full == 0) {')


def _cedo(endereco, nome):
    return (f'    let early = plan.draw == {endereco}; // mutation: the {nome} draw at once\n'
            '    if !early && (today_year < year || (today_year == year && (today_day as i16) < day as i16)) {\n        return 0;\n    }\n'
            '    if (early || (today_year == year && today_day == day)) && this.active() == 0 && (!plan.draw_needs_light_day || full == 0) {')


MUTACOES = [
    ('cqz5_asia_sorteio_ja', _TRECHO, _cedo('0x40fdf0', 'Asian Cup'), 'cqz_5 asian cup day'),
    ('cqz5_olimpiada_sorteio_ja', _TRECHO, _cedo('0x7a09e0', 'Olympic'), 'cqz_5 olympics day'),
    ('cqz5_confederacoes_sorteio_ja', _TRECHO, _cedo('0x58c6b0', 'Confederations Cup'), 'cqz_5 confederations cup day'),
]
