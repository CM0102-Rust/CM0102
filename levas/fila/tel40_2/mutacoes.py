# A rede: so se prova numa partida em rede (duas maquinas), em modo Rust; o comparador roda so a original e nao as
# acusa. Ficam para essa prova: a despedida sem o endereco e a conexao que falhou continuando ocupada.
MUTACOES = [
    ('tel40_2_despedida_curta', '        send(this, p, (n + 0x64) as u32, NETWORK_PORT, 0, 0, 0); // tel40_2: the goodbye datagram', '        send(this, p, n as u32, NETWORK_PORT, 0, 0, 0); // mutation: the text only'),
    ('tel40_2_conexao_presa', '            write_u16(this + g::SOCKETS + 2 * k, 0xffff); // tel40_2: the closed connection freed', '            // mutation: the closed connection stays in use'),
]
