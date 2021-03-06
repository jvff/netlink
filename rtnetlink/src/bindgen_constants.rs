//! This module has been partially autogenerated with bindgen and is for internal use only. The
//! different modules in the crate re-export constants when needed.

use libc::c_int as int;

// pub const NLMSGERR_ATTR_UNUSED: int = 0;
// pub const NLMSGERR_ATTR_MSG: int = 1;
// pub const NLMSGERR_ATTR_OFFS: int = 2;
// pub const NLMSGERR_ATTR_COOKIE: int = 3;
// pub const NLMSGERR_ATTR_MAX: int = 3;
//
// pub const NL_MMAP_STATUS_UNUSED: int = 0;
// pub const NL_MMAP_STATUS_RESERVED: int = 1;
// pub const NL_MMAP_STATUS_VALID: int = 2;
// pub const NL_MMAP_STATUS_COPY: int = 3;
// pub const NL_MMAP_STATUS_SKIP: int = 4;
//
// pub const NETLINK_UNCONNECTED: int = 0;
// pub const NETLINK_CONNECTED: int = 1;

pub const IFLA_UNSPEC: int = 0;
pub const IFLA_ADDRESS: int = 1;
pub const IFLA_BROADCAST: int = 2;
pub const IFLA_IFNAME: int = 3;
pub const IFLA_MTU: int = 4;
pub const IFLA_LINK: int = 5;
pub const IFLA_QDISC: int = 6;
pub const IFLA_STATS: int = 7;
pub const IFLA_COST: int = 8;
pub const IFLA_PRIORITY: int = 9;
pub const IFLA_MASTER: int = 10;
pub const IFLA_WIRELESS: int = 11;
pub const IFLA_PROTINFO: int = 12;
pub const IFLA_TXQLEN: int = 13;
pub const IFLA_MAP: int = 14;
pub const IFLA_WEIGHT: int = 15;
pub const IFLA_OPERSTATE: int = 16;
pub const IFLA_LINKMODE: int = 17;
pub const IFLA_LINKINFO: int = 18;
pub const IFLA_NET_NS_PID: int = 19;
pub const IFLA_IFALIAS: int = 20;
pub const IFLA_NUM_VF: int = 21;
pub const IFLA_VFINFO_LIST: int = 22;
pub const IFLA_STATS64: int = 23;
pub const IFLA_VF_PORTS: int = 24;
pub const IFLA_PORT_SELF: int = 25;
pub const IFLA_AF_SPEC: int = 26;
pub const IFLA_GROUP: int = 27;
pub const IFLA_NET_NS_FD: int = 28;
pub const IFLA_EXT_MASK: int = 29;
pub const IFLA_PROMISCUITY: int = 30;
pub const IFLA_NUM_TX_QUEUES: int = 31;
pub const IFLA_NUM_RX_QUEUES: int = 32;
pub const IFLA_CARRIER: int = 33;
pub const IFLA_PHYS_PORT_ID: int = 34;
pub const IFLA_CARRIER_CHANGES: int = 35;
pub const IFLA_PHYS_SWITCH_ID: int = 36;
pub const IFLA_LINK_NETNSID: int = 37;
pub const IFLA_PHYS_PORT_NAME: int = 38;
pub const IFLA_PROTO_DOWN: int = 39;
pub const IFLA_GSO_MAX_SEGS: int = 40;
pub const IFLA_GSO_MAX_SIZE: int = 41;
pub const IFLA_PAD: int = 42;
pub const IFLA_XDP: int = 43;
pub const IFLA_EVENT: int = 44;
pub const IFLA_NEW_NETNSID: int = 45;
pub const IFLA_IF_NETNSID: int = 46;
pub const IFLA_CARRIER_UP_COUNT: int = 47;
pub const IFLA_CARRIER_DOWN_COUNT: int = 48;
pub const IFLA_NEW_IFINDEX: int = 49;

pub const IFLA_INET_UNSPEC: int = 0;
pub const IFLA_INET_CONF: int = 1;

pub const IFLA_INET6_UNSPEC: int = 0;
pub const IFLA_INET6_FLAGS: int = 1;
pub const IFLA_INET6_CONF: int = 2;
pub const IFLA_INET6_STATS: int = 3;
// pub const IFLA_INET6_MCAST: int = 4;
pub const IFLA_INET6_CACHEINFO: int = 5;
pub const IFLA_INET6_ICMP6STATS: int = 6;
pub const IFLA_INET6_TOKEN: int = 7;
pub const IFLA_INET6_ADDR_GEN_MODE: int = 8;

// pub const IN6_ADDR_GEN_MODE_EUI64: int = 0;
// pub const IN6_ADDR_GEN_MODE_NONE: int = 1;
// pub const IN6_ADDR_GEN_MODE_STABLE_PRIVACY: int = 2;
// pub const IN6_ADDR_GEN_MODE_RANDOM: int = 3;

pub const IFLA_BR_UNSPEC: int = 0;
pub const IFLA_BR_FORWARD_DELAY: int = 1;
pub const IFLA_BR_HELLO_TIME: int = 2;
pub const IFLA_BR_MAX_AGE: int = 3;
pub const IFLA_BR_AGEING_TIME: int = 4;
pub const IFLA_BR_STP_STATE: int = 5;
pub const IFLA_BR_PRIORITY: int = 6;
pub const IFLA_BR_VLAN_FILTERING: int = 7;
pub const IFLA_BR_VLAN_PROTOCOL: int = 8;
pub const IFLA_BR_GROUP_FWD_MASK: int = 9;
pub const IFLA_BR_ROOT_ID: int = 10;
pub const IFLA_BR_BRIDGE_ID: int = 11;
pub const IFLA_BR_ROOT_PORT: int = 12;
pub const IFLA_BR_ROOT_PATH_COST: int = 13;
pub const IFLA_BR_TOPOLOGY_CHANGE: int = 14;
pub const IFLA_BR_TOPOLOGY_CHANGE_DETECTED: int = 15;
pub const IFLA_BR_HELLO_TIMER: int = 16;
pub const IFLA_BR_TCN_TIMER: int = 17;
pub const IFLA_BR_TOPOLOGY_CHANGE_TIMER: int = 18;
pub const IFLA_BR_GC_TIMER: int = 19;
pub const IFLA_BR_GROUP_ADDR: int = 20;
pub const IFLA_BR_FDB_FLUSH: int = 21;
pub const IFLA_BR_MCAST_ROUTER: int = 22;
pub const IFLA_BR_MCAST_SNOOPING: int = 23;
pub const IFLA_BR_MCAST_QUERY_USE_IFADDR: int = 24;
pub const IFLA_BR_MCAST_QUERIER: int = 25;
pub const IFLA_BR_MCAST_HASH_ELASTICITY: int = 26;
pub const IFLA_BR_MCAST_HASH_MAX: int = 27;
pub const IFLA_BR_MCAST_LAST_MEMBER_CNT: int = 28;
pub const IFLA_BR_MCAST_STARTUP_QUERY_CNT: int = 29;
pub const IFLA_BR_MCAST_LAST_MEMBER_INTVL: int = 30;
pub const IFLA_BR_MCAST_MEMBERSHIP_INTVL: int = 31;
pub const IFLA_BR_MCAST_QUERIER_INTVL: int = 32;
pub const IFLA_BR_MCAST_QUERY_INTVL: int = 33;
pub const IFLA_BR_MCAST_QUERY_RESPONSE_INTVL: int = 34;
pub const IFLA_BR_MCAST_STARTUP_QUERY_INTVL: int = 35;
pub const IFLA_BR_NF_CALL_IPTABLES: int = 36;
pub const IFLA_BR_NF_CALL_IP6TABLES: int = 37;
pub const IFLA_BR_NF_CALL_ARPTABLES: int = 38;
pub const IFLA_BR_VLAN_DEFAULT_PVID: int = 39;
pub const IFLA_BR_PAD: int = 40;
pub const IFLA_BR_VLAN_STATS_ENABLED: int = 41;
pub const IFLA_BR_MCAST_STATS_ENABLED: int = 42;
pub const IFLA_BR_MCAST_IGMP_VERSION: int = 43;
pub const IFLA_BR_MCAST_MLD_VERSION: int = 44;
//
// pub const BRIDGE_MODE_UNSPEC: int = 0;
// pub const BRIDGE_MODE_HAIRPIN: int = 1;
//
// pub const IFLA_BRPORT_UNSPEC: int = 0;
// pub const IFLA_BRPORT_STATE: int = 1;
// pub const IFLA_BRPORT_PRIORITY: int = 2;
// pub const IFLA_BRPORT_COST: int = 3;
// pub const IFLA_BRPORT_MODE: int = 4;
// pub const IFLA_BRPORT_GUARD: int = 5;
// pub const IFLA_BRPORT_PROTECT: int = 6;
// pub const IFLA_BRPORT_FAST_LEAVE: int = 7;
// pub const IFLA_BRPORT_LEARNING: int = 8;
// pub const IFLA_BRPORT_UNICAST_FLOOD: int = 9;
// pub const IFLA_BRPORT_PROXYARP: int = 10;
// pub const IFLA_BRPORT_LEARNING_SYNC: int = 11;
// pub const IFLA_BRPORT_PROXYARP_WIFI: int = 12;
// pub const IFLA_BRPORT_ROOT_ID: int = 13;
// pub const IFLA_BRPORT_BRIDGE_ID: int = 14;
// pub const IFLA_BRPORT_DESIGNATED_PORT: int = 15;
// pub const IFLA_BRPORT_DESIGNATED_COST: int = 16;
// pub const IFLA_BRPORT_ID: int = 17;
// pub const IFLA_BRPORT_NO: int = 18;
// pub const IFLA_BRPORT_TOPOLOGY_CHANGE_ACK: int = 19;
// pub const IFLA_BRPORT_CONFIG_PENDING: int = 20;
// pub const IFLA_BRPORT_MESSAGE_AGE_TIMER: int = 21;
// pub const IFLA_BRPORT_FORWARD_DELAY_TIMER: int = 22;
// pub const IFLA_BRPORT_HOLD_TIMER: int = 23;
// pub const IFLA_BRPORT_FLUSH: int = 24;
// pub const IFLA_BRPORT_MULTICAST_ROUTER: int = 25;
// pub const IFLA_BRPORT_PAD: int = 26;
// pub const IFLA_BRPORT_MCAST_FLOOD: int = 27;
// pub const IFLA_BRPORT_MCAST_TO_UCAST: int = 28;
// pub const IFLA_BRPORT_VLAN_TUNNEL: int = 29;
// pub const IFLA_BRPORT_BCAST_FLOOD: int = 30;
// pub const IFLA_BRPORT_GROUP_FWD_MASK: int = 31;
// pub const IFLA_BRPORT_NEIGH_SUPPRESS: int = 32;

pub const IFLA_INFO_UNSPEC: int = 0;
pub const IFLA_INFO_KIND: int = 1;
pub const IFLA_INFO_DATA: int = 2;
pub const IFLA_INFO_XSTATS: int = 3;
pub const IFLA_INFO_SLAVE_KIND: int = 4;
pub const IFLA_INFO_SLAVE_DATA: int = 5;

pub const IFLA_VLAN_UNSPEC: int = 0;
pub const IFLA_VLAN_ID: int = 1;
pub const IFLA_VLAN_FLAGS: int = 2;
pub const IFLA_VLAN_EGRESS_QOS: int = 3;
pub const IFLA_VLAN_INGRESS_QOS: int = 4;
pub const IFLA_VLAN_PROTOCOL: int = 5;

// pub const IFLA_VLAN_QOS_UNSPEC: int = 0;
// pub const IFLA_VLAN_QOS_MAPPING: int = 1;
//
// pub const IFLA_MACVLAN_UNSPEC: int = 0;
// pub const IFLA_MACVLAN_MODE: int = 1;
// pub const IFLA_MACVLAN_FLAGS: int = 2;
// pub const IFLA_MACVLAN_MACADDR_MODE: int = 3;
// pub const IFLA_MACVLAN_MACADDR: int = 4;
// pub const IFLA_MACVLAN_MACADDR_DATA: int = 5;
// pub const IFLA_MACVLAN_MACADDR_COUNT: int = 6;
//
// pub const MACVLAN_MODE_PRIVATE: int = 1;
// pub const MACVLAN_MODE_VEPA: int = 2;
// pub const MACVLAN_MODE_BRIDGE: int = 4;
// pub const MACVLAN_MODE_PASSTHRU: int = 8;
// pub const MACVLAN_MODE_SOURCE: int = 16;
//
// pub const MACVLAN_MACADDR_ADD: int = 0;
// pub const MACVLAN_MACADDR_DEL: int = 1;
// pub const MACVLAN_MACADDR_FLUSH: int = 2;
// pub const MACVLAN_MACADDR_SET: int = 3;
//
// pub const IFLA_VRF_UNSPEC: int = 0;
// pub const IFLA_VRF_TABLE: int = 1;
//
// pub const IFLA_VRF_PORT_UNSPEC: int = 0;
// pub const IFLA_VRF_PORT_TABLE: int = 1;
//
// pub const IFLA_MACSEC_UNSPEC: int = 0;
// pub const IFLA_MACSEC_SCI: int = 1;
// pub const IFLA_MACSEC_PORT: int = 2;
// pub const IFLA_MACSEC_ICV_LEN: int = 3;
// pub const IFLA_MACSEC_CIPHER_SUITE: int = 4;
// pub const IFLA_MACSEC_WINDOW: int = 5;
// pub const IFLA_MACSEC_ENCODING_SA: int = 6;
// pub const IFLA_MACSEC_ENCRYPT: int = 7;
// pub const IFLA_MACSEC_PROTECT: int = 8;
// pub const IFLA_MACSEC_INC_SCI: int = 9;
// pub const IFLA_MACSEC_ES: int = 10;
// pub const IFLA_MACSEC_SCB: int = 11;
// pub const IFLA_MACSEC_REPLAY_PROTECT: int = 12;
// pub const IFLA_MACSEC_VALIDATION: int = 13;
// pub const IFLA_MACSEC_PAD: int = 14;
//
// pub const MACSEC_VALIDATE_DISABLED: int = 0;
// pub const MACSEC_VALIDATE_CHECK: int = 1;
// pub const MACSEC_VALIDATE_STRICT: int = 2;
// pub const MACSEC_VALIDATE_MAX: int = 2;
//
// pub const IFLA_IPVLAN_UNSPEC: int = 0;
// pub const IFLA_IPVLAN_MODE: int = 1;
// pub const IFLA_IPVLAN_FLAGS: int = 2;
//
// pub const IPVLAN_MODE_L2: int = 0;
// pub const IPVLAN_MODE_L3: int = 1;
// pub const IPVLAN_MODE_L3S: int = 2;
// pub const IPVLAN_MODE_MAX: int = 3;
//
// pub const IFLA_VXLAN_UNSPEC: int = 0;
// pub const IFLA_VXLAN_ID: int = 1;
// pub const IFLA_VXLAN_GROUP: int = 2;
// pub const IFLA_VXLAN_LINK: int = 3;
// pub const IFLA_VXLAN_LOCAL: int = 4;
// pub const IFLA_VXLAN_TTL: int = 5;
// pub const IFLA_VXLAN_TOS: int = 6;
// pub const IFLA_VXLAN_LEARNING: int = 7;
// pub const IFLA_VXLAN_AGEING: int = 8;
// pub const IFLA_VXLAN_LIMIT: int = 9;
// pub const IFLA_VXLAN_PORT_RANGE: int = 10;
// pub const IFLA_VXLAN_PROXY: int = 11;
// pub const IFLA_VXLAN_RSC: int = 12;
// pub const IFLA_VXLAN_L2MISS: int = 13;
// pub const IFLA_VXLAN_L3MISS: int = 14;
// pub const IFLA_VXLAN_PORT: int = 15;
// pub const IFLA_VXLAN_GROUP6: int = 16;
// pub const IFLA_VXLAN_LOCAL6: int = 17;
// pub const IFLA_VXLAN_UDP_CSUM: int = 18;
// pub const IFLA_VXLAN_UDP_ZERO_CSUM6_TX: int = 19;
// pub const IFLA_VXLAN_UDP_ZERO_CSUM6_RX: int = 20;
// pub const IFLA_VXLAN_REMCSUM_TX: int = 21;
// pub const IFLA_VXLAN_REMCSUM_RX: int = 22;
// pub const IFLA_VXLAN_GBP: int = 23;
// pub const IFLA_VXLAN_REMCSUM_NOPARTIAL: int = 24;
// pub const IFLA_VXLAN_COLLECT_METADATA: int = 25;
// pub const IFLA_VXLAN_LABEL: int = 26;
// pub const IFLA_VXLAN_GPE: int = 27;
//
// pub const IFLA_GENEVE_UNSPEC: int = 0;
// pub const IFLA_GENEVE_ID: int = 1;
// pub const IFLA_GENEVE_REMOTE: int = 2;
// pub const IFLA_GENEVE_TTL: int = 3;
// pub const IFLA_GENEVE_TOS: int = 4;
// pub const IFLA_GENEVE_PORT: int = 5;
// pub const IFLA_GENEVE_COLLECT_METADATA: int = 6;
// pub const IFLA_GENEVE_REMOTE6: int = 7;
// pub const IFLA_GENEVE_UDP_CSUM: int = 8;
// pub const IFLA_GENEVE_UDP_ZERO_CSUM6_TX: int = 9;
// pub const IFLA_GENEVE_UDP_ZERO_CSUM6_RX: int = 10;
// pub const IFLA_GENEVE_LABEL: int = 11;
//
// pub const IFLA_PPP_UNSPEC: int = 0;
// pub const IFLA_PPP_DEV_FD: int = 1;
//
// pub const GTP_ROLE_GGSN: int = 0;
// pub const GTP_ROLE_SGSN: int = 1;
//
// pub const IFLA_GTP_UNSPEC: int = 0;
// pub const IFLA_GTP_FD0: int = 1;
// pub const IFLA_GTP_FD1: int = 2;
// pub const IFLA_GTP_PDP_HASHSIZE: int = 3;
// pub const IFLA_GTP_ROLE: int = 4;
//
// pub const IFLA_BOND_UNSPEC: int = 0;
// pub const IFLA_BOND_MODE: int = 1;
// pub const IFLA_BOND_ACTIVE_SLAVE: int = 2;
// pub const IFLA_BOND_MIIMON: int = 3;
// pub const IFLA_BOND_UPDELAY: int = 4;
// pub const IFLA_BOND_DOWNDELAY: int = 5;
// pub const IFLA_BOND_USE_CARRIER: int = 6;
// pub const IFLA_BOND_ARP_INTERVAL: int = 7;
// pub const IFLA_BOND_ARP_IP_TARGET: int = 8;
// pub const IFLA_BOND_ARP_VALIDATE: int = 9;
// pub const IFLA_BOND_ARP_ALL_TARGETS: int = 10;
// pub const IFLA_BOND_PRIMARY: int = 11;
// pub const IFLA_BOND_PRIMARY_RESELECT: int = 12;
// pub const IFLA_BOND_FAIL_OVER_MAC: int = 13;
// pub const IFLA_BOND_XMIT_HASH_POLICY: int = 14;
// pub const IFLA_BOND_RESEND_IGMP: int = 15;
// pub const IFLA_BOND_NUM_PEER_NOTIF: int = 16;
// pub const IFLA_BOND_ALL_SLAVES_ACTIVE: int = 17;
// pub const IFLA_BOND_MIN_LINKS: int = 18;
// pub const IFLA_BOND_LP_INTERVAL: int = 19;
// pub const IFLA_BOND_PACKETS_PER_SLAVE: int = 20;
// pub const IFLA_BOND_AD_LACP_RATE: int = 21;
// pub const IFLA_BOND_AD_SELECT: int = 22;
// pub const IFLA_BOND_AD_INFO: int = 23;
// pub const IFLA_BOND_AD_ACTOR_SYS_PRIO: int = 24;
// pub const IFLA_BOND_AD_USER_PORT_KEY: int = 25;
// pub const IFLA_BOND_AD_ACTOR_SYSTEM: int = 26;
// pub const IFLA_BOND_TLB_DYNAMIC_LB: int = 27;
//
// pub const IFLA_BOND_AD_INFO_UNSPEC: int = 0;
// pub const IFLA_BOND_AD_INFO_AGGREGATOR: int = 1;
// pub const IFLA_BOND_AD_INFO_NUM_PORTS: int = 2;
// pub const IFLA_BOND_AD_INFO_ACTOR_KEY: int = 3;
// pub const IFLA_BOND_AD_INFO_PARTNER_KEY: int = 4;
// pub const IFLA_BOND_AD_INFO_PARTNER_MAC: int = 5;
//
// pub const IFLA_BOND_SLAVE_UNSPEC: int = 0;
// pub const IFLA_BOND_SLAVE_STATE: int = 1;
// pub const IFLA_BOND_SLAVE_MII_STATUS: int = 2;
// pub const IFLA_BOND_SLAVE_LINK_FAILURE_COUNT: int = 3;
// pub const IFLA_BOND_SLAVE_PERM_HWADDR: int = 4;
// pub const IFLA_BOND_SLAVE_QUEUE_ID: int = 5;
// pub const IFLA_BOND_SLAVE_AD_AGGREGATOR_ID: int = 6;
// pub const IFLA_BOND_SLAVE_AD_ACTOR_OPER_PORT_STATE: int = 7;
// pub const IFLA_BOND_SLAVE_AD_PARTNER_OPER_PORT_STATE: int = 8;
//
// pub const IFLA_VF_INFO_UNSPEC: int = 0;
// pub const IFLA_VF_INFO: int = 1;
//
// pub const IFLA_VF_UNSPEC: int = 0;
// pub const IFLA_VF_MAC: int = 1;
// pub const IFLA_VF_VLAN: int = 2;
// pub const IFLA_VF_TX_RATE: int = 3;
// pub const IFLA_VF_SPOOFCHK: int = 4;
// pub const IFLA_VF_LINK_STATE: int = 5;
// pub const IFLA_VF_RATE: int = 6;
// pub const IFLA_VF_RSS_QUERY_EN: int = 7;
// pub const IFLA_VF_STATS: int = 8;
// pub const IFLA_VF_TRUST: int = 9;
// pub const IFLA_VF_IB_NODE_GUID: int = 10;
// pub const IFLA_VF_IB_PORT_GUID: int = 11;
// pub const IFLA_VF_VLAN_LIST: int = 12;
//
// pub const IFLA_VF_VLAN_INFO_UNSPEC: int = 0;
// pub const IFLA_VF_VLAN_INFO: int = 1;
//
// pub const TCA_ROOT_UNSPEC: int = 0;
// pub const TCA_ROOT_TAB: int = 1;
// pub const TCA_ROOT_FLAGS: int = 2;
// pub const TCA_ROOT_COUNT: int = 3;
// pub const TCA_ROOT_TIME_DELTA: int = 4;
//
// pub const NDUSEROPT_UNSPEC: int = 0;
// pub const NDUSEROPT_SRCADDR: int = 1;
//
// pub const RTNLGRP_NONE: int = 0;
// pub const RTNLGRP_LINK: int = 1;
// pub const RTNLGRP_NOTIFY: int = 2;
// pub const RTNLGRP_NEIGH: int = 3;
// pub const RTNLGRP_TC: int = 4;
// pub const RTNLGRP_IPV4_IFADDR: int = 5;
// pub const RTNLGRP_IPV4_MROUTE: int = 6;
// pub const RTNLGRP_IPV4_ROUTE: int = 7;
// pub const RTNLGRP_IPV4_RULE: int = 8;
// pub const RTNLGRP_IPV6_IFADDR: int = 9;
// pub const RTNLGRP_IPV6_MROUTE: int = 10;
// pub const RTNLGRP_IPV6_ROUTE: int = 11;
// pub const RTNLGRP_IPV6_IFINFO: int = 12;
// pub const RTNLGRP_DECNET_IFADDR: int = 13;
// pub const RTNLGRP_NOP2: int = 14;
// pub const RTNLGRP_DECNET_ROUTE: int = 15;
// pub const RTNLGRP_DECNET_RULE: int = 16;
// pub const RTNLGRP_NOP4: int = 17;
// pub const RTNLGRP_IPV6_PREFIX: int = 18;
// pub const RTNLGRP_IPV6_RULE: int = 19;
// pub const RTNLGRP_ND_USEROPT: int = 20;
// pub const RTNLGRP_PHONET_IFADDR: int = 21;
// pub const RTNLGRP_PHONET_ROUTE: int = 22;
// pub const RTNLGRP_DCB: int = 23;
// pub const RTNLGRP_IPV4_NETCONF: int = 24;
// pub const RTNLGRP_IPV6_NETCONF: int = 25;
// pub const RTNLGRP_MDB: int = 26;
// pub const RTNLGRP_MPLS_ROUTE: int = 27;
// pub const RTNLGRP_NSID: int = 28;
// pub const RTNLGRP_MPLS_NETCONF: int = 29;
// pub const RTNLGRP_IPV4_MROUTE_R: int = 30;
// pub const RTNLGRP_IPV6_MROUTE_R: int = 31;
//
// pub const IFLA_VF_LINK_STATE_AUTO: int = 0;
// pub const IFLA_VF_LINK_STATE_ENABLE: int = 1;
// pub const IFLA_VF_LINK_STATE_DISABLE: int = 2;
//
// pub const IFLA_VF_STATS_RX_PACKETS: int = 0;
// pub const IFLA_VF_STATS_TX_PACKETS: int = 1;
// pub const IFLA_VF_STATS_RX_BYTES: int = 2;
// pub const IFLA_VF_STATS_TX_BYTES: int = 3;
// pub const IFLA_VF_STATS_BROADCAST: int = 4;
// pub const IFLA_VF_STATS_MULTICAST: int = 5;
// pub const IFLA_VF_STATS_PAD: int = 6;
// pub const IFLA_VF_STATS_RX_DROPPED: int = 7;
// pub const IFLA_VF_STATS_TX_DROPPED: int = 8;
//
// pub const IFLA_VF_PORT_UNSPEC: int = 0;
// pub const IFLA_VF_PORT: int = 1;
//
// pub const IFLA_PORT_UNSPEC: int = 0;
// pub const IFLA_PORT_VF: int = 1;
// pub const IFLA_PORT_PROFILE: int = 2;
// pub const IFLA_PORT_VSI_TYPE: int = 3;
// pub const IFLA_PORT_INSTANCE_UUID: int = 4;
// pub const IFLA_PORT_HOST_UUID: int = 5;
// pub const IFLA_PORT_REQUEST: int = 6;
// pub const IFLA_PORT_RESPONSE: int = 7;
//
// pub const PORT_REQUEST_PREASSOCIATE: int = 0;
// pub const PORT_REQUEST_PREASSOCIATE_RR: int = 1;
// pub const PORT_REQUEST_ASSOCIATE: int = 2;
// pub const PORT_REQUEST_DISASSOCIATE: int = 3;
//
// pub const PORT_VDP_RESPONSE_SUCCESS: int = 0;
// pub const PORT_VDP_RESPONSE_INVALID_FORMAT: int = 1;
// pub const PORT_VDP_RESPONSE_INSUFFICIENT_RESOURCES: int = 2;
// pub const PORT_VDP_RESPONSE_UNUSED_VTID: int = 3;
// pub const PORT_VDP_RESPONSE_VTID_VIOLATION: int = 4;
// pub const PORT_VDP_RESPONSE_VTID_VERSION_VIOALTION: int = 5;
// pub const PORT_VDP_RESPONSE_OUT_OF_SYNC: int = 6;
// pub const PORT_PROFILE_RESPONSE_SUCCESS: int = 256;
// pub const PORT_PROFILE_RESPONSE_INPROGRESS: int = 257;
// pub const PORT_PROFILE_RESPONSE_INVALID: int = 258;
// pub const PORT_PROFILE_RESPONSE_BADSTATE: int = 259;
// pub const PORT_PROFILE_RESPONSE_INSUFFICIENT_RESOURCES: int = 260;
// pub const PORT_PROFILE_RESPONSE_ERROR: int = 261;
//
// pub const IFLA_IPOIB_UNSPEC: int = 0;
// pub const IFLA_IPOIB_PKEY: int = 1;
// pub const IFLA_IPOIB_MODE: int = 2;
// pub const IFLA_IPOIB_UMCAST: int = 3;
//
// pub const IPOIB_MODE_DATAGRAM: int = 0;
// pub const IPOIB_MODE_CONNECTED: int = 1;
//
// pub const IFLA_HSR_UNSPEC: int = 0;
// pub const IFLA_HSR_SLAVE1: int = 1;
// pub const IFLA_HSR_SLAVE2: int = 2;
// pub const IFLA_HSR_MULTICAST_SPEC: int = 3;
// pub const IFLA_HSR_SUPERVISION_ADDR: int = 4;
// pub const IFLA_HSR_SEQ_NR: int = 5;
// pub const IFLA_HSR_VERSION: int = 6;
//
// pub const IFLA_STATS_UNSPEC: int = 0;
// pub const IFLA_STATS_LINK_64: int = 1;
// pub const IFLA_STATS_LINK_XSTATS: int = 2;
// pub const IFLA_STATS_LINK_XSTATS_SLAVE: int = 3;
// pub const IFLA_STATS_LINK_OFFLOAD_XSTATS: int = 4;
// pub const IFLA_STATS_AF_SPEC: int = 5;
//
// pub const LINK_XSTATS_TYPE_UNSPEC: int = 0;
// pub const LINK_XSTATS_TYPE_BRIDGE: int = 1;
//
// pub const IFLA_OFFLOAD_XSTATS_UNSPEC: int = 0;
// pub const IFLA_OFFLOAD_XSTATS_CPU_HIT: int = 1;
//
// pub const XDP_ATTACHED_NONE: int = 0;
// pub const XDP_ATTACHED_DRV: int = 1;
// pub const XDP_ATTACHED_SKB: int = 2;
// pub const XDP_ATTACHED_HW: int = 3;
//
// pub const IFLA_XDP_UNSPEC: int = 0;
// pub const IFLA_XDP_FD: int = 1;
// pub const IFLA_XDP_ATTACHED: int = 2;
// pub const IFLA_XDP_FLAGS: int = 3;
// pub const IFLA_XDP_PROG_ID: int = 4;
//
// pub const IFLA_EVENT_NONE: int = 0;
// pub const IFLA_EVENT_REBOOT: int = 1;
// pub const IFLA_EVENT_FEATURES: int = 2;
// pub const IFLA_EVENT_BONDING_FAILOVER: int = 3;
// pub const IFLA_EVENT_NOTIFY_PEERS: int = 4;
// pub const IFLA_EVENT_IGMP_RESEND: int = 5;
// pub const IFLA_EVENT_BONDING_OPTIONS: int = 6;
//
pub const IFA_UNSPEC: int = 0;
pub const IFA_ADDRESS: int = 1;
pub const IFA_LOCAL: int = 2;
pub const IFA_LABEL: int = 3;
pub const IFA_BROADCAST: int = 4;
pub const IFA_ANYCAST: int = 5;
pub const IFA_CACHEINFO: int = 6;
pub const IFA_MULTICAST: int = 7;
pub const IFA_FLAGS: int = 8;
//
// pub const NDA_UNSPEC: int = 0;
// pub const NDA_DST: int = 1;
// pub const NDA_LLADDR: int = 2;
// pub const NDA_CACHEINFO: int = 3;
// pub const NDA_PROBES: int = 4;
// pub const NDA_VLAN: int = 5;
// pub const NDA_PORT: int = 6;
// pub const NDA_VNI: int = 7;
// pub const NDA_IFINDEX: int = 8;
// pub const NDA_MASTER: int = 9;
// pub const NDA_LINK_NETNSID: int = 10;
// pub const NDA_SRC_VNI: int = 11;
//
// pub const NDTPA_UNSPEC: int = 0;
// pub const NDTPA_IFINDEX: int = 1;
// pub const NDTPA_REFCNT: int = 2;
// pub const NDTPA_REACHABLE_TIME: int = 3;
// pub const NDTPA_BASE_REACHABLE_TIME: int = 4;
// pub const NDTPA_RETRANS_TIME: int = 5;
// pub const NDTPA_GC_STALETIME: int = 6;
// pub const NDTPA_DELAY_PROBE_TIME: int = 7;
// pub const NDTPA_QUEUE_LEN: int = 8;
// pub const NDTPA_APP_PROBES: int = 9;
// pub const NDTPA_UCAST_PROBES: int = 10;
// pub const NDTPA_MCAST_PROBES: int = 11;
// pub const NDTPA_ANYCAST_DELAY: int = 12;
// pub const NDTPA_PROXY_DELAY: int = 13;
// pub const NDTPA_PROXY_QLEN: int = 14;
// pub const NDTPA_LOCKTIME: int = 15;
// pub const NDTPA_QUEUE_LENBYTES: int = 16;
// pub const NDTPA_MCAST_REPROBES: int = 17;
// pub const NDTPA_PAD: int = 18;
//
// pub const NDTA_UNSPEC: int = 0;
// pub const NDTA_NAME: int = 1;
// pub const NDTA_THRESH1: int = 2;
// pub const NDTA_THRESH2: int = 3;
// pub const NDTA_THRESH3: int = 4;
// pub const NDTA_CONFIG: int = 5;
// pub const NDTA_PARMS: int = 6;
// pub const NDTA_STATS: int = 7;
// pub const NDTA_GC_INTERVAL: int = 8;
// pub const NDTA_PAD: int = 9;

/// Routing/neighbour discovery messages.
// pub const RTM_BASE: int = 16;
pub const RTM_NEWLINK: int = 16;
pub const RTM_DELLINK: int = 17;
pub const RTM_GETLINK: int = 18;
pub const RTM_SETLINK: int = 19;
pub const RTM_NEWADDR: int = 20;
pub const RTM_DELADDR: int = 21;
pub const RTM_GETADDR: int = 22;
pub const RTM_NEWROUTE: int = 24;
pub const RTM_DELROUTE: int = 25;
pub const RTM_GETROUTE: int = 26;
pub const RTM_NEWNEIGH: int = 28;
pub const RTM_DELNEIGH: int = 29;
pub const RTM_GETNEIGH: int = 30;
pub const RTM_NEWRULE: int = 32;
pub const RTM_DELRULE: int = 33;
pub const RTM_GETRULE: int = 34;
pub const RTM_NEWQDISC: int = 36;
pub const RTM_DELQDISC: int = 37;
pub const RTM_GETQDISC: int = 38;
pub const RTM_NEWTCLASS: int = 40;
pub const RTM_DELTCLASS: int = 41;
pub const RTM_GETTCLASS: int = 42;
pub const RTM_NEWTFILTER: int = 44;
pub const RTM_DELTFILTER: int = 45;
pub const RTM_GETTFILTER: int = 46;
pub const RTM_NEWACTION: int = 48;
pub const RTM_DELACTION: int = 49;
pub const RTM_GETACTION: int = 50;
pub const RTM_NEWPREFIX: int = 52;
pub const RTM_GETMULTICAST: int = 58;
pub const RTM_GETANYCAST: int = 62;
pub const RTM_NEWNEIGHTBL: int = 64;
pub const RTM_GETNEIGHTBL: int = 66;
pub const RTM_SETNEIGHTBL: int = 67;
pub const RTM_NEWNDUSEROPT: int = 68;
pub const RTM_NEWADDRLABEL: int = 72;
pub const RTM_DELADDRLABEL: int = 73;
pub const RTM_GETADDRLABEL: int = 74;
pub const RTM_GETDCB: int = 78;
pub const RTM_SETDCB: int = 79;
pub const RTM_NEWNETCONF: int = 80;
pub const RTM_DELNETCONF: int = 81;
pub const RTM_GETNETCONF: int = 82;
pub const RTM_NEWMDB: int = 84;
pub const RTM_DELMDB: int = 85;
pub const RTM_GETMDB: int = 86;
pub const RTM_NEWNSID: int = 88;
pub const RTM_DELNSID: int = 89;
pub const RTM_GETNSID: int = 90;
pub const RTM_NEWSTATS: int = 92;
pub const RTM_GETSTATS: int = 94;
pub const RTM_NEWCACHEREPORT: int = 96;

pub const RTN_UNSPEC: int = 0;
pub const RTN_UNICAST: int = 1;
pub const RTN_LOCAL: int = 2;
pub const RTN_BROADCAST: int = 3;
pub const RTN_ANYCAST: int = 4;
pub const RTN_MULTICAST: int = 5;
pub const RTN_BLACKHOLE: int = 6;
pub const RTN_UNREACHABLE: int = 7;
pub const RTN_PROHIBIT: int = 8;
pub const RTN_THROW: int = 9;
pub const RTN_NAT: int = 10;
pub const RTN_XRESOLVE: int = 11;

pub const RT_SCOPE_UNIVERSE: int = 0;
pub const RT_SCOPE_SITE: int = 200;
pub const RT_SCOPE_LINK: int = 253;
pub const RT_SCOPE_HOST: int = 254;
pub const RT_SCOPE_NOWHERE: int = 255;

pub const RT_TABLE_UNSPEC: int = 0;
pub const RT_TABLE_COMPAT: int = 252;
pub const RT_TABLE_DEFAULT: int = 253;
pub const RT_TABLE_MAIN: int = 254;
pub const RT_TABLE_LOCAL: int = 255;
// #[allow(overflowing_literals)]
// pub const RT_TABLE_MAX: int = 4294967295;

pub const RTA_UNSPEC: int = 0;
pub const RTA_DST: int = 1;
pub const RTA_SRC: int = 2;
pub const RTA_IIF: int = 3;
pub const RTA_OIF: int = 4;
pub const RTA_GATEWAY: int = 5;
pub const RTA_PRIORITY: int = 6;
pub const RTA_PREFSRC: int = 7;
pub const RTA_METRICS: int = 8;
pub const RTA_MULTIPATH: int = 9;
pub const RTA_PROTOINFO: int = 10;
pub const RTA_FLOW: int = 11;
pub const RTA_CACHEINFO: int = 12;
pub const RTA_SESSION: int = 13;
pub const RTA_MP_ALGO: int = 14;
pub const RTA_TABLE: int = 15;
pub const RTA_MARK: int = 16;
pub const RTA_MFC_STATS: int = 17;
pub const RTA_VIA: int = 18;
pub const RTA_NEWDST: int = 19;
pub const RTA_PREF: int = 20;
pub const RTA_ENCAP_TYPE: int = 21;
pub const RTA_ENCAP: int = 22;
pub const RTA_EXPIRES: int = 23;
pub const RTA_PAD: int = 24;
pub const RTA_UID: int = 25;
pub const RTA_TTL_PROPAGATE: int = 26;

pub const RTAX_UNSPEC: int = 0;
pub const RTAX_LOCK: int = 1;
pub const RTAX_MTU: int = 2;
pub const RTAX_WINDOW: int = 3;
pub const RTAX_RTT: int = 4;
pub const RTAX_RTTVAR: int = 5;
pub const RTAX_SSTHRESH: int = 6;
pub const RTAX_CWND: int = 7;
pub const RTAX_ADVMSS: int = 8;
pub const RTAX_REORDERING: int = 9;
pub const RTAX_HOPLIMIT: int = 10;
pub const RTAX_INITCWND: int = 11;
pub const RTAX_FEATURES: int = 12;
pub const RTAX_RTO_MIN: int = 13;
pub const RTAX_INITRWND: int = 14;
pub const RTAX_QUICKACK: int = 15;
pub const RTAX_CC_ALGO: int = 16;
pub const RTAX_FASTOPEN_NO_COOKIE: int = 17;

// pub const PREFIX_UNSPEC: int = 0;
// pub const PREFIX_ADDRESS: int = 1;
// pub const PREFIX_CACHEINFO: int = 2;
//
// pub const TCA_UNSPEC: int = 0;
// pub const TCA_KIND: int = 1;
// pub const TCA_OPTIONS: int = 2;
// pub const TCA_STATS: int = 3;
// pub const TCA_XSTATS: int = 4;
// pub const TCA_RATE: int = 5;
// pub const TCA_FCNT: int = 6;
// pub const TCA_STATS2: int = 7;
// pub const TCA_STAB: int = 8;
// pub const TCA_PAD: int = 9;
// pub const TCA_DUMP_INVISIBLE: int = 10;
// pub const TCA_CHAIN: int = 11;
// pub const TCA_HW_OFFLOAD: int = 12;
// pub const TCA_INGRESS_BLOCK: int = 13;
// pub const TCA_EGRESS_BLOCK: int = 14;
//
// pub const __BITS_PER_LONG: int = 64;
// pub const __FD_SETSIZE: int = 1024;
// pub const SI_LOAD_SHIFT: int = 16;
// pub const _K_SS_MAXSIZE: int = 128;
pub const NETLINK_ROUTE: int = 0;
pub const NETLINK_UNUSED: int = 1;
pub const NETLINK_USERSOCK: int = 2;
pub const NETLINK_FIREWALL: int = 3;
pub const NETLINK_SOCK_DIAG: int = 4;
pub const NETLINK_NFLOG: int = 5;
pub const NETLINK_XFRM: int = 6;
pub const NETLINK_SELINUX: int = 7;
pub const NETLINK_ISCSI: int = 8;
pub const NETLINK_AUDIT: int = 9;
pub const NETLINK_FIB_LOOKUP: int = 10;
pub const NETLINK_CONNECTOR: int = 11;
pub const NETLINK_NETFILTER: int = 12;
pub const NETLINK_IP6_FW: int = 13;
pub const NETLINK_DNRTMSG: int = 14;
pub const NETLINK_KOBJECT_UEVENT: int = 15;
pub const NETLINK_GENERIC: int = 16;
pub const NETLINK_SCSITRANSPORT: int = 18;
pub const NETLINK_ECRYPTFS: int = 19;
pub const NETLINK_RDMA: int = 20;
pub const NETLINK_CRYPTO: int = 21;
// pub const NETLINK_SMC: int = 22;
// pub const NETLINK_INET_DIAG: int = 4;
// pub const MAX_LINKS: int = 32;
// pub const NLM_F_REQUEST: int = 1;
// pub const NLM_F_MULTI: int = 2;
// pub const NLM_F_ACK: int = 4;
// pub const NLM_F_ECHO: int = 8;
// pub const NLM_F_DUMP_INTR: int = 16;
// pub const NLM_F_DUMP_FILTERED: int = 32;
// pub const NLM_F_ROOT: int = 256;
// pub const NLM_F_MATCH: int = 512;
// pub const NLM_F_ATOMIC: int = 1024;
// pub const NLM_F_DUMP: int = 768;
// pub const NLM_F_REPLACE: int = 256;
// pub const NLM_F_EXCL: int = 512;
// pub const NLM_F_CREATE: int = 1024;
// pub const NLM_F_APPEND: int = 2048;
pub const NLM_F_NONREC: int = 256;
pub const NLM_F_CAPPED: int = 256;
pub const NLM_F_ACK_TLVS: int = 512;
// pub const NLMSG_ALIGNTO: int = 4;
pub const NLMSG_NOOP: int = 1;
pub const NLMSG_ERROR: int = 2;
pub const NLMSG_DONE: int = 3;
pub const NLMSG_OVERRUN: int = 4;
// pub const NLMSG_MIN_TYPE: int = 16;
// pub const NETLINK_ADD_MEMBERSHIP: int = 1;
// pub const NETLINK_DROP_MEMBERSHIP: int = 2;
// pub const NETLINK_PKTINFO: int = 3;
// pub const NETLINK_BROADCAST_ERROR: int = 4;
// pub const NETLINK_NO_ENOBUFS: int = 5;
// pub const NETLINK_RX_RING: int = 6;
// pub const NETLINK_TX_RING: int = 7;
// pub const NETLINK_LISTEN_ALL_NSID: int = 8;
// pub const NETLINK_LIST_MEMBERSHIPS: int = 9;
// pub const NETLINK_CAP_ACK: int = 10;
// pub const NETLINK_EXT_ACK: int = 11;
// pub const NL_MMAP_MSG_ALIGNMENT: int = 4;
// pub const NET_MAJOR: int = 36;
pub const NLA_F_NESTED: int = 32768;
pub const NLA_F_NET_BYTEORDER: int = 16384;
pub const NLA_TYPE_MASK: int = -49153;
// pub const NLA_ALIGNTO: int = 4;
// pub const MACVLAN_FLAG_NOPROMISC: int = 1;
// pub const IPVLAN_F_PRIVATE: int = 1;
// pub const IPVLAN_F_VEPA: int = 2;
// pub const MAX_VLAN_LIST_LEN: int = 1;
// pub const PORT_PROFILE_MAX: int = 40;
// pub const PORT_UUID_MAX: int = 16;
// pub const PORT_SELF_VF: int = -1;
// pub const XDP_FLAGS_UPDATE_IF_NOEXIST: int = 1;
// pub const XDP_FLAGS_SKB_MODE: int = 2;
// pub const XDP_FLAGS_DRV_MODE: int = 4;
// pub const XDP_FLAGS_HW_MODE: int = 8;
// pub const XDP_FLAGS_MODES: int = 14;
// pub const XDP_FLAGS_MASK: int = 15;
// pub const IFA_F_SECONDARY: int = 1;
// pub const IFA_F_TEMPORARY: int = 1;
// pub const IFA_F_NODAD: int = 2;
// pub const IFA_F_OPTIMISTIC: int = 4;
// pub const IFA_F_DADFAILED: int = 8;
// pub const IFA_F_HOMEADDRESS: int = 16;
// pub const IFA_F_DEPRECATED: int = 32;
// pub const IFA_F_TENTATIVE: int = 64;
// pub const IFA_F_PERMANENT: int = 128;
// pub const IFA_F_MANAGETEMPADDR: int = 256;
// pub const IFA_F_NOPREFIXROUTE: int = 512;
// pub const IFA_F_MCAUTOJOIN: int = 1024;
// pub const IFA_F_STABLE_PRIVACY: int = 2048;
// pub const NTF_USE: int = 1;
// pub const NTF_SELF: int = 2;
// pub const NTF_MASTER: int = 4;
// pub const NTF_PROXY: int = 8;
// pub const NTF_EXT_LEARNED: int = 16;
// pub const NTF_OFFLOADED: int = 32;
// pub const NTF_ROUTER: int = 128;
// pub const NUD_INCOMPLETE: int = 1;
// pub const NUD_REACHABLE: int = 2;
// pub const NUD_STALE: int = 4;
// pub const NUD_DELAY: int = 8;
// pub const NUD_PROBE: int = 16;
// pub const NUD_FAILED: int = 32;
// pub const NUD_NOARP: int = 64;
// pub const NUD_PERMANENT: int = 128;
// pub const NUD_NONE: int = 0;
// pub const RTNL_FAMILY_IPMR: int = 128;
// pub const RTNL_FAMILY_IP6MR: int = 129;
// pub const RTNL_FAMILY_MAX: int = 129;
// pub const RTA_ALIGNTO: int = 4;
pub const RTPROT_UNSPEC: int = 0;
pub const RTPROT_REDIRECT: int = 1;
pub const RTPROT_KERNEL: int = 2;
pub const RTPROT_BOOT: int = 3;
pub const RTPROT_STATIC: int = 4;
pub const RTPROT_GATED: int = 8;
pub const RTPROT_RA: int = 9;
pub const RTPROT_MRT: int = 10;
pub const RTPROT_ZEBRA: int = 11;
pub const RTPROT_BIRD: int = 12;
pub const RTPROT_DNROUTED: int = 13;
pub const RTPROT_XORP: int = 14;
pub const RTPROT_NTK: int = 15;
pub const RTPROT_DHCP: int = 16;
pub const RTPROT_MROUTED: int = 17;
pub const RTPROT_BABEL: int = 42;
pub const RTM_F_NOTIFY: int = 256;
pub const RTM_F_CLONED: int = 512;
pub const RTM_F_EQUALIZE: int = 1024;
pub const RTM_F_PREFIX: int = 2048;
pub const RTM_F_LOOKUP_TABLE: int = 4096;
pub const RTM_F_FIB_MATCH: int = 8192;
// pub const RTNH_F_DEAD: int = 1;
// pub const RTNH_F_PERVASIVE: int = 2;
// pub const RTNH_F_ONLINK: int = 4;
// pub const RTNH_F_OFFLOAD: int = 8;
// pub const RTNH_F_LINKDOWN: int = 16;
// pub const RTNH_F_UNRESOLVED: int = 32;
// pub const RTNH_COMPARE_MASK: int = 25;
// pub const RTNH_ALIGNTO: int = 4;
// pub const RTNETLINK_HAVE_PEERINFO: int = 1;
// pub const RTAX_FEATURE_ECN: int = 1;
// pub const RTAX_FEATURE_SACK: int = 2;
// pub const RTAX_FEATURE_TIMESTAMP: int = 4;
// pub const RTAX_FEATURE_ALLFRAG: int = 8;
// pub const RTAX_FEATURE_MASK: int = 15;
// #[allow(overflowing_literals)]
// pub const TCM_IFINDEX_MAGIC_BLOCK: int = 4294967295;
// pub const RTMGRP_LINK: int = 1;
// pub const RTMGRP_NOTIFY: int = 2;
// pub const RTMGRP_NEIGH: int = 4;
// pub const RTMGRP_TC: int = 8;
// pub const RTMGRP_IPV4_IFADDR: int = 16;
// pub const RTMGRP_IPV4_MROUTE: int = 32;
// pub const RTMGRP_IPV4_ROUTE: int = 64;
// pub const RTMGRP_IPV4_RULE: int = 128;
// pub const RTMGRP_IPV6_IFADDR: int = 256;
// pub const RTMGRP_IPV6_MROUTE: int = 512;
// pub const RTMGRP_IPV6_ROUTE: int = 1024;
// pub const RTMGRP_IPV6_IFINFO: int = 2048;
// pub const RTMGRP_DECNET_IFADDR: int = 4096;
// pub const RTMGRP_DECNET_ROUTE: int = 16384;
// pub const RTMGRP_IPV6_PREFIX: int = 131072;
// pub const TCA_FLAG_LARGE_DUMP_ON: int = 1;
// pub const RTEXT_FILTER_VF: int = 1;
// pub const RTEXT_FILTER_BRVLAN: int = 2;
// pub const RTEXT_FILTER_BRVLAN_COMPRESSED: int = 4;
// pub const RTEXT_FILTER_SKIP_STATS: int = 8;

pub const ARPHRD_NETROM: int = 0;
pub const ARPHRD_ETHER: int = 1;
pub const ARPHRD_EETHER: int = 2;
pub const ARPHRD_AX25: int = 3;
pub const ARPHRD_PRONET: int = 4;
pub const ARPHRD_CHAOS: int = 5;
pub const ARPHRD_IEEE802: int = 6;
pub const ARPHRD_ARCNET: int = 7;
pub const ARPHRD_APPLETLK: int = 8;
pub const ARPHRD_DLCI: int = 15;
pub const ARPHRD_ATM: int = 19;
pub const ARPHRD_METRICOM: int = 23;
pub const ARPHRD_IEEE1394: int = 24;
pub const ARPHRD_EUI64: int = 27;
pub const ARPHRD_INFINIBAND: int = 32;
pub const ARPHRD_SLIP: int = 256;
pub const ARPHRD_CSLIP: int = 257;
pub const ARPHRD_SLIP6: int = 258;
pub const ARPHRD_CSLIP6: int = 259;
pub const ARPHRD_RSRVD: int = 260;
pub const ARPHRD_ADAPT: int = 264;
pub const ARPHRD_ROSE: int = 270;
pub const ARPHRD_X25: int = 271;
pub const ARPHRD_HWX25: int = 272;
pub const ARPHRD_CAN: int = 280;
pub const ARPHRD_PPP: int = 512;
// pub const ARPHRD_CISCO: int = 513;
pub const ARPHRD_HDLC: int = 513;
pub const ARPHRD_LAPB: int = 516;
pub const ARPHRD_DDCMP: int = 517;
pub const ARPHRD_RAWHDLC: int = 518;
pub const ARPHRD_RAWIP: int = 519;
pub const ARPHRD_TUNNEL: int = 768;
pub const ARPHRD_TUNNEL6: int = 769;
pub const ARPHRD_FRAD: int = 770;
pub const ARPHRD_SKIP: int = 771;
pub const ARPHRD_LOOPBACK: int = 772;
pub const ARPHRD_LOCALTLK: int = 773;
pub const ARPHRD_FDDI: int = 774;
pub const ARPHRD_BIF: int = 775;
pub const ARPHRD_SIT: int = 776;
pub const ARPHRD_IPDDP: int = 777;
pub const ARPHRD_IPGRE: int = 778;
pub const ARPHRD_PIMREG: int = 779;
pub const ARPHRD_HIPPI: int = 780;
pub const ARPHRD_ASH: int = 781;
pub const ARPHRD_ECONET: int = 782;
pub const ARPHRD_IRDA: int = 783;
pub const ARPHRD_FCPP: int = 784;
pub const ARPHRD_FCAL: int = 785;
pub const ARPHRD_FCPL: int = 786;
pub const ARPHRD_FCFABRIC: int = 787;
pub const ARPHRD_IEEE802_TR: int = 800;
pub const ARPHRD_IEEE80211: int = 801;
pub const ARPHRD_IEEE80211_PRISM: int = 802;
pub const ARPHRD_IEEE80211_RADIOTAP: int = 803;
pub const ARPHRD_IEEE802154: int = 804;
pub const ARPHRD_IEEE802154_MONITOR: int = 805;
pub const ARPHRD_PHONET: int = 820;
pub const ARPHRD_PHONET_PIPE: int = 821;
pub const ARPHRD_CAIF: int = 822;
pub const ARPHRD_IP6GRE: int = 823;
pub const ARPHRD_NETLINK: int = 824;
pub const ARPHRD_6LOWPAN: int = 825;
pub const ARPHRD_VSOCKMON: int = 826;
pub const ARPHRD_VOID: int = 65535;
pub const ARPHRD_NONE: int = 65534;
// pub const ARPOP_REQUEST: int = 1;
// pub const ARPOP_REPLY: int = 2;
