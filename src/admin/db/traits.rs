use crate::admin::structs_types::AdminResult;

#[async_trait::async_trait]
pub(crate) trait AsyncDbTrait: Send + Sized {
    fn new() -> AdminResult<Self>;
}
