# Mutações (sx09c_7). A multa roda pela AWOL na carreira de férias (o técnico da IA multa quem falta ao treino); a lista
# das multas é lida pela própria multa (multa anterior ainda valendo, 0x5a69b0) e pelas passadas diárias (0x5a5820).
MUTACOES = [
    ('sx09c_7_not_recorded',
     "exe::llist_add_tail(read_usize(this), rec.as_ptr() as usize, 0xe); // sx09c_7: the fine recorded",
     "let _ = rec; // mutation: the fine not recorded",
     'sx09c_7 fine'),
    ('sx09c_7_reason_kept',
     "reason = (reason & !0xff) | 5; // sx09c_7: absent without leave",
     "reason = (reason & !0xff) | 1; // mutation: the absence not seen",
     'sx09c_7 fine'),
    ('sx09c_7_staff_confidence',
     "fine_move_confidence(board() + board_record::STAFF_CONFIDENCE, -250); // sx09c_7: the staff lose confidence",
     "fine_move_confidence(board() + board_record::STAFF_CONFIDENCE, 250); // mutation: the staff gain confidence",
     'sx09c_7 fine'),
]
