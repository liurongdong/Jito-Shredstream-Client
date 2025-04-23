use solana_sdk::pubkey::Pubkey;
use std::error::Error;

#[derive(Debug)]
pub struct CreateEventInstruction {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub user: Pubkey,
}

#[derive(Debug)]
pub struct BuyInstruction {
    pub amount: u64,
    pub max_sol_cost: u64,
}

pub fn parse_instruction_data(data: &[u8]) -> Result<(String, Option<CreateEventInstruction>, Option<BuyInstruction>), Box<dyn Error>> {
    let discriminator = &data[0..8];
    
    match discriminator {
        [0x18, 0x1e, 0xc8, 0x28, 0x05, 0x1c, 0x07, 0x77] => {
            let mut offset = 8;
            let name_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            let name = String::from_utf8(data[offset..offset + name_len].to_vec())?;
            offset += name_len;

            let symbol_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            let symbol = String::from_utf8(data[offset..offset + symbol_len].to_vec())?;
            offset += symbol_len;

            let uri_len = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            let uri = String::from_utf8(data[offset..offset + uri_len].to_vec())?;
            offset += uri_len;

            let user = Pubkey::new_from_array(data[offset..offset + 32].try_into().unwrap());

            let instruction = CreateEventInstruction { name, symbol, uri, user};
            Ok(("CreateEvent".to_string(), Some(instruction), None))
        }

        [0x66, 0x06, 0x3d, 0x12, 0x01, 0xda, 0xeb, 0xea] => {
            let mut offset = 8;
            let amount = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
            offset += 8;
            let max_sol_cost = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());

            let instruction = BuyInstruction { amount, max_sol_cost };
            Ok(("Buy".to_string(), None, Some(instruction)))
        }
        _ => Err("Unknown instruction data".into()),
    }
} 