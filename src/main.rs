

use std::thread::sleep;
use std::time::Duration;
use std::path::Path;

pub mod server;
pub mod gameserver;

use self::gameserver::GameServer;
use self::server::unixserver::UnixServer;


fn main() {
	
// 	let addr = "127.0.0.1:1234".parse().unwrap();

	let addr = Path::new("\0rustifarm");
	
	let socketserver = UnixServer::new(&addr).expect("binding server failed");
	
	let mut gameserver = GameServer::new(socketserver);
	
	println!("listening on {:?}", addr);
	
// 	let mut players: HashMap<usize, String> = HashMap::new();
	
	loop {
		gameserver.update();
		sleep(Duration::from_millis(100));
	}
}


