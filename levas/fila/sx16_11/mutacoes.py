# Mutacoes (sx16_11) pela prova do save (mutacoes.py --prova-save N; CARGA). Nenhuma abre caixa.
# - o jogo novo com nove registros de reserva (e nao dez): o retire.dat gravado na prova muda;
# - a carga com a fila de regens vazia (a contagem lida e zerada depois dos registros): as regeneracoes pendentes do
#   save de ferias nao acontecem depois da carga.
MUTACOES = [
    ('sx16_11_nove_reservas',
     '        for k in 0..0xa {',
     '        for k in 0..9 { // mutation'),
    ('sx16_11_fila_vazia',
     '        if n != 0 && exe::zread(records, size, n, z) != n {\n            return fail(row_records, &mut z);\n        }',
     '        if n != 0 && exe::zread(records, size, n, z) != n {\n            return fail(row_records, &mut z);\n        }\n        if count == player_regen_fields::REGEN_QUEUE_COUNT { write_u32(this + count, 0); } // mutation'),
]
