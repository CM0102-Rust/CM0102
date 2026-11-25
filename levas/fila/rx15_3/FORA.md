# rx15: o que ficou de fora (leitor rx15, rodada 6, 25/11/2026)

Das 22 rotinas da lista `rx15.txt`, 10 entraram (rx15_1: 9 destrutores escalares; rx15_2: 0x5fb720). As outras 12:

| Endereço | Nome | Por quê |
|---|---|---|
| 0x8042c0 | SearchEditSession::unwind_destroy_working_criteria | Funclet de desenrolar do SEH (push 0x7ae4f0; push 0xa; inc ecx; push 0x2b0; push ecx; call 0x9462bb; ret): só roda numa exceção. Já decidido no sx19b_8. |
| 0x89f700 | thunk_tactics_screen_build | Thunk de um `jmp`. Já tratado no tel14_1 (`TACTICS_BUILD_RESERVE_FN`). |
| 0x8a4830 | thunk_tactics_build_team_instructions_dialog | Thunk de um `jmp` para 0x8a2db0 (tel15_5). |
| 0x8b0b20 | TRAINING_EDIT_SESSION::destroy_schedule_slots | Ação de desenrolar do SEH do construtor. Já decidido no sx20_10. |
| 0x404480 | shared_return_true | `mov eax, 1; ret`: corpo constante compartilhado (cx08_10). |
| 0x4110b0 | AusNsl::get_table_layout_type | `mov al, 3; ret`: 3 bytes, prólogo de menos de 5 (cx08_10). |
| 0x5961c0 | Finance::Finance | `mov eax, ecx; ret`: 3 bytes, prólogo de menos de 5. |
| 0x5a53c0 | shared_noop_ret0_5a53c0 | Corpo vazio compartilhado. |
| 0x5a8f60 | shared_noop_ret0_5a8f60 | Corpo vazio compartilhado. |
| 0x7e7850 | RulingBody::is_one_booking_from_ban | `xor eax, eax; ret 0x10`: corpo vazio da base (sx19_14). |
| 0x7f6750 | scout_manager_static_destroy_list_b74438 | `mov ecx, 0xb74438; jmp 0x68d040`: thunk de um `jmp` (sx19b_2). |
| 0x90d2e0 | VIRTUAL_STAFF::~VIRTUAL_STAFF | `add ecx, 0x5c; jmp 0x68d040` (LLIST::~LLIST): thunk de ajuste de um `jmp`, como o 0x7f6750. |
