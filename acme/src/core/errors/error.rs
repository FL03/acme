/*
    Appellation: errors <primitives>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

pub type Error = scsys::BoxError;


pub enum Errors {
    Generic(Error)
}