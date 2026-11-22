# A base da interface (2): a carreira automatica nao abre telas, entao a cadeia nao roda mutacoes nesta leva. Estas
# provam o comparador numa sessao do Lucas com o JOGAR_COMPARANDO.bat:
# - um tecnico sem registro conta como presente (is_manager_away; o comparador pelo valor pega quando se pergunta por
#   uma chave sem registro, ex. os tecnicos do computador nas telas de tecnicos);
# - um tecnico pronto continua podendo virar o atual (switch_to_manager, so com mais de um tecnico humano no jogo).
MUTACOES = [
    ('tel38_2_sem_registro_presente',
     "None => 1, // tel38_2: no record counts as away",
     "None => 0, // mutation: no record counts as present"),
    ('tel38_2_pronto_vira_atual',
     "if read_u32(rec + h::AWAY) != 0 || read_u32(rec + h::DONE) != 0 { // tel38_2: away or done managers stay put",
     "if read_u32(rec + h::AWAY) != 0 { // mutation: done managers can be switched in"),
]
