#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    use std::ffi::OsString;
    use windows_service::{
        service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType},
        service_manager::{ServiceManager, ServiceManagerAccess},
    };

    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::ENUMERATE_SERVICE;
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let services =  service_manager.enum_service(ServiceAccess::QUERY_CONFIG | ServiceAccess::QUERY_STATUS)?;
    println!("count: {}", services.len());
    for service in services {
        let config = match service.query_config() {
            Ok(val) => val,
            Err(_) => {continue}
        };
        let status = match service.query_status() {
            Ok(val) => val,
            Err(_) => {continue}
        };
        println!("name: {}, display name: {:?}, service type: {:?},  pid: {}",
                 service.get_name().unwrap_or_default(),
                 config.display_name,
                 config.service_type,
                 status.process_id.unwrap_or_default());
    }
    println!("over");
    Ok(())
}

#[cfg(not(windows))]
fn main() {
    panic!("This program is only intended to run on Windows.");
}
