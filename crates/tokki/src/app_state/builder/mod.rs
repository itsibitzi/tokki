mod follower;
mod leader;

pub use follower::FollowerBuilder;
pub use leader::LeaderBuilder;

pub struct AppStateBuilder {}

impl AppStateBuilder {
    pub fn leader(self) -> LeaderBuilder<Unset, Unset> {
        LeaderBuilder::default()
    }

    pub fn follower(self) -> FollowerBuilder<Unset, Unset, Unset, Unset> {
        FollowerBuilder::default()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Set;

#[derive(Debug, Clone, Copy, Default)]
pub struct Unset;
