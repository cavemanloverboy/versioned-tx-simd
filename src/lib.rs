#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

use solana_hash::Hash;
use solana_message::{compiled_instruction::CompiledInstruction, v0::MessageAddressTableLookup};
use solana_pubkey::Pubkey;

// Add CU price/limit to header
pub mod v1 {
    use super::*;

    #[cfg_attr(
        feature = "serde",
        derive(Deserialize, Serialize),
        serde(rename_all = "camelCase")
    )]
    pub struct Message {
        pub header: MessageHeader,

        #[cfg_attr(feature = "serde", serde(with = "solana_short_vec"))]
        pub account_keys: Vec<Pubkey>,

        pub recent_blockhash: Hash,

        #[cfg_attr(feature = "serde", serde(with = "solana_short_vec"))]
        pub instructions: Vec<CompiledInstruction>,

        #[cfg_attr(feature = "serde", serde(with = "solana_short_vec"))]
        pub address_table_lookups: Vec<MessageAddressTableLookup>,
    }

    #[cfg_attr(
        feature = "serde",
        derive(Deserialize, Serialize),
        serde(rename_all = "camelCase")
    )]
    pub struct MessageHeader {
        /* NEW FIELD */
        pub compute_unit_price: u64,
        /* NEW FIELD */
        pub compute_unit_limit: u32,
        pub num_required_signatures: u8,
        pub num_readonly_signed_accounts: u8,
        pub num_readonly_unsigned_accounts: u8,
    }
}

// Add CU price/limit + loaded accounts data size + requested heap bytes size to header
pub mod v2 {
    use super::*;

    #[cfg_attr(
        feature = "serde",
        derive(Deserialize, Serialize),
        serde(rename_all = "camelCase")
    )]
    pub struct Message {
        pub header: MessageHeader,

        #[cfg_attr(feature = "serde", serde(with = "solana_short_vec"))]
        pub account_keys: Vec<Pubkey>,

        pub recent_blockhash: Hash,

        #[cfg_attr(feature = "serde", serde(with = "solana_short_vec"))]
        pub instructions: Vec<CompiledInstruction>,

        #[cfg_attr(feature = "serde", serde(with = "solana_short_vec"))]
        pub address_table_lookups: Vec<MessageAddressTableLookup>,
    }

    #[cfg_attr(
        feature = "serde",
        derive(Deserialize, Serialize),
        serde(rename_all = "camelCase")
    )]
    pub struct MessageHeader {
        /* NEW FIELD */
        pub compute_unit_price: u64,
        /* NEW FIELD */
        pub compute_unit_limit: u32,
        /* NEW FIELD */
        pub loaded_accounts_data_limit: u32,
        /* NEW FIELD */
        pub requested_heap_bytes: u32,
        pub num_required_signatures: u8,
        pub num_readonly_signed_accounts: u8,
        pub num_readonly_unsigned_accounts: u8,
    }
}

// Add ComputeBudgetFlags + dynamic payload to end
#[rustfmt::skip]
pub mod v3 {
    use super::*;

    use bitflags::bitflags;
    use solana_message::MessageHeader;

    #[repr(C)]
    #[cfg_attr(
        feature = "serde",
        derive(Deserialize, Serialize),
        serde(rename_all = "camelCase")
    )]
    #[cfg_attr(test, derive(PartialEq, Eq))]
    #[derive(Debug)]
    pub struct Message {
        #[cfg_attr(feature = "serde", serde(with = "compute_budget_header_serde"))]
        pub compute_budget_header: ComputeBudgetHeader,

        pub header: MessageHeader,

        #[cfg_attr(feature = "serde", serde(with = "solana_short_vec"))]
        pub account_keys: Vec<Pubkey>,

        pub recent_blockhash: Hash,
        
        #[cfg_attr(feature = "serde", serde(with = "solana_short_vec"))]
        pub instructions: Vec<CompiledInstruction>,
        
        #[cfg_attr(feature = "serde", serde(with = "solana_short_vec"))]
        pub address_table_lookups: Vec<MessageAddressTableLookup>,
        
    }
    
    // This lets us use u64s and u32 at the end of Message!
    const _: () = assert!(core::mem::size_of::<Message>() % 8 == 0);
    const _: () = assert!(core::mem::align_of::<Message>() == 8);


    #[repr(C)]
    #[cfg_attr(
        feature = "serde",
        derive(Deserialize, Serialize),
        serde(rename_all = "camelCase")
    )]
    #[cfg_attr(test, derive(PartialEq, Eq))]
    #[derive(Debug)]
    pub struct ComputeBudgetHeader {
        flags: ComputeBudgetFlags,
        compute_unit_limit: Option<u32>,
        compute_unit_price: Option<u64>,
        loaded_accounts_data_limit: Option<u32>,
        requested_heap_bytes_limit: Option<u32>,
    }

    impl ComputeBudgetHeader {
        pub fn new(
            compute_unit_limit: Option<u32>,
            compute_unit_price: Option<u64>,
            loaded_accounts_data_limit: Option<u32>,
            requested_heap_bytes_limit: Option<u32>,
        ) -> ComputeBudgetHeader {
            let mut flags = ComputeBudgetFlags::empty();
    
            if compute_unit_limit.is_some() {
                flags |= ComputeBudgetFlags::COMPUTE_UNIT_LIMIT;
            }
            if compute_unit_price.is_some() {
                flags |= ComputeBudgetFlags::COMPUTE_UNIT_PRICE;
            }
            if loaded_accounts_data_limit.is_some() {
                flags |= ComputeBudgetFlags::LOADED_ACCOUNTS_DATA_LIMIT;
            }
            if requested_heap_bytes_limit.is_some() {
                flags |= ComputeBudgetFlags::REQUESTED_HEAP_BYTES_LIMIT;
            }
    
            ComputeBudgetHeader {
                flags,
                compute_unit_limit,
                compute_unit_price,
                loaded_accounts_data_limit,
                requested_heap_bytes_limit,
            }
        }
    }
    

    bitflags! {
        #[cfg_attr(
            feature = "serde",
            derive(Deserialize, Serialize),
            serde(rename_all = "camelCase")
        )]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct ComputeBudgetFlags: u8 {
            const COMPUTE_UNIT_LIMIT         = 0b00000001;
            const COMPUTE_UNIT_PRICE         = 0b00000010;
            const LOADED_ACCOUNTS_DATA_LIMIT = 0b00000100;
            const REQUESTED_HEAP_BYTES_LIMIT = 0b00001000;
        }
    }

    pub const fn const_max(a: usize, b: usize) -> usize {
        [a, b][(a < b) as usize]
    }


    #[cfg(feature = "serde")]
    mod compute_budget_header_serde {
        use crate::v3::ComputeBudgetFlags;

        use super::ComputeBudgetHeader;
        use serde::{de::SeqAccess, ser::SerializeStruct, Deserializer, Serializer};

        pub fn serialize<S>(
            value: &ComputeBudgetHeader,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let ComputeBudgetHeader {
                flags,
                compute_unit_limit,
                compute_unit_price,
                loaded_accounts_data_limit,
                requested_heap_bytes_limit
            } = value;

            let num_present_flags = flags.iter().count();
            let mut serde_state = serializer.serialize_struct("ComputeBudgetHeader", 1 + num_present_flags)? ;


            serde_state.serialize_field("flags", &value.flags)?;
            if let Some(compute_unit_limit)= compute_unit_limit {
                serde_state.serialize_field("compute_unit_limit", &compute_unit_limit)? ;
            }
            if let Some(compute_unit_price)= compute_unit_price {
                serde_state.serialize_field("compute_unit_price", &compute_unit_price)? ;
            }
            if let Some(loaded_accounts_data_limit)= loaded_accounts_data_limit {
                serde_state.serialize_field("loaded_accounts_data_limit", &loaded_accounts_data_limit)? ;
            }
            if let Some(requested_heap_bytes_limit)= requested_heap_bytes_limit {
                serde_state.serialize_field("requested_heap_bytes_limit", &requested_heap_bytes_limit)? ;
            }

            serde_state.end()
        }
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> Result<ComputeBudgetHeader, D::Error>
        where
            D: Deserializer<'de>,
        {
            use serde::de::{Visitor, MapAccess, Error};
            use std::fmt;
        
            #[derive(serde_derive::Deserialize)]
            #[serde(field_identifier, rename_all = "snake_case")]
            enum Field {
                Flags,
                ComputeUnitLimit,
                ComputeUnitPrice,
                LoadedAccountsDataSize,
                RequestedHeapBytesLimit,
            }
        
            struct ComputeBudgetHeaderVisitor;
        
            impl<'de> Visitor<'de> for ComputeBudgetHeaderVisitor {
                type Value = ComputeBudgetHeader;
        
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("struct ComputeBudgetHeader")
                }
        
                fn visit_map<V>(self, mut map: V) -> Result<ComputeBudgetHeader, V::Error>
                where
                    V: MapAccess<'de>,
                {
                    let mut flags: Option<ComputeBudgetFlags> = None;
                    let mut compute_unit_limit: Option<u32> = None;
                    let mut compute_unit_price: Option<u64> = None;
                    let mut loaded_accounts_data_limit: Option<u32> = None;
                    let mut requested_heap_bytes_limit: Option<u32> = None;
        
                    while let Some(key) = map.next_key()? {
                        match key {
                            Field::Flags => {
                                if flags.is_some() {
                                    return Err(Error::duplicate_field("flags"));
                                }
                                flags = Some(map.next_value()?);
                            }
                            Field::ComputeUnitLimit => {
                                if compute_unit_limit.is_some() {
                                    return Err(Error::duplicate_field("compute_unit_limit"));
                                }
                                compute_unit_limit = Some(map.next_value()?);
                            }
                            Field::ComputeUnitPrice => {
                                if compute_unit_price.is_some() {
                                    return Err(Error::duplicate_field("compute_unit_price"));
                                }
                                compute_unit_price = Some(map.next_value()?);
                            }
                            Field::LoadedAccountsDataSize => {
                                if loaded_accounts_data_limit.is_some() {
                                    return Err(Error::duplicate_field("loaded_accounts_data_limit"));
                                }
                                loaded_accounts_data_limit = Some(map.next_value()?);
                            }
                            Field::RequestedHeapBytesLimit => {
                                if requested_heap_bytes_limit.is_some() {
                                    return Err(Error::duplicate_field("requested_heap_bytes_limit"));
                                }
                                requested_heap_bytes_limit = Some(map.next_value()?);
                            }
                        }
                    }
        
                    let flags = flags.ok_or_else(|| Error::missing_field("flags"))?;

                    // reject invalid bits set in ComputeBudgetFlags::all()
                    let invalid_bits = flags.bits() & !ComputeBudgetFlags::all().bits();
                    if invalid_bits != 0 {
                        return Err(Error::custom(format!(
                            "invalid ComputeBudgetFlags bits: {:#08b}", invalid_bits
                        )));
                    }


        
                    Ok(ComputeBudgetHeader {
                        flags,
                        compute_unit_limit,
                        compute_unit_price,
                        loaded_accounts_data_limit,
                        requested_heap_bytes_limit,
                    })
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<ComputeBudgetHeader, A::Error>
                where
                    A: SeqAccess<'de>,
                {
                    // flags is always present
                    let flags: ComputeBudgetFlags = seq
                        .next_element()?
                        .ok_or_else(|| Error::invalid_length(0, &self))?;
                    let invalid = flags.bits() & !ComputeBudgetFlags::all().bits();
                    if invalid != 0 {
                        return Err(Error::custom(format!(
                            "invalid ComputeBudgetFlags bits: {:#08b}",
                            invalid
                        )));
                    }

                    // only read each option if the corresponding flag bit is set
                    let compute_unit_limit = if flags.contains(ComputeBudgetFlags::COMPUTE_UNIT_LIMIT) {
                        Some(seq.next_element()?.ok_or_else(|| Error::invalid_length(1, &self))?)
                    } else {
                        None
                    };

                    let compute_unit_price = if flags.contains(ComputeBudgetFlags::COMPUTE_UNIT_PRICE) {
                        Some(seq.next_element()?.ok_or_else(|| Error::invalid_length(2, &self))?)
                    } else {
                        None
                    };

                    let loaded_accounts_data_limit =
                        if flags.contains(ComputeBudgetFlags::LOADED_ACCOUNTS_DATA_LIMIT) {
                            Some(seq.next_element()?.ok_or_else(|| Error::invalid_length(3, &self))?)
                        } else {
                            None
                        };

                    let requested_heap_bytes_limit =
                        if flags.contains(ComputeBudgetFlags::REQUESTED_HEAP_BYTES_LIMIT) {
                            Some(seq.next_element()?.ok_or_else(|| Error::invalid_length(4, &self))?)
                        } else {
                            None
                        };


                    Ok(ComputeBudgetHeader {
                        flags,
                        compute_unit_limit,
                        compute_unit_price,
                        loaded_accounts_data_limit,
                        requested_heap_bytes_limit,
                    })
                }
            }
        
            const FIELDS: &[&str] = &[
                "flags",
                "compute_unit_limit",
                "compute_unit_price",
                "loaded_accounts_data_limit",
                "requested_heap_bytes_limit",
            ];
            deserializer.deserialize_struct(
                "ComputeBudgetHeader",
                FIELDS,
                ComputeBudgetHeaderVisitor,
            )
        }
    }

    #[test]
    fn test_roundtrip_header_all_bincode() {
        let cu_prices = [Some(12), None];
        let cu_limits = [Some(34), None];
        let loaded_data_limits = [Some(56), None];
        let heap_limits = [Some(78), None];

        for cu_price in cu_prices {
            for cu_limit in cu_limits {
                for loaded_data_limit in loaded_data_limits {
                    for heap_limit in heap_limits {
                        let header = ComputeBudgetHeader::new(cu_limit, cu_price, loaded_data_limit, heap_limit);
                        let result = bincode::deserialize::<ComputeBudgetHeader>(&bincode::serialize(&header).unwrap()).unwrap();
                        assert_eq!(header, result)
                    }
                }
            }
        }
    }

    #[test]
    fn test_roundtrip_message_all_bincode() {
        let cu_prices = [Some(12), None];
        let cu_limits = [Some(34), None];
        let loaded_data_limits = [Some(56), None];
        let heap_limits = [Some(78), None];

        for cu_price in cu_prices {
            for cu_limit in cu_limits {
                for loaded_data_limit in loaded_data_limits {
                    for heap_limit in heap_limits {
                        let message = Message {
                            compute_budget_header: ComputeBudgetHeader::new(cu_limit, cu_price, loaded_data_limit, heap_limit),
                            header: MessageHeader {
                                num_required_signatures: 1,
                                num_readonly_signed_accounts: 2,
                                num_readonly_unsigned_accounts: 3,
                            },
                            account_keys: vec![],
                            recent_blockhash: Hash::new_unique(),
                            instructions: vec![],
                            address_table_lookups: vec![],
                            
                        };
                        let result = bincode::deserialize::<Message>(&bincode::serialize(&message).unwrap()).unwrap();
                        assert_eq!(message, result)
                    }
                }
            }
        }
    }
}
