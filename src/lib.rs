pub mod xml;

pub mod proto {
    pub mod items {
        include!(concat!(env!("OUT_DIR"), "/osmpbf.rs"));
    }
}

#[cfg(test)]
mod tests {}
