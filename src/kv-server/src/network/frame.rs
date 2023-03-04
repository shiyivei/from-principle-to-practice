use crate::{CommandRequest, CommandResponse, KvError};
use bytes::{Buf, BufMut, BytesMut};
use std::io::{Read, Write};

use flate2::{read::GzDecoder, write::GzEncoder, Compress, Compression};
use prost::Message;
use tokio::io::{AsyncBufRead, AsyncReadExt};
use tracing::debug;

pub const LEN_LEN: usize = 4;
const MAX_FRAME: usize = 2 * 1024 * 1024 * 1024;
const COMPRESSION_LIMIT: usize = 1436;
const COMPRESSION_BIT: usize = 1 << 31;

pub trait FrameCoder
where
    Self: Message + Sized + Default,
{
    fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), KvError> {
        // 获取信息长度
        let size = self.encoded_len();

        // 是否大于定义的frame长度
        if size > MAX_FRAME {
            return Err(KvError::FrameError);
        }

        // 把信息长度装入buf

        buf.put_u32(size as _);

        // 判断 是否超过压缩长度

        if size > COMPRESSION_LIMIT {
            // 创建一个新buf
            let mut buf1 = Vec::with_capacity(size);
            // 编码进buf

            self.encode(&mut buf1)?;

            // 拿走4字节长度，清空

            let payload = buf.split_off(LEN_LEN);
            buf.clear();

            let mut encoder = GzEncoder::new(payload.writer(), Compression::default());
            encoder.write_all(&buf1[..]);

            //   let payload = encoder.finish()?.into_inner();
            let payload = encoder.finish();
            //   debug!("Encode a frame:size {}({})", size, payload.len());

            //   buf.unsplit(payload);

            Ok(())
        } else {
            self.encode(buf)?;
            Ok(())
        }
    }

    fn decode_from(buf: &mut BytesMut) -> Result<Self, KvError> {
        // 先取 4 字节，从中拿出长度和 compression bit
        let header = buf.get_u32() as usize;
        let (len, compressed) = decode_header(header);
        debug!("Got a frame: msg len {}, compressed {}", len, compressed);
        if compressed {
            let mut decoder = GzDecoder::new(&buf[..len]);
            let mut buf1 = Vec::with_capacity(len * 2);
            decoder.read_to_end(&mut buf1);
            buf.advance(len);
            // decode 成相应的消息
            Ok(Self::decode(&buf1[..buf1.len()])?)
        } else {
            let msg = Self::decode(&buf[..len])?;
            buf.advance(len);
            Ok(msg)
        }
    }
}

impl FrameCoder for CommandRequest {}
impl FrameCoder for CommandResponse {}

fn decode_header(header: usize) -> (usize, bool) {
    let len = header & !COMPRESSION_BIT;
    let compressed = header & COMPRESSION_BIT == COMPRESSION_BIT;
    (len, compressed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;
    use bytes::Bytes;

    #[test]
    fn command_request_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let cmd = CommandRequest::new_hdel("t1", "k1");
        cmd.encode_frame(&mut buf).unwrap();

        // 最高位没设置
        assert_eq!(is_compressed(&buf), false);

        let cmd1 = CommandRequest::decode_frame(&mut buf).unwrap();
        assert_eq!(cmd, cmd1);
    }

    #[test]
    fn command_response_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let values: Vec<Value> = vec![1.into(), "hello".into(), b"data".into()];
        let res: CommandResponse = values.into();
        res.encode_frame(&mut buf).unwrap();

        // 最高位没设置
        assert_eq!(is_compressed(&buf), false);

        let res1 = CommandResponse::decode_frame(&mut buf).unwrap();
        assert_eq!(res, res1);
    }

    #[test]
    fn command_response_compressed_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let value: Value = Bytes::from(vec![0u8; COMPRESSION_LIMIT + 1]).into();
        let res: CommandResponse = value.into();
        res.encode_frame(&mut buf).unwrap();

        // 最高位设置了
        assert_eq!(is_compressed(&buf), true);

        let res1 = CommandResponse::decode_frame(&mut buf).unwrap();
        assert_eq!(res, res1);
    }

    fn is_compressed(data: &[u8]) -> bool {
        if let &[v] = &data[..1] {
            v >> 7 == 1
        } else {
            false
        }
    }
}
