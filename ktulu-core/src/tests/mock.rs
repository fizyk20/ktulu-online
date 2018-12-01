use game::{Character, Manitou};
use interface::*;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use PlayerId;

pub struct TestEnvironmentImpl {
    manitou: Manitou<TestClient>,
    characters: Vec<Character<TestClient>>,
}

impl TestEnvironmentImpl {
    pub fn manitou(&self) -> &Manitou<TestClient> {
        &self.manitou
    }

    pub fn character(&self, i: usize) -> &Character<TestClient> {
        &self.characters[i]
    }
}

pub struct TestEnvironment(Rc<RefCell<TestEnvironmentImpl>>);

impl Deref for TestEnvironment {
    type Target = RefCell<TestEnvironmentImpl>;
    fn deref(&self) -> &RefCell<TestEnvironmentImpl> {
        &self.0
    }
}

pub struct TestClient {
    char_index: usize,
    env: Rc<RefCell<TestEnvironmentImpl>>,
}

impl ManitouClient for TestClient {
    fn connect(&mut self) -> PlayerId {
        let character = TestClient {
            char_index: self.char_index,
            env: self.env.clone(),
        };
        self.env.borrow_mut().manitou.connect(character)
    }
}

impl CharacterClient for TestClient {}

impl TestEnvironment {
    pub fn new() -> TestEnvironment {
        TestEnvironment(Rc::new(RefCell::new(TestEnvironmentImpl {
            manitou: Manitou::new(),
            characters: Vec::new(),
        })))
    }

    fn get_test_client(&self, char_index: usize) -> TestClient {
        TestClient {
            char_index: char_index,
            env: self.0.clone(),
        }
    }

    pub fn add_player(&self) {
        let new_index = self.0.borrow().characters.len();
        let player = Character::new(self.get_test_client(new_index));
        self.0.borrow_mut().characters.push(player);
    }
}
