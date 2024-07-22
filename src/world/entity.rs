use crate::vec2d::Vec2d;
use std::fmt::Debug;

macro_rules! entities {
    ($($name: ident: $entity:ty),*) => {
        #[derive(Debug)]
        pub enum Entity {
            $(
            $name(Box<$entity>),
            )*
        }
        impl EntityTrait for Entity {
            fn update(&mut self) {
                match self {
                    $(
                    Entity::$name(entity) => entity.update(),
                    )*
                }
            }
            fn pos(&self) -> Vec2d {
                match self {
                    $(
                    Entity::$name(entity) => entity.pos(),
                    )*
                }
            }
            fn set_pos(&mut self, pos: Vec2d) {
                match self {
                    $(
                    Entity::$name(entity) => entity.set_pos(pos),
                    )*
                }
            }
        }
    };
}

entities!(Other: dyn EntityTrait);

pub trait EntityTrait: Debug {
    fn update(&mut self);
    fn pos(&self) -> Vec2d;
    fn set_pos(&mut self, pos: Vec2d);
}
