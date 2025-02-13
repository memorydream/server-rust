use concat_idents::concat_idents;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use unm_engine::executor::Executor;

use crate::{
    engines::Engine,
    types::{Context, RetrievedSongInfo, Song, SongSearchInformation},
};

#[napi(js_name = "Executor")]
pub struct JsExecutor {
    executor: Executor,
}

#[napi]
impl JsExecutor {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[napi]
    pub async fn search(
        &self,
        engines: Vec<Engine>,
        song: Song,
        ctx: Context,
    ) -> Result<SongSearchInformation> {
        let engines = engines
            .into_iter()
            .map(|engine| engine.as_str())
            .collect::<Vec<&str>>();
        self.executor
            .search(&engines, &song.into(), &ctx.to_unm_context())
            .await
            .map(|v| v.into())
            .map_err(|e| Error::new(Status::GenericFailure, format!("Unable to search: {:?}", e)))
    }

    #[napi]
    pub async fn retrieve(
        &self,
        song: SongSearchInformation,
        ctx: Context,
    ) -> Result<RetrievedSongInfo> {
        self.executor
            .retrieve(&song.into(), &ctx.to_unm_context())
            .await
            .map(|v| v.into())
            .map_err(|e| {
                Error::new(
                    Status::GenericFailure,
                    format!("Unable to retrieve: {:?}", e),
                )
            })
    }
}

impl Default for JsExecutor {
    fn default() -> Self {
        Self {
            executor: construct_executor(),
        }
    }
}

fn construct_executor() -> Executor {
    let mut executor = Executor::new();

    macro_rules! push_engine {
        ($engine_name:ident: $engine_struct:ident) => {
            concat_idents!(engine_crate = unm_engine_, $engine_name {
                executor.register(engine_crate::ENGINE_ID, Arc::new(engine_crate::$engine_struct));
            })
        };
    }

    push_engine!(bilibili: BilibiliEngine);
    push_engine!(kugou: KugouEngine);
    push_engine!(migu: MiguEngine);
    push_engine!(pyncm: PyNCMEngine);
    push_engine!(ytdl: YtDlEngine);

    executor
}
