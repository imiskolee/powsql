pub type ProtocolConst = u32;

	//PREPARE

pub const COM__STMT_EXECUTE      :ProtocolConst = 0;
pub const	COM_STMT_PREPARE      :ProtocolConst = 0;
pub const	COM_STMT_CLOSE        :ProtocolConst = 0;
pub const	COM_STMT_SENDLONGDATA :ProtocolConst = 0;
pub const	COM_STMT_RESET        :ProtocolConst = 0;

	//PACKET

pub const	PACKET_HEAD_EROK   :ProtocolConst = 0;
pub const	PACKET_HEAD_ERRAW  :ProtocolConst = 0xFA;
pub const	PACKET_HEAD_ERNULL :ProtocolConst = 0xFB;
pub const	PACKET_HEAD_EREOF  :ProtocolConst = 0xFE;
pub const	PACKET_HEAD_ERERR  :ProtocolConst = 0xFF;

	//TEXT PROTOCOL
pub const	COM_SLEEP           :ProtocolConst = 0x00;
pub const	COM_QUIT            :ProtocolConst = 0x01;
pub const	COM_INITDB          :ProtocolConst = 0x02;
pub const	COM_QUERY           :ProtocolConst = 0x03;
pub const	COM_FIELDLIST       :ProtocolConst = 0x04;
pub const	COM_CREATEDB        :ProtocolConst = 0x05;
pub const	COM_DROPDB          :ProtocolConst = 0x06;
pub const	COM_REFRESH         :ProtocolConst = 0x07;
pub const	COM_SHUTDOWN        :ProtocolConst = 0x08;
pub const	COM_STATISTICS      :ProtocolConst = 0x09;
pub const	COM_PROCESS_INFO     :ProtocolConst = 0x0A;
pub const	COM_CONNECT         :ProtocolConst = 0x0B;
pub const	COM_PROCESS_KILL     :ProtocolConst = 0x0C;
pub const	COM_DEBUG           :ProtocolConst = 0x0D;
pub const	COM_PING            :ProtocolConst = 0x0E;
pub const	COM_TIME            :ProtocolConst = 0x0F;
pub const	COM_DELAYED_INSERT   :ProtocolConst = 0x10;
pub const	COM_CHANGE_USER      :ProtocolConst = 0x11;
pub const	COM_RESET_CONNECTION :ProtocolConst = 0x1F;
pub const	COM_DAEMON          :ProtocolConst = 0x1D;

	//SLAVE PROTOCOL

pub const	COM_BINLOGDUMP     :ProtocolConst = 0;
pub const	COM_BINLOGDUMPGTID :ProtocolConst = 0;
pub const	COM_TABLEDUMP      :ProtocolConst = 0;
pub const	COM_CONNECTOUT     :ProtocolConst = 0;
pub const	COM_REGISTERSLAVE  :ProtocolConst = 0;

	//STATUS FLAGS

pub const	SERVER_STATUS_INTRANS            :ProtocolConst = 0x0001;
pub const	SERVER_STATUS_AUTOCOM_MIT         :ProtocolConst = 0x0002;
pub const	SERVER_STATUS_REQUEST_EXITS       :ProtocolConst = 0x0008;
pub const	SERVER_STATUS_NO_GOOD_INDEX_USED    :ProtocolConst = 0x0010;
pub const	SERVER_STATUS_NO_INDE_XUSED        :ProtocolConst = 0x0020;
pub const	SERVER_STATUS_CURSOR_EXISTS       :ProtocolConst = 0x0040;
pub const	SERVER_STATUS_LAST_ROW_SENT        :ProtocolConst = 0x0080;
pub const	SERVER_STATUS_DB_DROPED           :ProtocolConst = 0x0100;
pub const	SERVER_STATUS_NO_BACKS_LASH_ESCAPES :ProtocolConst = 0x0200;
pub const	SERVER_STATUS_META_DATA_CHANGED    :ProtocolConst = 0x0400;
pub const	SERVER_QUERY_WASS_LOW             :ProtocolConst = 0x0800;
pub const	SERVER_PS_OUT_PARAMS              :ProtocolConst = 0x1000;
pub const	SERVER_STATUS_INT_RANSREAD_ONLY    :ProtocolConst = 0x2000;
pub const	SERVER_SESSION_STATE_CHANGED      :ProtocolConst = 0x4000;

pub const	CAPABILITY_FLAG_CLIENT_LONG_PASSWORD               :ProtocolConst = 0x00000001;
pub const	CAPABILITY_FLAG_CLIENT_FOUND_ROWS                  :ProtocolConst = 0x00000002;
pub const	CAPABILITY_FLAG_CLIENT_LONG_FLAG                   :ProtocolConst = 0x00000004;
pub const	CAPABILITY_FLAG_CLIENT_CONNECT_WITH_DB              :ProtocolConst = 0x00000008;
pub const	CAPABILITY_FLAG_CLIENT_NO_SCHEMA                   :ProtocolConst = 0x00000010;
pub const	CAPABILITY_FLAG_CLIENT_COM_PRESS                   :ProtocolConst = 0x00000020;
pub const	CAPABILITY_FLAG_CLIENT_ODBC                       :ProtocolConst = 0x00000040;
pub const	CAPABILITY_FLAG_CLIENT_LOCAL_FILES                 :ProtocolConst = 0x00000080;
pub const	CAPABILITY_FLAG_CLIENT_IGNORE_SPACE                :ProtocolConst = 0x00000100;
pub const	CAPABILITY_FLAG_CLIENT_PROTOCOL_41                 :ProtocolConst = 0x00000200;
pub const	CAPABILITY_FLAG_CLIENT_INTERACTIVE                :ProtocolConst = 0x00000400;
pub const	CAPABILITY_FLAG_CLIENT_SSL                        :ProtocolConst = 0x00000800;
pub const	CAPABILITY_FLAG_CLIENT_IGNORE_SIGPIPE              :ProtocolConst = 0x00001000;
pub const	CAPABILITY_FLAG_CLIENT_TRANSACTIONS               :ProtocolConst = 0x00002000;
pub const	CAPABILITY_FLAG_CLIENT_RESERVED                   :ProtocolConst = 0x00004000;
pub const	CAPABILITY_FLAG_CLIENT_SECURE_CONNECTION           :ProtocolConst = 0x00008000;
pub const	CAPABILITY_FLAG_CLIENT_MULTI_STATE_MENTS            :ProtocolConst = 0x00010000;
pub const	CAPABILITY_FLAG_CLIENT_MULTI_RESULTS               :ProtocolConst = 0x00020000;
pub const	CAPABILITY_FLAG_CLIENT_PS_MULTI_RESULTS             :ProtocolConst = 0x00040000;
pub const	CAPABILITY_FLAG_CLIENT_PLUGIN_AUTH                 :ProtocolConst = 0x00080000;
pub const	CAPABILITY_FLAG_CLIENT_CONNECT_ATTRS               :ProtocolConst = 0x00100000;
pub const	CAPABILITY_FLAG_CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA :ProtocolConst = 0x00200000;
pub const	CAPABILITY_FLAG_CLIENT_CAN_HANDLE_EXPIRED_PASSWORDS  :ProtocolConst = 0x00400000;
pub const	CAPABILITY_FLAG_CLIENT_SESSION_TRACK               :ProtocolConst = 0x00800000;
pub const	CAPABILITY_FLAG_CLIENT_DEPRECATE_EOF               :ProtocolConst = 0x01000000;

pub const	RESULT_FEILD_VALUE_NULL :ProtocolConst = 0xFB;

