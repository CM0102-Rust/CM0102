# Mutacoes. Os amistosos da IA mudam as datas, os adversarios, o desgaste e as financas dos clubes: o plano da
# pre-temporada sem criar o amistoso (515 chamadas ate 2002-06-01: os amistosos planejados somem), o amistoso pedido em
# casa com mando trocado (o publico e a renda vao para o outro clube) e a data sorteada da excursao um dia depois (356).
# Os ramos ficam depois de todas as condicoes, colados no efeito.
MUTACOES = [
    ('cps12_sem_amistoso', 'let m = if a == b { acrescentar(this, cps12_clube(a), e, cps12_clube(c), 1) } else { acrescentar(this, cps12_clube(a), e, cps12_clube(b), 0) }; // cps12: criar',
     'let m = 0usize; // mutacao: nao cria',
     'cps_12 amistoso planejado'),
    ('cps12_mando_trocado', '(clube, adversario) // cps12: o clube em casa',
     '(adversario, clube) // mutacao: mando trocado',
     'cps_12 amistoso em casa'),
    ('cps12_dia_seguinte', 'let n = (ini as i32).wrapping_add(sortear(fim as i32 - ini as i32)); // cps12: o dia sorteado',
     'let n = (ini as i32).wrapping_add(sortear(fim as i32 - ini as i32)).wrapping_add(1); // mutacao: um dia depois',
     'cps_12 data livre'),
]
