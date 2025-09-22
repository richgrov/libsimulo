use shipyard::Component;

pub(crate) type PoseData = [f32; 17 * 2];

/// All entities with this components are created at the top level and managed by the engine.
#[derive(Clone, Component)]
#[track(Insertion)]
pub struct Pose(pub(crate) PoseData);

impl Pose {
    pub const NOSE: usize = 0;
    pub const LEFT_EYE: usize = 1;
    pub const RIGHT_EYE: usize = 2;
    pub const LEFT_EAR: usize = 3;
    pub const RIGHT_EAR: usize = 4;
    pub const LEFT_SHOULDER: usize = 5;
    pub const RIGHT_SHOULDER: usize = 6;
    pub const LEFT_ELBOW: usize = 7;
    pub const RIGHT_ELBOW: usize = 8;
    pub const LEFT_WRIST: usize = 9;
    pub const RIGHT_WRIST: usize = 10;
    pub const LEFT_HIP: usize = 11;
    pub const RIGHT_HIP: usize = 12;
    pub const LEFT_KNEE: usize = 13;
    pub const RIGHT_KNEE: usize = 14;
    pub const LEFT_ANKLE: usize = 15;
    pub const RIGHT_ANKLE: usize = 16;

    pub fn nose(&self) -> glam::Vec2 {
        self.keypoint(Self::NOSE)
    }

    pub fn left_eye(&self) -> glam::Vec2 {
        self.keypoint(Self::LEFT_EYE)
    }

    pub fn right_eye(&self) -> glam::Vec2 {
        self.keypoint(Self::RIGHT_EYE)
    }

    pub fn left_ear(&self) -> glam::Vec2 {
        self.keypoint(Self::LEFT_EAR)
    }

    pub fn right_ear(&self) -> glam::Vec2 {
        self.keypoint(Self::RIGHT_EAR)
    }

    pub fn left_shoulder(&self) -> glam::Vec2 {
        self.keypoint(Self::LEFT_SHOULDER)
    }

    pub fn right_shoulder(&self) -> glam::Vec2 {
        self.keypoint(Self::RIGHT_SHOULDER)
    }

    pub fn left_elbow(&self) -> glam::Vec2 {
        self.keypoint(Self::LEFT_ELBOW)
    }

    pub fn right_elbow(&self) -> glam::Vec2 {
        self.keypoint(Self::RIGHT_ELBOW)
    }

    pub fn left_wrist(&self) -> glam::Vec2 {
        self.keypoint(Self::LEFT_WRIST)
    }

    pub fn right_wrist(&self) -> glam::Vec2 {
        self.keypoint(Self::RIGHT_WRIST)
    }

    pub fn left_hip(&self) -> glam::Vec2 {
        self.keypoint(Self::LEFT_HIP)
    }

    pub fn right_hip(&self) -> glam::Vec2 {
        self.keypoint(Self::RIGHT_HIP)
    }

    pub fn left_knee(&self) -> glam::Vec2 {
        self.keypoint(Self::LEFT_KNEE)
    }

    pub fn right_knee(&self) -> glam::Vec2 {
        self.keypoint(Self::RIGHT_KNEE)
    }

    pub fn left_ankle(&self) -> glam::Vec2 {
        self.keypoint(Self::LEFT_ANKLE)
    }

    pub fn right_ankle(&self) -> glam::Vec2 {
        self.keypoint(Self::RIGHT_ANKLE)
    }

    pub fn keypoint(&self, index: usize) -> glam::Vec2 {
        glam::Vec2::new(self.0[index * 2], self.0[index * 2 + 1])
    }
}
