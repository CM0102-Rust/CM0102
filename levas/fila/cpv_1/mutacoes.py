# Mutacoes (com o ramo de cada uma). Todas na oferta passada aos clubes (0x8c2f50, 1452 chamadas ate 2002-06-01 na
# cobertura de 17/10); as noticias so vao as caixas dos humanos. A espera da permissao de trabalho em 16 a 22 dias (e
# nao 18 a 24) muda a data em que a oferta volta (a transferencia de jogador de fora da UE fecha em outro dia). As
# rivais reagendadas em 10 dias (e nao 20) voltam antes a fila (outras respostas, outros dias). Sem a confirmacao do
# vendedor do computador, a oferta fica parada no status 0xb: a transferencia nao sai.
MUTACOES = [
    ('cpv1_permissao_16', 'let dias = (r as u32 & !0xff) | (r as u8).wrapping_add(0x12) as u32; // cpv1: a espera da permissao',
     'let dias = (r as u32 & !0xff) | (r as u8).wrapping_add(0x10) as u32; // mutacao: 16 a 22 dias',
     'cpv_1 permissao de trabalho'),
    ('cpv1_rivais_10', 'agendar(this, cpv1_d(cpv1_d(this) + cpv1_d(x) * 4), 0x14, 0); // cpv1: as rivais em 20 dias',
     'agendar(this, cpv1_d(cpv1_d(this) + cpv1_d(x) * 4), 0xa, 0); // mutacao: 10 dias',
     'cpv_1 ofertas rivais canceladas'),
    ('cpv1_sem_confirmar', 'confirmar(this, vaga, vendedor); // cpv1: o vendedor confirma',
     'let _ = (confirmar, vaga); // mutacao: o vendedor nao confirma',
     'cpv_1 confirmada pelo vendedor'),
]
