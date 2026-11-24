# Telas: a carreira automatica nao as roda; todas so a original com o plano no comparador, entao as mutacoes valem em
# modo Rust (o construtor roda em todo inicio de jogo, o remove ao sair de um tecnico numa partida de varios humanos).
MUTACOES = [
    ('tel46_2_display_errado', '    write_usize(sm + screen_manager::DISPLAY, sm + h::LOCAL_DISPLAY); // tel46_2: record 0 shows the local Display', '    write_usize(sm + screen_manager::DISPLAY, sm + SCRMAN_ECHO_DISPLAY); // mutation'),
    ('tel46_2_registros_parados', '        core::ptr::copy_nonoverlapping(global_record(c, j) as *const u8, global_record(c, j - 1) as *mut u8, h::RECORD_SIZE); // tel46_2: the records move up', '        let _ = j; // mutation: the records stay'),
]
