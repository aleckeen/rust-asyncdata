use async_trait::async_trait;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[async_trait]
pub trait AsyncReadBytesExt: AsyncReadExt + Unpin {
    async fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).await?;
        Ok(buf[0])
    }

    async fn read_i8(&mut self) -> io::Result<i8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf).await?;
        Ok(buf[0] as i8)
    }

    async fn read_u16_be(&mut self) -> io::Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf).await?;
        Ok(u16::from_be_bytes(buf))
    }

    async fn read_u16_le(&mut self) -> io::Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf).await?;
        Ok(u16::from_le_bytes(buf))
    }

    async fn read_i16_be(&mut self) -> io::Result<i16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf).await?;
        Ok(i16::from_be_bytes(buf))
    }

    async fn read_i16_le(&mut self) -> io::Result<i16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf).await?;
        Ok(i16::from_le_bytes(buf))
    }

    async fn read_u32_be(&mut self) -> io::Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(u32::from_be_bytes(buf))
    }

    async fn read_u32_le(&mut self) -> io::Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(u32::from_le_bytes(buf))
    }

    async fn read_i32_be(&mut self) -> io::Result<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(i32::from_be_bytes(buf))
    }

    async fn read_i32_le(&mut self) -> io::Result<i32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(i32::from_le_bytes(buf))
    }

    async fn read_u64_be(&mut self) -> io::Result<u64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(u64::from_be_bytes(buf))
    }

    async fn read_u64_le(&mut self) -> io::Result<u64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(u64::from_le_bytes(buf))
    }

    async fn read_i64_be(&mut self) -> io::Result<i64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(i64::from_be_bytes(buf))
    }

    async fn read_i64_le(&mut self) -> io::Result<i64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(i64::from_le_bytes(buf))
    }

    async fn read_u128_be(&mut self) -> io::Result<u128> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf).await?;
        Ok(u128::from_be_bytes(buf))
    }

    async fn read_u128_le(&mut self) -> io::Result<u128> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf).await?;
        Ok(u128::from_le_bytes(buf))
    }

    async fn read_i128_be(&mut self) -> io::Result<i128> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf).await?;
        Ok(i128::from_be_bytes(buf))
    }

    async fn read_i128_le(&mut self) -> io::Result<i128> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf).await?;
        Ok(i128::from_le_bytes(buf))
    }

    async fn read_f32_be(&mut self) -> io::Result<f32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(f32::from_be_bytes(buf))
    }

    async fn read_f32_le(&mut self) -> io::Result<f32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf).await?;
        Ok(f32::from_le_bytes(buf))
    }

    async fn read_f64_be(&mut self) -> io::Result<f64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(f64::from_be_bytes(buf))
    }

    async fn read_f64_le(&mut self) -> io::Result<f64> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf).await?;
        Ok(f64::from_le_bytes(buf))
    }
}

#[async_trait]
pub trait AsyncWriteBytesExt: AsyncWriteExt + Unpin {
    async fn write_u8(&mut self, n: u8) -> io::Result<()> {
        self.write_all(&[n]).await
    }

    async fn write_i8(&mut self, n: i8) -> io::Result<()> {
        self.write_all(&[n as u8]).await
    }

    async fn write_u16_be(&mut self, n: u16) -> io::Result<()> {
        self.write_all(&n.to_be_bytes()).await
    }

    async fn write_u16_le(&mut self, n: u16) -> io::Result<()> {
        self.write_all(&n.to_le_bytes()).await
    }

    async fn write_i16_be(&mut self, n: i16) -> io::Result<()> {
        self.write_all(&n.to_be_bytes()).await
    }

    async fn write_i16_le(&mut self, n: i16) -> io::Result<()> {
        self.write_all(&n.to_le_bytes()).await
    }

    async fn write_u32_be(&mut self, n: u32) -> io::Result<()> {
        self.write_all(&n.to_be_bytes()).await
    }

    async fn write_u32_le(&mut self, n: u32) -> io::Result<()> {
        self.write_all(&n.to_le_bytes()).await
    }

    async fn write_i32_be(&mut self, n: i32) -> io::Result<()> {
        self.write_all(&n.to_be_bytes()).await
    }

    async fn write_i32_le(&mut self, n: i32) -> io::Result<()> {
        self.write_all(&n.to_le_bytes()).await
    }

    async fn write_u64_be(&mut self, n: u64) -> io::Result<()> {
        self.write_all(&n.to_be_bytes()).await
    }

    async fn write_u64_le(&mut self, n: u64) -> io::Result<()> {
        self.write_all(&n.to_le_bytes()).await
    }

    async fn write_i64_be(&mut self, n: i64) -> io::Result<()> {
        self.write_all(&n.to_be_bytes()).await
    }

    async fn write_i64_le(&mut self, n: i64) -> io::Result<()> {
        self.write_all(&n.to_le_bytes()).await
    }

    async fn write_u128_be(&mut self, n: u128) -> io::Result<()> {
        self.write_all(&n.to_be_bytes()).await
    }

    async fn write_u128_le(&mut self, n: u128) -> io::Result<()> {
        self.write_all(&n.to_le_bytes()).await
    }

    async fn write_i128_be(&mut self, n: i128) -> io::Result<()> {
        self.write_all(&n.to_be_bytes()).await
    }

    async fn write_i128_le(&mut self, n: i128) -> io::Result<()> {
        self.write_all(&n.to_le_bytes()).await
    }

    async fn write_f32_be(&mut self, n: f32) -> io::Result<()> {
        self.write_all(&n.to_be_bytes()).await
    }

    async fn write_f32_le(&mut self, n: f32) -> io::Result<()> {
        self.write_all(&n.to_le_bytes()).await
    }

    async fn write_f64_be(&mut self, n: f64) -> io::Result<()> {
        self.write_all(&n.to_be_bytes()).await
    }

    async fn write_f64_le(&mut self, n: f64) -> io::Result<()> {
        self.write_all(&n.to_le_bytes()).await
    }
}
