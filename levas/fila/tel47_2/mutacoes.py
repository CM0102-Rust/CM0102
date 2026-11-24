# A rede: so se prova numa partida em rede (duas maquinas na mesma rede local), em modo Rust; os comparadores rodam
# so a original e nao as acusam. As tres mudam o que a outra maquina ve: a lista de servidores vazia (a difusao para o
# endereco da rede, ou nada enviado) ou toda conexao recusada pelo anfitriao.
MUTACOES = [
    ('tel47_2_difusao_errada', '        write_u32(mask + 4, (address & bits) | !bits); // tel47_2: the broadcast address',
     '        write_u32(mask + 4, address & bits); // mutation: the network address, not the broadcast one'),
    ('tel47_2_sem_difusao', "        sent = sent.wrapping_add(ws_sendto(s, data, length, address.as_ptr() as usize, 16)); // tel47_2: each adapter's broadcast",
     '        sent = sent.wrapping_add(length + 0 * address.len() as i32); // mutation: nothing sent'),
    ('tel47_2_conexao_recusada', '        if j < count { // tel47_2: a free slot found',
     '        if false && j < count { // mutation: every connection refused'),
]
