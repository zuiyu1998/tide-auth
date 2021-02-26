use async_trait::async_trait;

use tide::{log::info, Middleware, Next, Request, Response, StatusCode};

mod scheme;

pub use scheme::Basic;

pub struct Auth<T>(T);

impl<T: Scheme> Auth<T> {
    pub fn into_inner(inner: T) -> Self {
        Auth(inner)
    }
}

#[async_trait]
pub trait Scheme: Send + Sync + 'static {
    type User: Send + Sync + 'static;

    fn scheme() -> &'static str;

    fn parse(value: &str) -> Option<Self::User>;

    fn head_name() -> &'static str {
        "Authorization"
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static, T: Scheme> Middleware<State> for Auth<T> {
    async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> tide::Result {
        if request.ext::<T>().is_some() {
            return Ok(next.run(request).await);
        }

        //获取对应headename的值
        let value = request.header(T::head_name());

        match value {
            None => {
                //如果值不存在，则返回401响应
                return Ok(Response::new(StatusCode::Unauthorized));
            }
            Some(value) => {
                //否则解析对应的值

                match T::parse(value.as_str()) {
                    Some(user) => {
                        info!("sueccess");
                        request.set_ext(user);
                        return Ok(next.run(request).await);
                    }
                    None => {
                        //是否需要进行错误处理
                        return Ok(Response::new(StatusCode::Forbidden));
                    }
                }
            }
        }
    }
}
