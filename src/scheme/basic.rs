use std::str;

use crate::Scheme;

pub struct Basic;

#[derive(Debug, PartialEq)]
pub struct User {
    username: String,
    password: String,
}

impl Basic {
    pub fn new() -> Self {
        Basic
    }
}

impl Scheme for Basic {
    type User = User;

    fn scheme() -> &'static str {
        "Basic"
    }

    fn parse(value: &str) -> Option<User> {
        let mut parts = value.splitn(2, ' ');
        match parts.next() {
            Some(scheme) if scheme == Self::scheme() => (),
            _ => {
                return None;
            }
        }

        match parts.next() {
            Some(input) => match base64::decode(input) {
                Ok(data) => match str::from_utf8(&data) {
                    Err(_) => {
                        return None;
                    }
                    Ok(data) => {
                        let username: String;
                        let password: String;
                        let mut parts = data.splitn(2, ':');
                        match parts.next() {
                            Some(name) => {
                                username = name.to_string();
                            }
                            None => {
                                return None;
                            }
                        }

                        match parts.next() {
                            Some(word) => {
                                password = word.to_string();
                            }
                            None => {
                                return None;
                            }
                        }

                        return Some(User { username, password });
                    }
                },
                Err(_) => {
                    return None;
                }
            },
            None => {
                return None;
            }
        }
    }
}
