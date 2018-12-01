use game::{KtuluClient, KtuluServer};
use interface::*;
use messages::{KtuluMessage, KtuluPacket};
use std::collections::VecDeque;
use PlayerId;

pub const NAMES: &[&str] = &[
    "Alice", "Bob", "Carol", "Dave", "Eric", "Fred", "Gina", "Hank", "Iris", "Judy", "Kent",
    "Lucy", "Mike", "Nina", "Oran", "Paul", "Quin", "Rose", "Stan", "Tina",
];

type MockEndpoint = Option<usize>;

struct Packet {
    sender: MockEndpoint,
    recipient: MockEndpoint,
    packet: KtuluPacket,
}

pub struct TestEnvironment {
    manitou: KtuluServer<MockEndpoint>,
    characters: Vec<KtuluClient<MockEndpoint>>,
    packet_queue: VecDeque<Packet>,
}

impl TestEnvironment {
    pub fn manitou(&mut self) -> &mut KtuluServer<MockEndpoint> {
        &mut self.manitou
    }

    pub fn character(&mut self, i: usize) -> &mut KtuluClient<MockEndpoint> {
        &mut self.characters[i]
    }

    pub fn characters(
        &mut self,
    ) -> impl Iterator<Item = (MockEndpoint, &mut KtuluClient<MockEndpoint>)> {
        self.characters
            .iter_mut()
            .enumerate()
            .map(|(i, ch)| (Some(i), ch))
    }

    pub fn new() -> TestEnvironment {
        TestEnvironment {
            manitou: KtuluServer::new(),
            characters: Vec::new(),
            packet_queue: VecDeque::new(),
        }
    }

    pub fn add_player(&mut self, nick: String) {
        let new_client = KtuluClient::new(nick, None);
        self.characters.push(new_client);
    }

    pub fn send_message(&mut self, from: MockEndpoint, to: MockEndpoint, packet: KtuluPacket) {
        self.packet_queue.push_back(Packet {
            sender: from,
            recipient: to,
            packet,
        });
    }

    fn get_handler(
        &mut self,
        endpoint: MockEndpoint,
    ) -> &mut KtuluMessageHandler<Endpoint = MockEndpoint> {
        if let Some(index) = endpoint {
            &mut self.characters[index]
        } else {
            &mut self.manitou
        }
    }

    pub fn handle_all_messages(&mut self) {
        while !self.packet_queue.is_empty() {
            let Packet {
                sender,
                recipient,
                packet,
            } = self.packet_queue.pop_front().unwrap();
            // handle the packet from the front of the queue
            let new_messages = self.get_handler(recipient).handle_message(sender, packet);
            // send all responses from the recipient
            for KtuluMessage {
                recipient: to,
                packet,
            } in new_messages
            {
                self.send_message(recipient, to, packet);
            }
        }
    }
}
