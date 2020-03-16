use std::path::{PathBuf, Path};
use crate::arg_parser::Opts;
use crate::evidence_acquirer::EvidenceAcquirer;
use std::io::Result;
use crate::process_runner::{RemoteConnection, run_remote_blocking_and_save};
use std::fs::File;
use crate::remote_computer::{RemoteComputer, Local, RemoteComputerConnector, LOCAL_CONNECTOR, WMI_CONNECTOR};

pub struct WmiEvidenceAcquirer {
    pub remote_computer: RemoteComputer,
    pub store_directory: PathBuf,
}

impl WmiEvidenceAcquirer {
    #[allow(dead_code)]
    pub fn new(remote_computer: RemoteComputer,
               store_directory: PathBuf,
    ) -> WmiEvidenceAcquirer {
        WmiEvidenceAcquirer {
            remote_computer,
            store_directory,
        }
    }

    pub fn from_opts(opts: &Opts) -> WmiEvidenceAcquirer {
        WmiEvidenceAcquirer {
            remote_computer: RemoteComputer {
                address: opts.computer.clone(),
                username: opts.user.clone(),
                password: opts.password.clone(),
            },
            store_directory: PathBuf::from(opts.store_directory.clone()),
        }
    }
}


#[cfg(windows)]
impl EvidenceAcquirer for WmiEvidenceAcquirer {
    fn remote_computer(&self) -> &RemoteComputer {
        &self.remote_computer
    }

    fn store_directory(&self) -> &PathBuf {
        &self.store_directory
    }

    fn remote_connector(&self) -> &dyn RemoteComputerConnector {
        &WMI_CONNECTOR
    }

    fn firewall_state_command(&self) -> Vec<&'static str> {
        vec![]
    }

    fn network_state_command(&self) -> Vec<&'static str> {
        vec![
            "nic",
            "get",
            "AdapterType,",
            "Name,",
            "Installed,",
            "MACAddress,",
            "PowerManagementSupported,",
            "Speed",
        ]
        // wmic /NODE:"192.168.126.142" /USER:"IEUser" /PASSWORD:"Passw0rd!" nic get AdapterType, Name, Installed, MACAddress, PowerManagementSupported, Speed
    }

    fn logged_users_command(&self) -> Vec<&'static str> {
        vec![
            "COMPUTERSYSTEM",
            "GET",
            "USERNAME"
        ]
    }

    fn running_processes_command(&self) -> Vec<&'static str> {
        vec![]
    }

    fn active_network_connections_command(&self) -> Vec<&'static str> {
        vec![]
    }

    fn system_event_logs_command(&self) -> Vec<&'static str> {
        vec![]
    }

    fn application_event_logs_command(&self) -> Vec<&'static str> {
        vec![]
    }
}
