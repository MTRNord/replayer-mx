@0xeab570afa44bcb9d;

using Util = import "/utils.capnp";
using Events = import "/events.capnp";

struct MessageEvent {
    content @0: Events.MessageContent;
    roomId @1: Text;
    stateKey @2: Util.Option(Text);
    sender @3: Text;
    eventId @4: Text;
    originServerTs @5: UInt64;
    unsigned @6: Unsigned;
}

struct Unsigned(Content) {
    age @0: UInt64;
    transactionId @1: Text;
    prevContent @2: Util.Option(Content);
    redactedBecause @3: Util.Option(Text);
}