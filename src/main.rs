use std::mem::size_of;

use windows::core::Error;
use windows::Win32::Foundation::{self, CloseHandle, GetLastError, BOOL};
use windows::Win32::System::ProcessStatus::{self, EnumProcesses};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION};

fn main() -> Result<(), ()> {
    let mut processes: [u32; 1024] = [0; 1024];
    let cb: u32 = size_of::<u32>() as u32;
    let mut lpcbneeded: u32 = 0;

    unsafe {
        let enumproc = EnumProcesses(processes.as_mut_ptr(), cb, &mut lpcbneeded).map_err(|x| {
            println!("{x}");
            println!("Error: {:?}", GetLastError());
        });

        match enumproc {
            Ok(()) => {
                println!("enumerating on proceses...")
            }
            Err(()) => {
                println!("failed to enumerate through processes");
                return Err(());
            }
        }
    }

    let num_of_processes = lpcbneeded / 4;

    for i in 0..num_of_processes as usize {
        let proccess_id = processes[i];

        let process_handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION, BOOL(0), 18260) };
        let handle = match process_handle {
            Ok(handle) => handle,
            Err(err) => {
                println!("error {:?}", err);
                return Err(());
            }
        };



        unsafe {
            match CloseHandle(handle) {
                Ok(()) => {
                    println!("closing handle");
                }
                Err(err) => {
                    println!("failed to close handle");
                    println!("Error : {:?}", err);
                    return Err(());
                }
            }
        }
    }

    Ok(())
}
