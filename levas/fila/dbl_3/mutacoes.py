# Mutacoes da comissao do clube com a do parceiro. As tres rotinas rodam na carreira (distribuicao dos treinadores
# nas contratacoes e transferencias, regeneracao); a criacao do .qst so passa por elas se, nos dias ate a data de
# inicio, um tecnico assumir ou alguem da comissao for contratado: PODEM ESCAPAR da prova do save (lacuna); os
# comparadores rodam no jogo real comparando da cadeia. A do auxiliar devolve o do proprio clube (0) no lugar do
# parceiro; a dos treinadores pega os do proprio clube outra vez; a da contagem esquece os do parceiro.
MUTACOES = [
    ('dbl_auxiliar', '    rd(p + 0xd3) // o auxiliar do parceiro', '    rd(clube + 0xd3) // mutacao: o auxiliar do proprio clube no lugar do parceiro'),
    ('dbl_treinadores', '        let t = rd(p + 0x19f + 4 * j); // os treinadores do parceiro', '        let t = rd(clube + 0x19f + 4 * j); // mutacao: os treinadores do proprio clube no lugar dos do parceiro'),
    ('dbl_contar_treinadores', '        n = n.wrapping_add(contar(p)); // e os do parceiro', '        // mutacao: sem os treinadores do parceiro'),
]
