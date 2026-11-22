# Mutations (tel36_14). These only run in a network game and the comparers run the original only: the proof is a
# network game in Rust mode (two machines or two instances). The mutation shows there: the server list keeps 8 at most.
MUTACOES = [
    ('tel36_14_server_limit', 'if exe::crt_strncmp(p, NETWORK_LIST_ACK, ack) == 0 && count < 9 {', 'if exe::crt_strncmp(p, NETWORK_LIST_ACK, ack) == 0 && count < 8 {'),
]
