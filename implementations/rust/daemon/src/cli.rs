use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use ockam_message::message::{Route, RouterAddress};

use structopt::{clap::ArgSettings::Hidden, StructOpt};
use url::Url;

/// The port on which the config updater runs and accepts Config messages.
pub const DEFAULT_CONFIG_PORT: u16 = 11199;

pub const FILENAME_KEY_SUFFIX: &str = ".key";
pub const FILENAME_KEY_DEFAULT: &str = "1.key";

const DEFAULT_LOCAL_SOCKET: &str = "127.0.0.1:0";

/// Command-line arguments passed to `ockamd`.
#[allow(dead_code)]
#[derive(StructOpt)]
#[structopt(
    author = "Ockam Developers (ockam.io)",
    about = "Encrypt, route, and decrypt messages using the Ockam daemon."
)]
pub struct Args {
    /// Defines the kind of input from which a message should be read.
    #[structopt(
        long,
        default_value = "stdin",
        help = "Data source providing input to `ockamd`"
    )]
    input: InputKind,

    /// Defines the local socket address and port to bind a transport
    #[structopt(
        long,
        default_value = DEFAULT_LOCAL_SOCKET,
        help = "Local node address and port to bind"
    )]
    local_socket: SocketAddr,

    /// Defines the route where a message should be sent.
    #[structopt(
        long,
        default_value = "stdout",
        help = r#"Route to responder (sink), e.g. udp://host:port[,udp://host:port] (note comma-separation) or "stdout""#
    )]
    route_sink: OutputKind,

    /// Hub address and port to establish a listening channel.
    #[structopt(long, help = "Hub address and port to establish a listening channel")]
    route_hub: Option<SocketAddr>, // TODO: make this a Route so it can be multiple hops.

    /// Defines the kind of Ockam vault implementation to use.
    #[structopt(
        long,
        default_value = "FILESYSTEM",
        help = "Specify which type of Ockam vault to use for this instance of `ockamd`"
    )]
    vault: VaultKind,

    /// Path on disk where the vault data is stored (used with the FILESYSTEM vault).
    #[structopt(
        parse(from_os_str),
        long,
        default_value = "ockamd_vault",
        required_if("vault", "FILESYSTEM"),
        help = "Filepath on disk to pre-existing private keys to be used by the filesystem vault"
    )]
    vault_path: PathBuf,

    /// Start the `ockamd` process as the initiator or responder of a secure channel.
    #[structopt(
        long,
        default_value = "source",
        help = r#"Start `ockamd` as "source", "sink", or "router" of a secure channel"#
    )]
    role: ChannelRole,

    /// Define which private key to use as the initiator's identity.
    #[structopt(
        long,
        default_value = FILENAME_KEY_DEFAULT,
        help = "Name of the private key to use for the identity of the channel initiator"
    )]
    identity_name: String,

    /// Define the public key provided by the remote (sink) service.
    #[structopt(
        long,
        required_if("role", "source"),
        help = "The public key provided by the remote (sink) service"
    )]
    public_key_sink: Option<String>,

    /// Define the public key provided by the hub service.
    #[structopt(long, help = "The public key provided by the hub service")]
    public_key_hub: Option<String>,

    /// Address used to reach the service on remote machine.
    #[structopt(
        long,
        required_if("role", "source"),
        help = "Address used to reach the service on remote machine"
    )]
    service_address: Option<String>,

    /// Pre-defined configuration for an official Ockam Add-on, e.g. "influxdb,database_name,http://localhost:8086"
    #[structopt(
        long,
        help = r#"Pre-defined configuration for an official Ockam Add-on, e.g. "influxdb,database_name,http://localhost:8086""#
    )]
    addon: Option<Addon>,

    // TODO: expose `control` and `control_port` once runtime configuration is needed.
    #[structopt(
        short,
        long,
        help = "Execute `ockamd` in control-mode, otherwise will start as a long-running process",
        set = Hidden,
    )]
    control: bool,
    #[structopt(
        short = "p",
        long = "port",
        default_value = "11199",
        help = "Port for runtime configuration updates",
        set = Hidden,
    )]
    control_port: u16,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            control: false,
            control_port: DEFAULT_CONFIG_PORT,
            input: InputKind::Stdin,
            route_sink: OutputKind::Stdout,
            route_hub: Some(SocketAddr::from_str(DEFAULT_LOCAL_SOCKET).expect("bad socket addr")),
            local_socket: SocketAddr::from_str(DEFAULT_LOCAL_SOCKET).expect("bad socket addr"),
            vault: VaultKind::Filesystem,
            vault_path: PathBuf::from("ockamd_vault"),
            role: ChannelRole::Sink,
            service_address: None,
            identity_name: format!("1{}", FILENAME_KEY_SUFFIX),
            public_key_sink: None,
            public_key_hub: Some("default_key_vaule".into()),
            addon: None,
        }
    }
}

impl Args {
    /// Parse the command line options into the Args struct.
    pub fn parse() -> Args {
        // validate provided arguments & override possibly fallible options
        // TODO: what should be disallowed that the CLI validation wont handle?
        Args::from_args()
    }

    /// Checks which mode the executable was run in: Control or Server.
    pub fn exec_mode(&self) -> Mode {
        match self.control {
            true => Mode::Control,
            false => Mode::Server,
        }
    }

    pub fn role(&self) -> ChannelRole {
        self.role
    }

    pub fn output_kind(&self) -> OutputKind {
        self.route_sink.clone()
    }

    pub fn route_hub(&self) -> Option<SocketAddr> {
        self.route_hub.clone()
    }

    pub fn input_kind(&self) -> InputKind {
        self.input.clone()
    }

    pub fn local_socket(&self) -> SocketAddr {
        self.local_socket
    }

    // pub fn channel_to_sink(&self) -> Option<String> {
    //     self.channel_to_sink.clone()
    // }

    // pub fn router_socket(&self) -> Option<SocketAddr> {
    //     self.router_socket
    // }

    pub fn vault_path(&self) -> PathBuf {
        self.vault_path.clone()
    }

    pub fn public_key_sink(&self) -> Option<String> {
        self.public_key_sink.clone()
    }

    pub fn public_key_hub(&self) -> Option<String> {
        self.public_key_hub.clone()
    }

    pub fn service_address(&self) -> Option<String> {
        self.service_address.clone()
    }

    pub fn identity_name(&self) -> String {
        self.identity_name.clone()
    }

    pub fn addon(&self) -> Option<Addon> {
        self.addon.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Addon {
    InfluxDb(Url, String),
}

impl FromStr for Addon {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(',').collect::<Vec<&str>>().as_slice() {
            ["influxdb", dbname_url @ ..] => {
                if dbname_url.len() != 2 {
                    return Err("bad configuration: influx addon needs db and url".into());
                }

                if let Ok(u) = Url::parse(dbname_url[1]) {
                    Ok(Addon::InfluxDb(u, dbname_url[0].into()))
                } else {
                    Err("expected valid URL".into())
                }
            }
            _ => Err(format!("unknown configuration: {}", s)),
        }
    }
}

/// Specifies the implementation of a Ockam vault to be used.
pub enum VaultKind {
    Filesystem,
}

impl FromStr for VaultKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FILESYSTEM" => Ok(VaultKind::Filesystem),
            _ => Err("currently, 'FILESYSTEM' is the only supported vault option".into()),
        }
    }
}

/// Specifies which end of the secure channel the instance of `ockamd` is prepared to run in.
#[derive(Clone, Copy, Debug, StructOpt)]
pub enum ChannelRole {
    /// The Source role expects a channel Sink address, a router address (optional),
    /// and a public key to use in order to communicate with the Sink-Responder end of the channel.
    Source,
    /// The Sink role will optionally initiate a channel to a router, and create a channel
    /// responder for the source to initiate to.
    /// The Sink will also print its static public key for the Source to use..
    Sink,
    /// The Router role will route messages along the path, and is capable of acting as a responder
    /// to establish channels for tunneling
    Router,
}

impl FromStr for ChannelRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "source" => Ok(ChannelRole::Source),
            "sink" => Ok(ChannelRole::Sink),
            "router" => Ok(ChannelRole::Router),
            _ => Err("role must be set to either 'source', 'sink', or 'router'".into()),
        }
    }
}

/// The mode in which `ockamd` is to be run.
#[derive(Clone, Copy, Debug, StructOpt)]
pub enum Mode {
    /// Used for controlling configuration options at runtime, requiring that a
    /// Server process of `ockamd` is running.
    Control,
    /// Used to create a long-running process, to be executed with a particular
    /// input, e.g. Stdin.
    Server,
}

/// Specifies where input to `ockamd` should be read.
#[derive(Clone)]
pub enum InputKind {
    Stdin,
}

impl FromStr for InputKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stdin" => Ok(InputKind::Stdin),
            _ => Err("currently, only 'stdin' is a supported input type".into()),
        }
    }
}

/// Specifies where ouput from `ockamd` should be written.
#[derive(Clone)]
pub enum OutputKind {
    Stdout,
    Channel(Route),
}

impl FromStr for OutputKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = Ok(OutputKind::Stdout);

        if s == "stdout" {
            return ret;
        }

        let mut route = Route { addresses: vec![] };

        s.split(',').for_each(|part| match Url::parse(part) {
            Ok(u) => {
                if !u.has_host() {
                    ret = Err(format!("invalid URI: {}", part));
                }

                match u.scheme() {
                    "udp" => {
                        let addr = u.as_str().trim().trim_start_matches("udp://");
                        if let Ok(router_addr) = RouterAddress::udp_router_address_from_str(addr) {
                            route.addresses.push(router_addr);
                        }
                    }
                    "tcp" => {
                        let addr = u.as_str().trim().trim_start_matches("tcp://");
                        if let Ok(router_addr) = RouterAddress::tcp_router_address_from_str(addr) {
                            route.addresses.push(router_addr);
                        }
                    }
                    _ => ret = Err(format!("unsupported URL scheme for: {}", u.as_str())),
                }
            }
            Err(e) => {
                if let Ok(chan_addr) = RouterAddress::channel_router_address_from_str(part) {
                    route.addresses.push(chan_addr);
                } else {
                    ret = Err(format!("failed to parse route part '{}': {:?}", part, e));
                }
            }
        });

        if !route.addresses.is_empty() && ret.is_ok() {
            ret = Ok(OutputKind::Channel(route))
        }

        ret
    }
}

#[test]
fn test_cli_args_output() {
    use ockam_message::message::AddressType;

    if let Ok(output_kind) = OutputKind::from_str("udp://127.0.0.1:12345".into()) {
        match output_kind {
            OutputKind::Channel(route) => {
                assert_eq!(route.addresses.len(), 1);
            }
            _ => {}
        }
    }

    // UDP-only route test cases
    [
        // route
        "udp://10.10.1.3:9999,udp://192.168.33.4:4444,udp://10.2.22.2:22222",
        // number of hops in route
        "3",
        // etc..
        "udp://16.31.56.22, udp://ockam.network, udp://14.172.71.124, udp://44.178.238.169",
        "4",
    ]
    .windows(2)
    .for_each(|route_hop| {
        if let Ok(output_kind) = OutputKind::from_str(route_hop[0]) {
            match output_kind {
                OutputKind::Channel(route) => {
                    assert_eq!(route.addresses.len(), route_hop[1].parse().unwrap());
                    route.addresses.iter().for_each(|addr| {
                        assert_eq!(addr.a_type, AddressType::Udp);
                    })
                }
                _ => {}
            }
        }
    });

    // UDP and Channel address test cases
    let route =
        match OutputKind::from_str("udp://127.0.0.1:54201,udp://1.1.1.1:8000,87c4dd31,cd5d1fe9")
            .unwrap()
        {
            OutputKind::Channel(r) => r,
            _ => panic!("bad output kind, expected channel"),
        };
    for (i, addr) in route.addresses.iter().enumerate() {
        match i {
            0 | 1 => {
                assert!(matches!(
                    addr.a_type,
                    ockam_message::message::AddressType::Udp
                ));
            }
            2 | 3 => {
                assert!(matches!(
                    addr.a_type,
                    ockam_message::message::AddressType::Channel
                ));
            }
            _ => unimplemented!("a test for this value is not included"),
        }
    }

    // TCP-only route test cases
    [
        "tcp://127.0.0.1:12345,tcp://10.1.20.34:11111",
        "2",
        "tcp://99.234.21.34:8808, tcp://0.0.0.0:2341, tcp://1.1.1.1:3033",
        "3",
    ]
    .windows(2)
    .for_each(|route_hop| {
        if let Ok(output_kind) = OutputKind::from_str(route_hop[0]) {
            match output_kind {
                OutputKind::Channel(route) => {
                    assert_eq!(route.addresses.len(), route_hop[1].parse().unwrap());
                    route.addresses.iter().for_each(|addr| {
                        assert_eq!(addr.a_type, AddressType::Tcp);
                    })
                }
                _ => {}
            }
        }
    });
}
