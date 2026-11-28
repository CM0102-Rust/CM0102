# r7_05: o que ficou de fora da lista (rodada 7, 28/11/2026)

O grupo r7_05 tinha duas rotinas do `club_records.cpp`. A 0x449810 `club_records_format_record_text` está no item
r7_05_1. A outra continua fora, com o motivo conferido de novo na desmontagem:

| Endereço | Nome | Por quê |
|---|---|---|
| 0x4481b0 | season_history_entry_destruct | Só roda no desenrolar de uma exceção. A única referência no exe é o `jmp` do funclet de desenrolar 0x955056, na área dos tratadores SEH; nenhum `call` e nenhum endereço absoluto. O corpo monta o próprio quadro SEH e chama quatro vezes o destrutor vazio 0x5a53c0 (`ret`) sobre +0x156, +0x142, +0x12e e +0x11a do registro (this + 2 + 0x154...). Não tem efeito a comparar nem caminho que rode no jogo. É a mesma decisão do sx06 (`sobras.md`) e do rx08 (`rx08_11/FORA.md`). |
