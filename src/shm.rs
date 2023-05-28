use crate::error::last_os_error;
use std::io::Error;

pub trait Shm {
    fn shmget(region: String, flags: i32, mode: u32, size: i64) -> Result<i32, std::io::Error>;
}

pub struct SharedMemory {
    id: i32,
}

impl SharedMemory {
    fn new() -> Self {
        Self { id: 1 }
    }
}

impl Shm for SharedMemory {
    fn shmget(region: String, mut flags: i32, mode: u32, size: i64) -> Result<i32, std::io::Error> {
        let id = unsafe {
            let name = std::ffi::CString::new(region)?;
            flags = flags | libc::O_RDWR;
            libc::shm_open(name.as_ptr(), flags, mode)
        };

        if id == -1 {
            last_os_error(&Error::last_os_error());
            return Ok(-1);
        }

        unsafe {
            let res = libc::ftruncate(id, size);
            if res == -1 {
                last_os_error(&Error::last_os_error());
                return Ok(-1);
            }
        };

        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::{SharedMemory, Shm};

    #[test]
    fn shmopen() {
        let mut shm = SharedMemory::new();
        let id = SharedMemory::shmget(
            String::from("foo"),
            libc::O_CREAT,
            (libc::S_IRUSR | libc::S_IWUSR) as u32,
            1024,
        )
        .unwrap();
        println!("id: {id}");
    }
}
