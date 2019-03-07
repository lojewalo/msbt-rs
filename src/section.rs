pub mod ato1;
pub mod atr1;
pub mod lbl1;
pub mod nli1;
pub mod tsy1;
pub mod txt2;

pub use self::{
  ato1::Ato1,
  atr1::Atr1,
  lbl1::Lbl1,
  nli1::Nli1,
  tsy1::Tsy1,
  txt2::Txt2,
};

#[derive(Debug)]
pub struct Section {
  pub magic: [u8; 4],
  pub size: u32,
  pub padding: [u8; 8],
}
