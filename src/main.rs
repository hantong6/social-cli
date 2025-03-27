use std::error::Error;
use std::str::FromStr;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair, Signature};
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum SocialInstruction {
    Init(String),
    Follow(Pubkey),
    Unfollow(Pubkey),
    QueryFollows,
    Post(String),
    QueryPosts
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UserProfile {
    pub data_len: u16,
    pub followers: Vec<Pubkey>
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UserPost {
    pub post_count: u64
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Post {
    pub content: String,
    pub timestamp: u64
}

impl UserProfile {

    pub fn new() -> Self {
        UserProfile {
            data_len: 0,
            followers: vec![]
        }
    }

    pub fn follow(&mut self, follower: Pubkey) {
        self.followers.push(follower);
        self.data_len = self.followers.len() as u16;
    }

}

const USER_PROFILE_SEED: &str = "profile";
const USER_POST_SEED: &str = "post";

pub struct SocialClient {
    rpc_client: RpcClient,
    program_id: Pubkey
}

impl SocialClient {
    pub fn new(rpc_url: &str, program_id: Pubkey) -> Self {
        SocialClient {
            rpc_client: RpcClient::new(rpc_url.to_string()),
            program_id
        }
    }

    pub fn init_user(&self, user: &Keypair, seed_type: &str) -> Result<(), Box<dyn Error>> {
        let social_pda = get_social_pda(&self.program_id, &[user.pubkey().as_ref(), seed_type.as_bytes()]);
        let init_user_data = SocialInstruction::Init(seed_type.to_string());
        let init_user_acc = vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(social_pda, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false)
        ];
        let init_user_ins = Instruction::new_with_borsh(
            self.program_id,
            &init_user_data,
            init_user_acc
        );
        let sign = self.send_instruction(user, vec![init_user_ins])?;
        println!("init user success, sign: {:?}", sign);
        Ok(())
    }

    pub fn follow_user(&self, user: &Keypair, follow_user: Pubkey) -> Result<(), Box<dyn Error>> {
        let social_pda = get_social_pda(&self.program_id, &[user.pubkey().as_ref(), USER_PROFILE_SEED.as_bytes()]);
        let follow_user_data = SocialInstruction::Follow(follow_user);
        let follow_user_acc = vec![
            AccountMeta::new(social_pda, false),
        ];
        let follow_user_ins = Instruction::new_with_borsh(
            self.program_id,
            &follow_user_data,
            follow_user_acc
        );
        let sign = self.send_instruction(user, vec![follow_user_ins])?;
        println!("follow user success, sign: {:?}", sign);
        Ok(())
    }

    pub fn query_follow(&self, user: &Keypair) -> Result<(), Box<dyn Error>> {
        let social_pda = get_social_pda(&self.program_id, &[user.pubkey().as_ref(), USER_PROFILE_SEED.as_bytes()]);
        let query_follow_data = SocialInstruction::QueryFollows;
        let query_follow_acc = vec![
            AccountMeta::new(social_pda, false),
        ];
        let query_follow_ins = Instruction::new_with_borsh(
            self.program_id,
            &query_follow_data,
            query_follow_acc
        );
        let sign = self.send_instruction(user, vec![query_follow_ins])?;
        println!("query follow success, sign: {:?}", sign);
        Ok(())
    }

    pub fn unfollow_user(&self, user: &Keypair, follow_user: Pubkey) -> Result<(), Box<dyn Error>> {
        let social_pda = get_social_pda(&self.program_id, &[user.pubkey().as_ref(), USER_PROFILE_SEED.as_bytes()]);
        let unfollow_user_data = SocialInstruction::Unfollow(follow_user);
        let unfollow_user_acc = vec![
            AccountMeta::new(social_pda, false),
        ];
        let unfollow_user_ins = Instruction::new_with_borsh(
            self.program_id,
            &unfollow_user_data,
            unfollow_user_acc
        );
        let sign = self.send_instruction(user, vec![unfollow_user_ins])?;
        println!("unfollow user success, sign: {:?}", sign);
        Ok(())
    }

    pub fn post(&self, user: &Keypair, content:String, id: u64) -> Result<(), Box<dyn Error>> {
        let social_pda = get_social_pda(&self.program_id, &[user.pubkey().as_ref(), USER_POST_SEED.as_bytes()]);
        let social_post_pda = get_social_pda(&self.program_id, &[user.pubkey().as_ref(), USER_POST_SEED.as_bytes(), &[id as u8]]);
        let user_post_data = SocialInstruction::Post(content);
        let user_post_acc = vec![
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(social_pda, false),
            AccountMeta::new(social_post_pda, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        ];
        let user_post_ins = Instruction::new_with_borsh(
            self.program_id,
            &user_post_data,
            user_post_acc
        );
        let sign = self.send_instruction(user, vec![user_post_ins])?;
        println!("user post success, sign: {:?}", sign);
        Ok(())
    }

    pub fn query_post(&self, user: &Keypair, id: u64) -> Result<(), Box<dyn Error>> {
        let social_pda = get_social_pda(&self.program_id, &[user.pubkey().as_ref(), USER_POST_SEED.as_bytes()]);
        let social_post_pda = get_social_pda(&self.program_id, &[user.pubkey().as_ref(), USER_POST_SEED.as_bytes(), &[id as u8]]);
        let query_post_data = SocialInstruction::QueryPosts;
        let query_post_acc = vec![
            AccountMeta::new(social_pda, false),
            AccountMeta::new(social_post_pda, false),
        ];
        let query_post_ins = Instruction::new_with_borsh(
            self.program_id,
            &query_post_data,
            query_post_acc
        );
        let sign = self.send_instruction(user, vec![query_post_ins])?;
        println!("query post success, sign: {:?}", sign);
        Ok(())
    }

    pub fn send_instruction(&self, payer: &Keypair, instructions: Vec<Instruction>) -> Result<(Signature), Box<dyn Error>> {
        let latest_blockhash = self.rpc_client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &instructions,
            Some(&payer.pubkey()),
            &[payer],
            latest_blockhash
        );
        let sign = self.rpc_client.send_and_confirm_transaction(&tx)?;
        Ok(sign)
    }
}


fn get_social_pda(program_id: &Pubkey, seed: &[&[u8]]) -> Pubkey {
    let (social_pda, _bump) = Pubkey::find_program_address(seed, &program_id);
    println!("social_pda: {:?}", social_pda);
    social_pda
}

fn main() -> Result<(), Box<dyn Error>> {
    // let user_profile = UserProfile::new();
    // print!("user profile len is {:?}", borsh::to_vec(&user_profile).unwrap().len());
    let program_id = Pubkey::from_str("AbiXdK7xj8T3HsgUPKxhYhNQJ8DsujgxeBz9Q8bcfPxu")?;
    let user = read_keypair_file("/home/hantong/.config/solana/id-local.json")?;
    let social_client = SocialClient::new("http://127.0.0.1:8899", program_id);
    // 初始化账号
    social_client.init_user(&user, USER_PROFILE_SEED)?;
    let follower_user = Pubkey::from_str("CAz782xYgu4q8zcg5VDafaRLCLDq6FiYPXFSqX5xQtWJ")?;
    // 关注
    social_client.follow_user(&user, follower_user)?;
    // 查询关注
    social_client.query_follow(&user)?;
    // 取消关注
    social_client.unfollow_user(&user, follower_user)?;
    // 查询关注
    social_client.query_follow(&user)?;

    // // 初始化post账户
    // social_client.init_user(&user, USER_POST_SEED)?;
    // // 发送帖子
    // let id = 1;
    // let content = "1: hello".to_string();
    // social_client.post(&user, content, id)?;
    // // 查询帖子
    // social_client.query_post(&user, id)?;
    Ok(())
}
