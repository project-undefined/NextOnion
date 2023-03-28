

use crate::*;

pub struct Observer { 
    client: Client,
}

impl Observer { 
    pub async fn builder(mut self, server: &str) -> Result<()> { 
       self.client = Client::builder() 
            .with_node(server)?
            .finish()
            .await?; 
        Ok(())
    }

    pub async fn connect(mut self) {
        
    }
}