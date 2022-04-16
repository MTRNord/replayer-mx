@0x9e7f64b0577ea649;

using Util = import "/utils.capnp";

struct MessageContent {
    union {
        text: group {
            body @0: Text;
            format @1: Util.Option(Text);
            formattedBody @2: Util.Option(Text);
        }
        emote: group {
            body @3: Text;
            format @4: Util.Option(Text);
            formattedBody @5: Util.Option(Text);
        }
        notice: group {
            body @6: Text;
            format @7: Util.Option(Text);
            formattedBody @8: Util.Option(Text);
        }
        audio: group {
            body @9: Text;
            info @10: AudioInfo;
            url @11: Text;
        }
        file: group {
            body @12: Text;
            filename @13: Text;
            info @14: FileInfo;
            url @15: Text;
        }
        image: group {
            body @16: Text;
            info @17: ImageInfo;
            file @18: Util.Option(Text);
            url @19: Text;
        }
        location: group {
            body @20: Text;
            geoUri @21: Text;
            info @22: Util.Option(LocationInfo);
        }
        video: group {
            body @23: Text;
            info @24: VideoInfo;
            url @25: Text;
        }
        serverNotice : group {
            adminContact @26: Text;
            body @27: VideoInfo;
            limitType @28: LimitType;
            serverNoticeType @29: ServerNoticeType;
        }
    }

    enum ServerNoticeType {
        usageLimitExceeded @0;
    }

    enum LimitType {
        montlyActiveUser @0;
    }

    struct AudioInfo {
        duration @0: UInt64;
        mimeType @1: Text;
        size @2: UInt64;
    }

    struct FileInfo {
        mimeType @0: Text;
        size @1: UInt64;
    }

    struct ImageInfo {
        mimeType @0: Text;
        size @1: UInt64;
        h @2: UInt64;
        w @3: UInt64;
    }

    struct LocationInfo {
        thumbnailInfo @0: ImageInfo;
        thumbnailUrl @1: Text;
        thumbnailFile @2: Util.Option(Text);
    }

    struct VideoInfo {
        duration @0: UInt64;
        h @1: UInt64;
        w @2: UInt64;
        size @3: UInt64;
        mimeType @4: Text;
        thumbnailInfo @5: ImageInfo;
        thumbnailUrl @6: Text;
    }
}

enum MsgType {
    text @0;
    emote @1;
    notice @2;
    audio @3;
    file @4;
    image @5;
    location @6;
    video @7;
}