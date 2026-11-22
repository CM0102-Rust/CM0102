# Mutacoes (tel21_4): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas provam
# o comparador lado a lado da entrada dos filtros numa sessao do Lucas com o JOGAR_COMPARANDO.bat (Search > Filters,
# pagina Attributes; a abertura so roda a original, entao mutacao nela nao aparece):
# - o nivel do atributo pula de 15 para 1 (clicar o nivel de um atributo ligado ate passar de 15);
# - o atributo ligado com minimo 11 em vez de 10 (ligar um atributo desligado).
MUTACOES = [
    ('tel21_4_nivel_pula',
     "            } else if minimum >= 20 {\n                1",
     "            } else if minimum >= 15 { // mutation: from 15 straight back to 1\n                1",
     'tel21_4 filters input attribute level'),
    ('tel21_4_liga_com_11',
     "            let new = if minimum == 1 { 10 } else { 1 };",
     "            let new = if minimum == 1 { 11 } else { 1 }; // mutation: switched on at 11",
     'tel21_4 filters input attribute toggle'),
]
