# Mutacoes (sx02_1). A lista volta a quem posta a noticia; o comparador lado a lado acusa qualquer clube a mais ou a
# menos (a carreira de ferias tem um tecnico humano ingles: as noticias de lesao 0x5a, estatisticas 0xda e contrato
# 0x21a dos jogadores ingleses o alcancam).
# - o alcance nacional desligado (jogadores com reputacao nacional acima de 7200/8300 deixam de chegar a ele);
# - a forma da divisao contra 5,0 em vez de 9,0 (mais jogadores da divisao do clube humano chegam a ele);
# - o clube humano que observa o jogador deixado de fora.
MUTACOES = [
    ('sx02_1_sem_alcance_nacional',
     'r.national = true; // sx02_1: national by reputation',
     'r.national = false; // mutation: no national reach',
     'sx02_1 national reach'),
    ('sx02_1_forma_da_divisao',
     '(current > reputation * 350 && current > 2500) || in_form(read_f64(0x96d230)) // sx02_1: division form',
     '(current > reputation * 350 && current > 2500) || in_form(5.0) // mutation: division form against 5.0',
     'sx02_1 national reach'),
    ('sx02_1_sem_quem_observa',
     'push_recipient(out, &mut n, club.addr()); // sx02_1: a shortlisting human club',
     '{} // mutation: the shortlisting human club left out',
     'sx02_1 shortlisting human club'),
]
