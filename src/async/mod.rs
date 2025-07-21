// src/async_runtime.rs
pub trait AsyncRuntime {
    type JoinHandle<T>: Future<Output = Result<T, Self::JoinError>> + Send + 'static
    where
        T: Send + 'static;
    
    type JoinError: std::error::Error + Send + Sync + 'static;
    
    fn spawn_blocking<F, R>(f: F) -> Self::JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static;
}

// Tokio implementation
#[cfg(feature = "tokio")]
pub struct TokioRuntime;

#[cfg(feature = "tokio")]
impl AsyncRuntime for TokioRuntime {
    type JoinHandle<T> = impl Future<Output = Result<T, tokio::task::JoinError>>;
    type JoinError = tokio::task::JoinError;
    
    fn spawn_blocking<F, R>(f: F) -> Self::JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        async move {
            tokio::task::spawn_blocking(f).await
        }
    }
}

// Monoio + Rayon implementation
#[cfg(feature = "monoio")]
pub struct MonoioRuntime {
    thread_pool: rayon::ThreadPool,
}

#[cfg(feature = "monoio")]
impl AsyncRuntime for MonoioRuntime {
    type JoinHandle<T> = impl Future<Output = Result<T, MonoioJoinError>>;
    type JoinError = MonoioJoinError;
    
    fn spawn_blocking<F, R>(f: F) -> Self::JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        async move {
            let (tx, rx) = monoio::sync::oneshot::channel();
            
            self.thread_pool.spawn(move || {
                let result = f();
                let _ = tx.send(result);
            });
            
            rx.await.map_err(|_| MonoioJoinError)
        }
    }
}

// Your async database becomes generic
pub struct AsyncDatabase<R: AsyncRuntime = DefaultRuntime> {
    inner: SyncDatabase,
    _runtime: PhantomData<R>,
}

impl<R: AsyncRuntime> AsyncDatabase<R> {
    pub async fn get(&self, key: &[u8]) -> Result<Option<MyValue>, Error> {
        let key = key.to_vec();
        let inner = self.inner.clone();
        
        R::spawn_blocking(move || {
            inner.get_and_process(&key)
        }).await.map_err(|_| Error::JoinError)?
    }
}