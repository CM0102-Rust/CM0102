# The contract screens (4): the basic terms tab of the offer contract screen and its answers (contract_screens.cpp;
# reader tel24b, the screens round, 21/11/2026). Equal in cm0102-1080.exe (no byte of 0x4dd0f0..0x4e4e94 differs).
# Conventions checked in the disassembly:
# - 0x4e06e0 build_basic_terms_tab cdecl(terms, edited, staff, int top, int bottom) ret 0, SEH frame (Text; a local
#   STAFF_CONTRACT built by 0x85e410 and never destroyed). The x87: the threshold reputation × 1.25/0.75/0.5/0.25 +
#   10000/7500/5000/2500 (one _ftol, kept as a word); the interest = (float)(param 0x1a × 0.001f) stored as a float;
#   param 0x18 loaded once as a double: × 1.5 against the edited wage and param 0x16 (fcompp, st0 = limit × 1.5), the
#   limit itself (_ftol), the wage steps k × limit × 0.125 (_ftol, k = 1..8); the interest against 2.75, 3.15, 0.75
#   and 1.25 (fcomp). All exact in f64 (integers below 2^31 times binary fractions).
# - 0x4e2280 handle_basic_terms_action cdecl(short item) -> int, SEH frame (Cash locals); one _ftol of the wage limit
#   (job answer) and one of the desired contract's float × 1000.
# - Called by address: STAFF_CONTRACT::format_wage_text 0x85e750 thiscall(buffer, mode, style) ret 0xc;
#   format_expiry_text 0x85fa30 thiscall(buffer, style) ret 8; increase/decrease_wage_step 0x867b70/0x867c50
#   thiscall() ret 0 (the original reads param 8 before the first and drops it);
#   transfer_manager_build_refusal_reasons_text 0x8e4ae0 thiscall on [0xde1f54](club id, club id, manager id, manager
#   id, x, double interest, buffer) ret 0x20 -> al; Date::get_short_date_string 0x54a650 thiscall(buffer, size,
#   flags) ret 0xc; date_get_short_month_name 0x54d210 cdecl(month) (reads the low byte; the original pushes a dword
#   whose high bytes are a stack leftover); Cash::set_from_amount_table 0x43f720 thiscall ret 0xc returns the amount.
# Comparers: the tab side by side over the display list, the human's and the edited terms and the contract lookup
# global; the answers that only change the terms or the params (ids 6, 7, 8, 0xb, 0x16, 0x19, 0x1a, 0x1c, 0x1e, 0x30)
# side by side over the params, both terms and the global; the "+"/"-" steps (widgets and redraw), the squad status
# (set_to_desired_contract draws) and the missing terms (fatal box) only in the original.
# Depends on tel24_1, tel24_2, tel24_3 (cs_human_terms, cs_will_negotiate, cs_money_text, cs_fatal, cs_staff_wage_limit,
# CS_TERMS_*), tel26_1, tel17_2, tel29_1 (STR_EMPTY).
# Defects of the original: the wage and fee "+"/"-" answers call set_widget_id where set_widget_flags is meant on the
# neighbour widget (its flags never change when it is disabled or enabled again); a person with an edited squad status
# outside 1..6 and no non-player sheet leaves the text buffer as it was (the previous screen's text is shown); the
# job answer of a non-player loops forever when he can take none of the jobs 0..10; the name buffer is passed as 2000
# bytes and has 480 [inference: names are short, no overflow].
TELAS = True
ROTINAS = [('contract_screens_build_basic_terms_tab', 0x4e06e0, [0x6a, 0xff, 0x68, 0xf3, 0x71, 0x95, 0x00]),
           ('contract_screens_handle_basic_terms_action', 0x4e2280, [0x6a, 0xff, 0x68, 0x42, 0x72, 0x95, 0x00])]
_F = 'contract_screens.cpp'
DOCS = ('as telas de contrato (4): a aba Básico da oferta de contrato e as suas respostas',
        'the contract screens (4): the basic terms tab of the offer contract and its answers',
        [('contract_screens_build_basic_terms_tab', 'contract_screens_build_basic_terms_tab', '0x4e06e0', _F, 'cdecl',
          'monta a aba Básico: a fala da pessoa (espera termos quando pediu termos, o teto × 1,5 passa o salário pedido e o interesse passa de 2,75, ou já é do clube com a marca 2, ou o cargo é 8 a 10 com salário abaixo de 3200, ou tem menos de 24 anos com salário abaixo do limiar da reputação do clube pela situação no elenco + 3250: "espera", "quer", "está pedindo" ou "exige", com as luvas e o fim do contrato; senão, no próprio clube, as queixas do contrato; fora dele, as razões da recusa, o texto do clube com interesse abaixo de 0,75 ou o "sem interesse" da situação no elenco), o teto da diretoria (o salário do humano é cortado a ele; "jogador-chave" quando o presidente o favorece), as linhas de salário, luvas, data de início, fim do contrato, tipo e oferta pública com os botões -, + e =, os menus de 8 passos do salário e das luvas, e a data da transferência ou a próxima janela',
          'builds the basic terms tab: what the person says (he expects terms when he asked for terms, the limit × 1.5 is above the edited wage and the interest is above 2.75, or he is already at the club with the flag 2, or the job is 8 to 10 with a wage below 3200, or he is under 24 with a wage below the threshold of the club\'s reputation by squad status + 3250: "expects", "wants", "is requesting" or "is demanding", with the fee and the expiry; otherwise, at his own club, the contract\'s grievances; elsewhere, the refusal reasons, the club text with interest below 0.75 or the squad status\' "no interest"), the board\'s limit (the human\'s wage is cut to it; "key signing" when the chairman favours him), the wage, signing on fee, start date, expiry, type and public offer rows with their -, + and = buttons, the 8-step wage and fee popups, and the transfer date or the next window'),
         ('contract_screens_handle_basic_terms_action', 'contract_screens_handle_basic_terms_action', '0x4e2280', _F, 'cdecl',
          'responde à aba Básico: - e + do salário (um passo; o + desativa no teto do parâmetro 0x18) e das luvas (modo 4; o + desativa acima do orçamento) e redesenham; = e os menus voltam ao pedido ou ao passo escolhido (as luvas cortadas ao orçamento); - e + do fim do contrato mudam um ano (passado o primeiro) ou um mês, o - do primeiro mês alterna mês a mês e fixo, = volta ao fim pedido; a oferta pública, a data da transferência, o cargo (o jogador troca 11 e 15; os outros o próximo cargo possível até 10, com o teto salarial), tempo parcial e a situação no elenco (desce um degrau e refaz o contrato desejado); sem os termos do humano, a caixa fatal (linha 0xb34)',
          'answers the basic terms tab: - and + of the wage (one step; + disabled at the limit of param 0x18) and of the fee (mode 4; + disabled above the budget) and redraw; = and the popups go back to the request or to the chosen step (the fee cut to the budget); - and + of the expiry move a year (past the first) or a month, the first month\'s - swaps month to month and fixed, = goes back to the requested expiry; the public offer, the transfer date, the job (a player swaps 11 and 15; anyone else the next job he can take up to 10, with the wage limit), part time and the squad status (one step down, the desired contract made again); without the human\'s terms, the fatal box (line 0xb34)')])
