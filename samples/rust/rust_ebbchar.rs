#![no_std]

use kernel::prelude::*;
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};
use kernel::prelude::*;
use kernel::sync::{smutex::Mutex, Ref, RefBorrow};
use kernel::{file, miscdev};

module! {
    type: Ebbchar,
    name: b"ebbchar",
    description: b"A simple ebbchar example",
    license: b"GPL",
}

struct Ebbchar {
    _dev : Pin<Box<miscdev::Registration<Ebbchar>>>,
}

struct Device {
    number: usize,
    contents: Mutex<Vec<u8>>,
}

#[vtable]
impl file::Operations for Ebbchar {
    type OpenData = Ref<Device>;
    type Data = Ref<Device>;

    fn open(context: &Ref<Device>, file: &file::File) -> Result<Ref<Device>> {
        pr_info!("File for device {} was opened\n", context.number);
        if file.flags() & file::flags::O_ACCMODE == file::flags::O_WRONLY {
            context.contents.lock().clear();
        }
        Ok(context.clone())
    }

    fn read(
        data: RefBorrow<'_, Device>,
        _file: &file::File,
        writer: &mut impl IoBufferWriter,
        offset: u64,
    ) -> Result<usize> {
        pr_info!("File for device {} was read\n", data.number);
        let offset = offset.try_into()?;
        let vec = data.contents.lock();
        let len = core::cmp::min(writer.len(), vec.len().saturating_sub(offset));
        writer.write_slice(&vec[offset..][..len])?;
        Ok(len)
    }

    fn write(
        data: RefBorrow<'_, Device>,
        _file: &file::File,
        reader: &mut impl IoBufferReader,
        offset: u64,
    ) -> Result<usize> {
        pr_info!("File for device {} was written\n", data.number);
        let offset = offset.try_into()?;
        let len = reader.len();
        let new_len = len.checked_add(offset).ok_or(EINVAL)?;
        let mut vec = data.contents.lock();
        if new_len > vec.len() {
            vec.try_resize(new_len, 0)?;
        }
        reader.read_slice(&mut vec[offset..][..len])?;
        Ok(len)
    }
}

impl kernel::Module for Ebbchar {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("--------------------------------------\n");
        pr_info!("--------------------------------------\n");
        pr_info!("--------------------------------------\n");
        pr_info!("Ebbchar initialized!\n");
        pr_info!("--------------------------------------\n");
        pr_info!("--------------------------------------\n");
        pr_info!("--------------------------------------\n");
        let dev = Ref::try_new(Device {
            number: 0,
            contents: Mutex::new(Vec::new()),
        })?;
        let reg = miscdev::Registration::new_pinned(fmt!("ebbchar"), dev)?;
        Ok(Self { _dev: reg })
    }
}
