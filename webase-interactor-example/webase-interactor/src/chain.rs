pub struct Chain{
    ip: String,
}

impl Chain{
    pub fn new(ip: String) -> Chain {
        Chain { ip }
    }
    pub fn get_ip(&self) -> String {
        self.ip.to_string()
    }

    pub fn get_block_number(&self) -> Result<String, reqwest::Error>{
        let mut url =self.ip.to_string();
        url += &"WeBASE-Front/1/web3/blockNumber/".to_string();
        let resp = 
            reqwest::blocking::get(&url)?
            // .await?
            .text();
            // .await?;

        resp
    }
}