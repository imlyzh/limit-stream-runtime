use crate::{Ser, Deser};



#[derive(Default)]
pub struct Send<T: Ser, N> ([T; 0], [N; 0]);

impl<T: Ser, N> Send<T, N> {
  pub async fn send(&self, i: &T) -> Result<N, Self> {
    todo!()
  }
}

#[derive(Default)]
pub struct Recv<T: Deser, N> ([T; 0], [N; 0]);

impl<T: Deser, N> Recv<T, N> {
  pub async fn recv(&self) -> Result<(T, N), Self> {
    todo!()
  }
}

// pub struct Offer<T: Deser> {
//   _:
// }