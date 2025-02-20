use async_trait::async_trait;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

pub trait Event: Send + Sync {
    fn get_name_static() -> &'static str
    where
        Self: Sized;

    fn get_name(&self) -> &'static str;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_any(&self) -> &dyn Any;
}

#[async_trait]
pub trait EventHandler<E: Event>: Send + Sync {
    async fn handle(&self, _event: &E) {
        unimplemented!();
    }
}

#[async_trait]
pub trait AnyEventHandler: Send + Sync {
    async fn handle_any(&self, _event: &(dyn Event)) {
        unimplemented!();
    }
}

struct PtWarEventHandler<E, H>
where
    E: Event + Send + Sync + 'static,
    H: EventHandler<E> + Send + Sync,
{
    handler: Arc<H>,
    _phantom: std::marker::PhantomData<E>,
}

#[async_trait]
impl<E, H> AnyEventHandler for PtWarEventHandler<E, H>
where
    E: Event + Send + Sync + 'static,
    H: EventHandler<E> + Send + Sync,
{
    async fn handle_any(&self, event: &(dyn Event)) {
        if let Some(event) = event.as_any().downcast_ref::<E>() {
            self.handler.handle(event).await;
        }
    }
}

type AnyEventHandlerMap = HashMap<TypeId, Arc<dyn AnyEventHandler>>;
