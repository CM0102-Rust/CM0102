# Mutacoes (rx14_5): a fase apagada sem liberar o objeto e sem rodar o destrutor da classe (as tabelas da fase
# vazam); os dois vazamentos mudam os enderecos das alocacoes seguintes no alocador da simulacao.
MUTACOES = [
    ('rx14_5_fase_sem_liberar_o_objeto', 'delete_through(this, flags, 0x523300) // rx14_5: the stage', 'delete_through(this, 0, 0x523300) // mutacao', 'rx14_5 cup stage scalar delete'),
    ('rx14_5_fase_sem_o_destrutor', 'delete_through(this, flags, 0x523300) // rx14_5: the stage', '{ exe::crt_operator_delete(this); this } // mutacao', 'rx14_5 cup stage scalar delete'),
]
