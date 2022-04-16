@0xeab570afa44bcb9d;

struct MessageEvent {
    content: union {
        text: group {
            body @0: Text;
            # Format is optional
            format @1: Text;
            formattedBody @2: Text;
        }
        emote: group {
            body @3: Text;
            # Format is optional
            format @4: Text;
            formattedBody @5: Text;
        }
        notice: group {
            body @6: Text;
            # Format is optional
            format @7: Text;
            formattedBody @8: Text;
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
            url @18: Text;
            # File only exists if e2ee encrypted
            file @19: Text;
        }
        location: group {
            body @20: Text;
            geoUri @21: Text;
            # Info can be null
            info @22: LocationInfo;
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
    roomId @30: Text;
    sender @31: Text;
    eventId @32: Text;
    originServerTs @33: UInt64;
    unsigned @34: Unsigned;

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
        # Only exists on E2EE encrypted messages
        thumbnailFile @2: Text;
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

struct Unsigned(Content) {
    age @0: UInt64;
    transactionId @1: Text;
    prevContent @2: Content;
    redactedBecause @3: Text;
}