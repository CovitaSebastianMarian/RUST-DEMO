

use nalgebra_glm as glm;

#[derive(Clone, Copy, Debug)]
pub struct Clip {
    pub position: glm::Vec2,
    pub size: glm::Vec2,
}
impl Clip {
    pub fn new() -> Self {
        Self {
            position: glm::Vec2::zeros(),
            size: glm::Vec2::zeros(),
        }
    }
    pub fn from(pos: glm::Vec2, size: glm::Vec2) -> Self {
        Self {
            position: pos,
            size: size,
        }
    }
    pub fn intersect(&self, other: &Clip) -> Clip {
        let x1 = self.position.x.max(other.position.x);
        let y1 = self.position.y.max(other.position.y);
        let x2 = (self.position.x + self.size.x).min(other.position.x + other.size.x);
        let y2 = (self.position.y + self.size.y).min(other.position.y + other.size.y);

        Clip {
            position: glm::vec2(x1, y1),
            size: glm::vec2((x2 - x1).max(0.0), (y2 - y1).max(0.0)),
        }
    }
}

