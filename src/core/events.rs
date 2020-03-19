#![allow(non_snake_case)]

pub mod Battle{
    use crate::core::event::EventKey;

    pub mod Ability{
        use crate::core::event::EventKey;
        use rlua::UserData;

        #[derive(Copy,Clone,PartialEq,Eq,Hash)]
        pub enum Lifetime{
            Start,
            End,
            Refresh
        }
        impl EventKey for Lifetime{}

        #[derive(Copy,Clone,PartialEq,Eq,Hash)]
        pub enum LocalLifetime{
            Start,
            End
        }
        impl EventKey for LocalLifetime{}

        #[derive(Copy,Clone,PartialEq,Eq,Hash)]
        pub enum Root{
            Transfer,
            Replace
        }
        impl EventKey for Root{}
        pub use Root::*;
    }

    #[derive(Copy,Clone,PartialEq,Eq,Hash)]
    pub enum Combat{
        MoveUsed,
        CheckAccuracy,
        CalculateDamage,
        CalculateSpecials,
        CalculateTypeEffectiveness,
        MoveHits,
        MoveMisses,
        MoveFaintsPokemon
    }
    impl EventKey for Combat{}

    #[derive(Copy,Clone,PartialEq,Eq,Hash)]
    pub enum Move{
        MoveExecuted,
        StatusExecuted,
        ApplySecondaries,
        MoveType
    }
    impl EventKey for Move{}

    #[derive(Copy,Clone,PartialEq,Eq,Hash)]
    pub enum StatusCombat{
        StatusMoveUsed,
        CheckStatusAccuracy,
        StatusMoveEffect,
        StatusMoveFailed
    }
    impl EventKey for StatusCombat{}
}