use std::collections::HashMap;
use std::io::{BufRead, Cursor};

use tokio::io::AsyncReadExt;

pub struct UserInfo {
    pub name: u64,
    pub hp: u8,
    pub san: u8,
    pub mp: u8,
    pub commands: HashMap<String, String>,
}

impl UserInfo {
    pub fn new(name: u64) -> Self {
        UserInfo {
            name,
            hp: 0,
            san: 0,
            mp: 0,
            commands: HashMap::new(),
        }
    }

    async fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let name = cursor.read_u64().await?;
        let hp = cursor.read_u8().await?;
        let san = cursor.read_u8().await?;
        let mp = cursor.read_u8().await?;

        let mut commands: HashMap<String, String> = HashMap::new();
        let length = cursor.read_u8().await?;
        for _ in 0..length {
            let mut buffer = vec![];
            let length = cursor.read_until(b'\x03', &mut buffer)?;
            let command_name = String::from_utf8(buffer[..(length - 1)].to_vec()).unwrap();

            let mut buffer = vec![];
            let length = cursor.read_until(b'\x03', &mut buffer)?;
            let command_content = String::from_utf8(buffer[..(length - 1)].to_vec()).unwrap();
            commands.insert(command_name, command_content);
        }

        Ok(Self {
            name,
            hp,
            san,
            mp,
            commands,
        })
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        result.append(&mut self.name.to_be_bytes().to_vec());
        result.push(self.hp);
        result.push(self.san);
        result.push(self.mp);
        result.push(self.commands.len() as u8);
        for (key, value) in &self.commands {
            result.append(&mut key.as_bytes().to_vec());
            result.push(b'\x03');
            result.append(&mut value.as_bytes().to_vec());
            result.push(b'\x03');
        }

        result
    }

    pub async fn save_file(
        file: &str,
        data: &HashMap<u64, UserInfo>,
    ) -> Result<(), std::io::Error> {
        let mut content: Vec<u8> = Vec::new();
        for info in data.values() {
            content.append(&mut info.to_vec());
        }

        tokio::fs::write(file, content).await?;

        Ok(())
    }

    pub async fn load_file(file: &str) -> Result<HashMap<u64, UserInfo>, std::io::Error> {
        let loaded = tokio::fs::read(file).await?;

        let loaded = &loaded[..];
        let mut cursor = Cursor::new(loaded);

        let mut data_vec: Vec<UserInfo> = Vec::new();

        while cursor.position() < loaded.len() as u64 {
            let info = UserInfo::read(&mut cursor).await?;
            data_vec.push(info);
        }

        let mut data: HashMap<u64, UserInfo> = HashMap::new();
        for info in data_vec {
            data.insert(info.name, info);
        }

        Ok(data)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Save data to the file and load it.
    /// That should be the same data.
    #[tokio::test]
    async fn user_info_test1() {
        let test_file = "./test.coc";

        let user_id = 12345;
        let hp = 10;
        let san = 11;
        let mp = 12;
        let mut commands: HashMap<String, String> = HashMap::new();
        commands.insert("SpotHidden".to_string(), "/r 50".to_string());

        let mut info = UserInfo::new(user_id);
        info.hp = hp;
        info.san = san;
        info.mp = mp;
        info.commands = commands;

        let mut data: HashMap<u64, UserInfo> = HashMap::new();
        data.insert(user_id, info);

        UserInfo::save_file(test_file, &data).await.unwrap();

        let data = UserInfo::load_file(test_file).await.unwrap();

        assert_eq!(data.get(&user_id).unwrap().name, user_id);
        assert_eq!(data.get(&user_id).unwrap().hp, hp);
        assert_eq!(data.get(&user_id).unwrap().san, san);
        assert_eq!(data.get(&user_id).unwrap().mp, mp);
        assert_eq!(
            data.get(&user_id)
                .unwrap()
                .commands
                .get("SpotHidden")
                .unwrap(),
            "/r 50"
        );
    }
}
