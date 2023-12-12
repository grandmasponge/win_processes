use std::mem::{size_of, MaybeUninit};
use std::str;

use windows::Win32::Foundation::{CloseHandle, GetLastError, BOOL, FALSE, HMODULE};
use windows::Win32::System::ProcessStatus::{
    EnumProcessModules, EnumProcesses, GetModuleBaseNameA,
};
use windows::Win32::System::Threading::{
    OpenProcess, PROCESS_ALL_ACCESS, PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION,
    PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
};

fn main() -> Result<(), ()> {
    let mut processes = Vec::<u32>::with_capacity(1024);
    let cb: u32 = 1024;
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

    unsafe {
        processes.set_len(lpcbneeded as usize);
    }

    let num_of_processes = lpcbneeded / 4;

    for i in 100..num_of_processes as usize {
        let proccess_id = processes[i];

        if proccess_id == 0 {
            continue;
        }

        let process_handle = unsafe {
            OpenProcess(
                PROCESS_ALL_ACCESS | PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ,
                FALSE,
                proccess_id,
            )
        };
        let handle = match process_handle {
            Ok(handle) => handle,
            Err(err) => {
                println!("{:?}", err);
                return Err(());
            }
        };

        let mut hmodule = MaybeUninit::<HMODULE>::uninit();
        let mut size = 0;

        let res = unsafe {
            EnumProcessModules(
                handle,
                hmodule.as_mut_ptr(),
                size_of::<HMODULE>() as u32,
                &mut size,
            )
        };

        match res {
            Ok(()) => {}
            Err(err) => {
                println!("{err}");
                return Err(());
            }
        }

        let mut process_name: [u8; 1024] = [0; 1024];

        let hmodule = unsafe { hmodule.assume_init() };

        let _processinfo = unsafe { GetModuleBaseNameA(handle, hmodule, &mut process_name) };

        let process_name = str::from_utf8(&process_name).unwrap();

        println!("Process: {process_name} {proccess_id}");

        unsafe {
            match CloseHandle(handle) {
                Ok(()) => {}
                Err(err) => {
                    println!("failed to close handle");
                    println!("{:?}", err);
                    return Err(());
                }
            }
        }
    }
    Ok(())
}
