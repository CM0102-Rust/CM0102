# Mutacoes (tel18_2): provam os comparadores numa sessao do Lucas (JOGAR_COMPARANDO.bat):
# - a coluna da taxa da tela de transferencias escreve "Free" num emprestimo (Transfers > Transfers, com a lista ja
#   montada: trocar de pagina);
# - o botao de pagina +10 anda 9 (Transfers > Transfers com mais de uma pagina > >>).
MUTACOES = [
    ('tel18_2_emprestimo_livre',
     "lang(TS_TEXT, match kind { 1 => 0x98e620, 2 => 0x98e5fc, 4 => 0x98e61c, _ => 0x98e614 });",
     "lang(TS_TEXT, match kind { 1 => 0x98e620, 2 => 0x98e614, 4 => 0x98e61c, _ => 0x98e614 }); // mutation: a loan shown as free",
     'tel18_2 transfers screen'),
    ('tel18_2_pagina_mais_nove',
     "8 => Some((page + 10).min(pages)),",
     "8 => Some((page + 9).min(pages)), // mutation: +9 pages",
     'tel18_2 transfers: next pages'),
]
