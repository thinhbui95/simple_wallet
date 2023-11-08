use ethers::providers::{Provider, Http};
use ethers::prelude::*;
use ethers::contract::Contract;
use ethers::abi::Abi;
mod wallet;
use std::sync::Arc;
use std::str::FromStr;
use ethers::types::H160;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    abigen!(
        ABIToken,
        "./src/ABI.json",
        event_derives(serde::Deserialize, serde::Serialize)
    );
    let provider = Provider::<Http>::try_from("INPUT RPC")?;
       
    // Get the chain ID
    let chain_id  = provider.get_chainid().await?;
    println!("Chain id: {}", chain_id);

    let contract_abi: &[u8] = include_bytes!("./ABI.json");

    // Convert the byte slice to a JSON string
    let json_string = std::str::from_utf8(contract_abi).unwrap();

    // Parse the JSON string as an `ethers` ABI
    let abi = serde_json::from_str(&json_string).expect("Failed to parse ABI");
    // Parse the ABI
    let abi_contract = Abi::from(abi);

       // Create list of features to select from
    let selections = &[
        "Get Balance",
        "Get name token",
        "Get symbol token",
        "Transfer token",
        "Transfer native",
        "Import wallet",
        "Add new wallet",
        "Exit",
    ];
    loop {
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => {
                // Get balance 
                let token = dialoguer::Input::<Address>::new()
                    .with_prompt("Enter token address")
                    .interact()
                    .unwrap();

                let user = dialoguer::Input::<Address>::new()
                    .with_prompt("Enter address user")
                    .interact()
                    .unwrap();


                // Create a contract instance
                let contract_query: ContractInstance<Arc<Provider<Http>>, _> = Contract::new(token, abi_contract.clone(), provider.clone().into());
                
                // Query a function on the contract
                let result= contract_query.method::<H160, U256>("balanceOf", user).expect("Fail").call().await.unwrap();
                println!("{}" , result);

            }
            1 => {
                // Get name
                let token = dialoguer::Input::<Address>::new()
                    .with_prompt("Enter token address")
                    .interact()
                    .unwrap();

                    // Create a contract instance
                let contract_query: ContractInstance<Arc<Provider<Http>>, _> = Contract::new(token, abi_contract.clone(), provider.clone().into());
                
                // Query a function on the contract
                let result= contract_query.method::<_, String>("name", ()).expect("Fail").call().await.unwrap();
                println!("{}" , result);

            }
            2 => {
                // Get symbol
                let token = dialoguer::Input::<Address>::new()
                    .with_prompt("Enter token address")
                    .interact()
                    .unwrap();
        
                // Create a contract instance
                let contract_query: ContractInstance<Arc<Provider<Http>>, _> = Contract::new(token, abi_contract.clone(), provider.clone().into());
                        
                // Query a function on the contract
                let result= contract_query.method::<_, String>("symbol", ()).expect("Fail").call().await.unwrap();
                println!("{}" , result);
  
            }
            3 => {
                // Transfer token 
                let token = dialoguer::Input::<String>::new()
                    .with_prompt("Enter token address")
                    .interact()
                    .unwrap();  
                let key_file = dialoguer::Input::<String>::new()
                    .with_prompt("Choose your wallet ")
                    .interact()
                    .unwrap();
        
                let amount = dialoguer::Input::<u128>::new()
                    .with_prompt("Enter amount ")
                    .interact()
                    .unwrap();
        
                let receiver = dialoguer::Input::<String>::new()
                    .with_prompt("Enter receiver ")
                    .interact()
                    .unwrap();

                let sercret = wallet::wallet::load_key(key_file.clone());
                let wallet: LocalWallet = sercret.parse::<LocalWallet>()?.with_chain_id(chain_id.as_u64()) ;
                let client = SignerMiddleware::new(provider.clone(), wallet.clone());
                let contract = ABIToken::new::<H160>(H160::from_str(&token).unwrap(), Arc::new(client.clone()));
                let binding = contract.transfer(H160::from_str(&receiver.clone()).unwrap(),U256::from(amount));
                let tx = binding.send().await?.await?;
                println!("{:?}", tx);
            }
            4=> {
                // Transfer native
                let key_file = dialoguer::Input::<String>::new()
                    .with_prompt("Choose your wallet ")
                    .interact()
                    .unwrap();
                
                let amount = dialoguer::Input::<u128>::new()
                    .with_prompt("Enter amount ")
                    .interact()
                    .unwrap();
                
                let receiver = dialoguer::Input::<Address>::new()
                    .with_prompt("Enter receiver ")
                    .interact()
                    .unwrap();
                let tx = TransactionRequest::new()
                    .to(receiver)
                    .value(U256::from(amount))
                    .from(H160::from_str(&key_file.clone()).unwrap());

                let sercret = wallet::wallet::load_key(key_file.clone());
                let wallet: LocalWallet = sercret.parse::<LocalWallet>()?.with_chain_id(chain_id.as_u64()) ;
                let client = SignerMiddleware::new(provider.clone(), wallet.clone());
                let tx = client.send_transaction(tx, None).await?.await?;
                println!("{:?}", tx);
   
            }

            5=> {
                // Import wallet 
                let secret = dialoguer::Input::<String>::new()
                    .with_prompt("Please your private key :  ")
                    .interact()
                    .unwrap();
            
                 wallet::wallet::import_wallet(secret);
                }
            6=> {
                //Create new wallet 
                let address = wallet::wallet::create_wallet();
                println!("Address :  {}", address);
            }
            7=> {
                // Exit
                println!("Goodbye!");
                break;
            }
             _ => {
                println!("Invalid selection");
            }
        }
    }
    Ok(())
}



