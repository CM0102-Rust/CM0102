# Mutacoes (tel29_1): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva; estas provam
# os comparadores numa sessao do Lucas (JOGAR_COMPARANDO.bat):
# - o Voltar nao trata tela marcada como raiz (o comparador lado a lado de is_back_unavailable pega numa tela de
#   instancia unica, como o Inbox);
# - os registros prontos contam como ativos (o comparador de count_active_records pega quando o gerente esta pronto
#   para continuar o dia).
MUTACOES = [
    ('tel29_1_voltar_raiz',
     "(read_u32(node + n::FLAG) != 0) as u32 // tel29_1: a flagged screen is a root for Back",
     "0 // mutation: no screen is a root for Back",
     'tel29_1 screen opened'),
    ('tel29_1_ativos',
     "if read_u32(rec + h::AWAY) == 0 && read_u32(rec + h::DONE) == 0 { // tel29_1: away and done both clear",
     "if read_u32(rec + h::AWAY) == 0 { // mutation: the done flag ignored",
     'tel29_1 screen opened'),
]
