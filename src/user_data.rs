use std::collections::HashMap;
use std::io::Cursor;

use tokio::io::AsyncReadExt;

pub struct UserInfo {
    pub name: u64,
    pub hp: u8,
    pub san: u8,
    pub mp: u8,
    pub commands: [[u8; 20]; 10],
}

impl UserInfo {
    pub fn new(name: u64) -> Self {
        UserInfo {
            name,
            hp: 0,
            san: 0,
            mp: 0,
            commands: [[0u8; 20]; 10],
        }
    }

    async fn read(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let name = cursor.read_u64().await?;
        let hp = cursor.read_u8().await?;
        let san = cursor.read_u8().await?;
        let mp = cursor.read_u8().await?;

        let mut commands = [[0u8; 20]; 10];
        for i in 0..10 {
            cursor.read_exact(&mut commands[i]).await?;
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
        for i in 0..10 {
            result.append(&mut self.commands[i].into());
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

        let mut info = UserInfo::new(user_id);
        info.hp = hp;
        info.san = san;
        info.mp = mp;

        let mut data: HashMap<u64, UserInfo> = HashMap::new();
        data.insert(user_id, info);

        UserInfo::save_file(test_file, &data).await.unwrap();

        let data = UserInfo::load_file(test_file).await.unwrap();

        assert_eq!(data.get(&user_id).unwrap().name, user_id);
        assert_eq!(data.get(&user_id).unwrap().hp, hp);
        assert_eq!(data.get(&user_id).unwrap().san, san);
        assert_eq!(data.get(&user_id).unwrap().mp, mp);
    }
}
