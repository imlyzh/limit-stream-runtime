use std::future::Future;

use crate::{Deser, Ser};

pub trait Adapter {
    fn send(&self) -> Box<dyn Future<Output = ()>> {
        todo!()
    }
    fn recv(&self) -> Box<dyn Future<Output = ()>> {
        todo!()
    }
}

pub trait FromAdapter {
    fn from_adapter(adapt: Box<dyn Adapter>) -> Self;
}

pub trait GetNext<N> {
    fn next(self) -> N;
}

pub struct Send<T: Ser, N: FromAdapter>(pub Box<dyn Adapter>, [T; 0], [N; 0]);

impl<T: Ser, N: FromAdapter> FromAdapter for Send<T, N> {
    fn from_adapter(adapt: Box<dyn Adapter>) -> Self {
        Send(adapt, Default::default(), Default::default())
    }
}

impl<T: Ser, N: FromAdapter> GetNext<N> for Send<T, N> {
    fn next(self) -> N {
        N::from_adapter(self.0)
    }
}

impl<T: Ser, N: FromAdapter> Send<T, N> {
    pub async fn send(&self, _i: &T) -> Result<N, Self> {
        todo!()
    }
}

pub struct Recv<T: Deser, N: FromAdapter>(pub Box<dyn Adapter>, [T; 0], [N; 0]);

impl<T: Deser, N: FromAdapter> FromAdapter for Recv<T, N> {
    fn from_adapter(adapt: Box<dyn Adapter>) -> Self {
        Recv(adapt, Default::default(), Default::default())
    }
}

impl<T: Deser, N: FromAdapter> GetNext<N> for Recv<T, N> {
    fn next(self) -> N {
        N::from_adapter(self.0)
    }
}

impl<T: Deser, N: FromAdapter> Recv<T, N> {
    pub async fn recv(&self) -> Result<(T, N), Self> {
        todo!()
    }
}

pub struct Endpoint(pub Box<dyn Adapter>);

impl FromAdapter for Endpoint {
    fn from_adapter(adapt: Box<dyn Adapter>) -> Self {
        Endpoint(adapt)
    }
}

// pub struct Offer<T: Deser> {
//   _:
// }
