# Mutacoes (cpw_7), no dia das faltas ao treino (0x4190d0, todo dia; as faltas marcadas pela 0x41a360 passam de 64 ate
# 2002-06-01, contador "cps_6 falta ao treino" da 255a): o caso repetido sem os sorteios (um ou dois sorteios a menos
# por caso), a falta resolvida duas semanas depois em vez de uma (os sorteios do caso mudam de dia) e o caso resolvido
# sem sair da lista (volta a ser resolvido todo dia, com sorteios a mais). As tres mudam a contagem dos sorteios e tudo
# o que sorteia depois. A primeira pode escapar se nenhum caso for repetido (o byte +0xc do registro vem da pilha da
# 0x41a360: os contadores "cpw_7 caso repetido" e "cpw_7 caso novo" mostram qual ramo roda).
MUTACOES = [
    ('cpw7_repetido_sem_sorteio', '            if cpw7_sortear(cpw7_b(pessoa + 0x5b) as i8 as i32) == 0 && cpw7_sortear(cpw7_b(pessoa + 0x5d) as i8 as i32) == 0 { // cpw7: os sorteios do caso repetido',
     '            if false { // mutacao: sem os sorteios',
     'cpw_7 caso repetido'),
    ('cpw7_duas_semanas', '                if weeks_after_have_passed(r + 4, 1) { // cpw7: uma semana depois',
     '                if weeks_after_have_passed(r + 4, 2) { // mutacao: duas semanas',
     'cpw_7 falta vencida'),
    ('cpw7_nao_sai_da_lista', '    tirar(cpw7_d(this + 4), r); // cpw7: tira o caso da lista',
     '    let _ = tirar; // mutacao: nao sai da lista',
     'cpw_7 falta tirada da lista'),
]
