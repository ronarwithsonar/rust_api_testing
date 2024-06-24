mod public_world;
mod authenticated_world;

/// holds all cucumber steps and worlds
use cucumber::World;
use futures::FutureExt as _;
use public_world::public_user_world::PublicWorld;
use authenticated_world::authenticated_user_world::AuthenticatedWorld;

#[tokio::main]
async fn main() {
    PublicWorld::cucumber()
        .init_tracing()
        .run("features/publicUser.feature").await;

    AuthenticatedWorld::cucumber()
        .after(|_, _, _, _, _world| {
            async {_world
                    .unwrap()
                    .cleanup()
                }.boxed_local()
        })
        .run("features/authenticatedUser.feature").await;
}