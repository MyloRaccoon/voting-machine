use async_trait::async_trait;
use tokio::net::UdpSocket;

use crate::{interfaces::{cli_interface::handle_line, lexicon::Lexicon}, service::Service, storage::Storage, use_cases::VotingController};

pub struct UdpService<Store> {
	port: u16,
	lexicon: Lexicon,
	controller: VotingController<Store>,
}

#[async_trait]
impl<Store: Storage + Send + Sync> Service<Store> for UdpService<Store> {

	fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
		Self {
			port,
			lexicon,
			controller,
		}
	}

	async fn serve(&mut self) -> Result<(), anyhow::Error> {
		let server_endpoint = format!("127.0.0.1:{}", self.port);
		let socket = UdpSocket::bind(server_endpoint).await?;
		let mut buf = vec![0; 1000];

	    loop {
	    	let (len, src) = socket.recv_from(&mut buf).await?;
	    	let message = String::from_utf8_lossy(&buf[..len - 1]);
	    	let res = handle_line(&message, &mut self.controller, &self.lexicon).await?;
	    	if res == self.lexicon.quit {
	            break;
	        } else {
	        	socket.send_to(res.as_bytes(), &src).await?;
	            println!("{}", self.lexicon.please);
	        }
	    }
	    
	    Ok(())
	}

}