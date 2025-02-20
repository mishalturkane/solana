use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use solana_sdk::system_instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::message::Message;

fn main() {
    // Set up the Solana RPC client
    let rpc_url = "https://api.devnet.solana.com"; // Change to mainnet-beta if needed
    let client = RpcClient::new(rpc_url.to_string());

    // Load the sender's keypair (Replace with actual keypair loading logic)
    let sender = Keypair::new(); // Replace with actual key loading

    // Define recipient address (Replace with actual recipient address)
    let recipient_pubkey = Pubkey::from_str("RecipientPublicKeyHere").unwrap();

    // Define the amount of lamports to send (1 SOL = 1_000_000_000 lamports)
    let amount = 1_000_000_000; // 1 SOL

    // Create the transfer instruction
    let transfer_instruction = system_instruction::transfer(
        &sender.pubkey(),
        &recipient_pubkey,
        amount,
    );

    // Create the message and transaction
    let message = Message::new(&[transfer_instruction], Some(&sender.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);

    // Fetch recent blockhash
    let recent_blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");
    transaction.sign(&[&sender], recent_blockhash);

    // Send transaction
    let signature = client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
    println!("Transaction successful! Signature: {}", signature);
}
