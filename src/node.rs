use crate::prover::ProverWork;
use futures_util::sink::SinkExt;
use rand::thread_rng;
use rand::Rng;
use snarkos::environment::Prover;
use snarkos::helpers::{NodeType, State};
use snarkos::{Data, Message};
use snarkos_storage::BlockLocators;
use snarkvm::dpc::testnet2::Testnet2;
use snarkvm::dpc::{Address, BlockHeader};
use snarkvm::prelude::BlockTemplate;
use snarkvm::traits::Network;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex};
use tokio::task;
use tokio::time::{sleep, timeout};
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::{debug, error, info, warn};

pub struct Node {
    address: Address<Testnet2>,
    operator: String,
    router: Arc<Sender<SendMessage>>,
    receiver: Arc<Mutex<Receiver<SendMessage>>>,
}

#[derive(Debug)]
pub struct SendMessage {
    pub(crate) message: Message<Testnet2, Prover<Testnet2>>,
}

impl Node {
    pub fn init(address: Address<Testnet2>, operator: String) -> Arc<Self> {
        let (router_tx, router_rx) = mpsc::channel(1024);
        Arc::new(Self {
            address,
            operator,
            router: Arc::new(router_tx),
            receiver: Arc::new(Mutex::new(router_rx)),
        })
    }

    pub fn router(&self) -> Arc<Sender<SendMessage>> {
        self.router.clone()
    }

    pub fn receiver(&self) -> Arc<Mutex<Receiver<SendMessage>>> {
        self.receiver.clone()
    }
}

pub fn start(
    prover_router: Arc<Sender<ProverWork>>,
    node: Arc<Node>,
    receiver: Arc<Mutex<Receiver<SendMessage>>>,
) {
    let test = true;
    if test {
       task::spawn(async move {
        loop {
            let sdata = "{\"previous_block_hash\":\"ab1p4v0mkver2l72ye7jzqa8f87j6z36l69d89fg8928f6x7twdkczsuywve7\",\"block_height\":327910,\"block_timestamp\":1645604590,\"difficulty_target\":425983861548280,\"cumulative_weight\":122172486703,\"previous_ledger_root\":\"al1xz9xlx3vstznnqef8luulna4kczy46up965pttr70wq0vzm3jyqq9k7zez\",\"transactions\":{\"transactions\":[{\"transaction_id\":\"at1gyl262r97m7pv9tmzw7atqlszwrm4sppae2dp6wgu6aavlfy7ypq8hhpaf\",\"inner_circuit_id\":\"ic13cstkmt5j4qqzfu5am8jx2rhxm0hqplyzcgzyueefz7n32xl4h53n4xmxvhjyzaq2c0f7l70a4xszau2ryc\",\"ledger_root\":\"al1enk2kwh9nuzcj2q9kdutekavlf8ayjqcuszgezsfax8qxn9k0yxqfr9fr2\",\"transitions\":[{\"transition_id\":\"as14z30tsum9h7r5yekrpux6jsx627el9qs53j7wvrrk22amjtgsqzqdcgf7p\",\"serial_numbers\":[\"sn1y3uqsgvxl77dqj2dw4u0w693v60yycwkgahhl5eplenc5484zgrqjnme67\",\"sn1hxxcaudqmuxl96ahh8szg88ds48xnsy4lps8v5ms857qnhsytyfq2r9fc8\"],\"commitments\":[\"cm1hvy94yar9elxat7sd9z3tzsa2rxsyse9reqhm3adens5tgn3pg8slh75n5\",\"cm16zazpa8x5w37fpe3jdupsd4leuxtfjp2a8pja356lmw8aptvaqgs9vmw3v\"],\"ciphertexts\":[\"recd16gjza0er6mpz5gr7elmsx54k0j29eurnva98p5q88k54prwdj5xlhecjmwx4mw5s3zzxjv3h8s6xc9gde69t92guwz52g894v3w7zq7yt6aswk7056a850r5qwe67h89h4d5mqec9hywxe77j7dplux4qda4wfgrzyn6f2c4dmk8ps4y4ne8uy9c6khsfj9w7hcm4xxfnwasraj0w7kq7eeqrff4yesutcszd3earfzuqh77szfssc7q0rhy9ugfs6r0a4qqcx6y78z2a4jm5q6m5cv2e2sec722p9plp0z8lezrhyqaqfc4maay5glu4s8x0x50vgey88pp8mk53k258l0qhwdn76szjzr54xnl8847kr0cvqwalkm9ntstas5txqsct68u26qx45lyjw0sp8z3sem426y7guqkxjxzpguendyy6z0ev2cxtnegap6gn82pattsxxn6xwn\",\"recd1ttx3sj30fwvesxh7ll2ya04zy0ah7r3q8dw852lfuh0yhs4ncuyjv58m7r9evc58zfnk7ygk6d3826k8e8myhvjkkd40h4z9ljz5jy9syeg3s3py2apvr43zlew2qy4llw0jq4ap23gwr8tvsxvthg09q5vug7tk0qeztxf0xe6rvey74gtvdsra3k5dzmcnxc026kqwyzgs36gx6xe453apmmrn5etpv2tewclfcuetyvzelcdyt22e2py96ucw6ym96fcfjan6ej9k9dhv49lys89vxlljjwhe23xeht6a8t6u9vf9fhtqps7gjyrx7xff7k7lgxq6hnhjsssrk7azgcuksjmyc957ypknnztnyt696ecwmfr0xms4anfhlf6hjmcnl085myekyfwxjxkrqxuxjkl7sa38zz6yjkk59a03eegw2500cje5686hhfpgg46syl5sg3wlura\"],\"value_balance\":-100000000,\"events\":[{\"id\":1,\"index\":0,\"record_view_key\":\"rcvk18xflfxag5ys2a5t2e32jktucfd6fenrlz86mjmj69kzm65yzkcrs7knd34\"},{\"id\":1,\"index\":1,\"record_view_key\":\"rcvk1386ap960cl6e5ktnmxx9gx4454xgf90lm5xvw0q64tfsly3t9ypszxsl5s\"}],\"proof\":\"ozkp1l9r85nrjyu560uvs95x29jp0f5unlzv0zgvc8apvv3spk9k8ha37qz27qhqtu9lszhfpz67prp58a7rkjyq6tg4jmdkdwpnwrk4tu4uygaxm6l0zj95wvn6r4al7gx9tyxcuhvqx7lszxtzhwkn9cr48sqnrfxjfpdtuc3xjy93zwujd6ae6uetd98pgfnyrgs7enxfnfjhxk00tvxshclva422gwsau24chp8kmeql2jzkmwuvfxu3glr6j8m8fjceekxku6dxr2dws5g8jmdr3rumtpg88eppw4nchgy3j45l7w6qt5ka9s2fngh8sv00jsh74dec7572vn838duqk9ad898jz97d58lzgj6dp8649gt3aqw44mcz92vhu280rcrh8duzgvzrf760qf8uqxwc56xnlevcztkfd5s7f9asw2c8u4peedyw4zdh60xt6l4xca7xqqqgc5urmg\"}]}]},\"coinbase_record\":{\"owner\":\"aleo18ueay9wlxgyh2l9cvj2elryww9jcxfxlfuskwu3q00e37z70xups5kycny\",\"value\":100000000,\"payload\":\"0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\",\"program_id\":\"ap1lhj3g5uzervu3km7rl0rsd0u5j6pj9ujum6yxrvms4mx8r2qhew88ga849hnjypghswxceh02frszs45qmd\",\"randomizer\":\"rr16gjza0er6mpz5gr7elmsx54k0j29eurnva98p5q88k54prwdj5xskdcxkr\",\"record_view_key\":\"rcvk18xflfxag5ys2a5t2e32jktucfd6fenrlz86mjmj69kzm65yzkcrs7knd34\",\"commitment\":\"cm1hvy94yar9elxat7sd9z3tzsa2rxsyse9reqhm3adens5tgn3pg8slh75n5\"}}";
            let block_template: BlockTemplate<Testnet2> = serde_json::from_str(&sdata).unwrap();
            info!("auto send ProverWork");
            if let Err(e) = prover_router.send(ProverWork::new(426034951614309, block_template)).await {
                error!("Error sending work to prover: {:?}", e);
            } else {
                debug!("Sent work to prover");
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }); 
    } else {
        task::spawn(async move {
            loop {
                info!("Connecting to operator...");
                match timeout(Duration::from_secs(5), TcpStream::connect(&node.operator)).await {
                    Ok(socket) => match socket {
                        Ok(socket) => {
                            info!("Connected to {}", node.operator);
                            let mut framed =
                                Framed::new(socket, Message::<Testnet2, Prover<Testnet2>>::PeerRequest);
                            let challenge = Message::ChallengeRequest(
                                12,
                                Testnet2::ALEO_MAXIMUM_FORK_DEPTH,
                                NodeType::Prover,
                                State::Ready,
                                4132,
                                thread_rng().gen(),
                                0,
                            );
                            if let Err(e) = framed.send(challenge).await {
                                error!("Error sending challenge request: {}", e);
                            } else {
                                debug!("Sent challenge request");
                            }
                            let receiver = &mut *receiver.lock().await;
                            loop {
                                tokio::select! {
                                    Some(message) = receiver.recv() => {
                                        let message = message.message.clone();
                                        debug!("Sending {} to operator", message.name());
                                        if let Err(e) = framed.send(message.clone()).await {
                                            error!("Error sending {}: {:?}", message.name(), e);
                                        }
                                    }
                                    result = framed.next() => match result {
                                        Some(Ok(message)) => {
                                            debug!("Received {} from operator", message.name());
                                            match message {
                                                Message::ChallengeRequest(..) => {
                                                    let resp = Message::ChallengeResponse(Data::Object(Testnet2::genesis_block().header().clone()));
                                                    if let Err(e) = framed.send(resp).await {
                                                        error!("Error sending challenge response: {:?}", e);
                                                    } else {
                                                        debug!("Sent challenge response");
                                                    }
                                                }
                                                Message::ChallengeResponse(..) => {
                                                    let ping = Message::<Testnet2, Prover<Testnet2>>::Ping(
                                                        12,
                                                        Testnet2::ALEO_MAXIMUM_FORK_DEPTH,
                                                        NodeType::Prover,
                                                        State::Ready,
                                                        Testnet2::genesis_block().hash(),
                                                        Data::Object(Testnet2::genesis_block().header().clone()),
                                                    );
                                                    if let Err(e) = framed.send(ping).await {
                                                        error!("Error sending ping: {:?}", e);
                                                    } else {
                                                        debug!("Sent ping");
                                                    }
                                                }
                                                Message::Ping(..) => {
                                                    let mut locators: BTreeMap<u32, (<Testnet2 as Network>::BlockHash, Option<BlockHeader<Testnet2>>)> = BTreeMap::new();
                                                    locators.insert(0, (Testnet2::genesis_block().hash(), None));
                                                    let resp = Message::<Testnet2, Prover<Testnet2>>::Pong(None, Data::Object(BlockLocators::<Testnet2>::from(locators).unwrap_or_default()));
                                                    if let Err(e) = framed.send(resp).await {
                                                        error!("Error sending pong: {:?}", e);
                                                    } else {
                                                        debug!("Sent pong");
                                                    }
                                                }
                                                Message::Pong(..) => {
                                                    let register = Message::<Testnet2, Prover<Testnet2>>::PoolRegister(node.address);
                                                    if let Err(e) = framed.send(register).await {
                                                        error!("Error sending pool register: {:?}", e);
                                                    } else {
                                                        debug!("Sent pool register");
                                                    }
                                                }
                                                Message::PoolRequest(share_difficulty, block_template) => {
                                                    if let Ok(block_template) = block_template.deserialize().await {
                                                        if let Err(e) = prover_router.send(ProverWork::new(share_difficulty, block_template)).await {
                                                            error!("Error sending work to prover: {:?}", e);
                                                        } else {
                                                            debug!("Sent work to prover");
                                                        }
                                                    } else {
                                                        error!("Error deserializing block template");
                                                    }
                                                }
                                                Message::UnconfirmedBlock(..) => {}
                                                _ => {
                                                    debug!("Unhandled message: {}", message.name());
                                                }
                                            }
                                        }
                                        Some(Err(e)) => {
                                            warn!("Failed to read the message: {:?}", e);
                                        }
                                        None => {
                                            error!("Disconnected from operator");
                                            sleep(Duration::from_secs(5)).await;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to connect to operator: {}", e);
                            sleep(Duration::from_secs(5)).await;
                        }
                    },
                    Err(_) => {
                        error!("Failed to connect to operator: Timed out");
                        sleep(Duration::from_secs(5)).await;
                    }
                }
            }
    
        });
    }
}
