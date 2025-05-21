use solana_hash::Hash;
use solana_message::{MessageHeader, v0};
use solana_pubkey::Pubkey;
use v1tx::{v1, v2, v3};

fn main() {
    let blockhash = Hash::new_unique();
    let payer = Pubkey::new_unique();

    // ————————————————
    // v0: noop / limit+price / full
    // ————————————————
    // noop
    let v0_noop = v0::Message::try_compile(&payer, &[], &[], blockhash).unwrap();
    println!(
        "v0 noop                    = {}",
        bincode::serialized_size(&v0_noop).unwrap()
    );
    // limit + price
    let limit_price_instruction_set = [
        solana_compute_budget_interface::ComputeBudgetInstruction::set_compute_unit_limit(12345),
        solana_compute_budget_interface::ComputeBudgetInstruction::set_compute_unit_price(12345),
    ];
    let v0_limit_price =
        v0::Message::try_compile(&payer, &limit_price_instruction_set, &[], blockhash).unwrap();
    println!(
        "v0 with cu limit + price   = {}",
        bincode::serialized_size(&v0_limit_price).unwrap()
    );
    // full
    let full_ix_set = [
        solana_compute_budget_interface::ComputeBudgetInstruction::set_compute_unit_limit(12345),
        solana_compute_budget_interface::ComputeBudgetInstruction::set_compute_unit_price(12345),
        solana_compute_budget_interface::ComputeBudgetInstruction::set_loaded_accounts_data_size_limit(12345),
        solana_compute_budget_interface::ComputeBudgetInstruction::request_heap_frame(12345),
    ];
    let v0_full = v0::Message::try_compile(&payer, &full_ix_set, &[], blockhash).unwrap();
    println!(
        "v0 with full cb ix set     = {}\n",
        bincode::serialized_size(&v0_full).unwrap()
    );

    // ————————————————
    // v1: noop / limit+price / full
    // ————————————————
    // noop
    let v1_noop = v1::Message {
        header: v1::MessageHeader {
            compute_unit_price: 0,
            compute_unit_limit: 0,
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![payer],
        recent_blockhash: blockhash,
        instructions: vec![],
        address_table_lookups: vec![],
    };
    println!(
        "v1 noop                    = {}",
        bincode::serialized_size(&v1_noop).unwrap()
    );
    // limit + price
    let v1_limit_price = v1::Message {
        header: v1::MessageHeader {
            compute_unit_price: 12345,
            compute_unit_limit: 12345,
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![payer], /* v1 does not need compute budget program for cu limit/price */
        recent_blockhash: blockhash,
        instructions: vec![],
        address_table_lookups: vec![],
    };
    println!(
        "v1 with cu limit + price   = {}",
        bincode::serialized_size(&v1_limit_price).unwrap()
    );
    // full
    let v1_full = v1::Message {
        header: v1::MessageHeader {
            compute_unit_price: 12345,
            compute_unit_limit: 12345,
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 1,
        },
        account_keys: vec![payer, solana_compute_budget_interface::ID], /* v1 does need compute budget program for loaded accounts/heap */
        recent_blockhash: blockhash,
        instructions: vec![
            v0_full.instructions[2].clone(),
            v0_full.instructions[3].clone(),
        ],
        address_table_lookups: vec![],
    };
    println!(
        "v1 with full cb ix set     = {}\n",
        bincode::serialized_size(&v1_full).unwrap()
    );

    // ————————————————
    // v2: noop / limit+price / full
    // ————————————————
    // noop
    let v2_noop = v2::Message {
        header: v2::MessageHeader {
            compute_unit_price: 0,
            compute_unit_limit: 0,
            loaded_accounts_data_limit: 0,
            requested_heap_bytes: 0,
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![payer], /* v2 does NOT need compute budget program now, but may need it if more resources are added later */
        recent_blockhash: blockhash,
        instructions: vec![],
        address_table_lookups: vec![],
    };
    println!(
        "v2 noop                    = {}",
        bincode::serialized_size(&v2_noop).unwrap()
    );
    // limit + price
    let v2_limit_price = v2::Message {
        header: v2::MessageHeader {
            compute_unit_price: 12345,
            compute_unit_limit: 12345,
            loaded_accounts_data_limit: 0,
            requested_heap_bytes: 0,
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![payer], /* v2 does NOT need compute budget program now, but may need it if more resources are added later */
        recent_blockhash: blockhash,
        instructions: vec![],
        address_table_lookups: vec![],
    };
    println!(
        "v2 with cu limit + price   = {}",
        bincode::serialized_size(&v2_limit_price).unwrap()
    );
    // full
    let v2_full = v2::Message {
        header: v2::MessageHeader {
            compute_unit_price: 12345,
            compute_unit_limit: 12345,
            loaded_accounts_data_limit: 12345,
            requested_heap_bytes: 12345,
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 1,
        },
        account_keys: vec![payer], /* v2 does NOT need compute budget program now, but may need it if more resources are added later */
        recent_blockhash: blockhash,
        instructions: vec![],
        address_table_lookups: vec![],
    };
    println!(
        "v2 with full cb ix set     = {}\n",
        bincode::serialized_size(&v2_full).unwrap()
    );

    // ————————————————
    // v3: noop / limit+price / full
    // ————————————————
    // noop
    let v3_noop = v3::Message {
        compute_budget_header: v3::ComputeBudgetHeader::new(None, None, None, None),
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![payer], /* v3 will never need compute budget program. if solana ever gets to >8 limits we should press the off button and go home */
        recent_blockhash: blockhash,
        instructions: vec![],
        address_table_lookups: vec![],
    };
    println!(
        "v3 noop                    = {}",
        bincode::serialized_size(&v3_noop).unwrap()
    );
    // limit + price
    let v3_limit_price = v3::Message {
        compute_budget_header: v3::ComputeBudgetHeader::new(Some(12345), Some(12345), None, None),
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![payer], /* v3 will never need compute budget program. if solana ever gets to >8 limits we should press the off button and go home */
        recent_blockhash: blockhash,
        instructions: vec![],
        address_table_lookups: vec![],
    };
    println!(
        "v3 with cu limit + price   = {}",
        bincode::serialized_size(&v3_limit_price).unwrap()
    );
    // full
    let v3_full = v3::Message {
        compute_budget_header: v3::ComputeBudgetHeader::new(
            Some(12345),
            Some(12345),
            Some(12345),
            Some(12345),
        ),
        header: MessageHeader {
            num_required_signatures: 1,
            num_readonly_signed_accounts: 0,
            num_readonly_unsigned_accounts: 0,
        },
        account_keys: vec![payer], /* v3 will never need compute budget program. if solana ever gets to >8 limits we should press the off button and go home */
        recent_blockhash: blockhash,
        instructions: vec![],
        address_table_lookups: vec![],
    };
    println!(
        "v3 with full cb ix set     = {}",
        bincode::serialized_size(&v3_full).unwrap()
    );
}
