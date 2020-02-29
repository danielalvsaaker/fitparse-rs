use super::field_types::MesgNum;
use super::{FieldInfo, MessageInfo};
/// Auto generated profile from FIT SDK Release: XXX
use std::collections::HashMap;

fn file_id_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "type",
        field_type: "file",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "manufacturer",
        field_type: "manufacturer",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "product",
        field_type: "uint16",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "serial_number",
        field_type: "uint32z",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "time_created",
        field_type: "date_time",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "number",
        field_type: "uint16",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "product_name",
        field_type: "string",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    MessageInfo {
        name: "FileId",
        fields: fields,
    }
}

fn file_creator_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "software_version",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "hardware_version",
        field_type: "uint8",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    MessageInfo {
        name: "FileCreator",
        fields: fields,
    }
}

fn timestamp_correlation_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "fractional_timestamp",
        field_type: "uint16",
        def_number: 0,
        scale: 32768.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "system_timestamp",
        field_type: "date_time",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "fractional_system_timestamp",
        field_type: "uint16",
        def_number: 2,
        scale: 32768.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "local_timestamp",
        field_type: "local_date_time",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: "uint16",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "system_timestamp_ms",
        field_type: "uint16",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(5, field);

    MessageInfo {
        name: "TimestampCorrelation",
        fields: fields,
    }
}

fn software_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "version",
        field_type: "uint16",
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "part_number",
        field_type: "string",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    MessageInfo {
        name: "Software",
        fields: fields,
    }
}

fn slave_device_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "manufacturer",
        field_type: "manufacturer",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "product",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    MessageInfo {
        name: "SlaveDevice",
        fields: fields,
    }
}

fn capabilities_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "languages",
        field_type: "uint8z",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "sports",
        field_type: "sport_bits_0",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "workouts_supported",
        field_type: "workout_capabilities",
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(21, field);

    let field = FieldInfo {
        name: "connectivity_supported",
        field_type: "connectivity_capabilities",
        def_number: 23,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(23, field);

    MessageInfo {
        name: "Capabilities",
        fields: fields,
    }
}

fn file_capabilities_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "type",
        field_type: "file",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "flags",
        field_type: "file_flags",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "directory",
        field_type: "string",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "max_count",
        field_type: "uint16",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "max_size",
        field_type: "uint32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "bytes",
    };
    fields.insert(4, field);

    MessageInfo {
        name: "FileCapabilities",
        fields: fields,
    }
}

fn mesg_capabilities_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "file",
        field_type: "file",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "mesg_num",
        field_type: "mesg_num",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "count_type",
        field_type: "mesg_count",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "count",
        field_type: "uint16",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    MessageInfo {
        name: "MesgCapabilities",
        fields: fields,
    }
}

fn field_capabilities_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "file",
        field_type: "file",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "mesg_num",
        field_type: "mesg_num",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "field_num",
        field_type: "uint8",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "count",
        field_type: "uint16",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    MessageInfo {
        name: "FieldCapabilities",
        fields: fields,
    }
}

fn device_settings_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "active_time_zone",
        field_type: "uint8",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "utc_offset",
        field_type: "uint32",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "time_offset",
        field_type: "uint32",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "time_mode",
        field_type: "time_mode",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "time_zone_offset",
        field_type: "sint8",
        def_number: 5,
        scale: 4.000000,
        offset: 0.000000,
        units: "hr",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "backlight_mode",
        field_type: "backlight_mode",
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(12, field);

    let field = FieldInfo {
        name: "activity_tracker_enabled",
        field_type: "bool",
        def_number: 36,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(36, field);

    let field = FieldInfo {
        name: "clock_time",
        field_type: "date_time",
        def_number: 39,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(39, field);

    let field = FieldInfo {
        name: "pages_enabled",
        field_type: "uint16",
        def_number: 40,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(40, field);

    let field = FieldInfo {
        name: "move_alert_enabled",
        field_type: "bool",
        def_number: 46,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(46, field);

    let field = FieldInfo {
        name: "date_mode",
        field_type: "date_mode",
        def_number: 47,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(47, field);

    let field = FieldInfo {
        name: "display_orientation",
        field_type: "display_orientation",
        def_number: 55,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(55, field);

    let field = FieldInfo {
        name: "mounting_side",
        field_type: "side",
        def_number: 56,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(56, field);

    let field = FieldInfo {
        name: "default_page",
        field_type: "uint16",
        def_number: 57,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(57, field);

    let field = FieldInfo {
        name: "autosync_min_steps",
        field_type: "uint16",
        def_number: 58,
        scale: 1.000000,
        offset: 0.000000,
        units: "steps",
    };
    fields.insert(58, field);

    let field = FieldInfo {
        name: "autosync_min_time",
        field_type: "uint16",
        def_number: 59,
        scale: 1.000000,
        offset: 0.000000,
        units: "minutes",
    };
    fields.insert(59, field);

    let field = FieldInfo {
        name: "lactate_threshold_autodetect_enabled",
        field_type: "bool",
        def_number: 80,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(80, field);

    let field = FieldInfo {
        name: "ble_auto_upload_enabled",
        field_type: "bool",
        def_number: 86,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(86, field);

    let field = FieldInfo {
        name: "auto_sync_frequency",
        field_type: "auto_sync_frequency",
        def_number: 89,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(89, field);

    let field = FieldInfo {
        name: "auto_activity_detect",
        field_type: "auto_activity_detect",
        def_number: 90,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(90, field);

    let field = FieldInfo {
        name: "number_of_screens",
        field_type: "uint8",
        def_number: 94,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(94, field);

    let field = FieldInfo {
        name: "smart_notification_display_orientation",
        field_type: "display_orientation",
        def_number: 95,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(95, field);

    let field = FieldInfo {
        name: "tap_interface",
        field_type: "switch",
        def_number: 134,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(134, field);

    MessageInfo {
        name: "DeviceSettings",
        fields: fields,
    }
}

fn user_profile_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "friendly_name",
        field_type: "string",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "gender",
        field_type: "gender",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "age",
        field_type: "uint8",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "years",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "height",
        field_type: "uint8",
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "weight",
        field_type: "uint16",
        def_number: 4,
        scale: 10.000000,
        offset: 0.000000,
        units: "kg",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "language",
        field_type: "language",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "elev_setting",
        field_type: "display_measure",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "weight_setting",
        field_type: "display_measure",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "resting_heart_rate",
        field_type: "uint8",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "default_max_running_heart_rate",
        field_type: "uint8",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "default_max_biking_heart_rate",
        field_type: "uint8",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "default_max_heart_rate",
        field_type: "uint8",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "hr_setting",
        field_type: "display_heart",
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(12, field);

    let field = FieldInfo {
        name: "speed_setting",
        field_type: "display_measure",
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(13, field);

    let field = FieldInfo {
        name: "dist_setting",
        field_type: "display_measure",
        def_number: 14,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(14, field);

    let field = FieldInfo {
        name: "power_setting",
        field_type: "display_power",
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(16, field);

    let field = FieldInfo {
        name: "activity_class",
        field_type: "activity_class",
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(17, field);

    let field = FieldInfo {
        name: "position_setting",
        field_type: "display_position",
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(18, field);

    let field = FieldInfo {
        name: "temperature_setting",
        field_type: "display_measure",
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(21, field);

    let field = FieldInfo {
        name: "local_id",
        field_type: "user_local_id",
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(22, field);

    let field = FieldInfo {
        name: "global_id",
        field_type: "byte",
        def_number: 23,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(23, field);

    let field = FieldInfo {
        name: "wake_time",
        field_type: "localtime_into_day",
        def_number: 28,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(28, field);

    let field = FieldInfo {
        name: "sleep_time",
        field_type: "localtime_into_day",
        def_number: 29,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(29, field);

    let field = FieldInfo {
        name: "height_setting",
        field_type: "display_measure",
        def_number: 30,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(30, field);

    let field = FieldInfo {
        name: "user_running_step_length",
        field_type: "uint16",
        def_number: 31,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(31, field);

    let field = FieldInfo {
        name: "user_walking_step_length",
        field_type: "uint16",
        def_number: 32,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(32, field);

    let field = FieldInfo {
        name: "depth_setting",
        field_type: "display_measure",
        def_number: 47,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(47, field);

    let field = FieldInfo {
        name: "dive_count",
        field_type: "uint32",
        def_number: 49,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(49, field);

    MessageInfo {
        name: "UserProfile",
        fields: fields,
    }
}

fn hrm_profile_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "enabled",
        field_type: "bool",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "hrm_ant_id",
        field_type: "uint16z",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "log_hrv",
        field_type: "bool",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "hrm_ant_id_trans_type",
        field_type: "uint8z",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    MessageInfo {
        name: "HrmProfile",
        fields: fields,
    }
}

fn sdm_profile_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "enabled",
        field_type: "bool",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "sdm_ant_id",
        field_type: "uint16z",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "sdm_cal_factor",
        field_type: "uint16",
        def_number: 2,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "odometer",
        field_type: "uint32",
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "speed_source",
        field_type: "bool",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "sdm_ant_id_trans_type",
        field_type: "uint8z",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "odometer_rollover",
        field_type: "uint8",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    MessageInfo {
        name: "SdmProfile",
        fields: fields,
    }
}

fn bike_profile_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "sport",
        field_type: "sport",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "sub_sport",
        field_type: "sub_sport",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "odometer",
        field_type: "uint32",
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "bike_spd_ant_id",
        field_type: "uint16z",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "bike_cad_ant_id",
        field_type: "uint16z",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "bike_spdcad_ant_id",
        field_type: "uint16z",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "bike_power_ant_id",
        field_type: "uint16z",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "custom_wheelsize",
        field_type: "uint16",
        def_number: 8,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "auto_wheelsize",
        field_type: "uint16",
        def_number: 9,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "bike_weight",
        field_type: "uint16",
        def_number: 10,
        scale: 10.000000,
        offset: 0.000000,
        units: "kg",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "power_cal_factor",
        field_type: "uint16",
        def_number: 11,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "auto_wheel_cal",
        field_type: "bool",
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(12, field);

    let field = FieldInfo {
        name: "auto_power_zero",
        field_type: "bool",
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(13, field);

    let field = FieldInfo {
        name: "id",
        field_type: "uint8",
        def_number: 14,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(14, field);

    let field = FieldInfo {
        name: "spd_enabled",
        field_type: "bool",
        def_number: 15,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(15, field);

    let field = FieldInfo {
        name: "cad_enabled",
        field_type: "bool",
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(16, field);

    let field = FieldInfo {
        name: "spdcad_enabled",
        field_type: "bool",
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(17, field);

    let field = FieldInfo {
        name: "power_enabled",
        field_type: "bool",
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(18, field);

    let field = FieldInfo {
        name: "crank_length",
        field_type: "uint8",
        def_number: 19,
        scale: 2.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(19, field);

    let field = FieldInfo {
        name: "enabled",
        field_type: "bool",
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(20, field);

    let field = FieldInfo {
        name: "bike_spd_ant_id_trans_type",
        field_type: "uint8z",
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(21, field);

    let field = FieldInfo {
        name: "bike_cad_ant_id_trans_type",
        field_type: "uint8z",
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(22, field);

    let field = FieldInfo {
        name: "bike_spdcad_ant_id_trans_type",
        field_type: "uint8z",
        def_number: 23,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(23, field);

    let field = FieldInfo {
        name: "bike_power_ant_id_trans_type",
        field_type: "uint8z",
        def_number: 24,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(24, field);

    let field = FieldInfo {
        name: "odometer_rollover",
        field_type: "uint8",
        def_number: 37,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(37, field);

    let field = FieldInfo {
        name: "front_gear_num",
        field_type: "uint8z",
        def_number: 38,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(38, field);

    let field = FieldInfo {
        name: "front_gear",
        field_type: "uint8z",
        def_number: 39,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(39, field);

    let field = FieldInfo {
        name: "rear_gear_num",
        field_type: "uint8z",
        def_number: 40,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(40, field);

    let field = FieldInfo {
        name: "rear_gear",
        field_type: "uint8z",
        def_number: 41,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(41, field);

    let field = FieldInfo {
        name: "shimano_di2_enabled",
        field_type: "bool",
        def_number: 44,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(44, field);

    MessageInfo {
        name: "BikeProfile",
        fields: fields,
    }
}

fn connectivity_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "bluetooth_enabled",
        field_type: "bool",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "bluetooth_le_enabled",
        field_type: "bool",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "ant_enabled",
        field_type: "bool",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "live_tracking_enabled",
        field_type: "bool",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "weather_conditions_enabled",
        field_type: "bool",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "weather_alerts_enabled",
        field_type: "bool",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "auto_activity_upload_enabled",
        field_type: "bool",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "course_download_enabled",
        field_type: "bool",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "workout_download_enabled",
        field_type: "bool",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "gps_ephemeris_download_enabled",
        field_type: "bool",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "incident_detection_enabled",
        field_type: "bool",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "grouptrack_enabled",
        field_type: "bool",
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(12, field);

    MessageInfo {
        name: "Connectivity",
        fields: fields,
    }
}

fn watchface_settings_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "mode",
        field_type: "watchface_mode",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "layout",
        field_type: "byte",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    MessageInfo {
        name: "WatchfaceSettings",
        fields: fields,
    }
}

fn ohr_settings_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "enabled",
        field_type: "switch",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    MessageInfo {
        name: "OhrSettings",
        fields: fields,
    }
}

fn zones_target_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "max_heart_rate",
        field_type: "uint8",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "threshold_heart_rate",
        field_type: "uint8",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "functional_threshold_power",
        field_type: "uint16",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "hr_calc_type",
        field_type: "hr_zone_calc",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "pwr_calc_type",
        field_type: "pwr_zone_calc",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    MessageInfo {
        name: "ZonesTarget",
        fields: fields,
    }
}

fn sport_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "sport",
        field_type: "sport",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "sub_sport",
        field_type: "sub_sport",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    MessageInfo {
        name: "Sport",
        fields: fields,
    }
}

fn hr_zone_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "high_bpm",
        field_type: "uint8",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    MessageInfo {
        name: "HrZone",
        fields: fields,
    }
}

fn speed_zone_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "high_value",
        field_type: "uint16",
        def_number: 0,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    MessageInfo {
        name: "SpeedZone",
        fields: fields,
    }
}

fn cadence_zone_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "high_value",
        field_type: "uint8",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    MessageInfo {
        name: "CadenceZone",
        fields: fields,
    }
}

fn power_zone_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "high_value",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    MessageInfo {
        name: "PowerZone",
        fields: fields,
    }
}

fn met_zone_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "high_bpm",
        field_type: "uint8",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "calories",
        field_type: "uint16",
        def_number: 2,
        scale: 10.000000,
        offset: 0.000000,
        units: "kcal / min",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "fat_calories",
        field_type: "uint8",
        def_number: 3,
        scale: 10.000000,
        offset: 0.000000,
        units: "kcal / min",
    };
    fields.insert(3, field);

    MessageInfo {
        name: "MetZone",
        fields: fields,
    }
}

fn dive_settings_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "model",
        field_type: "tissue_model_type",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "gf_low",
        field_type: "uint8",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "gf_high",
        field_type: "uint8",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "water_type",
        field_type: "water_type",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "water_density",
        field_type: "float32",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "kg/m^3",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "po2_warn",
        field_type: "uint8",
        def_number: 6,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "po2_critical",
        field_type: "uint8",
        def_number: 7,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "po2_deco",
        field_type: "uint8",
        def_number: 8,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "safety_stop_enabled",
        field_type: "bool",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "bottom_depth",
        field_type: "float32",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "bottom_time",
        field_type: "uint32",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "apnea_countdown_enabled",
        field_type: "bool",
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(12, field);

    let field = FieldInfo {
        name: "apnea_countdown_time",
        field_type: "uint32",
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(13, field);

    let field = FieldInfo {
        name: "backlight_mode",
        field_type: "dive_backlight_mode",
        def_number: 14,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(14, field);

    let field = FieldInfo {
        name: "backlight_brightness",
        field_type: "uint8",
        def_number: 15,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(15, field);

    let field = FieldInfo {
        name: "backlight_timeout",
        field_type: "backlight_timeout",
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(16, field);

    let field = FieldInfo {
        name: "repeat_dive_interval",
        field_type: "uint16",
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(17, field);

    let field = FieldInfo {
        name: "safety_stop_time",
        field_type: "uint16",
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(18, field);

    let field = FieldInfo {
        name: "heart_rate_source_type",
        field_type: "source_type",
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(19, field);

    let field = FieldInfo {
        name: "heart_rate_source",
        field_type: "uint8",
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(20, field);

    MessageInfo {
        name: "DiveSettings",
        fields: fields,
    }
}

fn dive_alarm_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "depth",
        field_type: "uint32",
        def_number: 0,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "time",
        field_type: "sint32",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "enabled",
        field_type: "bool",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "alarm_type",
        field_type: "dive_alarm_type",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "sound",
        field_type: "tone",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "dive_types",
        field_type: "sub_sport",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    MessageInfo {
        name: "DiveAlarm",
        fields: fields,
    }
}

fn dive_gas_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "helium_content",
        field_type: "uint8",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "oxygen_content",
        field_type: "uint8",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "status",
        field_type: "dive_gas_status",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    MessageInfo {
        name: "DiveGas",
        fields: fields,
    }
}

fn goal_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "sport",
        field_type: "sport",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "sub_sport",
        field_type: "sub_sport",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "start_date",
        field_type: "date_time",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "end_date",
        field_type: "date_time",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "type",
        field_type: "goal",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "value",
        field_type: "uint32",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "repeat",
        field_type: "bool",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "target_value",
        field_type: "uint32",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "recurrence",
        field_type: "goal_recurrence",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "recurrence_value",
        field_type: "uint16",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "enabled",
        field_type: "bool",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "source",
        field_type: "goal_source",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(11, field);

    MessageInfo {
        name: "Goal",
        fields: fields,
    }
}

fn activity_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "total_timer_time",
        field_type: "uint32",
        def_number: 0,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "num_sessions",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "type",
        field_type: "activity",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "event",
        field_type: "event",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "event_type",
        field_type: "event_type",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "local_timestamp",
        field_type: "local_date_time",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "event_group",
        field_type: "uint8",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    MessageInfo {
        name: "Activity",
        fields: fields,
    }
}

fn session_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "event",
        field_type: "event",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "event_type",
        field_type: "event_type",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "start_time",
        field_type: "date_time",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "start_position_lat",
        field_type: "sint32",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "start_position_long",
        field_type: "sint32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "sport",
        field_type: "sport",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "sub_sport",
        field_type: "sub_sport",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "total_elapsed_time",
        field_type: "uint32",
        def_number: 7,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "total_timer_time",
        field_type: "uint32",
        def_number: 8,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "total_distance",
        field_type: "uint32",
        def_number: 9,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "total_cycles",
        field_type: "uint32",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "cycles",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "total_calories",
        field_type: "uint16",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "total_fat_calories",
        field_type: "uint16",
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
    };
    fields.insert(13, field);

    let field = FieldInfo {
        name: "avg_speed",
        field_type: "uint16",
        def_number: 14,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(14, field);

    let field = FieldInfo {
        name: "max_speed",
        field_type: "uint16",
        def_number: 15,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(15, field);

    let field = FieldInfo {
        name: "avg_heart_rate",
        field_type: "uint8",
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(16, field);

    let field = FieldInfo {
        name: "max_heart_rate",
        field_type: "uint8",
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(17, field);

    let field = FieldInfo {
        name: "avg_cadence",
        field_type: "uint8",
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(18, field);

    let field = FieldInfo {
        name: "max_cadence",
        field_type: "uint8",
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(19, field);

    let field = FieldInfo {
        name: "avg_power",
        field_type: "uint16",
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(20, field);

    let field = FieldInfo {
        name: "max_power",
        field_type: "uint16",
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(21, field);

    let field = FieldInfo {
        name: "total_ascent",
        field_type: "uint16",
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(22, field);

    let field = FieldInfo {
        name: "total_descent",
        field_type: "uint16",
        def_number: 23,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(23, field);

    let field = FieldInfo {
        name: "total_training_effect",
        field_type: "uint8",
        def_number: 24,
        scale: 10.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(24, field);

    let field = FieldInfo {
        name: "first_lap_index",
        field_type: "uint16",
        def_number: 25,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(25, field);

    let field = FieldInfo {
        name: "num_laps",
        field_type: "uint16",
        def_number: 26,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(26, field);

    let field = FieldInfo {
        name: "event_group",
        field_type: "uint8",
        def_number: 27,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(27, field);

    let field = FieldInfo {
        name: "trigger",
        field_type: "session_trigger",
        def_number: 28,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(28, field);

    let field = FieldInfo {
        name: "nec_lat",
        field_type: "sint32",
        def_number: 29,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(29, field);

    let field = FieldInfo {
        name: "nec_long",
        field_type: "sint32",
        def_number: 30,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(30, field);

    let field = FieldInfo {
        name: "swc_lat",
        field_type: "sint32",
        def_number: 31,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(31, field);

    let field = FieldInfo {
        name: "swc_long",
        field_type: "sint32",
        def_number: 32,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(32, field);

    let field = FieldInfo {
        name: "normalized_power",
        field_type: "uint16",
        def_number: 34,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(34, field);

    let field = FieldInfo {
        name: "training_stress_score",
        field_type: "uint16",
        def_number: 35,
        scale: 10.000000,
        offset: 0.000000,
        units: "tss",
    };
    fields.insert(35, field);

    let field = FieldInfo {
        name: "intensity_factor",
        field_type: "uint16",
        def_number: 36,
        scale: 1000.000000,
        offset: 0.000000,
        units: "if",
    };
    fields.insert(36, field);

    let field = FieldInfo {
        name: "left_right_balance",
        field_type: "left_right_balance_100",
        def_number: 37,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(37, field);

    let field = FieldInfo {
        name: "avg_stroke_count",
        field_type: "uint32",
        def_number: 41,
        scale: 10.000000,
        offset: 0.000000,
        units: "strokes/lap",
    };
    fields.insert(41, field);

    let field = FieldInfo {
        name: "avg_stroke_distance",
        field_type: "uint16",
        def_number: 42,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(42, field);

    let field = FieldInfo {
        name: "swim_stroke",
        field_type: "swim_stroke",
        def_number: 43,
        scale: 1.000000,
        offset: 0.000000,
        units: "swim_stroke",
    };
    fields.insert(43, field);

    let field = FieldInfo {
        name: "pool_length",
        field_type: "uint16",
        def_number: 44,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(44, field);

    let field = FieldInfo {
        name: "threshold_power",
        field_type: "uint16",
        def_number: 45,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(45, field);

    let field = FieldInfo {
        name: "pool_length_unit",
        field_type: "display_measure",
        def_number: 46,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(46, field);

    let field = FieldInfo {
        name: "num_active_lengths",
        field_type: "uint16",
        def_number: 47,
        scale: 1.000000,
        offset: 0.000000,
        units: "lengths",
    };
    fields.insert(47, field);

    let field = FieldInfo {
        name: "total_work",
        field_type: "uint32",
        def_number: 48,
        scale: 1.000000,
        offset: 0.000000,
        units: "J",
    };
    fields.insert(48, field);

    let field = FieldInfo {
        name: "avg_altitude",
        field_type: "uint16",
        def_number: 49,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(49, field);

    let field = FieldInfo {
        name: "max_altitude",
        field_type: "uint16",
        def_number: 50,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(50, field);

    let field = FieldInfo {
        name: "gps_accuracy",
        field_type: "uint8",
        def_number: 51,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(51, field);

    let field = FieldInfo {
        name: "avg_grade",
        field_type: "sint16",
        def_number: 52,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(52, field);

    let field = FieldInfo {
        name: "avg_pos_grade",
        field_type: "sint16",
        def_number: 53,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(53, field);

    let field = FieldInfo {
        name: "avg_neg_grade",
        field_type: "sint16",
        def_number: 54,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(54, field);

    let field = FieldInfo {
        name: "max_pos_grade",
        field_type: "sint16",
        def_number: 55,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(55, field);

    let field = FieldInfo {
        name: "max_neg_grade",
        field_type: "sint16",
        def_number: 56,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(56, field);

    let field = FieldInfo {
        name: "avg_temperature",
        field_type: "sint8",
        def_number: 57,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(57, field);

    let field = FieldInfo {
        name: "max_temperature",
        field_type: "sint8",
        def_number: 58,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(58, field);

    let field = FieldInfo {
        name: "total_moving_time",
        field_type: "uint32",
        def_number: 59,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(59, field);

    let field = FieldInfo {
        name: "avg_pos_vertical_speed",
        field_type: "sint16",
        def_number: 60,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(60, field);

    let field = FieldInfo {
        name: "avg_neg_vertical_speed",
        field_type: "sint16",
        def_number: 61,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(61, field);

    let field = FieldInfo {
        name: "max_pos_vertical_speed",
        field_type: "sint16",
        def_number: 62,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(62, field);

    let field = FieldInfo {
        name: "max_neg_vertical_speed",
        field_type: "sint16",
        def_number: 63,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(63, field);

    let field = FieldInfo {
        name: "min_heart_rate",
        field_type: "uint8",
        def_number: 64,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(64, field);

    let field = FieldInfo {
        name: "time_in_hr_zone",
        field_type: "uint32",
        def_number: 65,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(65, field);

    let field = FieldInfo {
        name: "time_in_speed_zone",
        field_type: "uint32",
        def_number: 66,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(66, field);

    let field = FieldInfo {
        name: "time_in_cadence_zone",
        field_type: "uint32",
        def_number: 67,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(67, field);

    let field = FieldInfo {
        name: "time_in_power_zone",
        field_type: "uint32",
        def_number: 68,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(68, field);

    let field = FieldInfo {
        name: "avg_lap_time",
        field_type: "uint32",
        def_number: 69,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(69, field);

    let field = FieldInfo {
        name: "best_lap_index",
        field_type: "uint16",
        def_number: 70,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(70, field);

    let field = FieldInfo {
        name: "min_altitude",
        field_type: "uint16",
        def_number: 71,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(71, field);

    let field = FieldInfo {
        name: "player_score",
        field_type: "uint16",
        def_number: 82,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(82, field);

    let field = FieldInfo {
        name: "opponent_score",
        field_type: "uint16",
        def_number: 83,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(83, field);

    let field = FieldInfo {
        name: "opponent_name",
        field_type: "string",
        def_number: 84,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(84, field);

    let field = FieldInfo {
        name: "stroke_count",
        field_type: "uint16",
        def_number: 85,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(85, field);

    let field = FieldInfo {
        name: "zone_count",
        field_type: "uint16",
        def_number: 86,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(86, field);

    let field = FieldInfo {
        name: "max_ball_speed",
        field_type: "uint16",
        def_number: 87,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(87, field);

    let field = FieldInfo {
        name: "avg_ball_speed",
        field_type: "uint16",
        def_number: 88,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(88, field);

    let field = FieldInfo {
        name: "avg_vertical_oscillation",
        field_type: "uint16",
        def_number: 89,
        scale: 10.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(89, field);

    let field = FieldInfo {
        name: "avg_stance_time_percent",
        field_type: "uint16",
        def_number: 90,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(90, field);

    let field = FieldInfo {
        name: "avg_stance_time",
        field_type: "uint16",
        def_number: 91,
        scale: 10.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(91, field);

    let field = FieldInfo {
        name: "avg_fractional_cadence",
        field_type: "uint8",
        def_number: 92,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(92, field);

    let field = FieldInfo {
        name: "max_fractional_cadence",
        field_type: "uint8",
        def_number: 93,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(93, field);

    let field = FieldInfo {
        name: "total_fractional_cycles",
        field_type: "uint8",
        def_number: 94,
        scale: 128.000000,
        offset: 0.000000,
        units: "cycles",
    };
    fields.insert(94, field);

    let field = FieldInfo {
        name: "avg_total_hemoglobin_conc",
        field_type: "uint16",
        def_number: 95,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
    };
    fields.insert(95, field);

    let field = FieldInfo {
        name: "min_total_hemoglobin_conc",
        field_type: "uint16",
        def_number: 96,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
    };
    fields.insert(96, field);

    let field = FieldInfo {
        name: "max_total_hemoglobin_conc",
        field_type: "uint16",
        def_number: 97,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
    };
    fields.insert(97, field);

    let field = FieldInfo {
        name: "avg_saturated_hemoglobin_percent",
        field_type: "uint16",
        def_number: 98,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(98, field);

    let field = FieldInfo {
        name: "min_saturated_hemoglobin_percent",
        field_type: "uint16",
        def_number: 99,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(99, field);

    let field = FieldInfo {
        name: "max_saturated_hemoglobin_percent",
        field_type: "uint16",
        def_number: 100,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(100, field);

    let field = FieldInfo {
        name: "avg_left_torque_effectiveness",
        field_type: "uint8",
        def_number: 101,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(101, field);

    let field = FieldInfo {
        name: "avg_right_torque_effectiveness",
        field_type: "uint8",
        def_number: 102,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(102, field);

    let field = FieldInfo {
        name: "avg_left_pedal_smoothness",
        field_type: "uint8",
        def_number: 103,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(103, field);

    let field = FieldInfo {
        name: "avg_right_pedal_smoothness",
        field_type: "uint8",
        def_number: 104,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(104, field);

    let field = FieldInfo {
        name: "avg_combined_pedal_smoothness",
        field_type: "uint8",
        def_number: 105,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(105, field);

    let field = FieldInfo {
        name: "sport_index",
        field_type: "uint8",
        def_number: 111,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(111, field);

    let field = FieldInfo {
        name: "time_standing",
        field_type: "uint32",
        def_number: 112,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(112, field);

    let field = FieldInfo {
        name: "stand_count",
        field_type: "uint16",
        def_number: 113,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(113, field);

    let field = FieldInfo {
        name: "avg_left_pco",
        field_type: "sint8",
        def_number: 114,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(114, field);

    let field = FieldInfo {
        name: "avg_right_pco",
        field_type: "sint8",
        def_number: 115,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(115, field);

    let field = FieldInfo {
        name: "avg_left_power_phase",
        field_type: "uint8",
        def_number: 116,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(116, field);

    let field = FieldInfo {
        name: "avg_left_power_phase_peak",
        field_type: "uint8",
        def_number: 117,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(117, field);

    let field = FieldInfo {
        name: "avg_right_power_phase",
        field_type: "uint8",
        def_number: 118,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(118, field);

    let field = FieldInfo {
        name: "avg_right_power_phase_peak",
        field_type: "uint8",
        def_number: 119,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(119, field);

    let field = FieldInfo {
        name: "avg_power_position",
        field_type: "uint16",
        def_number: 120,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(120, field);

    let field = FieldInfo {
        name: "max_power_position",
        field_type: "uint16",
        def_number: 121,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(121, field);

    let field = FieldInfo {
        name: "avg_cadence_position",
        field_type: "uint8",
        def_number: 122,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(122, field);

    let field = FieldInfo {
        name: "max_cadence_position",
        field_type: "uint8",
        def_number: 123,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(123, field);

    let field = FieldInfo {
        name: "enhanced_avg_speed",
        field_type: "uint32",
        def_number: 124,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(124, field);

    let field = FieldInfo {
        name: "enhanced_max_speed",
        field_type: "uint32",
        def_number: 125,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(125, field);

    let field = FieldInfo {
        name: "enhanced_avg_altitude",
        field_type: "uint32",
        def_number: 126,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(126, field);

    let field = FieldInfo {
        name: "enhanced_min_altitude",
        field_type: "uint32",
        def_number: 127,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(127, field);

    let field = FieldInfo {
        name: "enhanced_max_altitude",
        field_type: "uint32",
        def_number: 128,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(128, field);

    let field = FieldInfo {
        name: "avg_lev_motor_power",
        field_type: "uint16",
        def_number: 129,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(129, field);

    let field = FieldInfo {
        name: "max_lev_motor_power",
        field_type: "uint16",
        def_number: 130,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(130, field);

    let field = FieldInfo {
        name: "lev_battery_consumption",
        field_type: "uint8",
        def_number: 131,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(131, field);

    let field = FieldInfo {
        name: "avg_vertical_ratio",
        field_type: "uint16",
        def_number: 132,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(132, field);

    let field = FieldInfo {
        name: "avg_stance_time_balance",
        field_type: "uint16",
        def_number: 133,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(133, field);

    let field = FieldInfo {
        name: "avg_step_length",
        field_type: "uint16",
        def_number: 134,
        scale: 10.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(134, field);

    let field = FieldInfo {
        name: "total_anaerobic_training_effect",
        field_type: "uint8",
        def_number: 137,
        scale: 10.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(137, field);

    let field = FieldInfo {
        name: "avg_vam",
        field_type: "uint16",
        def_number: 139,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(139, field);

    let field = FieldInfo {
        name: "total_grit",
        field_type: "float32",
        def_number: 181,
        scale: 1.000000,
        offset: 0.000000,
        units: "kGrit",
    };
    fields.insert(181, field);

    let field = FieldInfo {
        name: "total_flow",
        field_type: "float32",
        def_number: 182,
        scale: 1.000000,
        offset: 0.000000,
        units: "Flow",
    };
    fields.insert(182, field);

    let field = FieldInfo {
        name: "jump_count",
        field_type: "uint16",
        def_number: 183,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(183, field);

    let field = FieldInfo {
        name: "avg_grit",
        field_type: "float32",
        def_number: 186,
        scale: 1.000000,
        offset: 0.000000,
        units: "kGrit",
    };
    fields.insert(186, field);

    let field = FieldInfo {
        name: "avg_flow",
        field_type: "float32",
        def_number: 187,
        scale: 1.000000,
        offset: 0.000000,
        units: "Flow",
    };
    fields.insert(187, field);

    MessageInfo {
        name: "Session",
        fields: fields,
    }
}

fn lap_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "event",
        field_type: "event",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "event_type",
        field_type: "event_type",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "start_time",
        field_type: "date_time",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "start_position_lat",
        field_type: "sint32",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "start_position_long",
        field_type: "sint32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "end_position_lat",
        field_type: "sint32",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "end_position_long",
        field_type: "sint32",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "total_elapsed_time",
        field_type: "uint32",
        def_number: 7,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "total_timer_time",
        field_type: "uint32",
        def_number: 8,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "total_distance",
        field_type: "uint32",
        def_number: 9,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "total_cycles",
        field_type: "uint32",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "cycles",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "total_calories",
        field_type: "uint16",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "total_fat_calories",
        field_type: "uint16",
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
    };
    fields.insert(12, field);

    let field = FieldInfo {
        name: "avg_speed",
        field_type: "uint16",
        def_number: 13,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(13, field);

    let field = FieldInfo {
        name: "max_speed",
        field_type: "uint16",
        def_number: 14,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(14, field);

    let field = FieldInfo {
        name: "avg_heart_rate",
        field_type: "uint8",
        def_number: 15,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(15, field);

    let field = FieldInfo {
        name: "max_heart_rate",
        field_type: "uint8",
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(16, field);

    let field = FieldInfo {
        name: "avg_cadence",
        field_type: "uint8",
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(17, field);

    let field = FieldInfo {
        name: "max_cadence",
        field_type: "uint8",
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(18, field);

    let field = FieldInfo {
        name: "avg_power",
        field_type: "uint16",
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(19, field);

    let field = FieldInfo {
        name: "max_power",
        field_type: "uint16",
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(20, field);

    let field = FieldInfo {
        name: "total_ascent",
        field_type: "uint16",
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(21, field);

    let field = FieldInfo {
        name: "total_descent",
        field_type: "uint16",
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(22, field);

    let field = FieldInfo {
        name: "intensity",
        field_type: "intensity",
        def_number: 23,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(23, field);

    let field = FieldInfo {
        name: "lap_trigger",
        field_type: "lap_trigger",
        def_number: 24,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(24, field);

    let field = FieldInfo {
        name: "sport",
        field_type: "sport",
        def_number: 25,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(25, field);

    let field = FieldInfo {
        name: "event_group",
        field_type: "uint8",
        def_number: 26,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(26, field);

    let field = FieldInfo {
        name: "num_lengths",
        field_type: "uint16",
        def_number: 32,
        scale: 1.000000,
        offset: 0.000000,
        units: "lengths",
    };
    fields.insert(32, field);

    let field = FieldInfo {
        name: "normalized_power",
        field_type: "uint16",
        def_number: 33,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(33, field);

    let field = FieldInfo {
        name: "left_right_balance",
        field_type: "left_right_balance_100",
        def_number: 34,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(34, field);

    let field = FieldInfo {
        name: "first_length_index",
        field_type: "uint16",
        def_number: 35,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(35, field);

    let field = FieldInfo {
        name: "avg_stroke_distance",
        field_type: "uint16",
        def_number: 37,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(37, field);

    let field = FieldInfo {
        name: "swim_stroke",
        field_type: "swim_stroke",
        def_number: 38,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(38, field);

    let field = FieldInfo {
        name: "sub_sport",
        field_type: "sub_sport",
        def_number: 39,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(39, field);

    let field = FieldInfo {
        name: "num_active_lengths",
        field_type: "uint16",
        def_number: 40,
        scale: 1.000000,
        offset: 0.000000,
        units: "lengths",
    };
    fields.insert(40, field);

    let field = FieldInfo {
        name: "total_work",
        field_type: "uint32",
        def_number: 41,
        scale: 1.000000,
        offset: 0.000000,
        units: "J",
    };
    fields.insert(41, field);

    let field = FieldInfo {
        name: "avg_altitude",
        field_type: "uint16",
        def_number: 42,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(42, field);

    let field = FieldInfo {
        name: "max_altitude",
        field_type: "uint16",
        def_number: 43,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(43, field);

    let field = FieldInfo {
        name: "gps_accuracy",
        field_type: "uint8",
        def_number: 44,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(44, field);

    let field = FieldInfo {
        name: "avg_grade",
        field_type: "sint16",
        def_number: 45,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(45, field);

    let field = FieldInfo {
        name: "avg_pos_grade",
        field_type: "sint16",
        def_number: 46,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(46, field);

    let field = FieldInfo {
        name: "avg_neg_grade",
        field_type: "sint16",
        def_number: 47,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(47, field);

    let field = FieldInfo {
        name: "max_pos_grade",
        field_type: "sint16",
        def_number: 48,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(48, field);

    let field = FieldInfo {
        name: "max_neg_grade",
        field_type: "sint16",
        def_number: 49,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(49, field);

    let field = FieldInfo {
        name: "avg_temperature",
        field_type: "sint8",
        def_number: 50,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(50, field);

    let field = FieldInfo {
        name: "max_temperature",
        field_type: "sint8",
        def_number: 51,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(51, field);

    let field = FieldInfo {
        name: "total_moving_time",
        field_type: "uint32",
        def_number: 52,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(52, field);

    let field = FieldInfo {
        name: "avg_pos_vertical_speed",
        field_type: "sint16",
        def_number: 53,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(53, field);

    let field = FieldInfo {
        name: "avg_neg_vertical_speed",
        field_type: "sint16",
        def_number: 54,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(54, field);

    let field = FieldInfo {
        name: "max_pos_vertical_speed",
        field_type: "sint16",
        def_number: 55,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(55, field);

    let field = FieldInfo {
        name: "max_neg_vertical_speed",
        field_type: "sint16",
        def_number: 56,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(56, field);

    let field = FieldInfo {
        name: "time_in_hr_zone",
        field_type: "uint32",
        def_number: 57,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(57, field);

    let field = FieldInfo {
        name: "time_in_speed_zone",
        field_type: "uint32",
        def_number: 58,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(58, field);

    let field = FieldInfo {
        name: "time_in_cadence_zone",
        field_type: "uint32",
        def_number: 59,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(59, field);

    let field = FieldInfo {
        name: "time_in_power_zone",
        field_type: "uint32",
        def_number: 60,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(60, field);

    let field = FieldInfo {
        name: "repetition_num",
        field_type: "uint16",
        def_number: 61,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(61, field);

    let field = FieldInfo {
        name: "min_altitude",
        field_type: "uint16",
        def_number: 62,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(62, field);

    let field = FieldInfo {
        name: "min_heart_rate",
        field_type: "uint8",
        def_number: 63,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(63, field);

    let field = FieldInfo {
        name: "wkt_step_index",
        field_type: "message_index",
        def_number: 71,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(71, field);

    let field = FieldInfo {
        name: "opponent_score",
        field_type: "uint16",
        def_number: 74,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(74, field);

    let field = FieldInfo {
        name: "stroke_count",
        field_type: "uint16",
        def_number: 75,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(75, field);

    let field = FieldInfo {
        name: "zone_count",
        field_type: "uint16",
        def_number: 76,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(76, field);

    let field = FieldInfo {
        name: "avg_vertical_oscillation",
        field_type: "uint16",
        def_number: 77,
        scale: 10.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(77, field);

    let field = FieldInfo {
        name: "avg_stance_time_percent",
        field_type: "uint16",
        def_number: 78,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(78, field);

    let field = FieldInfo {
        name: "avg_stance_time",
        field_type: "uint16",
        def_number: 79,
        scale: 10.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(79, field);

    let field = FieldInfo {
        name: "avg_fractional_cadence",
        field_type: "uint8",
        def_number: 80,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(80, field);

    let field = FieldInfo {
        name: "max_fractional_cadence",
        field_type: "uint8",
        def_number: 81,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(81, field);

    let field = FieldInfo {
        name: "total_fractional_cycles",
        field_type: "uint8",
        def_number: 82,
        scale: 128.000000,
        offset: 0.000000,
        units: "cycles",
    };
    fields.insert(82, field);

    let field = FieldInfo {
        name: "player_score",
        field_type: "uint16",
        def_number: 83,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(83, field);

    let field = FieldInfo {
        name: "avg_total_hemoglobin_conc",
        field_type: "uint16",
        def_number: 84,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
    };
    fields.insert(84, field);

    let field = FieldInfo {
        name: "min_total_hemoglobin_conc",
        field_type: "uint16",
        def_number: 85,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
    };
    fields.insert(85, field);

    let field = FieldInfo {
        name: "max_total_hemoglobin_conc",
        field_type: "uint16",
        def_number: 86,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
    };
    fields.insert(86, field);

    let field = FieldInfo {
        name: "avg_saturated_hemoglobin_percent",
        field_type: "uint16",
        def_number: 87,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(87, field);

    let field = FieldInfo {
        name: "min_saturated_hemoglobin_percent",
        field_type: "uint16",
        def_number: 88,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(88, field);

    let field = FieldInfo {
        name: "max_saturated_hemoglobin_percent",
        field_type: "uint16",
        def_number: 89,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(89, field);

    let field = FieldInfo {
        name: "avg_left_torque_effectiveness",
        field_type: "uint8",
        def_number: 91,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(91, field);

    let field = FieldInfo {
        name: "avg_right_torque_effectiveness",
        field_type: "uint8",
        def_number: 92,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(92, field);

    let field = FieldInfo {
        name: "avg_left_pedal_smoothness",
        field_type: "uint8",
        def_number: 93,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(93, field);

    let field = FieldInfo {
        name: "avg_right_pedal_smoothness",
        field_type: "uint8",
        def_number: 94,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(94, field);

    let field = FieldInfo {
        name: "avg_combined_pedal_smoothness",
        field_type: "uint8",
        def_number: 95,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(95, field);

    let field = FieldInfo {
        name: "time_standing",
        field_type: "uint32",
        def_number: 98,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(98, field);

    let field = FieldInfo {
        name: "stand_count",
        field_type: "uint16",
        def_number: 99,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(99, field);

    let field = FieldInfo {
        name: "avg_left_pco",
        field_type: "sint8",
        def_number: 100,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(100, field);

    let field = FieldInfo {
        name: "avg_right_pco",
        field_type: "sint8",
        def_number: 101,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(101, field);

    let field = FieldInfo {
        name: "avg_left_power_phase",
        field_type: "uint8",
        def_number: 102,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(102, field);

    let field = FieldInfo {
        name: "avg_left_power_phase_peak",
        field_type: "uint8",
        def_number: 103,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(103, field);

    let field = FieldInfo {
        name: "avg_right_power_phase",
        field_type: "uint8",
        def_number: 104,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(104, field);

    let field = FieldInfo {
        name: "avg_right_power_phase_peak",
        field_type: "uint8",
        def_number: 105,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(105, field);

    let field = FieldInfo {
        name: "avg_power_position",
        field_type: "uint16",
        def_number: 106,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(106, field);

    let field = FieldInfo {
        name: "max_power_position",
        field_type: "uint16",
        def_number: 107,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(107, field);

    let field = FieldInfo {
        name: "avg_cadence_position",
        field_type: "uint8",
        def_number: 108,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(108, field);

    let field = FieldInfo {
        name: "max_cadence_position",
        field_type: "uint8",
        def_number: 109,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(109, field);

    let field = FieldInfo {
        name: "enhanced_avg_speed",
        field_type: "uint32",
        def_number: 110,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(110, field);

    let field = FieldInfo {
        name: "enhanced_max_speed",
        field_type: "uint32",
        def_number: 111,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(111, field);

    let field = FieldInfo {
        name: "enhanced_avg_altitude",
        field_type: "uint32",
        def_number: 112,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(112, field);

    let field = FieldInfo {
        name: "enhanced_min_altitude",
        field_type: "uint32",
        def_number: 113,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(113, field);

    let field = FieldInfo {
        name: "enhanced_max_altitude",
        field_type: "uint32",
        def_number: 114,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(114, field);

    let field = FieldInfo {
        name: "avg_lev_motor_power",
        field_type: "uint16",
        def_number: 115,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(115, field);

    let field = FieldInfo {
        name: "max_lev_motor_power",
        field_type: "uint16",
        def_number: 116,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(116, field);

    let field = FieldInfo {
        name: "lev_battery_consumption",
        field_type: "uint8",
        def_number: 117,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(117, field);

    let field = FieldInfo {
        name: "avg_vertical_ratio",
        field_type: "uint16",
        def_number: 118,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(118, field);

    let field = FieldInfo {
        name: "avg_stance_time_balance",
        field_type: "uint16",
        def_number: 119,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(119, field);

    let field = FieldInfo {
        name: "avg_step_length",
        field_type: "uint16",
        def_number: 120,
        scale: 10.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(120, field);

    let field = FieldInfo {
        name: "avg_vam",
        field_type: "uint16",
        def_number: 121,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(121, field);

    let field = FieldInfo {
        name: "total_grit",
        field_type: "float32",
        def_number: 149,
        scale: 1.000000,
        offset: 0.000000,
        units: "kGrit",
    };
    fields.insert(149, field);

    let field = FieldInfo {
        name: "total_flow",
        field_type: "float32",
        def_number: 150,
        scale: 1.000000,
        offset: 0.000000,
        units: "Flow",
    };
    fields.insert(150, field);

    let field = FieldInfo {
        name: "jump_count",
        field_type: "uint16",
        def_number: 151,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(151, field);

    let field = FieldInfo {
        name: "avg_grit",
        field_type: "float32",
        def_number: 153,
        scale: 1.000000,
        offset: 0.000000,
        units: "kGrit",
    };
    fields.insert(153, field);

    let field = FieldInfo {
        name: "avg_flow",
        field_type: "float32",
        def_number: 154,
        scale: 1.000000,
        offset: 0.000000,
        units: "Flow",
    };
    fields.insert(154, field);

    MessageInfo {
        name: "Lap",
        fields: fields,
    }
}

fn length_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "event",
        field_type: "event",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "event_type",
        field_type: "event_type",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "start_time",
        field_type: "date_time",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "total_elapsed_time",
        field_type: "uint32",
        def_number: 3,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "total_timer_time",
        field_type: "uint32",
        def_number: 4,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "total_strokes",
        field_type: "uint16",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "avg_speed",
        field_type: "uint16",
        def_number: 6,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "swim_stroke",
        field_type: "swim_stroke",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "swim_stroke",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "avg_swimming_cadence",
        field_type: "uint8",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes/min",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "event_group",
        field_type: "uint8",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "total_calories",
        field_type: "uint16",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "length_type",
        field_type: "length_type",
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(12, field);

    let field = FieldInfo {
        name: "player_score",
        field_type: "uint16",
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(18, field);

    let field = FieldInfo {
        name: "opponent_score",
        field_type: "uint16",
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(19, field);

    let field = FieldInfo {
        name: "stroke_count",
        field_type: "uint16",
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(20, field);

    let field = FieldInfo {
        name: "zone_count",
        field_type: "uint16",
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(21, field);

    MessageInfo {
        name: "Length",
        fields: fields,
    }
}

fn record_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "position_lat",
        field_type: "sint32",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "position_long",
        field_type: "sint32",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "altitude",
        field_type: "uint16",
        def_number: 2,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "heart_rate",
        field_type: "uint8",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "cadence",
        field_type: "uint8",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "distance",
        field_type: "uint32",
        def_number: 5,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "speed",
        field_type: "uint16",
        def_number: 6,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "power",
        field_type: "uint16",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "compressed_speed_distance",
        field_type: "byte",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "m/s,
m",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "grade",
        field_type: "sint16",
        def_number: 9,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "resistance",
        field_type: "uint8",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "time_from_course",
        field_type: "sint32",
        def_number: 11,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "cycle_length",
        field_type: "uint8",
        def_number: 12,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(12, field);

    let field = FieldInfo {
        name: "temperature",
        field_type: "sint8",
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(13, field);

    let field = FieldInfo {
        name: "speed_1s",
        field_type: "uint8",
        def_number: 17,
        scale: 16.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(17, field);

    let field = FieldInfo {
        name: "cycles",
        field_type: "uint8",
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "cycles",
    };
    fields.insert(18, field);

    let field = FieldInfo {
        name: "total_cycles",
        field_type: "uint32",
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "cycles",
    };
    fields.insert(19, field);

    let field = FieldInfo {
        name: "compressed_accumulated_power",
        field_type: "uint16",
        def_number: 28,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(28, field);

    let field = FieldInfo {
        name: "accumulated_power",
        field_type: "uint32",
        def_number: 29,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(29, field);

    let field = FieldInfo {
        name: "left_right_balance",
        field_type: "left_right_balance",
        def_number: 30,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(30, field);

    let field = FieldInfo {
        name: "gps_accuracy",
        field_type: "uint8",
        def_number: 31,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(31, field);

    let field = FieldInfo {
        name: "vertical_speed",
        field_type: "sint16",
        def_number: 32,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(32, field);

    let field = FieldInfo {
        name: "calories",
        field_type: "uint16",
        def_number: 33,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
    };
    fields.insert(33, field);

    let field = FieldInfo {
        name: "vertical_oscillation",
        field_type: "uint16",
        def_number: 39,
        scale: 10.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(39, field);

    let field = FieldInfo {
        name: "stance_time_percent",
        field_type: "uint16",
        def_number: 40,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(40, field);

    let field = FieldInfo {
        name: "stance_time",
        field_type: "uint16",
        def_number: 41,
        scale: 10.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(41, field);

    let field = FieldInfo {
        name: "activity_type",
        field_type: "activity_type",
        def_number: 42,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(42, field);

    let field = FieldInfo {
        name: "left_torque_effectiveness",
        field_type: "uint8",
        def_number: 43,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(43, field);

    let field = FieldInfo {
        name: "right_torque_effectiveness",
        field_type: "uint8",
        def_number: 44,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(44, field);

    let field = FieldInfo {
        name: "left_pedal_smoothness",
        field_type: "uint8",
        def_number: 45,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(45, field);

    let field = FieldInfo {
        name: "right_pedal_smoothness",
        field_type: "uint8",
        def_number: 46,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(46, field);

    let field = FieldInfo {
        name: "combined_pedal_smoothness",
        field_type: "uint8",
        def_number: 47,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(47, field);

    let field = FieldInfo {
        name: "time128",
        field_type: "uint8",
        def_number: 48,
        scale: 128.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(48, field);

    let field = FieldInfo {
        name: "stroke_type",
        field_type: "stroke_type",
        def_number: 49,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(49, field);

    let field = FieldInfo {
        name: "zone",
        field_type: "uint8",
        def_number: 50,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(50, field);

    let field = FieldInfo {
        name: "ball_speed",
        field_type: "uint16",
        def_number: 51,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(51, field);

    let field = FieldInfo {
        name: "cadence256",
        field_type: "uint16",
        def_number: 52,
        scale: 256.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(52, field);

    let field = FieldInfo {
        name: "fractional_cadence",
        field_type: "uint8",
        def_number: 53,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(53, field);

    let field = FieldInfo {
        name: "total_hemoglobin_conc",
        field_type: "uint16",
        def_number: 54,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
    };
    fields.insert(54, field);

    let field = FieldInfo {
        name: "total_hemoglobin_conc_min",
        field_type: "uint16",
        def_number: 55,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
    };
    fields.insert(55, field);

    let field = FieldInfo {
        name: "total_hemoglobin_conc_max",
        field_type: "uint16",
        def_number: 56,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
    };
    fields.insert(56, field);

    let field = FieldInfo {
        name: "saturated_hemoglobin_percent",
        field_type: "uint16",
        def_number: 57,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(57, field);

    let field = FieldInfo {
        name: "saturated_hemoglobin_percent_min",
        field_type: "uint16",
        def_number: 58,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(58, field);

    let field = FieldInfo {
        name: "saturated_hemoglobin_percent_max",
        field_type: "uint16",
        def_number: 59,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(59, field);

    let field = FieldInfo {
        name: "device_index",
        field_type: "device_index",
        def_number: 62,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(62, field);

    let field = FieldInfo {
        name: "left_pco",
        field_type: "sint8",
        def_number: 67,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(67, field);

    let field = FieldInfo {
        name: "right_pco",
        field_type: "sint8",
        def_number: 68,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(68, field);

    let field = FieldInfo {
        name: "left_power_phase",
        field_type: "uint8",
        def_number: 69,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(69, field);

    let field = FieldInfo {
        name: "left_power_phase_peak",
        field_type: "uint8",
        def_number: 70,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(70, field);

    let field = FieldInfo {
        name: "right_power_phase",
        field_type: "uint8",
        def_number: 71,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(71, field);

    let field = FieldInfo {
        name: "right_power_phase_peak",
        field_type: "uint8",
        def_number: 72,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(72, field);

    let field = FieldInfo {
        name: "enhanced_speed",
        field_type: "uint32",
        def_number: 73,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(73, field);

    let field = FieldInfo {
        name: "enhanced_altitude",
        field_type: "uint32",
        def_number: 78,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(78, field);

    let field = FieldInfo {
        name: "battery_soc",
        field_type: "uint8",
        def_number: 81,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(81, field);

    let field = FieldInfo {
        name: "motor_power",
        field_type: "uint16",
        def_number: 82,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(82, field);

    let field = FieldInfo {
        name: "vertical_ratio",
        field_type: "uint16",
        def_number: 83,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(83, field);

    let field = FieldInfo {
        name: "stance_time_balance",
        field_type: "uint16",
        def_number: 84,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(84, field);

    let field = FieldInfo {
        name: "step_length",
        field_type: "uint16",
        def_number: 85,
        scale: 10.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(85, field);

    let field = FieldInfo {
        name: "absolute_pressure",
        field_type: "uint32",
        def_number: 91,
        scale: 1.000000,
        offset: 0.000000,
        units: "Pa",
    };
    fields.insert(91, field);

    let field = FieldInfo {
        name: "depth",
        field_type: "uint32",
        def_number: 92,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(92, field);

    let field = FieldInfo {
        name: "next_stop_depth",
        field_type: "uint32",
        def_number: 93,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(93, field);

    let field = FieldInfo {
        name: "next_stop_time",
        field_type: "uint32",
        def_number: 94,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(94, field);

    let field = FieldInfo {
        name: "time_to_surface",
        field_type: "uint32",
        def_number: 95,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(95, field);

    let field = FieldInfo {
        name: "ndl_time",
        field_type: "uint32",
        def_number: 96,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(96, field);

    let field = FieldInfo {
        name: "cns_load",
        field_type: "uint8",
        def_number: 97,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(97, field);

    let field = FieldInfo {
        name: "n2_load",
        field_type: "uint16",
        def_number: 98,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(98, field);

    let field = FieldInfo {
        name: "grit",
        field_type: "float32",
        def_number: 114,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(114, field);

    let field = FieldInfo {
        name: "flow",
        field_type: "float32",
        def_number: 115,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(115, field);

    let field = FieldInfo {
        name: "ebike_travel_range",
        field_type: "uint16",
        def_number: 117,
        scale: 1.000000,
        offset: 0.000000,
        units: "km",
    };
    fields.insert(117, field);

    let field = FieldInfo {
        name: "ebike_battery_level",
        field_type: "uint8",
        def_number: 118,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(118, field);

    let field = FieldInfo {
        name: "ebike_assist_mode",
        field_type: "uint8",
        def_number: 119,
        scale: 1.000000,
        offset: 0.000000,
        units: "depends on sensor",
    };
    fields.insert(119, field);

    let field = FieldInfo {
        name: "ebike_assist_level_percent",
        field_type: "uint8",
        def_number: 120,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(120, field);

    MessageInfo {
        name: "Record",
        fields: fields,
    }
}

fn event_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "event",
        field_type: "event",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "event_type",
        field_type: "event_type",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "data16",
        field_type: "uint16",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "data",
        field_type: "uint32",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "event_group",
        field_type: "uint8",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "score",
        field_type: "uint16",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "opponent_score",
        field_type: "uint16",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "front_gear_num",
        field_type: "uint8z",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "front_gear",
        field_type: "uint8z",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "rear_gear_num",
        field_type: "uint8z",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "rear_gear",
        field_type: "uint8z",
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(12, field);

    let field = FieldInfo {
        name: "device_index",
        field_type: "device_index",
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(13, field);

    MessageInfo {
        name: "Event",
        fields: fields,
    }
}

fn device_info_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "device_index",
        field_type: "device_index",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "device_type",
        field_type: "uint8",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "manufacturer",
        field_type: "manufacturer",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "serial_number",
        field_type: "uint32z",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "product",
        field_type: "uint16",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "software_version",
        field_type: "uint16",
        def_number: 5,
        scale: 100.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "hardware_version",
        field_type: "uint8",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "cum_operating_time",
        field_type: "uint32",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "battery_voltage",
        field_type: "uint16",
        def_number: 10,
        scale: 256.000000,
        offset: 0.000000,
        units: "V",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "battery_status",
        field_type: "battery_status",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "sensor_position",
        field_type: "body_location",
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(18, field);

    let field = FieldInfo {
        name: "descriptor",
        field_type: "string",
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(19, field);

    let field = FieldInfo {
        name: "ant_transmission_type",
        field_type: "uint8z",
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(20, field);

    let field = FieldInfo {
        name: "ant_device_number",
        field_type: "uint16z",
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(21, field);

    let field = FieldInfo {
        name: "ant_network",
        field_type: "ant_network",
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(22, field);

    let field = FieldInfo {
        name: "source_type",
        field_type: "source_type",
        def_number: 25,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(25, field);

    let field = FieldInfo {
        name: "product_name",
        field_type: "string",
        def_number: 27,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(27, field);

    MessageInfo {
        name: "DeviceInfo",
        fields: fields,
    }
}

fn training_file_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "type",
        field_type: "file",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "manufacturer",
        field_type: "manufacturer",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "product",
        field_type: "uint16",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "serial_number",
        field_type: "uint32z",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "time_created",
        field_type: "date_time",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    MessageInfo {
        name: "TrainingFile",
        fields: fields,
    }
}

fn hrv_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "time",
        field_type: "uint16",
        def_number: 0,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(0, field);

    MessageInfo {
        name: "Hrv",
        fields: fields,
    }
}

fn weather_conditions_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "weather_report",
        field_type: "weather_report",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "temperature",
        field_type: "sint8",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "condition",
        field_type: "weather_status",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "wind_direction",
        field_type: "uint16",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "wind_speed",
        field_type: "uint16",
        def_number: 4,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "precipitation_probability",
        field_type: "uint8",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "temperature_feels_like",
        field_type: "sint8",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "relative_humidity",
        field_type: "uint8",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "location",
        field_type: "string",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "observed_at_time",
        field_type: "date_time",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "observed_location_lat",
        field_type: "sint32",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "observed_location_long",
        field_type: "sint32",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "day_of_week",
        field_type: "day_of_week",
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(12, field);

    let field = FieldInfo {
        name: "high_temperature",
        field_type: "sint8",
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(13, field);

    let field = FieldInfo {
        name: "low_temperature",
        field_type: "sint8",
        def_number: 14,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(14, field);

    MessageInfo {
        name: "WeatherConditions",
        fields: fields,
    }
}

fn weather_alert_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "report_id",
        field_type: "string",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "issue_time",
        field_type: "date_time",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "expire_time",
        field_type: "date_time",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "severity",
        field_type: "weather_severity",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "type",
        field_type: "weather_severe_type",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    MessageInfo {
        name: "WeatherAlert",
        fields: fields,
    }
}

fn gps_metadata_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "position_lat",
        field_type: "sint32",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "position_long",
        field_type: "sint32",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "enhanced_altitude",
        field_type: "uint32",
        def_number: 3,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "enhanced_speed",
        field_type: "uint32",
        def_number: 4,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "heading",
        field_type: "uint16",
        def_number: 5,
        scale: 100.000000,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "utc_timestamp",
        field_type: "date_time",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "velocity",
        field_type: "sint16",
        def_number: 7,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(7, field);

    MessageInfo {
        name: "GpsMetadata",
        fields: fields,
    }
}

fn camera_event_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "camera_event_type",
        field_type: "camera_event_type",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "camera_file_uuid",
        field_type: "string",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "camera_orientation",
        field_type: "camera_orientation_type",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    MessageInfo {
        name: "CameraEvent",
        fields: fields,
    }
}

fn gyroscope_data_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "sample_time_offset",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "gyro_x",
        field_type: "uint16",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "gyro_y",
        field_type: "uint16",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "gyro_z",
        field_type: "uint16",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "calibrated_gyro_x",
        field_type: "float32",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "deg/s",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "calibrated_gyro_y",
        field_type: "float32",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "deg/s",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "calibrated_gyro_z",
        field_type: "float32",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "deg/s",
    };
    fields.insert(7, field);

    MessageInfo {
        name: "GyroscopeData",
        fields: fields,
    }
}

fn accelerometer_data_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "sample_time_offset",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "accel_x",
        field_type: "uint16",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "accel_y",
        field_type: "uint16",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "accel_z",
        field_type: "uint16",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "calibrated_accel_x",
        field_type: "float32",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "g",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "calibrated_accel_y",
        field_type: "float32",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "g",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "calibrated_accel_z",
        field_type: "float32",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "g",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "compressed_calibrated_accel_x",
        field_type: "sint16",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "mG",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "compressed_calibrated_accel_y",
        field_type: "sint16",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "mG",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "compressed_calibrated_accel_z",
        field_type: "sint16",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "mG",
    };
    fields.insert(10, field);

    MessageInfo {
        name: "AccelerometerData",
        fields: fields,
    }
}

fn magnetometer_data_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "sample_time_offset",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "mag_x",
        field_type: "uint16",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "mag_y",
        field_type: "uint16",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "mag_z",
        field_type: "uint16",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "calibrated_mag_x",
        field_type: "float32",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "G",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "calibrated_mag_y",
        field_type: "float32",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "G",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "calibrated_mag_z",
        field_type: "float32",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "G",
    };
    fields.insert(7, field);

    MessageInfo {
        name: "MagnetometerData",
        fields: fields,
    }
}

fn barometer_data_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "sample_time_offset",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "baro_pres",
        field_type: "uint32",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "Pa",
    };
    fields.insert(2, field);

    MessageInfo {
        name: "BarometerData",
        fields: fields,
    }
}

fn three_d_sensor_calibration_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "sensor_type",
        field_type: "sensor_type",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "calibration_factor",
        field_type: "uint32",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "calibration_divisor",
        field_type: "uint32",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "level_shift",
        field_type: "uint32",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "offset_cal",
        field_type: "sint32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "orientation_matrix",
        field_type: "sint32",
        def_number: 5,
        scale: 65535.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    MessageInfo {
        name: "ThreeDSensorCalibration",
        fields: fields,
    }
}

fn one_d_sensor_calibration_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "sensor_type",
        field_type: "sensor_type",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "calibration_factor",
        field_type: "uint32",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "calibration_divisor",
        field_type: "uint32",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "level_shift",
        field_type: "uint32",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "offset_cal",
        field_type: "sint32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    MessageInfo {
        name: "OneDSensorCalibration",
        fields: fields,
    }
}

fn video_frame_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "frame_number",
        field_type: "uint32",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    MessageInfo {
        name: "VideoFrame",
        fields: fields,
    }
}

fn obdii_data_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "time_offset",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "pid",
        field_type: "byte",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "raw_data",
        field_type: "byte",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "pid_data_size",
        field_type: "uint8",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "system_time",
        field_type: "uint32",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "start_timestamp",
        field_type: "date_time",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "start_timestamp_ms",
        field_type: "uint16",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(7, field);

    MessageInfo {
        name: "ObdiiData",
        fields: fields,
    }
}

fn nmea_sentence_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "sentence",
        field_type: "string",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    MessageInfo {
        name: "NmeaSentence",
        fields: fields,
    }
}

fn aviation_attitude_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "system_time",
        field_type: "uint32",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "pitch",
        field_type: "sint16",
        def_number: 2,
        scale: 10430.380000,
        offset: 0.000000,
        units: "radians",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "roll",
        field_type: "sint16",
        def_number: 3,
        scale: 10430.380000,
        offset: 0.000000,
        units: "radians",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "accel_lateral",
        field_type: "sint16",
        def_number: 4,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s^2",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "accel_normal",
        field_type: "sint16",
        def_number: 5,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s^2",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "turn_rate",
        field_type: "sint16",
        def_number: 6,
        scale: 1024.000000,
        offset: 0.000000,
        units: "radians/second",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "stage",
        field_type: "attitude_stage",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "attitude_stage_complete",
        field_type: "uint8",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "track",
        field_type: "uint16",
        def_number: 9,
        scale: 10430.380000,
        offset: 0.000000,
        units: "radians",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "validity",
        field_type: "attitude_validity",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    MessageInfo {
        name: "AviationAttitude",
        fields: fields,
    }
}

fn video_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "url",
        field_type: "string",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "hosting_provider",
        field_type: "string",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "duration",
        field_type: "uint32",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(2, field);

    MessageInfo {
        name: "Video",
        fields: fields,
    }
}

fn video_title_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "message_count",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "text",
        field_type: "string",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    MessageInfo {
        name: "VideoTitle",
        fields: fields,
    }
}

fn video_description_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "message_count",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "text",
        field_type: "string",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    MessageInfo {
        name: "VideoDescription",
        fields: fields,
    }
}

fn video_clip_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "clip_number",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "start_timestamp",
        field_type: "date_time",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "start_timestamp_ms",
        field_type: "uint16",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "end_timestamp",
        field_type: "date_time",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "end_timestamp_ms",
        field_type: "uint16",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "clip_start",
        field_type: "uint32",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "clip_end",
        field_type: "uint32",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
    };
    fields.insert(7, field);

    MessageInfo {
        name: "VideoClip",
        fields: fields,
    }
}

fn set_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "duration",
        field_type: "uint32",
        def_number: 0,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "repetitions",
        field_type: "uint16",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "weight",
        field_type: "uint16",
        def_number: 4,
        scale: 16.000000,
        offset: 0.000000,
        units: "kg",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "set_type",
        field_type: "set_type",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "start_time",
        field_type: "date_time",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "category",
        field_type: "exercise_category",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "category_subtype",
        field_type: "uint16",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "weight_display_unit",
        field_type: "fit_base_unit",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "wkt_step_index",
        field_type: "message_index",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(11, field);

    MessageInfo {
        name: "Set",
        fields: fields,
    }
}

fn jump_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "distance",
        field_type: "float32",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "height",
        field_type: "float32",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "rotations",
        field_type: "uint8",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "hang_time",
        field_type: "float32",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "score",
        field_type: "float32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "position_lat",
        field_type: "sint32",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "position_long",
        field_type: "sint32",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "speed",
        field_type: "uint16",
        def_number: 7,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "enhanced_speed",
        field_type: "uint32",
        def_number: 8,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(8, field);

    MessageInfo {
        name: "Jump",
        fields: fields,
    }
}

fn course_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "sport",
        field_type: "sport",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "capabilities",
        field_type: "course_capabilities",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "sub_sport",
        field_type: "sub_sport",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    MessageInfo {
        name: "Course",
        fields: fields,
    }
}

fn course_point_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "position_lat",
        field_type: "sint32",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "position_long",
        field_type: "sint32",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "distance",
        field_type: "uint32",
        def_number: 4,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "type",
        field_type: "course_point",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "favorite",
        field_type: "bool",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    MessageInfo {
        name: "CoursePoint",
        fields: fields,
    }
}

fn segment_id_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "uuid",
        field_type: "string",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "sport",
        field_type: "sport",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "enabled",
        field_type: "bool",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "user_profile_primary_key",
        field_type: "uint32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "device_id",
        field_type: "uint32",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "default_race_leader",
        field_type: "uint8",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "delete_status",
        field_type: "segment_delete_status",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "selection_type",
        field_type: "segment_selection_type",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    MessageInfo {
        name: "SegmentId",
        fields: fields,
    }
}

fn segment_leaderboard_entry_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "type",
        field_type: "segment_leaderboard_type",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "group_primary_key",
        field_type: "uint32",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "activity_id",
        field_type: "uint32",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "segment_time",
        field_type: "uint32",
        def_number: 4,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "activity_id_string",
        field_type: "string",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    MessageInfo {
        name: "SegmentLeaderboardEntry",
        fields: fields,
    }
}

fn segment_point_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "position_lat",
        field_type: "sint32",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "position_long",
        field_type: "sint32",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "distance",
        field_type: "uint32",
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "altitude",
        field_type: "uint16",
        def_number: 4,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "leader_time",
        field_type: "uint32",
        def_number: 5,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(5, field);

    MessageInfo {
        name: "SegmentPoint",
        fields: fields,
    }
}

fn segment_lap_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "event",
        field_type: "event",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "event_type",
        field_type: "event_type",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "start_time",
        field_type: "date_time",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "start_position_lat",
        field_type: "sint32",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "start_position_long",
        field_type: "sint32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "end_position_lat",
        field_type: "sint32",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "end_position_long",
        field_type: "sint32",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "total_elapsed_time",
        field_type: "uint32",
        def_number: 7,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "total_timer_time",
        field_type: "uint32",
        def_number: 8,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "total_distance",
        field_type: "uint32",
        def_number: 9,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "total_cycles",
        field_type: "uint32",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "cycles",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "total_calories",
        field_type: "uint16",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "total_fat_calories",
        field_type: "uint16",
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
    };
    fields.insert(12, field);

    let field = FieldInfo {
        name: "avg_speed",
        field_type: "uint16",
        def_number: 13,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(13, field);

    let field = FieldInfo {
        name: "max_speed",
        field_type: "uint16",
        def_number: 14,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(14, field);

    let field = FieldInfo {
        name: "avg_heart_rate",
        field_type: "uint8",
        def_number: 15,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(15, field);

    let field = FieldInfo {
        name: "max_heart_rate",
        field_type: "uint8",
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(16, field);

    let field = FieldInfo {
        name: "avg_cadence",
        field_type: "uint8",
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(17, field);

    let field = FieldInfo {
        name: "max_cadence",
        field_type: "uint8",
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(18, field);

    let field = FieldInfo {
        name: "avg_power",
        field_type: "uint16",
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(19, field);

    let field = FieldInfo {
        name: "max_power",
        field_type: "uint16",
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(20, field);

    let field = FieldInfo {
        name: "total_ascent",
        field_type: "uint16",
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(21, field);

    let field = FieldInfo {
        name: "total_descent",
        field_type: "uint16",
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(22, field);

    let field = FieldInfo {
        name: "sport",
        field_type: "sport",
        def_number: 23,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(23, field);

    let field = FieldInfo {
        name: "event_group",
        field_type: "uint8",
        def_number: 24,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(24, field);

    let field = FieldInfo {
        name: "nec_lat",
        field_type: "sint32",
        def_number: 25,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(25, field);

    let field = FieldInfo {
        name: "nec_long",
        field_type: "sint32",
        def_number: 26,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(26, field);

    let field = FieldInfo {
        name: "swc_lat",
        field_type: "sint32",
        def_number: 27,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(27, field);

    let field = FieldInfo {
        name: "swc_long",
        field_type: "sint32",
        def_number: 28,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(28, field);

    let field = FieldInfo {
        name: "name",
        field_type: "string",
        def_number: 29,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(29, field);

    let field = FieldInfo {
        name: "normalized_power",
        field_type: "uint16",
        def_number: 30,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(30, field);

    let field = FieldInfo {
        name: "left_right_balance",
        field_type: "left_right_balance_100",
        def_number: 31,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(31, field);

    let field = FieldInfo {
        name: "sub_sport",
        field_type: "sub_sport",
        def_number: 32,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(32, field);

    let field = FieldInfo {
        name: "total_work",
        field_type: "uint32",
        def_number: 33,
        scale: 1.000000,
        offset: 0.000000,
        units: "J",
    };
    fields.insert(33, field);

    let field = FieldInfo {
        name: "avg_altitude",
        field_type: "uint16",
        def_number: 34,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(34, field);

    let field = FieldInfo {
        name: "max_altitude",
        field_type: "uint16",
        def_number: 35,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(35, field);

    let field = FieldInfo {
        name: "gps_accuracy",
        field_type: "uint8",
        def_number: 36,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(36, field);

    let field = FieldInfo {
        name: "avg_grade",
        field_type: "sint16",
        def_number: 37,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(37, field);

    let field = FieldInfo {
        name: "avg_pos_grade",
        field_type: "sint16",
        def_number: 38,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(38, field);

    let field = FieldInfo {
        name: "avg_neg_grade",
        field_type: "sint16",
        def_number: 39,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(39, field);

    let field = FieldInfo {
        name: "max_pos_grade",
        field_type: "sint16",
        def_number: 40,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(40, field);

    let field = FieldInfo {
        name: "max_neg_grade",
        field_type: "sint16",
        def_number: 41,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(41, field);

    let field = FieldInfo {
        name: "avg_temperature",
        field_type: "sint8",
        def_number: 42,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(42, field);

    let field = FieldInfo {
        name: "max_temperature",
        field_type: "sint8",
        def_number: 43,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(43, field);

    let field = FieldInfo {
        name: "total_moving_time",
        field_type: "uint32",
        def_number: 44,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(44, field);

    let field = FieldInfo {
        name: "avg_pos_vertical_speed",
        field_type: "sint16",
        def_number: 45,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(45, field);

    let field = FieldInfo {
        name: "avg_neg_vertical_speed",
        field_type: "sint16",
        def_number: 46,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(46, field);

    let field = FieldInfo {
        name: "max_pos_vertical_speed",
        field_type: "sint16",
        def_number: 47,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(47, field);

    let field = FieldInfo {
        name: "max_neg_vertical_speed",
        field_type: "sint16",
        def_number: 48,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
    };
    fields.insert(48, field);

    let field = FieldInfo {
        name: "time_in_hr_zone",
        field_type: "uint32",
        def_number: 49,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(49, field);

    let field = FieldInfo {
        name: "time_in_speed_zone",
        field_type: "uint32",
        def_number: 50,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(50, field);

    let field = FieldInfo {
        name: "time_in_cadence_zone",
        field_type: "uint32",
        def_number: 51,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(51, field);

    let field = FieldInfo {
        name: "time_in_power_zone",
        field_type: "uint32",
        def_number: 52,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(52, field);

    let field = FieldInfo {
        name: "repetition_num",
        field_type: "uint16",
        def_number: 53,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(53, field);

    let field = FieldInfo {
        name: "min_altitude",
        field_type: "uint16",
        def_number: 54,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
    };
    fields.insert(54, field);

    let field = FieldInfo {
        name: "min_heart_rate",
        field_type: "uint8",
        def_number: 55,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(55, field);

    let field = FieldInfo {
        name: "active_time",
        field_type: "uint32",
        def_number: 56,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(56, field);

    let field = FieldInfo {
        name: "wkt_step_index",
        field_type: "message_index",
        def_number: 57,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(57, field);

    let field = FieldInfo {
        name: "sport_event",
        field_type: "sport_event",
        def_number: 58,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(58, field);

    let field = FieldInfo {
        name: "avg_left_torque_effectiveness",
        field_type: "uint8",
        def_number: 59,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(59, field);

    let field = FieldInfo {
        name: "avg_right_torque_effectiveness",
        field_type: "uint8",
        def_number: 60,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(60, field);

    let field = FieldInfo {
        name: "avg_left_pedal_smoothness",
        field_type: "uint8",
        def_number: 61,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(61, field);

    let field = FieldInfo {
        name: "avg_right_pedal_smoothness",
        field_type: "uint8",
        def_number: 62,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(62, field);

    let field = FieldInfo {
        name: "avg_combined_pedal_smoothness",
        field_type: "uint8",
        def_number: 63,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(63, field);

    let field = FieldInfo {
        name: "status",
        field_type: "segment_lap_status",
        def_number: 64,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(64, field);

    let field = FieldInfo {
        name: "uuid",
        field_type: "string",
        def_number: 65,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(65, field);

    let field = FieldInfo {
        name: "avg_fractional_cadence",
        field_type: "uint8",
        def_number: 66,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(66, field);

    let field = FieldInfo {
        name: "max_fractional_cadence",
        field_type: "uint8",
        def_number: 67,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(67, field);

    let field = FieldInfo {
        name: "total_fractional_cycles",
        field_type: "uint8",
        def_number: 68,
        scale: 128.000000,
        offset: 0.000000,
        units: "cycles",
    };
    fields.insert(68, field);

    let field = FieldInfo {
        name: "front_gear_shift_count",
        field_type: "uint16",
        def_number: 69,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(69, field);

    let field = FieldInfo {
        name: "rear_gear_shift_count",
        field_type: "uint16",
        def_number: 70,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(70, field);

    let field = FieldInfo {
        name: "time_standing",
        field_type: "uint32",
        def_number: 71,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(71, field);

    let field = FieldInfo {
        name: "stand_count",
        field_type: "uint16",
        def_number: 72,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(72, field);

    let field = FieldInfo {
        name: "avg_left_pco",
        field_type: "sint8",
        def_number: 73,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(73, field);

    let field = FieldInfo {
        name: "avg_right_pco",
        field_type: "sint8",
        def_number: 74,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
    };
    fields.insert(74, field);

    let field = FieldInfo {
        name: "avg_left_power_phase",
        field_type: "uint8",
        def_number: 75,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(75, field);

    let field = FieldInfo {
        name: "avg_left_power_phase_peak",
        field_type: "uint8",
        def_number: 76,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(76, field);

    let field = FieldInfo {
        name: "avg_right_power_phase",
        field_type: "uint8",
        def_number: 77,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(77, field);

    let field = FieldInfo {
        name: "avg_right_power_phase_peak",
        field_type: "uint8",
        def_number: 78,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
    };
    fields.insert(78, field);

    let field = FieldInfo {
        name: "avg_power_position",
        field_type: "uint16",
        def_number: 79,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(79, field);

    let field = FieldInfo {
        name: "max_power_position",
        field_type: "uint16",
        def_number: 80,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
    };
    fields.insert(80, field);

    let field = FieldInfo {
        name: "avg_cadence_position",
        field_type: "uint8",
        def_number: 81,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(81, field);

    let field = FieldInfo {
        name: "max_cadence_position",
        field_type: "uint8",
        def_number: 82,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
    };
    fields.insert(82, field);

    let field = FieldInfo {
        name: "manufacturer",
        field_type: "manufacturer",
        def_number: 83,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(83, field);

    let field = FieldInfo {
        name: "total_grit",
        field_type: "float32",
        def_number: 84,
        scale: 1.000000,
        offset: 0.000000,
        units: "kGrit",
    };
    fields.insert(84, field);

    let field = FieldInfo {
        name: "total_flow",
        field_type: "float32",
        def_number: 85,
        scale: 1.000000,
        offset: 0.000000,
        units: "Flow",
    };
    fields.insert(85, field);

    let field = FieldInfo {
        name: "avg_grit",
        field_type: "float32",
        def_number: 86,
        scale: 1.000000,
        offset: 0.000000,
        units: "kGrit",
    };
    fields.insert(86, field);

    let field = FieldInfo {
        name: "avg_flow",
        field_type: "float32",
        def_number: 87,
        scale: 1.000000,
        offset: 0.000000,
        units: "Flow",
    };
    fields.insert(87, field);

    MessageInfo {
        name: "SegmentLap",
        fields: fields,
    }
}

fn segment_file_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "file_uuid",
        field_type: "string",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "enabled",
        field_type: "bool",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "user_profile_primary_key",
        field_type: "uint32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "leader_type",
        field_type: "segment_leaderboard_type",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "leader_group_primary_key",
        field_type: "uint32",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "leader_activity_id",
        field_type: "uint32",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "leader_activity_id_string",
        field_type: "string",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "default_race_leader",
        field_type: "uint8",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(11, field);

    MessageInfo {
        name: "SegmentFile",
        fields: fields,
    }
}

fn workout_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "sport",
        field_type: "sport",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "capabilities",
        field_type: "workout_capabilities",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "num_valid_steps",
        field_type: "uint16",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "wkt_name",
        field_type: "string",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "sub_sport",
        field_type: "sub_sport",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "pool_length",
        field_type: "uint16",
        def_number: 14,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(14, field);

    let field = FieldInfo {
        name: "pool_length_unit",
        field_type: "display_measure",
        def_number: 15,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(15, field);

    MessageInfo {
        name: "Workout",
        fields: fields,
    }
}

fn workout_session_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "sport",
        field_type: "sport",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "sub_sport",
        field_type: "sub_sport",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "num_valid_steps",
        field_type: "uint16",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "first_step_index",
        field_type: "uint16",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "pool_length",
        field_type: "uint16",
        def_number: 4,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "pool_length_unit",
        field_type: "display_measure",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    MessageInfo {
        name: "WorkoutSession",
        fields: fields,
    }
}

fn workout_step_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "wkt_step_name",
        field_type: "string",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "duration_type",
        field_type: "wkt_step_duration",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "duration_value",
        field_type: "uint32",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "target_type",
        field_type: "wkt_step_target",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "target_value",
        field_type: "uint32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "custom_target_value_low",
        field_type: "uint32",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "custom_target_value_high",
        field_type: "uint32",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "intensity",
        field_type: "intensity",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "notes",
        field_type: "string",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "equipment",
        field_type: "workout_equipment",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "exercise_category",
        field_type: "exercise_category",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "exercise_name",
        field_type: "uint16",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "exercise_weight",
        field_type: "uint16",
        def_number: 12,
        scale: 100.000000,
        offset: 0.000000,
        units: "kg",
    };
    fields.insert(12, field);

    let field = FieldInfo {
        name: "weight_display_unit",
        field_type: "fit_base_unit",
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(13, field);

    MessageInfo {
        name: "WorkoutStep",
        fields: fields,
    }
}

fn exercise_title_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "exercise_category",
        field_type: "exercise_category",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "exercise_name",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "wkt_step_name",
        field_type: "string",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    MessageInfo {
        name: "ExerciseTitle",
        fields: fields,
    }
}

fn schedule_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "manufacturer",
        field_type: "manufacturer",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "product",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "serial_number",
        field_type: "uint32z",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "time_created",
        field_type: "date_time",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "completed",
        field_type: "bool",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "type",
        field_type: "schedule",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "scheduled_time",
        field_type: "local_date_time",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    MessageInfo {
        name: "Schedule",
        fields: fields,
    }
}

fn totals_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(254, field);

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "timer_time",
        field_type: "uint32",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "distance",
        field_type: "uint32",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "calories",
        field_type: "uint32",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "sport",
        field_type: "sport",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "elapsed_time",
        field_type: "uint32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "sessions",
        field_type: "uint16",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "active_time",
        field_type: "uint32",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "sport_index",
        field_type: "uint8",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(9, field);

    MessageInfo {
        name: "Totals",
        fields: fields,
    }
}

fn weight_scale_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "weight",
        field_type: "weight",
        def_number: 0,
        scale: 100.000000,
        offset: 0.000000,
        units: "kg",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "percent_fat",
        field_type: "uint16",
        def_number: 1,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "percent_hydration",
        field_type: "uint16",
        def_number: 2,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "visceral_fat_mass",
        field_type: "uint16",
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "kg",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "bone_mass",
        field_type: "uint16",
        def_number: 4,
        scale: 100.000000,
        offset: 0.000000,
        units: "kg",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "muscle_mass",
        field_type: "uint16",
        def_number: 5,
        scale: 100.000000,
        offset: 0.000000,
        units: "kg",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "basal_met",
        field_type: "uint16",
        def_number: 7,
        scale: 4.000000,
        offset: 0.000000,
        units: "kcal/day",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "physique_rating",
        field_type: "uint8",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "active_met",
        field_type: "uint16",
        def_number: 9,
        scale: 4.000000,
        offset: 0.000000,
        units: "kcal/day",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "metabolic_age",
        field_type: "uint8",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "years",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "visceral_fat_rating",
        field_type: "uint8",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "user_profile_index",
        field_type: "message_index",
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(12, field);

    MessageInfo {
        name: "WeightScale",
        fields: fields,
    }
}

fn blood_pressure_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "systolic_pressure",
        field_type: "uint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "mmHg",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "diastolic_pressure",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "mmHg",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "mean_arterial_pressure",
        field_type: "uint16",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "mmHg",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "map_3_sample_mean",
        field_type: "uint16",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "mmHg",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "map_morning_values",
        field_type: "uint16",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "mmHg",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "map_evening_values",
        field_type: "uint16",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "mmHg",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "heart_rate",
        field_type: "uint8",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "heart_rate_type",
        field_type: "hr_type",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "status",
        field_type: "bp_status",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "user_profile_index",
        field_type: "message_index",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(9, field);

    MessageInfo {
        name: "BloodPressure",
        fields: fields,
    }
}

fn monitoring_info_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "local_timestamp",
        field_type: "local_date_time",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "activity_type",
        field_type: "activity_type",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "cycles_to_distance",
        field_type: "uint16",
        def_number: 3,
        scale: 5000.000000,
        offset: 0.000000,
        units: "m/cycle",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "cycles_to_calories",
        field_type: "uint16",
        def_number: 4,
        scale: 5000.000000,
        offset: 0.000000,
        units: "kcal/cycle",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "resting_metabolic_rate",
        field_type: "uint16",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal / day",
    };
    fields.insert(5, field);

    MessageInfo {
        name: "MonitoringInfo",
        fields: fields,
    }
}

fn monitoring_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "device_index",
        field_type: "device_index",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "calories",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "distance",
        field_type: "uint32",
        def_number: 2,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "cycles",
        field_type: "uint32",
        def_number: 3,
        scale: 2.000000,
        offset: 0.000000,
        units: "cycles",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "active_time",
        field_type: "uint32",
        def_number: 4,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "activity_type",
        field_type: "activity_type",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "activity_subtype",
        field_type: "activity_subtype",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "activity_level",
        field_type: "activity_level",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "distance_16",
        field_type: "uint16",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "100 * m",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "cycles_16",
        field_type: "uint16",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "2 * cycles (steps)",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "active_time_16",
        field_type: "uint16",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "local_timestamp",
        field_type: "local_date_time",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(11, field);

    let field = FieldInfo {
        name: "temperature",
        field_type: "sint16",
        def_number: 12,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(12, field);

    let field = FieldInfo {
        name: "temperature_min",
        field_type: "sint16",
        def_number: 14,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(14, field);

    let field = FieldInfo {
        name: "temperature_max",
        field_type: "sint16",
        def_number: 15,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
    };
    fields.insert(15, field);

    let field = FieldInfo {
        name: "activity_time",
        field_type: "uint16",
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "minutes",
    };
    fields.insert(16, field);

    let field = FieldInfo {
        name: "active_calories",
        field_type: "uint16",
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
    };
    fields.insert(19, field);

    let field = FieldInfo {
        name: "current_activity_type_intensity",
        field_type: "byte",
        def_number: 24,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(24, field);

    let field = FieldInfo {
        name: "timestamp_min_8",
        field_type: "uint8",
        def_number: 25,
        scale: 1.000000,
        offset: 0.000000,
        units: "min",
    };
    fields.insert(25, field);

    let field = FieldInfo {
        name: "timestamp_16",
        field_type: "uint16",
        def_number: 26,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(26, field);

    let field = FieldInfo {
        name: "heart_rate",
        field_type: "uint8",
        def_number: 27,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(27, field);

    let field = FieldInfo {
        name: "intensity",
        field_type: "uint8",
        def_number: 28,
        scale: 10.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(28, field);

    let field = FieldInfo {
        name: "duration_min",
        field_type: "uint16",
        def_number: 29,
        scale: 1.000000,
        offset: 0.000000,
        units: "min",
    };
    fields.insert(29, field);

    let field = FieldInfo {
        name: "duration",
        field_type: "uint32",
        def_number: 30,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(30, field);

    let field = FieldInfo {
        name: "ascent",
        field_type: "uint32",
        def_number: 31,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(31, field);

    let field = FieldInfo {
        name: "descent",
        field_type: "uint32",
        def_number: 32,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(32, field);

    let field = FieldInfo {
        name: "moderate_activity_minutes",
        field_type: "uint16",
        def_number: 33,
        scale: 1.000000,
        offset: 0.000000,
        units: "minutes",
    };
    fields.insert(33, field);

    let field = FieldInfo {
        name: "vigorous_activity_minutes",
        field_type: "uint16",
        def_number: 34,
        scale: 1.000000,
        offset: 0.000000,
        units: "minutes",
    };
    fields.insert(34, field);

    MessageInfo {
        name: "Monitoring",
        fields: fields,
    }
}

fn hr_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "fractional_timestamp",
        field_type: "uint16",
        def_number: 0,
        scale: 32768.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "time256",
        field_type: "uint8",
        def_number: 1,
        scale: 256.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "filtered_bpm",
        field_type: "uint8",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "event_timestamp",
        field_type: "uint32",
        def_number: 9,
        scale: 1024.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "event_timestamp_12",
        field_type: "byte",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(10, field);

    MessageInfo {
        name: "Hr",
        fields: fields,
    }
}

fn stress_level_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "stress_level_value",
        field_type: "sint16",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "stress_level_time",
        field_type: "date_time",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(1, field);

    MessageInfo {
        name: "StressLevel",
        fields: fields,
    }
}

fn memo_glob_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "part_index",
        field_type: "uint32",
        def_number: 250,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(250, field);

    let field = FieldInfo {
        name: "memo",
        field_type: "byte",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "message_number",
        field_type: "uint16",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "message_index",
        field_type: "message_index",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    MessageInfo {
        name: "MemoGlob",
        fields: fields,
    }
}

fn ant_channel_id_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "channel_number",
        field_type: "uint8",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "device_type",
        field_type: "uint8z",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "device_number",
        field_type: "uint16z",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "transmission_type",
        field_type: "uint8z",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "device_index",
        field_type: "device_index",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    MessageInfo {
        name: "AntChannelId",
        fields: fields,
    }
}

fn ant_rx_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "fractional_timestamp",
        field_type: "uint16",
        def_number: 0,
        scale: 32768.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "mesg_id",
        field_type: "byte",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "mesg_data",
        field_type: "byte",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "channel_number",
        field_type: "uint8",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "data",
        field_type: "byte",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    MessageInfo {
        name: "AntRx",
        fields: fields,
    }
}

fn ant_tx_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "fractional_timestamp",
        field_type: "uint16",
        def_number: 0,
        scale: 32768.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "mesg_id",
        field_type: "byte",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "mesg_data",
        field_type: "byte",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "channel_number",
        field_type: "uint8",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "data",
        field_type: "byte",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    MessageInfo {
        name: "AntTx",
        fields: fields,
    }
}

fn exd_screen_configuration_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "screen_index",
        field_type: "uint8",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "field_count",
        field_type: "uint8",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "layout",
        field_type: "exd_layout",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "screen_enabled",
        field_type: "bool",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    MessageInfo {
        name: "ExdScreenConfiguration",
        fields: fields,
    }
}

fn exd_data_field_configuration_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "screen_index",
        field_type: "uint8",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "concept_field",
        field_type: "byte",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "field_id",
        field_type: "uint8",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "concept_count",
        field_type: "uint8",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "display_type",
        field_type: "exd_display_type",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "title",
        field_type: "string",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    MessageInfo {
        name: "ExdDataFieldConfiguration",
        fields: fields,
    }
}

fn exd_data_concept_configuration_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "screen_index",
        field_type: "uint8",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "concept_field",
        field_type: "byte",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "field_id",
        field_type: "uint8",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "concept_index",
        field_type: "uint8",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "data_page",
        field_type: "uint8",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "concept_key",
        field_type: "uint8",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "scaling",
        field_type: "uint8",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "data_units",
        field_type: "exd_data_units",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "qualifier",
        field_type: "exd_qualifiers",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "descriptor",
        field_type: "exd_descriptors",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "is_signed",
        field_type: "bool",
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(11, field);

    MessageInfo {
        name: "ExdDataConceptConfiguration",
        fields: fields,
    }
}

fn field_description_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "developer_data_index",
        field_type: "uint8",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "field_definition_number",
        field_type: "uint8",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "fit_base_type_id",
        field_type: "fit_base_type",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "field_name",
        field_type: "string",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "array",
        field_type: "uint8",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "components",
        field_type: "string",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "scale",
        field_type: "uint8",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "offset",
        field_type: "sint8",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "units",
        field_type: "string",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "bits",
        field_type: "string",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "accumulate",
        field_type: "string",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "fit_base_unit_id",
        field_type: "fit_base_unit",
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(13, field);

    let field = FieldInfo {
        name: "native_mesg_num",
        field_type: "mesg_num",
        def_number: 14,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(14, field);

    let field = FieldInfo {
        name: "native_field_num",
        field_type: "uint8",
        def_number: 15,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(15, field);

    MessageInfo {
        name: "FieldDescription",
        fields: fields,
    }
}

fn developer_data_id_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "developer_id",
        field_type: "byte",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "application_id",
        field_type: "byte",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "manufacturer_id",
        field_type: "manufacturer",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "developer_data_index",
        field_type: "uint8",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "application_version",
        field_type: "uint32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    MessageInfo {
        name: "DeveloperDataId",
        fields: fields,
    }
}

fn dive_summary_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "reference_mesg",
        field_type: "mesg_num",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "reference_index",
        field_type: "message_index",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "avg_depth",
        field_type: "uint32",
        def_number: 2,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "max_depth",
        field_type: "uint32",
        def_number: 3,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "surface_interval",
        field_type: "uint32",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "start_cns",
        field_type: "uint8",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(5, field);

    let field = FieldInfo {
        name: "end_cns",
        field_type: "uint8",
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(6, field);

    let field = FieldInfo {
        name: "start_n2",
        field_type: "uint16",
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(7, field);

    let field = FieldInfo {
        name: "end_n2",
        field_type: "uint16",
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
    };
    fields.insert(8, field);

    let field = FieldInfo {
        name: "o2_toxicity",
        field_type: "uint16",
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "OTUs",
    };
    fields.insert(9, field);

    let field = FieldInfo {
        name: "dive_number",
        field_type: "uint32",
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(10, field);

    let field = FieldInfo {
        name: "bottom_time",
        field_type: "uint32",
        def_number: 11,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(11, field);

    MessageInfo {
        name: "DiveSummary",
        fields: fields,
    }
}

fn climb_pro_message() -> MessageInfo {
    let mut fields = HashMap::new();

    let field = FieldInfo {
        name: "timestamp",
        field_type: "date_time",
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
    };
    fields.insert(253, field);

    let field = FieldInfo {
        name: "position_lat",
        field_type: "sint32",
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(0, field);

    let field = FieldInfo {
        name: "position_long",
        field_type: "sint32",
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
    };
    fields.insert(1, field);

    let field = FieldInfo {
        name: "climb_pro_event",
        field_type: "climb_pro_event",
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(2, field);

    let field = FieldInfo {
        name: "climb_number",
        field_type: "uint16",
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(3, field);

    let field = FieldInfo {
        name: "climb_category",
        field_type: "uint8",
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
    };
    fields.insert(4, field);

    let field = FieldInfo {
        name: "current_dist",
        field_type: "float32",
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
    };
    fields.insert(5, field);

    MessageInfo {
        name: "ClimbPro",
        fields: fields,
    }
}

impl MesgNum {
    pub fn message_info(&self) -> Option<MessageInfo> {
        match self {
            MesgNum::AccelerometerData => Some(accelerometer_data_message()),
            MesgNum::Activity => Some(activity_message()),
            MesgNum::AntChannelId => Some(ant_channel_id_message()),
            MesgNum::AntRx => Some(ant_rx_message()),
            MesgNum::AntTx => Some(ant_tx_message()),
            MesgNum::AviationAttitude => Some(aviation_attitude_message()),
            MesgNum::BarometerData => Some(barometer_data_message()),
            MesgNum::BikeProfile => Some(bike_profile_message()),
            MesgNum::BloodPressure => Some(blood_pressure_message()),
            MesgNum::CadenceZone => Some(cadence_zone_message()),
            MesgNum::CameraEvent => Some(camera_event_message()),
            MesgNum::Capabilities => Some(capabilities_message()),
            MesgNum::ClimbPro => Some(climb_pro_message()),
            MesgNum::Connectivity => Some(connectivity_message()),
            MesgNum::Course => Some(course_message()),
            MesgNum::CoursePoint => Some(course_point_message()),
            MesgNum::DeveloperDataId => Some(developer_data_id_message()),
            MesgNum::DeviceInfo => Some(device_info_message()),
            MesgNum::DeviceSettings => Some(device_settings_message()),
            MesgNum::DiveAlarm => Some(dive_alarm_message()),
            MesgNum::DiveGas => Some(dive_gas_message()),
            MesgNum::DiveSettings => Some(dive_settings_message()),
            MesgNum::DiveSummary => Some(dive_summary_message()),
            MesgNum::Event => Some(event_message()),
            MesgNum::ExdDataConceptConfiguration => Some(exd_data_concept_configuration_message()),
            MesgNum::ExdDataFieldConfiguration => Some(exd_data_field_configuration_message()),
            MesgNum::ExdScreenConfiguration => Some(exd_screen_configuration_message()),
            MesgNum::ExerciseTitle => Some(exercise_title_message()),
            MesgNum::FieldCapabilities => Some(field_capabilities_message()),
            MesgNum::FieldDescription => Some(field_description_message()),
            MesgNum::FileCapabilities => Some(file_capabilities_message()),
            MesgNum::FileCreator => Some(file_creator_message()),
            MesgNum::FileId => Some(file_id_message()),
            MesgNum::Goal => Some(goal_message()),
            MesgNum::GpsMetadata => Some(gps_metadata_message()),
            MesgNum::GyroscopeData => Some(gyroscope_data_message()),
            MesgNum::Hr => Some(hr_message()),
            MesgNum::HrZone => Some(hr_zone_message()),
            MesgNum::HrmProfile => Some(hrm_profile_message()),
            MesgNum::Hrv => Some(hrv_message()),
            MesgNum::Jump => Some(jump_message()),
            MesgNum::Lap => Some(lap_message()),
            MesgNum::Length => Some(length_message()),
            MesgNum::MagnetometerData => Some(magnetometer_data_message()),
            MesgNum::MemoGlob => Some(memo_glob_message()),
            MesgNum::MesgCapabilities => Some(mesg_capabilities_message()),
            MesgNum::MetZone => Some(met_zone_message()),
            MesgNum::Monitoring => Some(monitoring_message()),
            MesgNum::MonitoringInfo => Some(monitoring_info_message()),
            MesgNum::NmeaSentence => Some(nmea_sentence_message()),
            MesgNum::ObdiiData => Some(obdii_data_message()),
            MesgNum::OhrSettings => Some(ohr_settings_message()),
            MesgNum::OneDSensorCalibration => Some(one_d_sensor_calibration_message()),
            MesgNum::PowerZone => Some(power_zone_message()),
            MesgNum::Record => Some(record_message()),
            MesgNum::Schedule => Some(schedule_message()),
            MesgNum::SdmProfile => Some(sdm_profile_message()),
            MesgNum::SegmentFile => Some(segment_file_message()),
            MesgNum::SegmentId => Some(segment_id_message()),
            MesgNum::SegmentLap => Some(segment_lap_message()),
            MesgNum::SegmentLeaderboardEntry => Some(segment_leaderboard_entry_message()),
            MesgNum::SegmentPoint => Some(segment_point_message()),
            MesgNum::Session => Some(session_message()),
            MesgNum::Set => Some(set_message()),
            MesgNum::SlaveDevice => Some(slave_device_message()),
            MesgNum::Software => Some(software_message()),
            MesgNum::SpeedZone => Some(speed_zone_message()),
            MesgNum::Sport => Some(sport_message()),
            MesgNum::StressLevel => Some(stress_level_message()),
            MesgNum::ThreeDSensorCalibration => Some(three_d_sensor_calibration_message()),
            MesgNum::TimestampCorrelation => Some(timestamp_correlation_message()),
            MesgNum::Totals => Some(totals_message()),
            MesgNum::TrainingFile => Some(training_file_message()),
            MesgNum::UserProfile => Some(user_profile_message()),
            MesgNum::Video => Some(video_message()),
            MesgNum::VideoClip => Some(video_clip_message()),
            MesgNum::VideoDescription => Some(video_description_message()),
            MesgNum::VideoFrame => Some(video_frame_message()),
            MesgNum::VideoTitle => Some(video_title_message()),
            MesgNum::WatchfaceSettings => Some(watchface_settings_message()),
            MesgNum::WeatherAlert => Some(weather_alert_message()),
            MesgNum::WeatherConditions => Some(weather_conditions_message()),
            MesgNum::WeightScale => Some(weight_scale_message()),
            MesgNum::Workout => Some(workout_message()),
            MesgNum::WorkoutSession => Some(workout_session_message()),
            MesgNum::WorkoutStep => Some(workout_step_message()),
            MesgNum::ZonesTarget => Some(zones_target_message()),
            _ => None,
        }
    }
}