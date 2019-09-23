

use std::thread::sleep;
use std::time::Duration;
use std::path::Path;
// use std::net::SocketAddr;

pub mod server;
pub mod gameserver;
pub mod simpleworld;

use self::gameserver::GameServer;
use self::server::unixserver::UnixServer;
use self::server::tcpserver::TcpServer;
use self::server::Server;


fn main() {
	

	let addr = Path::new("\0rustifarm");
	let unixserver = UnixServer::new(&addr).expect("binding unix server failed");
	
	let addr = "127.0.0.1:1234".parse().unwrap();
	let inetserver = TcpServer::new(&addr).expect("binding inet server failed");
	
	
	let servers: Vec<Box<Server>> = vec![Box::new(unixserver), Box::new(inetserver)];
	let mut gameserver = GameServer::new(servers);
	
// 	println!("listening on {:?}", addr);
	
// 	let mut players: HashMap<usize, String> = HashMap::new();
	
	loop {
		gameserver.update();
		sleep(Duration::from_millis(100));
	}
}


